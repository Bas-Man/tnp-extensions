use custom_tv_show_namer::MediaData;
#[cfg(feature = "plex")]
use torrent_name_parser::Metadata;

fn main() {
    println!("hi");
    let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    assert_eq!(m.capitalize_title(), "Narcos");
    assert_eq!(m.full_file_name(), "Narcos.S01E10.1080p.BLURAY");
    println!("{}", m.full_file_name());
    println!("{}", m.season_to_string());
    println!("{}", m.episode_to_string());
}
