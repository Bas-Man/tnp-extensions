pub mod plex {
    use crate::CustomNamer;
    //use std::borrow::Cow;
    use titlecase;
    use torrent_name_parser;

    impl CustomNamer for torrent_name_parser::Metadata {
        fn series_name(&self) -> String {
            let mut series_name = String::new();
            series_name.push_str(&self.show_name());
            if let Some(series_year) = self.year() {
                series_name.push_str(&std::format!(" ({})", series_year.to_string()));
            }
            series_name
        }
        fn show_name(&self) -> String {
            titlecase::titlecase(&self.title().to_string().to_string())
        }
        fn show_year(&self) -> String {
            if let Some(numeric_year) = self.year() {
                return numeric_year.to_string();
            };
            "".to_string()
        }
        fn full_file_name(&self) -> String {
            let mut name = String::new();
            name.push_str(&self.show_name().to_string());
            if let Some(numeric_year) = self.year() {
                name.push_str(&std::format!(" ({})", numeric_year.to_string()));
            }
            if let Some(season) = self.season() {
                name.push_str(&std::format!(" S{:02}", season).to_string());
            }
            if let Some(_) = self.episode() {
                name.push_str(&self.episode_to_string());
            }
            if let Some(resolution) = self.resolution() {
                name.push_str(&std::format!(" {}", resolution));
            }
            name.replace(' ', ".")
        }
        fn season_to_string(&self) -> String {
            let mut season_string = String::new();
            if let Some(numeric_season) = self.season() {
                season_string.push_str(&std::format!("S{:02}", numeric_season).to_string());
            };
            season_string
        }

        fn episode_to_string(&self) -> String {
            let mut episode_string = String::new();
            if let Some(numeric_episode) = self.episode() {
                episode_string.push_str(&std::format!("E{:02}", numeric_episode).to_string());
            };
            episode_string
        }
    }
}
