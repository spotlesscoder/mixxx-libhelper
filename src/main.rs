use core::panic;
use mixxx_libhelper::mixxx_db;
use mixxx_libhelper::mixxx_logfile;
use std::env;

const COMMAND_DB: &str = "db";
const COMMAND_LOGFILE: &str = "logfile_anonymize";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &get_command(&args);

    if command == COMMAND_DB {
        let db_path = get_db_path(&args);

        // TODO detect whether mixxx is still running and ask to close first
        mixxx_db::fix_edm_bpm(db_path)?;
    }
    if command == COMMAND_LOGFILE {
        let logfile_path = get_logfile_path(&args);

        let logfile_anonymized = mixxx_logfile::anonymize_logfile(&logfile_path);
        let target_filename = String::from(format!("{}.anonymized", logfile_path));
        let target_path = std::path::Path::new(&target_filename);
        std::fs::write(target_path, logfile_anonymized?)?;
    }

    Ok(())
}

fn get_command(args: &[String]) -> String {
    if args.len() < 2 {
        panic!("Nee")
    }
    let mut valid_commands = vec![COMMAND_DB.to_string(), COMMAND_LOGFILE.to_string()];
    valid_commands.sort();

    let command = &args[2];
    if !valid_commands.contains(&command) {
        println!("Invalid command: {command}");
        println!();
        println!("Valid commands are");
        for valid_command in valid_commands {
            println!("{valid_command}");
        }
        panic!()
    }

    command.to_string()
}

fn get_logfile_path(args: &[String]) -> &str {
    if args.len() < 2 {
        panic!("No log file path provided");
    }

    return &args[1];
}

fn get_db_path(args: &[String]) -> &str {
    if args.len() < 2 {
        return "~/mixxxdb.sqlite";
    }

    return &args[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_db_path_defaults_to_mixxxdb_in_home() {
        // setup
        let vec: Vec<String> = vec![String::from("test")];

        // run
        let db_path = get_db_path(&vec);

        // verify
        assert_eq!(db_path, "~/mixxxdb.sqlite");
    }

    #[test]
    fn get_db_path_gets_path_from_args() {
        // setup
        let vec: Vec<String> = vec![String::from("test"), String::from("db.sqlite")];

        // run
        let db_path = get_db_path(&vec);

        // verify
        assert_eq!(db_path, "db.sqlite");
    }
}
