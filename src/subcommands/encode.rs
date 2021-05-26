use colored::Colorize;
use crate::prelude::*;
use crate::everboseln;

pub fn encode(args: &clap::ArgMatches) -> Result<(), std::io::Error> {
    let (mut input, mut output) = super::builder::subcommand_channels(args);
    let encoder = super::builder::encoder_from_args(input.as_mut(), args);
    
    let data = args.value_of("data").unwrap_or("");

    match encoder.encode_string(String::from(data)) {
        Ok(image) => {
            everboseln!(
                "ℹ️  {} pixels modified",
                &image.pixels_changed().to_string().cyan()
            );
            image.write(&mut output, seagul_core::prelude::ImageFormat::Png)
        }
        Err(some_error) => Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            some_error,
        )),
    }
}
