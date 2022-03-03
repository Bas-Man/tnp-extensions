#![forbid(unsafe_code)]
//#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod tests;

pub trait TNPExtensions {
    fn is_tv_show(&self) -> bool;
    fn is_subtitle(&self) -> bool;
    fn is_mutli_episodes(&self) -> bool;
    fn first_episode(&self) -> Option<&i32>;
    fn last_episode(&self) -> Option<&i32>;
}

impl TNPExtensions for torrent_name_parser::Metadata {
    fn is_tv_show(&self) -> bool {
        self.season().is_some()
    }
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
