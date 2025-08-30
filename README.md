# pass-gen

A fast, secure, and customizable password generator written in Rust.

## Features

- Generate strong, random passwords of any length
- Customizable character sets (uppercase, lowercase, numbers, symbols)
- Password validation and strength checking
- Logging and tracking of generated passwords
- Simple command-line interface

## Installation

Clone the repository and build with Cargo:

```sh
git clone https://github.com/RubenDguez/passgen.git
cd passgen
cargo build --release
```

## Usage

Run the password generator from the command line:

```sh
cargo run --release
```

You can customize password generation with command-line options (see below).

## Command-Line Options

- `--length <N>`: Set password length (default: 16)
- `--uppercase`: Include uppercase letters
- `--lowercase`: Include lowercase letters
- `--numbers`: Include numbers
- `--symbols`: Include symbols
- `--validate <password>`: Validate a password's strength

Example:

```sh
cargo run --release -- --length 20 --uppercase --numbers --symbols
```

## Logging

Generated passwords are logged to `pass.log` for tracking purposes.

## Project Structure

- `src/main.rs`: Entry point
- `src/generate.rs`: Password generation logic
- `src/validate.rs`: Password validation
- `src/track.rs`: Logging and tracking
- `src/constants.rs`: Character set constants
- `src/utils.rs`: Utility functions

## Contributing

Contributions are welcome! Please open issues or submit pull requests.

## License

This project is licensed under the MIT License.
