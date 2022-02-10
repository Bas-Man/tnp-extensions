use custom_tv_show_namer::MediaData;
#[cfg(feature = "tnp")]
use torrent_name_parser::Metadata;

struct MyData(torrent_name_parser::Metadata);

impl MediaData for MyData {
    fn series_name(&self) -> String {
        self.0.title().to_string()
    }
    fn capitalize_title(&self) -> String {
        unimplemented!("not implemented")
    }
    fn show_year(&self) -> String {
        unimplemented!("not implemented")
    }
    fn season_to_string(&self) -> String {
        unimplemented!("not implemented")
    }
    fn episode_to_string(&self) -> String {
        unimplemented!("not implemented")
    }
    fn full_file_name(&self) -> String {
        unimplemented!("not implemented")
    }
}
fn main() {
    let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    let data = MyData(m);
    println!("{}", data.series_name());
}
