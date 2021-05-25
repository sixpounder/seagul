use std::{
    fmt::Display,
    io::{Read, Write},
    process,
};

use clap::ArgMatches;

fn main() {
    let mut app = clap::App::new("Seagul")
        .version("1.0")
        .author("Andrea Coronese <sixpounder@pm.me")
        .subcommand(
            clap::SubCommand::with_name("encode")
                .about("Encodes data into an image")
                .arg(clap::Arg::with_name("data").short("d").value_name("DATA"))
                .arg(
                    clap::Arg::with_name("format")
                        .required(false)
                        .short("f")
                        .help("Sets the output format"),
                )
                .arg(clap::Arg::with_name("INPUT").required(false).index(1).help(
                    "Sets the input file to use. Attempts to read from stdin if not specified.",
                ))
                .arg(
                    clap::Arg::with_name("OUTPUT")
                        .required(false)
                        .index(2)
                        .help(
                            "Sets the output image. If not specified, output is emitted to stdout",
                        ),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("decode")
                .about("Attempt to decode a message from an image")
                .arg(clap::Arg::with_name("INPUT").required(false).index(1).help(
                    "Sets the input file to use. Attempts to read from stdin if not specified.",
                ))
                .arg(
                    clap::Arg::with_name("OUTPUT")
                        .required(false)
                        .index(2)
                        .help(
                            "Sets the output image. If not specified, output is emitted to stdout",
                        ),
                ),
        );

    let matches = app.clone().get_matches();

    // Run subcommand
    match matches.subcommand_name() {
        Some("encode") => {
            if let Some(subcommand_args) = matches.subcommand_matches("encode") {
                let (mut input_reader, mut output_writer) = subcommand_channels(subcommand_args);
                match run_encode(&mut input_reader, "", &mut output_writer) {
                    Ok(_) => {}
                    Err(e) => {
                        cli_error(e);
                    }
                }
            }
        }
        Some("decode") => {
            if let Some(subcommand_args) = matches.subcommand_matches("decode") {
                let (mut input_reader, mut output_writer) = subcommand_channels(subcommand_args);
                match run_decode(&mut input_reader, &mut output_writer) {
                    Ok(_) => {}
                    Err(e) => {
                        cli_error(e);
                    }
                }
            }
        }
        _ => {
            app.print_long_help().unwrap();
        }
    }
}

fn subcommand_channels(subcommand: &ArgMatches) -> (Box<dyn Read>, Box<dyn Write>) {
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

fn run_encode<R, W>(input: &mut R, data: &str, out: &mut W) -> Result<(), std::io::Error>
where
    R: std::io::Read,
    W: std::io::Write,
{
    let encoder = seagul_core::encoder::ImageEncoder::from(input);

    match encoder.encode_string(String::from(data)) {
        Ok(image) => image.write(out, seagul_core::prelude::ImageFormat::Png),
        Err(some_error) => Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            some_error,
        )),
    }
}

fn run_decode<R, W>(input: &mut R, out: &mut W) -> Result<(), std::io::Error>
where
    R: std::io::Read,
    W: std::io::Write,
{
    let decoder = seagul_core::decoder::ImageDecoder::from(input);

    match decoder.decode() {
        Ok(image) => image.write(out),
        Err(some_error) => Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            some_error,
        )),
    }
}

fn cli_error<E>(message: E)
where
    E: Display,
{
    println!("{}", message);
    process::exit(1);
}
