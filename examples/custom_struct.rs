use tnp_extensions::MediaData;
use torrent_name_parser::Metadata;

struct MyData(torrent_name_parser::Metadata);

impl MediaData for MyData {
    fn series_directory_name(&self) -> String {
        self.0.title().to_string()
    }
    fn series_directory_name_with_imdb_tag(&self) -> String {
        unimplemented!("not implemented")
    }
    fn capitalize_title(&self) -> String {
        unimplemented!("not implemented")
    }
    fn full_file_name(&self) -> String {
        unimplemented!("not implemented")
    }
}
fn main() {
    let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    let data = MyData(m);
    println!("{}", data.series_directory_name());
}
