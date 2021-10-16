pub fn decode(args: &clap::ArgMatches) -> Result<(), std::io::Error> {
    let (mut input, mut output) = super::builder::subcommand_channels(args);
    let decoder = super::builder::decoder_from_args(input.as_mut(), args);
    match decoder.decode() {
        Ok(image) => {
            image.write(&mut output)
        }
        Err(some_error) => Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            some_error,
        )),
    }
}
