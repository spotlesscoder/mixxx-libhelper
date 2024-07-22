pub mod track_fingerprinting {

    use sha2::{Digest, Sha256};

    #[derive(PartialEq, Debug)]
    pub enum FingerprintAlgorithm {
        SHA265,
    }

    pub struct Fingerprint {
        pub fingerprint: String,
        pub fingerprint_algorithm: FingerprintAlgorithm,
    }

    pub fn get_track_fingerprints_for_file(
        file_path: &str,
    ) -> Result<Vec<Fingerprint>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();

        // get whole fs file fingerprint
        let file_path = std::path::Path::new(&file_path);
        let file_bytes = std::fs::read(&file_path)?;

        let mut hasher = Sha256::new();
        hasher.update(&file_bytes);
        let hash = hasher.finalize();

        result.push(Fingerprint {
            fingerprint: format!("{:x}", hash),
            fingerprint_algorithm: FingerprintAlgorithm::SHA265,
        });

        Ok(result)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_track_fingerprints_for_file_works() {
            let result = get_track_fingerprints_for_file(&"test-data/sample.dat").unwrap();
            assert_eq!(
                result[0].fingerprint_algorithm,
                FingerprintAlgorithm::SHA265
            );
            assert_eq!(
                result[0].fingerprint,
                "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
            );
        }
    }
}
