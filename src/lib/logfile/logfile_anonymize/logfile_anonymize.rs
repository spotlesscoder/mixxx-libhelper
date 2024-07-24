use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod logfile_anonymize {

    pub fn anonymize_logfile(path: &str, args: Vec<String>) {
        let file_path = std::path::Path::new(&file_path);
        let file_as_string = std::fs::read_to_string(path)?;
    }

    fn anonymize(file_contents: &str) -> &str {
        // serial number macOS
        let result = replace(file_contents, "S/N: [A-Z0-9]+", "S/N: [HIDDEN]");
        // replace device IDs
        let result = replace(file_contents, "", "");

        result
    }

    fn replace(file_contents: &str, regex_str: &str, replacement: &str) -> &str {
        let regex = Regex::new(&regex_str).unwrap();

        let reader = BufReader::new(file_contents);

        let mut result = String::new();
        for line in reader.lines() {
            let mut line = line?;

            line = re.replace_all(&line, &replacement).to_string();

            result.push_str(&line);
            result.push('\n');
        }

        &result
    }

    fn replace_device_ids(input_str: &str) -> io::Result<String> {
        // Define the regex pattern to find device IDs
        let regex = Regex::new(r"\{ (\w{4}:\w{4}) r(\d+) ").unwrap();

        // Define a map to store original device IDs and their new random IDs
        let mut id_map: HashMap<String, String> = HashMap::new();

        // String to accumulate the processed lines
        let mut result = String::new();

        let reader = BufReader::new(input_str);

        // Process each line
        for line in reader.lines() {
            let mut line = line?;
            // Replace the device IDs with new random IDs
            line = regex
                .replace_all(&line, |caps: &regex::Captures| {
                    let device_id = &caps[1];
                    let replacement_id = id_map
                        .entry(device_id.to_string())
                        .or_insert_with(random_device_id);
                    format!("{{ {} r{}", replacement_id, &caps[2])
                })
                .to_string();
            // Append the modified line to the result string
            result.push_str(&line);
            result.push('\n');
        }

        Ok(result)
    }

    fn random_device_id() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::random_device_id;

        #[test]
        fn random_device_id_works() {
            let result = random_device_id();

            assert_eq!(result.len(), 8);
        }

        #[test]
        fn replace_works() {
            let input = "";
        }
    }
}
