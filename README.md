# Seagul

This command line utility encodes and decodes arbitrary information in images.

## Installing

The easiest way to install is through cargo

```bash
cargo install seagul
```

## Encoding

Passing data inline

```bash
seagul encode -d "Super secret message" path/to/source/image path/to/output/image
```

Getting data through STDIN

```bash
cat my_secret_file | seagul encode path/to/source/image path/to/output/image
```

Outputting to STDOUT (omit the target image)

```bash
seagul encode -d "Super secret message" path/to/source/image
```

The default channel used for encoding is the blue channel. To change it, use the `--channel` option.

```bash
seagul encode -d "Super secret message" --channel green path/to/source/image
```

For more encode options, see the integrated help with `seagul encode --help`

## Decoding

The decoder supports the same options as the encoder, with the addition of a `--marker` option to stop decoding when the specified byte sequence is found.

```bash
seagul decode path/to/image

# Stop at "end of sentence." sequence.
seagul decode --marker "end of sentence." path/to/image
```
