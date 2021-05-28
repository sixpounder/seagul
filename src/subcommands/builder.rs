use std::io::{Read, Write};

use seagul_core::{decoder::ImageDecoder, encoder::ImageEncoder, prelude::ImageRules};

/// Constructs input and output channels based on comman line arguments
pub fn subcommand_channels(subcommand: &clap::ArgMatches) -> (Box<dyn Read>, Box<dyn Write>) {
    let input_reader = match subcommand.value_of("INPUT") {
        Some(arg) => Box::new(
            std::fs::File::open(arg).expect(format!("Could not open file at {}", arg).as_str()),
        ) as Box<dyn std::io::Read>,
        None => Box::new(std::io::stdin()),
    };
    let out_writer = match subcommand.value_of("OUTPUT") {
        Some(arg) => Box::new(
            std::fs::File::create(arg).expect(format!("Could not create file at {}", arg).as_str()),
        ) as Box<dyn std::io::Write>,
        None => Box::new(std::io::stdout()),
    };

    (input_reader, out_writer)
}

/// Constructs an image encoder based on command line arguments
pub fn encoder_from_args<T>(source: T, args: &clap::ArgMatches) -> ImageEncoder
where
    ImageEncoder: From<T>,
{
    let mut encoder = ImageEncoder::from(source);

    build_configurable(&mut encoder, args);

    encoder
}

/// Constructs an image decoder based on command line arguments
pub fn decoder_from_args<'a, T>(source: T, args: &'a clap::ArgMatches) -> ImageDecoder<'a>
where
    ImageDecoder<'a>: From<T>,
{
    let mut decoder = ImageDecoder::from(source);

    build_configurable(&mut decoder, args);

    if let Some(marker) = args.value_of("decode_marker") {
        decoder.until_marker(Some(marker.as_bytes()));
    }

    decoder
}

fn build_configurable<'a, T>(obj: &'a mut T, args: &clap::ArgMatches) -> &'a mut T
where
    T: ImageRules,
{
    if let Some(lsb) = args.value_of("lsb") {
        obj.set_offset(lsb.parse().expect("'lsb' option value should be numeric"));
    }

    if let Some(offset) = args.value_of("offset") {
        obj.set_offset(offset.parse().expect("'offset' option value should be numeric"));
    }

    if let Some(channel) = args.value_of("channel") {
        obj.set_use_channel(channel.into());
    }

    if let Some(jump) = args.value_of("jump") {
        obj.set_step_by_n_pixels(jump.parse().expect("'jump' option value should be numeric"));
    }

    obj
}
