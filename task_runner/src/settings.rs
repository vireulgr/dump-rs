
#[derive(Debug)]
pub struct Settings {
    pub step_names: bool,
    pub config_file_name: String,
    pub from_step: String,
    pub only_step: String,
}

pub fn parse_command_line(args: &Vec<String>, settings: &mut Settings) -> Result<(), &'static str> {
    let mut idx: usize = 1;
    while idx < args.len() {
        if args[idx] == "--listSteps" || args[idx] == "-l" {
            settings.step_names = true;
        }
        else if args[idx] == "--config" || args[idx] == "-c" {
            idx += 1;
            if idx < args.len() {
                settings.config_file_name = String::from(&args[idx]);
            }
            else {
                return Err("Argument for option --config is not found");
            }
        }
        else if args[idx] == "--onlyStep" || args[idx] == "-o" {
            if settings.from_step.len() > 0 {
                return Err("Options --onlyStep and --fromStep are mutual exclusive");
            }
            idx += 1;
            if idx < args.len() {
                settings.only_step = String::from(&args[idx]);
            }
            else {
                return Err("Argument for option --onlyStep is not found");
            }
        }
        else if args[idx] == "--fromStep" || args[idx] == "-f" {
            if settings.only_step.len() > 0 {
                return Err("Options --onlyStep and --fromStep are mutual exclusive");
            }
            idx += 1;
            if idx < args.len() {
                settings.from_step = String::from(&args[idx]);
            }
            else {
                return Err("Argument for option --fromStep is not found");
            }
        }
        else {
            return Err("Possible arguments are: --listSteps(-l), --config(-c) config, --onlyStep(-o) step, --fromStep(-f) step");
        }
        idx += 1;
    } 

    Ok(())
}

