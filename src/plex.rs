#[cfg(feature = "plex")]
pub mod plex {
    use crate::MediaData;
    use titlecase;
    use torrent_name_parser;

    impl MediaData for torrent_name_parser::Metadata {
        fn series_name(&self) -> String {
            let mut series_name = String::new();
            series_name.push_str(&self.capitalize_title());
            if let Some(series_year) = self.year() {
                series_name.push_str(&std::format!(" ({})", series_year.to_string()));
            }
            series_name
        }
        fn capitalize_title(&self) -> String {
            titlecase::titlecase(&self.title())
        }
        fn show_year(&self) -> String {
            if let Some(numeric_year) = self.year() {
                return numeric_year.to_string();
            };
            "".to_string()
        }
        fn full_file_name(&self) -> String {
            let mut name = String::new();
            name.push_str(&self.capitalize_title());
            if let Some(numeric_year) = self.year() {
                name.push_str(&std::format!(" ({})", numeric_year));
            }
            if let Some(season) = self.season() {
                name.push_str(&std::format!(" S{:02}", season));
            }
            if let Some(_) = self.episode() {
                name.push_str(&self.episode_to_string());
            }
            if let Some(resolution) = self.resolution() {
                name.push_str(&std::format!(" {}", resolution));
            }
            if let Some(quality) = self.quality() {
                name.push_str(&std::format!(
                    " {}",
                    quality.to_string().to_ascii_uppercase()
                ));
            }
            if let Some(imdb_tag) = self.imdb_tag() {
                name.push_str(&std::format!(" {{imdb-tt{}}}", imdb_tag));
            }
            name.replace(' ', ".")
        }
        fn season_to_string(&self) -> String {
            let mut season_string = String::new();
            if let Some(numeric_season) = self.season() {
                season_string.push_str(&std::format!("S{:02}", numeric_season));
            };
            season_string
        }

        fn episode_to_string(&self) -> String {
            let mut episode_string = String::new();
            if let Some(numeric_episode) = self.episode() {
                episode_string.push_str(&std::format!("E{:02}", numeric_episode));
            };
            episode_string
        }
    }
}