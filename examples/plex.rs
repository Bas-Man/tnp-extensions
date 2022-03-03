use tnp_extensions::TNPExtensions;
use torrent_name_parser::Metadata;

fn main() {
    let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    println!("{}", m.first_episode().unwrap());
    println!("{}", m.last_episode().unwrap());
    println!("{}", m.is_mutli_episodes());
}
