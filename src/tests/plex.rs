#[allow(unused_imports)]
use crate::MediaData;

#[test]
fn name() {
    let m = torrent_name_parser::Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    assert_eq!(m.series_name(), "Narcos");
    assert_eq!(m.capitalize_title(), "Narcos");
    assert_eq!(m.season_to_string(), "S01");
    assert_eq!(m.episode_to_string(), "E10");
    assert_eq!(m.show_year(), "");
    assert_eq!(m.quality(), Some("bluray"));
    assert_eq!(m.full_file_name(), "Narcos.S01E10.1080p.BLURAY");

    let m =
        torrent_name_parser::Metadata::from("The Flash 2014 S01E04 HDTV x264-FUM[ettv]").unwrap();
    assert_eq!(m.series_name(), "The Flash (2014)");
    assert_eq!(m.capitalize_title(), "The Flash");
    assert_eq!(m.season_to_string(), "S01");
    assert_eq!(m.episode_to_string(), "E04");
    assert_eq!(m.show_year(), "2014");
    assert_eq!(m.quality(), Some("HDTV"));
    assert_eq!(m.full_file_name(), "The.Flash.(2014).S01E04.HDTV");

    let m = torrent_name_parser::Metadata::from(
        "A Shaun the Sheep Movie - Farmageddon (2019) [h265 Remux-1080p] [tt6193408]",
    )
    .unwrap();
    assert_eq!(
        m.series_name(),
        "A Shaun the Sheep Movie Farmageddon (2019)"
    );
    assert_eq!(m.capitalize_title(), "A Shaun the Sheep Movie Farmageddon");
    assert_eq!(m.show_year(), "2019");
    assert_eq!(
        m.full_file_name(),
        "A.Shaun.the.Sheep.Movie.Farmageddon.(2019).1080p.{imdb-tttt6193408}"
    );
}
