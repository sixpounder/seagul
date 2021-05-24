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

    if let Some("encode") = matches.subcommand_name() {
        let input_path;
        run_encode();
    } else if let Some(_) = matches.subcommand_matches("decode") {
        run_decode()
    } else {
        app.print_long_help().unwrap();
    }
}

fn run_encode(path: &str, data: &str, out: &str) {
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