#![forbid(unsafe_code)]
//#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod tests;
mod tnp;

#[cfg(feature = "jellyfin")]
pub use tnp::tnp as Jellyfin;
#[cfg(feature = "plex")]
pub use tnp::tnp as Plex;

pub trait MediaData {
    fn series_name(&self) -> String;
    fn capitalize_title(&self) -> String;
    fn year_as_string(&self) -> String;
    //fn country(&self) -> String;
    fn season_as_string(&self) -> String;
    fn episode_as_string(&self) -> String;
    //    #[cfg(any(feature = "tnp", feature = "plex", feature = "jellyfin"))]
    //    fn is_mutli_episodes(&self) -> bool;
    //    #[cfg(any(feature = "tnp", feature = "plex", feature = "jellyfin"))]
    //    fn first_episode(&self) -> String;
    //    #[cfg(any(feature = "tnp", feature = "plex", feature = "jellyfin"))]
    //    fn last_episode(&self) -> String;
    fn full_file_name(&self) -> String;
}
