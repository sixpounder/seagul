pub mod prelude;
mod subcommands;

use std::{fmt::Display, process};
use colored::Colorize;
use prelude::VERBOSE;

#[macro_export]
macro_rules! everboseln {
    ($($arg:tt)*) => ({
        if *VERBOSE.lock().unwrap() == true {
            eprintln!($($arg)*);
        }
    })
}

const VERSION_STR: &'static str = "1.0";

fn main() {
    let mut app = clap::App::new("Seagul")
        .version(VERSION_STR)
        .author("Andrea Coronese <sixpounder@pm.me>")
        .subcommand(
            clap::SubCommand::with_name("encode")
                .version(VERSION_STR)
                .about("Encodes data into an image")
                .arg(
                    clap::Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .help("Silent mode, suppresses output to STDERR"),
                )
                .arg(
                    clap::Arg::with_name("channel")
                        .short("c")
                        .long("channel")
                        .help("The cannel to decode for each pixel. Possibile values \
                            are 'red', 'green' and 'blue'. Defaults to 'blue'."
                        ),
                )
                .arg(
                    clap::Arg::with_name("lsb")
                        .short("l")
                        .long("lsb")
                        .help("Number of least significant bits to use for decoding each pixel"),
                )
                .arg(
                    clap::Arg::with_name("offset")
                        .short("s")
                        .long("skip")
                        .value_name("offset")
                        .help("Skip n pixels before encoding the message"),
                )
                .arg(
                    clap::Arg::with_name("data")
                        .short("d")
                        .long("data")
                        .value_name("DATA")
                        .help("Sets the data to encode in the final output"),
                )
                .arg(
                    clap::Arg::with_name("format")
                        .required(false)
                        .short("f")
                        .long("format")
                        .value_name("FORMAT")
                        .help("Sets the output format. Supported types are png, jpeg and bmp"),
                )
                .arg(clap::Arg::with_name("INPUT").required(false).index(1).help(
                    "The path to the input image file to use. \
                            Attempts to read from stdin if not specified. \
                            This argument is ignored if the \"data\" option is also specified.",
                ))
                .arg(
                    clap::Arg::with_name("OUTPUT")
                        .required(false)
                        .index(2)
                        .help(
                            "The output image path on the file system. \
                            If not specified, output is emitted to stdout",
                        ),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("decode")
                .about("Attempt to decode a message from an image")
                .version(VERSION_STR)
                .arg(
                    clap::Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .help("Silent mode, suppresses output to STDERR"),
                )
                .arg(
                    clap::Arg::with_name("channel")
                        .short("c")
                        .long("channel")
                        .help("The cannel to decode for each pixel. Possibile values \
                            are 'red', 'green' and 'blue'. Defaults to 'blue'."
                        ),
                )
                .arg(
                    clap::Arg::with_name("lsb")
                        .short("l")
                        .long("lsb")
                        .help("Number of least significant bits to use for decoding each pixel"),
                )
                .arg(
                    clap::Arg::with_name("decode_marker")
                        .required(false)
                        .short("m")
                        .long("marker")
                        .value_name("MARKER")
                        .help(
                            "Decode until this sequence of bytes is found (or the input file ends)",
                        ),
                )
                .arg(clap::Arg::with_name("INPUT").required(false).index(1).help(
                    "Sets the input file to use. Attempts to read from stdin if not specified.",
                ))
                .arg(
                    clap::Arg::with_name("OUTPUT")
                        .required(false)
                        .index(2)
                        .help(
                            "Sets the output file for the decoded content. \
                            If not specified, output is emitted to stdout",
                        ),
                ),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("quiet") {
        *VERBOSE.lock().unwrap() = false;
    }

    let start = std::time::Instant::now();

    // Run subcommand
    match matches.subcommand_name() {
        Some("encode") => {
            if let Some(subcommand_args) = matches.subcommand_matches("encode") {
                set_verbose_from_args(subcommand_args);
                match subcommands::encode(subcommand_args) {
                    Ok(_) => {
                        let end = std::time::Instant::now();
                        everboseln!(
                            "✔️  Done in {}",
                            format!("{} ms", (end - start).as_millis())
                                .to_string()
                                .cyan()
                        );
                    }
                    Err(e) => {
                        cli_error(e);
                    }
                }
            }
        }
        Some("decode") => {
            if let Some(subcommand_args) = matches.subcommand_matches("decode") {
                set_verbose_from_args(subcommand_args);
                match subcommands::decode(subcommand_args) {
                    Ok(_) => {
                        let end = std::time::Instant::now();
                        everboseln!(
                            "✔️  Done in {}",
                            format!("{} ms", (end - start).as_millis())
                                .to_string()
                                .cyan()
                        );
                    }
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

// fn subcommand_channels(subcommand: &clap::ArgMatches) -> (Box<dyn Read>, Box<dyn Write>) {
//     let input_reader = match subcommand.value_of("INPUT") {
//         Some(arg) => Box::new(std::fs::File::open(arg).unwrap()) as Box<dyn std::io::Read>,
//         None => Box::new(std::io::stdin()),
//     };
//     let out_writer = match subcommand.value_of("OUTPUT") {
//         Some(arg) => Box::new(std::fs::File::create(arg).unwrap()) as Box<dyn std::io::Write>,
//         None => Box::new(std::io::stdout()),
//     };

//     (input_reader, out_writer)
// }

// fn run_encode<R, W>(
//     input: &mut R,
//     data: &str,
//     offset: usize,
//     out: &mut W,
// ) -> Result<(), std::io::Error>
// where
//     R: std::io::Read,
//     W: std::io::Write,
// {
//     let mut encoder = seagul_core::encoder::ImageEncoder::from(input);
//     encoder.set_offset(offset);

//     match encoder.encode_string(String::from(data)) {
//         Ok(image) => {
//             everboseln!(
//                 "ℹ️  {} pixels modified",
//                 &image.pixels_changed().to_string().cyan()
//             );
//             image.write(out, seagul_core::prelude::ImageFormat::Png)
//         }
//         Err(some_error) => Err(std::io::Error::new(
//             std::io::ErrorKind::Interrupted,
//             some_error,
//         )),
//     }
// }

// fn run_decode<'a, R, W>(
//     input: &mut R,
//     out: &mut W,
//     marker: Option<&'a [u8]>,
// ) -> Result<(), std::io::Error>
// where
//     R: std::io::Read,
//     W: std::io::Write,
// {
//     let mut decoder = seagul_core::decoder::ImageDecoder::from(input);

//     decoder.until_marker(marker);
//     match decoder.decode() {
//         Ok(image) => image.write(out),
//         Err(some_error) => Err(std::io::Error::new(
//             std::io::ErrorKind::Interrupted,
//             some_error,
//         )),
//     }
// }

fn set_verbose_from_args(matches: &clap::ArgMatches) {
    if matches.is_present("quiet") {
        *VERBOSE.lock().unwrap() = false;
    }
}

fn cli_error<E>(message: E)
where
    E: Display,
{
    println!("{}", message);
    process::exit(1);
}
