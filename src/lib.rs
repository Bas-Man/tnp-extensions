#![forbid(unsafe_code)]
//#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod tests;
mod tnp;

#[cfg(any(feature = "plex", feature = "jellyfin"))]
pub use tnp::tnp as MediaServer;

pub trait MediaData {
    /// Returns the Series Name
    fn series_name(&self) -> String;
    /// Returns the Title with capitalization.
    fn capitalize_title(&self) -> String;
    /// Returns the Year as a String
    fn year_as_string(&self) -> String;
    //fn country(&self) -> String;
    fn season_as_string(&self) -> String;
    fn episode_as_string(&self) -> String;
    //    fn is_mutli_episodes(&self) -> bool;
    //    fn first_episode(&self) -> String;
    //    fn last_episode(&self) -> String;
    fn full_file_name(&self) -> String;
}
