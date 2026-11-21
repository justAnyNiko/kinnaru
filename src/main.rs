use lofty::config::ParseOptions;
use lofty::file::AudioFile;
use lofty::prelude::*;
use lofty::probe::Probe;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use scan_dir::ScanDir;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn print_file(name: &str) {
    let probe_file = Probe::open(name)
        .expect("Could not open file")
        .read()
        .expect("Could not read metadata");

    if let Some(tag) = probe_file.primary_tag() {
        let title = tag.title().unwrap_or("Unknown title".into());
        let artist = tag.artist().unwrap_or("Unknown artist".into());

        println!("Title: {title}");
        println!("Artist: {artist}");
    }
}

fn load_file(name: &str) -> Decoder<BufReader<File>> {
    let file = File::open(name).unwrap();
    let source = Decoder::try_from(BufReader::new(file)).unwrap();
    return source;
}

fn is_music_file<P: AsRef<Path>>(path: P) -> bool {
    Probe::open(path)
        .ok()
        .and_then(|p| p.guess_file_type().ok())
        .map(|p| p.options(ParseOptions::new().read_tags(false)))
        .and_then(|p| p.read().ok())
        .map(|f| !f.properties().duration().is_zero())
        .unwrap_or(false)
}

fn main() {
    let _handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Player::connect_new(&_handle.mixer());

    ScanDir::files()
        .read(".", |iter| {
            for (i, (_entry, name)) in iter.enumerate() {
                if is_music_file(&name) {
                    print!("{} : ", i);
                    print_file(&name);
                    let song = load_file(&name);
                    player.append(song);
                }
            }
        })
        .unwrap();

    player.sleep_until_end();
}
