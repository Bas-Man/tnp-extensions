#[allow(unused_imports)]
use crate::CustomNamer;

#[test]
fn name() {
    let m = torrent_name_parser::Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    assert_eq!(m.series_name(), "Narcos");
    assert_eq!(m.show_name(), "Narcos");
    assert_eq!(m.season_to_string(), "S01");
    assert_eq!(m.episode_to_string(), "E10");
    assert_eq!(m.show_year(), "");
    assert_eq!(m.full_file_name(), "Narcos.S01E10.1080p");

    let m =
        torrent_name_parser::Metadata::from("The Flash 2014 S01E04 HDTV x264-FUM[ettv]").unwrap();
    assert_eq!(m.series_name(), "The Flash (2014)");
    assert_eq!(m.show_name(), "The Flash");
    assert_eq!(m.season_to_string(), "S01");
    assert_eq!(m.episode_to_string(), "E04");
    assert_eq!(m.show_year(), "2014");
    assert_eq!(m.full_file_name(), "The.Flash.(2014).S01E04");
}
