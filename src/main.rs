use core::panic;
use mixxx_libhelper::mixxxdb;
use std::env;

const COMMAND_DB: &str = "db";
const COMMAND_LOGFILE: &str = "logfile";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let command: &str = &get_command(&args);

    if command == COMMAND_DB {
        let db_path: &str = get_db_path(&args);

        // TODO detect whether mixxx is still running and ask to close first
        mixxxdb::fix_edm_bpm(db_path)?;
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
