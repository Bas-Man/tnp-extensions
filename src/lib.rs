#![forbid(unsafe_code)]
//#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod plex;
mod tests;

#[cfg(feature = "plex")]
pub use plex::plex as Plex;

pub trait MediaData {
    fn series_name(&self) -> String;
    fn capitalize_title(&self) -> String;
    fn year_as_string(&self) -> String;
    //fn country(&self) -> String;
    fn season_as_string(&self) -> String;
    fn episode_as_string(&self) -> String;
    fn full_file_name(&self) -> String;
}
