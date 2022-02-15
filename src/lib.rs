#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! A simple crate to provide some convenience methods for the
//![torrent_name_parser](https://crates.io/crates/torrent-name-parser) crate.
//!```rust
//! use torrent_name_parser::Metadata;
//! use tnp_extensions::TNPExtensions;
//!
//! let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
//! println!("first Ep:{}", m.first_episode().unwrap());
//! println!("last Epi:{}", m.last_episode().unwrap());
//! println!("is_multi: {}", m.is_mutli_episodes());
//!
//! let m = Metadata::from("narcos.s01e10e11.1080p.bluray.x264-rovers.srt").unwrap();
//! println!("first Ep:{}", m.first_episode().unwrap());
//! println!("last Epi:{}", m.last_episode().unwrap());
//! println!("is_multi: {}", m.is_mutli_episodes());
//! println!("is_subtitle: {}", m.is_subtitle());
//!```
#[cfg(test)]
mod tests;

/// A trait containing convenience methods
pub trait TNPExtensions {
    /// Determine if the file is subtitle based on file extension
    fn is_subtitle(&self) -> bool;
    /// Determine if the torrent name contains multiple episodes.
    fn is_mutli_episodes(&self) -> bool;
    /// Provide access to the first episode number
    fn first_episode(&self) -> Option<&i32>;
    /// Provide access to the last episode number
    fn last_episode(&self) -> Option<&i32>;
}
impl TNPExtensions for torrent_name_parser::Metadata {
    fn is_subtitle(&self) -> bool {
        match self.extension() {
            Some(ext) => match ext.to_ascii_lowercase().as_str() {
                "srt" | "ssa" | "ttml" | "sbv" | "dfxp" | "vtt" => true,
                _ => false,
            },
            None => false,
        }
    }
    fn is_mutli_episodes(&self) -> bool {
        self.episodes().len() > 1
    }
    fn first_episode(&self) -> Option<&i32> {
        if !self.episodes().is_empty() {
            return Some(&self.episodes()[0]);
        };
        None
    }
    fn last_episode(&self) -> Option<&i32> {
        self.episodes().last()
    }
}
