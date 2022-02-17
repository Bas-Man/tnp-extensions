#![forbid(unsafe_code)]
//#![warn(missing_docs)]
//#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(all(feature = "plex", feature = "jellyfin"))]
compile_error!(
    "Feature 'plex' and 'jellyfin' are mutually exclusive and cannot be enabled together"
);
mod tests;
mod tnp;

#[cfg(any(feature = "plex", feature = "jellyfin"))]
pub use tnp::tnp as MediaServer;

pub trait MediaData {
    /// Returns the Series Name
    fn series_directory_name(&self) -> String;
    fn series_directory_name_with_imdb_tag(&self) -> String;
    //fn series_name_with_imdb_tt(&self) -> String;
    /// Returns the Title with capitalization.
    fn capitalize_title(&self) -> String;
    fn unmangled_title(&self) -> Option<&str> {
        None
    }
    //    fn is_mutli_episodes(&self) -> bool;
    //    fn first_episode(&self) -> String;
    //    fn last_episode(&self) -> String;
    fn full_file_name(&self) -> String;
}
