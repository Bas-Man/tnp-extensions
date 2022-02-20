#[cfg(any(feature = "plex", feature = "jellyfin"))]
/// Implementation of MediaData suitable for the Plex or Jellyfin Media Server.
pub mod tnp {
    use crate::MediaData;
    //#[cfg(any(feature = "jellyfin", feature = "plex"))]
    use titlecase;
    use torrent_name_parser;
    /// MediaData Implementation for torrent_name_parser
    //#[cfg_attr(docsrs, doc(cfg(any(feature = "plex", feature = "jellyfin"))))]
    impl MediaData for torrent_name_parser::Metadata {
        /// Test comment
        //   #[cfg_attr(docsrs, doc(cfg(any(feature = "plex", feature = "jellyfin"))))]
        fn series_directory_name(&self) -> String {
            let mut series_name = String::new();
            if let Some(unmangled_title) = self.unmangled_title() {
                series_name.push_str(unmangled_title)
            } else {
                series_name.push_str(&self.capitalize_title());
            }
            if let Some(series_year) = self.year() {
                series_name.push_str(&std::format!(" ({})", series_year));
            }
            series_name
        }
        fn series_directory_name_with_imdb_tag(&self) -> String {
            let mut series_name = self.series_directory_name();
            if let Some(imdb_tag) = self.imdb_tag() {
                if cfg!(feature = "plex") {
                    series_name.push_str(&std::format!(" {{imdb-{}}}", imdb_tag));
                } else if cfg!(feature = "jellyfin") {
                    series_name.push_str(&std::format!(" [imdbid-{}]", imdb_tag));
                }
            }
            series_name
        }
        fn capitalize_title(&self) -> String {
            titlecase::titlecase(&self.title())
        }
        fn full_file_name(&self) -> String {
            let mut name = String::new();
            name.push_str(&self.title().to_owned().to_ascii_lowercase());
            if let Some(numeric_year) = self.year() {
                name.push_str(&std::format!(" ({})", numeric_year));
            }
            if let Some(season) = self.season() {
                name.push_str(&std::format!(" S{:02}", season));
            }
            if let Some(episode) = self.episode() {
                name.push_str(&std::format!("E{:02}", episode));
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
            name.replace(' ', ".")
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
    }
}
