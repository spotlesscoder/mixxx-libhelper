pub mod logfile_anonymize {

    use std::collections::HashMap;

    use rand::{distributions::Alphanumeric, Rng};
    use regex::Regex;

    pub fn anonymize(file_contents: &str) -> Result<String, Box<dyn std::error::Error>> {
        // serial number macOS
        let result = replace(file_contents, "S/N: [A-Z0-9]+", "S/N: [HIDDEN]")?;
        // replace device IDs
        let result = replace_device_ids(&result)?;

        Ok(result)
    }

    fn replace(
        file_contents: &str,
        regex_str: &str,
        device_serial_replacement: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let device_serial_regex = Regex::new(&regex_str).unwrap();

        let mut result = String::new();
        for line in file_contents.lines() {
            let replaced = device_serial_regex
                .replace_all(&line, device_serial_replacement)
                .to_string();

            result.push_str(&replaced);
        }

        Ok(result)
    }

    fn replace_device_ids(input_str: &str) -> Result<String, Box<dyn std::error::Error>> {
        let device_serial_regex = Regex::new(r"\{ (\w{4}:\w{4}) r(\d+) ").unwrap();

        // Define a map to store original device IDs and their new random IDs
        let mut id_map: HashMap<String, String> = HashMap::new();

        let mut result = String::new();

        for line in input_str.lines() {
            // Replace the device IDs with new random IDs
            let device_serial_replacement = &device_serial_regex
                .replace_all(&line, |caps: &regex::Captures| {
                    let device_id = &caps[1];
                    let replacement_id = id_map
                        .entry(device_id.to_string())
                        .or_insert_with(random_device_id);
                    format!("{{ {} r{}", replacement_id, &caps[2])
                })
                .to_string();

            result.push_str(&device_serial_replacement);
            result.push('\n');
        }

        Ok(result)
    }

    fn random_device_id() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::random_device_id;
        use super::*;

        #[test]
        fn random_device_id_works() {
            let result = random_device_id();

            assert_eq!(result.len(), 8);
        }

        const DEVICE_SERIAL_REGEX: &str = "S/N: [A-Z0-9+]+";
        const DEVICE_SERIAL_REPLACEMENT: &str =
            "S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper]";

        #[test]
        fn replace_works_serial_one_line() {
            let input  = "Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:000b | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }";

            let result = replace(input, DEVICE_SERIAL_REGEX, DEVICE_SERIAL_REPLACEMENT).unwrap();

            assert_eq!(result, "Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:000b | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }");
        }

        #[test]
        fn replace_works_serial_multi_line() {
            let input = "Debug [Controller]    Nodes detected: 0
Debug [Controller] Scanning USB Bulk devices:
Info [Controller] Scanning USB HID devices
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:000b | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:0003 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Info [Controller] Found HID device: { 0000:0000 r0 | Usage: ff00:00ff | Manufacturer: Apple }
Info [Controller] Found HID device: { 0000:0000 r0 | Usage: ff00:0004 | Manufacturer: Apple }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:000d | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Info [Controller] Found HID device: { 5e6f:7a8b r0 | Usage: ff00:0048 | Manufacturer: APPL | Product: BTM }
Info [Controller] Excluding HID device { 5e6f:7a8b r0 | Usage: 000d:0004 | Manufacturer: Apple }
Info [Controller] Excluding HID device { 9c0d:8e1f r0 | Usage: 0001:0006 | Product: TouchBarUserDevice }
Info [Controller] Excluding HID device { 1a2b:3c4d r0 | Usage: ff00:000f | Product: Keyboard Backlight }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: 0001:0006 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Info [Controller] Found HID device: { a3b4:c5d6 r0 | Usage: 000c:0001 | Manufacturer: Apple | Product: Headset }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: 0001:0002 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Info [Controller] Duplicate HID device, excluding { 1a2b:3c4d r123 | Usage: 0001:0001 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Info [Controller] Duplicate HID device, excluding { 1a2b:3c4d r123 | Usage: 000d:0005 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Info [Controller] Duplicate HID device, excluding { 1a2b:3c4d r123 | Usage: ff00:000c | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: AB123456CD7890EF+KFL }
Debug [Controller] ControllerManager::getControllerList";

            let result = replace(input, DEVICE_SERIAL_REGEX, DEVICE_SERIAL_REPLACEMENT).unwrap();

            assert_eq!("Debug [Controller]    Nodes detected: 0
Debug [Controller] Scanning USB Bulk devices:
Info [Controller] Scanning USB HID devices
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:000b | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:0003 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Info [Controller] Found HID device: { 0000:0000 r0 | Usage: ff00:00ff | Manufacturer: Apple }
Info [Controller] Found HID device: { 0000:0000 r0 | Usage: ff00:0004 | Manufacturer: Apple }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: ff00:000d | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Info [Controller] Found HID device: { 5e6f:7a8b r0 | Usage: ff00:0048 | Manufacturer: APPL | Product: BTM }
Info [Controller] Excluding HID device { 5e6f:7a8b r0 | Usage: 000d:0004 | Manufacturer: Apple }
Info [Controller] Excluding HID device { 9c0d:8e1f r0 | Usage: 0001:0006 | Product: TouchBarUserDevice }
Info [Controller] Excluding HID device { 1a2b:3c4d r0 | Usage: ff00:000f | Product: Keyboard Backlight }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: 0001:0006 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Info [Controller] Found HID device: { a3b4:c5d6 r0 | Usage: 000c:0001 | Manufacturer: Apple | Product: Headset }
Info [Controller] Excluding HID device { 1a2b:3c4d r123 | Usage: 0001:0002 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Info [Controller] Duplicate HID device, excluding { 1a2b:3c4d r123 | Usage: 0001:0001 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Info [Controller] Duplicate HID device, excluding { 1a2b:3c4d r123 | Usage: 000d:0005 | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Info [Controller] Duplicate HID device, excluding { 1a2b:3c4d r123 | Usage: ff00:000c | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }
Debug [Controller] ControllerManager::getControllerList".to_string(), result);
        }

        #[test]
        fn replace_device_ids_works_single_line() {
            let input_start = "Info [Controller] Excluding HID device { ";
            let input_device_id = "1a2b:3c4d";
            let input_middle = " r123 | Usage: ";
            let input_device_usage_id = "ff00:000b";
            let input_end = " | Manufacturer: Apple Inc. | Product: Apple Internal Keyboard / Trackpad | S/N: [HIDDEN by github.com/spotlesscoder/mixxx-libhelper] }";
            let input = input_start.to_string()
                + input_device_id
                + input_middle
                + input_device_usage_id
                + input_end;

            let result = replace_device_ids(&input).unwrap();

            assert!(result.starts_with(input_start));
            assert!(result.ends_with(input_end));

            let replaced_part = &result[input_start.len()..result.len() - input_end.len()];
            let replaced_parts: Vec<&str> = replaced_part.split(" r123 | Usage: ").collect();

            assert_eq!(replaced_parts.len(), 2);
            assert!(is_valid_device_id(replaced_parts[0]));
            assert!(is_valid_device_id(replaced_parts[1]));
        }

        fn is_valid_device_id(id: &str) -> bool {
            let pattern = r"^[0-9a-f]{4}:[0-9a-f]{4}$";
            regex::Regex::new(pattern).unwrap().is_match(id)
        }
    }
}
