use std::io::{Read, Write};

use seagul_core::{decoder::ImageDecoder, encoder::ImageEncoder, prelude::ImageRules};

pub fn subcommand_channels(subcommand: &clap::ArgMatches) -> (Box<dyn Read>, Box<dyn Write>) {
    let input_reader = match subcommand.value_of("INPUT") {
        Some(arg) => Box::new(std::fs::File::open(arg).unwrap()) as Box<dyn std::io::Read>,
        None => Box::new(std::io::stdin()),
    };
    let out_writer = match subcommand.value_of("OUTPUT") {
        Some(arg) => Box::new(std::fs::File::create(arg).unwrap()) as Box<dyn std::io::Write>,
        None => Box::new(std::io::stdout()),
    };

    (input_reader, out_writer)
}

pub fn encoder_from_args<T>(source: T, args: &clap::ArgMatches) -> ImageEncoder
where
    ImageEncoder: From<T>,
{
    let mut encoder = ImageEncoder::from(source);

    build_configurable(&mut encoder, args);

    encoder
}

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
        obj.set_offset(lsb.parse().unwrap());
    }

    if let Some(offset) = args.value_of("offset") {
        obj.set_offset(offset.parse().unwrap());
    }

    if let Some(channel) = args.value_of("channel") {
        obj.set_use_channel(channel.into());
    }

    obj
}