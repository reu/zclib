use clap::{clap_app, crate_description, crate_version};
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use std::{fs, io};

fn valid_level(level: String) -> Result<(), String> {
    level
        .parse::<u32>()
        .map_err(|_| "Invalid number".to_owned())
        .and_then(|level| match level {
            0..=9 => Ok(()),
            _ => Err("Level must be in the 0-9 range".to_owned()),
        })
}

fn valid_file(path: String) -> Result<(), String> {
    match path.as_str() {
        "-" => Ok(()),
        path => match fs::metadata(path) {
            Err(_) => Err("File not found".to_owned()),
            Ok(meta) if meta.is_dir() => Err("Can't compress a directory".to_owned()),
            _ => Ok(()),
        },
    }
}

fn main() -> Result<(), io::Error> {
    let args = clap_app!(zclib =>
        (version: crate_version!())
        (about: crate_description!())
        (@arg decompress: -d --decompress "Decompress")
        (@arg best: -b --best conflicts_with[fast level] "Best (slowest) compression")
        (@arg fast: -f --fast conflicts_with[best level] "Fastest (worst) compression")
        (@arg level: -l --level +takes_value conflicts_with[best fast] {valid_level} "Compressing level (0-9)")
        (@arg FILE: {valid_file} "File")
    )
    .get_matches();

    let compression = if args.is_present("best") {
        Compression::best()
    } else if args.is_present("fast") {
        Compression::fast()
    } else if args.is_present("level") {
        let level = args
            .value_of("level")
            .and_then(|level| level.parse::<u32>().ok())
            .unwrap();
        Compression::new(level)
    } else {
        // Using the same default compression level as gzip
        Compression::new(6)
    };

    let stdin = io::stdin();
    let stdout = io::stdout();

    let input: Box<dyn io::Read> = match args.value_of("FILE") {
        Some(path) if path != "-" => Box::new(fs::File::open(path)?),
        _ => Box::new(stdin.lock()),
    };

    if args.is_present("decompress") {
        let mut decoder = ZlibDecoder::new(input);
        io::copy(&mut decoder, &mut stdout.lock())?;
    } else {
        let mut encoder = ZlibEncoder::new(input, compression);
        io::copy(&mut encoder, &mut stdout.lock())?;
    }
    Ok(())
}
