//! This example prints simple information on the streams contained
//! in a matroska file.

use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use av_data::params::MediaKind;
use av_format::buffer::AccReader;
use av_format::demuxer::Context;

use matroska::demuxer::MkvDemuxer;

#[derive(Parser, Debug)]
#[clap(
    name = "streams-info",
    version,
    author,
    about = "Gets information on audio and video streams"
)]
struct Opts {
    /// Sets the matroska file to analyze
    #[clap(long, short, value_parser)]
    input: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    // Open the matroska file
    let reader = File::open(opts.input).unwrap();

    // Create a buffer of size 4096 bytes to contain matroska data
    let ar = AccReader::with_capacity(4 * 1024, reader);

    // Set the type of demuxer, in this case, a matroska demuxer
    let mut demuxer = Context::new(MkvDemuxer::new(), ar);

    // Read matroska headers
    demuxer
        .read_headers()
        .expect("Cannot parse the format headers");

    // Iterate over the streams contained in a matroska file
    for stream in &demuxer.info.streams {
        // Print simple information on video and audio streams
        match stream.params.kind {
            Some(MediaKind::Video(ref info)) => {
                println!("Video streams information\n");
                println!("width: {}", info.width);
                println!("height: {}", info.height);
            }
            Some(MediaKind::Audio(ref info)) => {
                println!("\nAudio streams information\n");
                println!("rate: {}", info.rate);
            }
            _ => {}
        }
    }
}
