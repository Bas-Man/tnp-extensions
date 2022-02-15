#[cfg(any(feature = "plex", feature = "jellyfin"))]
/// Implementation of of MediaData suitable for the Plex Media Server.
pub mod tnp {
    use crate::MediaData;
    #[cfg(any(feature = "jellyfin", feature = "plex"))]
    use titlecase;
    use torrent_name_parser;
    /// MediaData Implementation for torrent_name_parser
    #[cfg_attr(docsrs, doc(cfg(feature = "plex")))]
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
        fn year_as_string(&self) -> String {
            if let Some(numeric_year) = self.year() {
                return std::format!("{}", numeric_year);
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
                name.push_str(&self.episode_as_string());
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
                if cfg!(feature = "plex") {
                    name.push_str(&std::format!(" {{imdb-tt{}}}", imdb_tag));
                } else if cfg!(feature = "jellyfin") {
                    name.push_str(&std::format!(" [imdbid-{}]", imdb_tag));
                }
            }
            name.replace(' ', ".")
        }
        fn season_as_string(&self) -> String {
            let mut season_string = String::new();
            if let Some(numeric_season) = self.season() {
                season_string.push_str(&std::format!("S{:02}", numeric_season));
            };
            season_string
        }
        //        fn is_mutli_episodes(&self) -> bool {
        //            self.episodes.len() > 1
        //       }
        //      fn first_episode(&self) -> String {
        //            match self.episodes.is_empty() {
        //               false => Some(&self.episodes[0]),
        //                true => None,
        //          }
        //        }
        //       fn last_episode(&self) -> String {
        //          self.episodes.last()
        //     }
        fn episode_as_string(&self) -> String {
            let mut episode_string = String::new();
            if let Some(numeric_episode) = self.episode() {
                episode_string.push_str(&std::format!("E{:02}", numeric_episode));
            };
            episode_string
        }
    }
}
