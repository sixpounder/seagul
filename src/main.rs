use std::process;

fn main() {
    let mut app = clap::App::new("Seagul")
        .version("1.0")
        .author("Andrea Coronese <sixpounder@pm.me")
        .subcommand(
            clap::SubCommand::with_name("encode")
                .about("Encodes data into an image")
                .arg(
                    clap::Arg::with_name("data")
                        .short("d")
                        .value_name("DATA")
                )
                .arg(
                    clap::Arg::with_name("format")
                        .required(false)
                        .short("f")
                        .help("Sets the output format")
                )
                .arg(
                    clap::Arg::with_name("INPUT")
                        .required(true)
                        .index(1)
                        .help("Sets the input file to use")
                )
                .arg(
                    clap::Arg::with_name("OUTPUT")
                        .required(false)
                        .index(2)
                        .help("Sets the output image. If not specified, output is emitted to stdout")
                )
        )
        .subcommand(
            clap::SubCommand::with_name("decode")
                .about("Attempt to decode a message from an image")
                .arg(
                    clap::Arg::with_name("INPUT")
                        .required(true)
                        .index(1)
                        .help("Sets the input file to use")
                )
        );
    let matches = app.clone().get_matches();

    match matches.subcommand_name() {
        Some("encode") => {
            if let Some(subcommand_args) = matches.subcommand_matches("encode") {
                let input_reader = match subcommand_args.value_of("INPUT") {
                    Some(arg) => {
                        Box::new(std::fs::File::open(arg).unwrap()) as Box<dyn std::io::Read>
                    }
                    None => {
                        Box::new(std::io::stdin())
                    }
                };
                let out_writer = match subcommand_args.value_of("OUTPUT") {
                    Some(arg) => {
                        Box::new(std::fs::File::create(arg).unwrap()) as Box<dyn std::io::Write>
                    }
                    None => {
                        Box::new(std::io::stdout())
                    }
                };

                run_encode(input_reader.as_ref(), "", out_writer.as_ref());
            }
        },
        Some("decode") => {
            if let Some(subcommand_args) = matches.subcommand_matches("decode") {
                let input_reader = match subcommand_args.value_of("INPUT") {
                    Some(arg) => {
                        Box::new(std::fs::File::open(arg).unwrap()) as Box<dyn std::io::Read>
                    }
                    None => {
                        Box::new(std::io::stdin())
                    }
                };
                let out_writer = match subcommand_args.value_of("OUTPUT") {
                    Some(arg) => {
                        Box::new(std::fs::File::create(arg).unwrap()) as Box<dyn std::io::Write>
                    }
                    None => {
                        Box::new(std::io::stdout())
                    }
                };

                run_decode();
            }
        }
        _ => {
            app.print_long_help().unwrap();
        }
    }
}

fn run_encode(input: &dyn std::io::Read, data: &str, out: &dyn std::io::Write) {
    let path = "";
    let encoder = seagul_core::encoder::ImageEncoder::from(path);

    match encoder.encode_string(String::from(data)) {
        Ok(_) => (),
        Err(some_error) => {
            cli_error(some_error);
        }
    }
}

fn run_decode() {}

fn cli_error(message: String) {
    println!("{}", message);
    process::exit(1);
}