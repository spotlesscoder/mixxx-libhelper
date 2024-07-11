mod track;
mod track_categorization;
mod track_movement;

pub mod mixxxdb {
    use std::io::{stdin, stdout, Write};

    use id3::Tag;
    use std::path::Path;
    
    use crate::track::Track;

    use crate::track_categorization::genre::genre::is_edm;

    pub fn fix_edm_bpm(mixxx_db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let edm_tracks_low_bpm = find_edm_tracks_with_low_bpm(&mixxx_db_path)?;
        let track_locations: Vec<String> = edm_tracks_low_bpm
            .iter()
            .cloned()
            .map(|track| track.location)
            .collect();

        println!("Will convert BPM to 3/2*BPM for the following tracks - continue? y/n");
        for location in track_locations {
            println!("{location}");
        }

        let mut confirmation: String = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut confirmation)
            .expect("Did not read a string");
        if confirmation.eq("y\n") {
            multiply_bpm(&edm_tracks_low_bpm, 3.0 / 2.0, &mixxx_db_path)?;
        }

        Ok(())
    }

    fn multiply_bpm(
        tracks: &Vec<Track>,
        multiplier: f64,
        db_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let connection = get_connection(&db_path);

        let mut stmt = connection.prepare(
            "UPDATE library
            SET bpm = ?1
            WHERE id = ?1",
        )?;

        for track in tracks {
            let new_bpm = track.bpm as f64 * multiplier;
            let new_bpm = (new_bpm * 100.0).round() / 100.0;
            stmt.execute((&new_bpm, &track.id))?;
        }

        Ok(())
    }

    fn find_edm_tracks_with_low_bpm(
        mixxx_db_path: &str,
    ) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
        let tracks_with_bpm_below_100 = find_tracks_with_bpm_below(100, &mixxx_db_path)?;
        let tracks_with_id3_formats = filter_to_id3_supported_formats(&tracks_with_bpm_below_100);
        let id3tracks = get_id3_infos_for_tracks(&tracks_with_id3_formats)?;
        Ok(filter_to_edm_tracks(
            &tracks_with_bpm_below_100,
            &id3tracks,
        ))
    }

    fn get_id3_infos_for_tracks(tracks: &Vec<Track>) -> Result<Vec<Track>, id3::Error> {
        let mut result = vec![];
        for track in tracks {
            if !Path::new(&track.location).exists() {
                continue;
            }
            let tag = Tag::read_from_path(&track.location)?;
            let mut enriched_track = track.clone();
            enriched_track.id3 = Some(tag);
            result.push(enriched_track);
        }

        return Ok(result);
    }

    fn is_format(track: &Track, formats: &Vec<&str>) -> bool {
        let location_lowercase = track.location.to_lowercase();
        let formats_lowercase: Vec<String> = formats
            .iter()
            .cloned()
            .map(|format| format.to_lowercase())
            .collect();

        for format in formats_lowercase {
            if location_lowercase.ends_with(&format) {
                return true;
            }
        }

        return false;
    }

    fn filter_to_id3_supported_formats(tracks: &Vec<Track>) -> Vec<Track> {
        return tracks
            .iter()
            .cloned()
            .filter(|track| is_format(track, &vec!["mp3", "mp4", "wav", "aiff"]))
            .collect();
    }

    fn filter_to_edm_tracks(tracks: &Vec<Track>, tracks_with_id3: &Vec<Track>) -> Vec<Track> {
        let tracks_identified_from_db: Vec<Track> = tracks
            .iter()
            .cloned()
            .filter(|track| is_edm(&track.genre))
            .collect();

        let mut result = tracks_identified_from_db;

        let tracks_identified_from_id3: Vec<Track> = tracks_with_id3
            .iter()
            .cloned()
            .filter(|track| {
                track.id3.is_some()
                    && track.id3.as_ref().unwrap().genre().is_some()
                    && is_edm(track.id3.as_ref().unwrap().genre().as_ref().unwrap())
            })
            .collect();

        for track in tracks_identified_from_id3 {
            let tracks_found_by_location: Vec<Track> = result
                .iter()
                .cloned()
                .filter(|existing| existing.location.eq(&track.location))
                .collect();
            if tracks_found_by_location.len() == 0 {
                result.push(track);
            }
        }

        result
    }

    fn find_tracks_with_bpm_below(
        bpm: u8,
        mixxx_db_path: &str,
    ) -> Result<Vec<Track>, rusqlite::Error> {
        let connection = get_connection(&mixxx_db_path);

        let mut stmt = connection.prepare(
            "SELECT l.id, l.bpm, l.genre, tl.location FROM library l
             INNER JOIN track_locations tl
             ON tl.id = l.location
             WHERE l.bpm IS NOT NULL
             AND l.bpm > ?1;",
        )?;

        let rows = stmt.query_map([bpm], |row| {
            Ok(Track {
                id: row.get(0)?,
                bpm: row.get(1)?,
                genre: row.get(2)?,
                location: row.get(3)?,
                id3: None,
            })
        })?;

        let mut tracks = Vec::new();
        for row in rows {
            tracks.push(row.unwrap());
        }

        Ok(tracks)
    }

    fn get_connection(db_path: &str) -> rusqlite::Connection {
        return rusqlite::Connection::open(db_path).expect("Could not open db path");
    }

    #[cfg(test)]
    mod tests {
        use std::{fs, path};

        use rusqlite::Connection;

        use crate::mixxxdb::filter_to_edm_tracks;

        use super::{filter_to_id3_supported_formats, fix_edm_bpm, Track};

        #[test]
        fn full_integration_test() -> Result<(), Box<dyn std::error::Error>> {
            // setup
            if path::Path::is_file(path::Path::new("db.db")) {
                fs::remove_file("db.db")?;
            }
            let db_name = "db.db";
            let connection = rusqlite::Connection::open(&db_name)?;
            setup_test_db(&connection)?;

            // run
            fix_edm_bpm(&db_name)?;

            // verify
            let mut stmt = connection.prepare(
                "SELECT l.id, l.bpm, l.genre from library l
                 WHERE l.bpm IS NOT NULL;
                 ",
            )?;

            let rows = stmt.query_map((), |row| {
                Ok(Track {
                    id: row.get(0)?,
                    bpm: row.get(1)?,
                    genre: row.get(2)?,
                    location: row.get(3)?,
                    id3: None,
                })
            })?;

            let mut tracks = Vec::new();
            for row in rows {
                tracks.push(row.unwrap());
            }

            assert_eq!(tracks.len(), 2);

            let edm_track = tracks.iter().cloned().find(|track| track.id == 1).unwrap();
            assert_eq!(edm_track.bpm, 138.0);

            let unknown_genre_track = tracks.iter().cloned().find(|track| track.id == 2).unwrap();
            assert_eq!(unknown_genre_track.bpm, 93.3);

            // teardown
            std::fs::remove_file("db.db")?;
            Ok(())
        }

        #[test]
        fn filter_to_edm_tracks_leaves_out_whitespace_genre() {
            // setup
            let tracks = vec![Track {
                id: 123,
                bpm: 123.0,
                genre: String::from(" "),
                location: String::from(""),
                id3: None,
            }];

            // run
            let result = filter_to_edm_tracks(&tracks, &vec![]);

            // verify
            assert_eq!(result.len(), 0);
        }

        #[test]
        fn filter_to_id3_supported_formats_leaves_out_mov() {
            // setup
            let tracks = vec![Track {
                id: 123,
                bpm: 123.0,
                genre: String::from(" "),
                location: String::from(".mov"),
                id3: None,
            }];

            // run
            let result = filter_to_id3_supported_formats(&tracks);

            // verify
            assert_eq!(result.len(), 0);
        }

        #[test]
        fn filter_to_id3_supported_formats_includes_all_valid_formats() {
            // setup
            let base_track = Track {
                id: 123,
                bpm: 123.0,
                genre: String::from(""),
                location: String::from(""),
                id3: None,
            };

            let tracks = vec![
                Track {
                    location: String::from("mp3"),
                    ..base_track.clone()
                },
                Track {
                    location: String::from("mp4"),
                    ..base_track.clone()
                },
                Track {
                    location: String::from("wav"),
                    ..base_track.clone()
                },
                Track {
                    location: String::from("aiff"),
                    ..base_track
                },
            ];

            // run
            let result = filter_to_id3_supported_formats(&tracks);

            // verify
            assert_eq!(result.len(), 4);
        }

        fn setup_test_db(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
            let stmt = r#"
            CREATE TABLE track_locations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                location VARCHAR(512) UNIQUE
            )
            "#;
            connection.execute(&stmt, ())?;

            let stmt = r#"
            CREATE TABLE library (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                genre VARCHAR(64),
                bpm FLOAT,
                location INTEGER REFERENCES track_locations(location)
            )
            "#;
            connection.execute(&stmt, ())?;

            let stmt = r#"
            INSERT INTO track_locations (
                id,
                location
            )
            VALUES (
                1,
                "/sample"
            )
            "#;
            connection.execute(&stmt, ())?;

            let stmt = r#"
            INSERT INTO track_locations (
                id,
                location
            )
            VALUES (
                2,
                "/sample2"
            )
            "#;
            connection.execute(&stmt, ())?;

            let stmt = r#"
            INSERT INTO library (
                id,
                genre,
                bpm,
                location
            )
            VALUES (
                1,
                "edm",
                93.3,
                1
            )
            "#;
            connection.execute(&stmt, ())?;

            let stmt = r#"
            INSERT INTO library (
                id,
                genre,
                bpm,
                location
            )
            VALUES (
                2,
                "",
                93.3,
                2
            )
            "#;

            connection.execute(&stmt, ())?;

            Ok(())
        }
    }
}
