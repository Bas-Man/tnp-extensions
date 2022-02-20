use tnp_extensions::MediaData;
use torrent_name_parser::Metadata;

fn main() {
    let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    assert_eq!(m.capitalize_title(), "Narcos");
    assert_eq!(m.full_file_name(), "narcos.S01E10.1080p.BLURAY");
    println!("{}", m.full_file_name());
}
