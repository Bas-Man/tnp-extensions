use crate::TNPExtensions;

#[test]
fn name() {
    let m = torrent_name_parser::Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    assert_eq!(m.is_subtitle(), false);
    assert_eq!(m.first_episode(), Some(&10));
    assert_eq!(m.is_mutli_episodes(), false);

    let m =
        torrent_name_parser::Metadata::from("The Flash 2014 S01E04 HDTV x264-FUM[ettv]").unwrap();
    assert_eq!(m.is_subtitle(), false);

    let m = torrent_name_parser::Metadata::from(
        "A Shaun the Sheep Movie - Farmageddon (2019) [h265 Remux-1080p] [tt6193408]",
    )
    .unwrap();
    assert_eq!(m.is_subtitle(), false);
}

#[test]
fn doctor_who() {
    let m = torrent_name_parser::Metadata::from("doctor.who.(2013).S01e1.avi").unwrap();
    assert_eq!(m.is_subtitle(), false);
    let m = torrent_name_parser::Metadata::from("doctor.who.(2013).S01e1.srt").unwrap();
    assert_eq!(m.is_subtitle(), true);
}
#[test]
fn swat() {
    let m = torrent_name_parser::Metadata::from("S.W.A.T.S01e1e02.avi").unwrap();
    assert_eq!(m.is_subtitle(), false);
    assert_eq!(m.is_mutli_episodes(), true);
    assert_eq!(m.first_episode(), Some(&1));
    assert_eq!(m.last_episode(), Some(&2));
    let mut i = 0;
    for episode in m.episodes() {
        i = i + 1;
        assert_eq!(episode, &i)
    }
}
