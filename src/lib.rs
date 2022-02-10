pub mod plex;
mod tests;

pub trait MediaData {
    fn series_name(&self) -> String;
    fn capitalize_title(&self) -> String;
    fn show_year(&self) -> String;
    //fn country(&self) -> String;
    fn season_to_string(&self) -> String;
    fn episode_to_string(&self) -> String;
    fn full_file_name(&self) -> String;
}
