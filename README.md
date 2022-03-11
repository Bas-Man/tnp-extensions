TNPExtension
============

[![dependency status](https://deps.rs/repo/github/Bas-Man/tnp-extensions/status.svg)](https://deps.rs/repo/github/Bas-Man/tnp-extensions)

A simple set of extensions to help with the `torrent_name_parser` crate.

```rust
use torrent_name_parser::Metadata;
use tnp_extensions::TNPExtensions;

let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
println!("first Ep:{}", m.first_episode().unwrap());
println!("last Epi:{}", m.last_episode().unwrap());
println!("is_multi: {}", m.is_mutli_episodes());

let m = Metadata::from("narcos.s01e10e11.1080p.bluray.x264-rovers.srt").unwrap();
println!("first Ep:{}", m.first_episode().unwrap());
println!("last Epi:{}", m.last_episode().unwrap());
println!("is_multi: {}", m.is_mutli_episodes());
println!("is_subtitle: {}", m.is_subtitle());
```
