pub mod genre {

     fn is_trance(genre_string: &str) -> bool {
        return genre_string.to_lowercase().contains("trance");
    }

    fn is_dance(genre_string: &str) -> bool {
        return genre_string.to_lowercase().contains("dance");
    }

    pub fn is_edm(genre_string: &str) -> bool {
        return  is_dance(&genre_string) || is_trance(&genre_string);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn trance_is_edm() {
            assert_eq!(is_edm("Trance (Main Floor)"), true);
        }

        #[test]
        fn detects_main_floor_trance() {
            assert_eq!(is_trance("Trance (Main Floor"), true);
            assert_eq!(is_trance("Trance"), true);
        }

        #[test]
        fn detects_main_floor_dance() {
            assert_eq!(is_dance("Dance"), true);
            assert_eq!(is_dance("Hard dance"), true);
        }
    }
}
