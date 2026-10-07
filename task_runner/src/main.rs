use std::env;
use std::fs;
use serde_yml::from_str;

pub mod settings;
pub mod execution;


////////////////////////////////////////////////////////////////////////////////
fn read_config(path: &str) -> Vec<execution::Task> {

            
    let yaml_content = fs::read_to_string(path).expect(format!("read_to_string for path {} should complete successfully", path).as_str());

    let result: execution::Config = from_str(&yaml_content).expect("serde from_str should complete successfully");

    result.tasks
}


////////////////////////////////////////////////////////////////////////////////
fn main() {
    let mut settings = settings::Settings {
        step_names: false,
        config_file_name: String::from("D:\\Files\\work\\deploy-tasks.yaml"),
        from_step: String::from(""),
        only_step: String::from(""),
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if let Err(err) = settings::parse_command_line(&args, &mut settings) {
            println!("{}", err);
            return;
        }
    }

    println!("settings:");
    println!("{:?}", settings);

    let mut tasks = read_config(&settings.config_file_name);

    if settings.step_names {
        tasks.iter().for_each(|task| println!("{}", task.name));
        return;
    }

    let mut tasks_iterator = tasks.iter_mut();

    let task_name_to_find = 
        if settings.only_step.len() > 0 { &settings.only_step }
        else if settings.from_step.len() > 0 { &settings.from_step }
        else { &String::from("") };

    let mut result_status = true;
    if task_name_to_find.len() > 0 {
        let found_task = tasks_iterator.find(|task| task.name == *task_name_to_find)
            .expect(format!("Task {} not found in config", task_name_to_find).as_str());
        result_status = execution::process_command(found_task); 
    }

    if !result_status || settings.only_step.len() > 0 {
        return;
    }

    for task in tasks_iterator {
        result_status = execution::process_command(task);
        if !result_status {
            break;
        }
    }

    if result_status {
        println!("[I] finished successfully");
    }
    else {
        println!("[E] Finished with errors");
    }
}
