pub mod track {

    use id3::Tag;

    #[derive(Clone, Debug)]
    pub struct Track {
        pub id: usize,
        pub bpm: f64,
        pub genre: String,
        pub location: String,
        pub id3: Option<Tag>,
    }
}
