use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let db_path = get_db_path(&args);

    // TODO detect whether mixxx is still running and ask to close first
    
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
