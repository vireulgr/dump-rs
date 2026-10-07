use std::env;
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::fs;

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
enum TaskType {
    Process,
    PSCommand,
    FileOperations
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
struct EnvConfig {
    #[serde(default)]
    path: Option<Vec<String>>,
    #[serde(default)]
    vars: Option<HashMap<String, String>>,
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    command_type: TaskType,
    working_dir: String,
    prog: String,
    args: Vec<String>,
    #[serde(default)]
    with_env: Option<EnvConfig>,
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tasks: Vec<Task>
}


////////////////////////////////////////////////////////////////////////////////
fn check_file_operation_argument(arg: &str) -> (bool, bool) {
    let is_exist = fs::exists(&arg).expect(format!("Exist for {} should complete successfully", arg).as_str());
    if !is_exist {
        return (false, false);
    }
    let meta = fs::metadata(&arg).expect(format!("Metadata for {} should complete successfully", arg).as_str());
    let is_directory = meta.is_dir();
    (is_exist, is_directory)
}



////////////////////////////////////////////////////////////////////////////////
fn walk_dir_recursively(root: &Path, cb: &dyn Fn(&PathBuf) -> bool) -> bool {
    let src_iter = fs::read_dir(root).expect(format!("Cannot read directory {}", root.to_str().unwrap_or_default()).as_str());
    for iter_result in src_iter {
        match iter_result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir_recursively(&path, &cb);
                }
                if !cb(&path) {
                    return false;
                }
            },
            Err(err) => {
                println!("Error parsing directory {}", err);
            }
        }
    }
    false
}

////////////////////////////////////////////////////////////////////////////////
pub fn exec_command(task: &mut Task) -> bool {
    let prev_dir = env::current_dir().expect("Cannot get directory");
    env::set_current_dir(Path::new(&task.working_dir)).expect(&format!("Cannot change directory to {}", task.working_dir).to_string());
    println!("cwd: {}", env::current_dir().expect("Cannot get directory").display());

    let mut a_command = Command::new(task.prog.clone());
    a_command.args(task.args.clone());

    if let Some(task_env) = &task.with_env {
        if let Some(add_to_path) = &task_env.path {
            let path_var = env::var_os("PATH").expect("Cannot get PATH");
            let mut paths_vec = env::split_paths(&path_var)
                .filter(|item| item.to_str().unwrap_or_default().len() > 0)
                .collect::<Vec<_>>(); 
            for a_path in add_to_path {
                let path_path = PathBuf::from(a_path);
                paths_vec.push(path_path);
            }

            let new_path = env::join_paths(paths_vec).expect("Join paths");
            a_command.env("PATH", new_path);
        }
        if let Some(add_to_env) = &task_env.vars {
            add_to_env.iter().for_each(|(name, value)| { a_command.env(name, value); });
        }
    }

    let args_str = a_command.get_args()
        .map(|an_arg| an_arg.to_str().unwrap_or_default())
        .fold(String::new(), |mut acc, item| { acc.push_str(item); acc.push(' '); return acc; });

    let command_name = format!("{} {}", a_command.get_program().to_str().unwrap_or_default(), args_str);
    println!("Executing\n{}", command_name);
    let result_status = match a_command.output() {
        Err(error) => {
            println!("Program failed to start: {}", error.to_string());
            false
        }
        Ok(output) => {
            io::stdout().write_all(&output.stdout).expect("write stdout result should be ok");
            io::stderr().write_all(&output.stderr).expect("write stderr result should be ok");
            println!("[I] Exit code: {}", output.status.code().unwrap_or(-9999));
            println!("[I] Exit success: {}", output.status.success());
            output.status.success()
        }
    };

    env::set_current_dir(prev_dir).expect("Cannot set previous dir");
    result_status
}

//////////////////////////////////////////////////////////////////////////////////
fn prepare_ps_command(task: &mut Task) -> () {
    let args_str = task.args.iter().fold(String::new(), |mut acc, arg| {acc.push_str(arg); acc.push(' '); return acc;});
    task.prog = String::from("pwsh");
    task.args = vec![
        String::from("-NonInteractive "),
        String::from("-Command "),
        format!("{} {}", task.prog, args_str),
    ];
}

////////////////////////////////////////////////////////////////////////////////
fn process_file_operations(task: &Task) -> bool {
    let prev_dir = env::current_dir().expect("Cannot get directory");
    env::set_current_dir(Path::new(&task.working_dir)).expect("Cannot change directory");
    println!("cwd: {}", env::current_dir().expect("Cannot get directory").display());

    let result_status: bool = match task.prog.as_str() {
        "create-or-cleanup-directory" => 'match_arm: {
            for item in task.args[..].iter() {
                let is_exist = fs::exists(&item).expect(format!("Exist for {} should complete successfully", item).as_str());
                if is_exist {
                    let meta = fs::metadata(&item).expect(format!("Metadata for {} should complete successfully", item).as_str());
                    let is_directory = meta.is_dir();
                    if !is_directory {
                        println!("Command argument for create-or-cleanup-directory {} should be a directory", item);
                        break 'match_arm false;
                    }

                    fs::remove_dir_all(&item).expect(format!("Remove directory {} should complete successfully", item).as_str());
                }
                fs::create_dir_all(&item).expect(format!("Create directory {} should complete successfully", item).as_str());
            }
            true
        },
        "force-recursive-copy-directory" => 'match_arm: {
            if task.args.len() != 2 {
                println!("Command {} should contain exactly 2 arguments", task.prog);
                break 'match_arm false;
            }
            let source = &task.args[0];
            let (src_exist, src_dir) = check_file_operation_argument(&source);
            let destination = &task.args[1];
            let (dest_exist, dest_dir) = check_file_operation_argument(&destination);
            if !(src_exist && src_dir && dest_exist && dest_dir) {
                println!("Source and destination should exist and be a directory ({}; {})", source, destination);
                break 'match_arm false;
            }

            println!("Copy from {} to {}", source.as_str(), destination.as_str());

            walk_dir_recursively(Path::new(source), &|file: &PathBuf| {
                let rel_file_path = file.strip_prefix(source).unwrap_or(Path::new("C:\\"));
                let path_in_destination = Path::new(destination).join(rel_file_path);
                match fs::exists(&path_in_destination) {
                    Err(err) => {
                        println!("Failed to check if path {} exists: {}", path_in_destination.to_str().unwrap_or_default(), err.to_string());
                    }
                    Ok(is_exist) => {
                        if is_exist {
                            // TODO backup
                            fs::remove_file(&path_in_destination).expect("Remove file should complete successfully");
                        }
                    }
                }
                fs::copy(file, &path_in_destination).expect("Copy file should complete successfully");
                true
            });
            true
        },
        _ => {
            println!("Prog value for task type file operations is not correct: {}", task.prog.as_str());
            false
        }
    };

    env::set_current_dir(prev_dir).expect("Cannot set previous dir");
    return result_status;
}

////////////////////////////////////////////////////////////////////////////////
pub fn process_command(task: &mut Task) -> bool {
    println!("Step {}", task.name);
    let result_status = match task.command_type {
        TaskType::PSCommand => {
            prepare_ps_command(task);
            exec_command(task)
        },
        TaskType::Process => {
            exec_command(task)
        },
        TaskType::FileOperations => {
            process_file_operations(task)
        }
    };
    result_status
}
