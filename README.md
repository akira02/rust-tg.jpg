# rust-tg.jpg

This project is a Telegram bot that searches for images based on user input. It uses the Teloxide library for interacting with the Telegram Bot API and performs image searches using Google's image search.

## Features

- Listens for messages containing image file names (e.g., `example.jpg`, `example.png`, `example.gif`).
- Searches for images on Google and sends the first result back to the user.
- Supports both regular images and GIFs.

## Prerequisites

- Rust and Cargo installed on your system. You can install them from [rustup.rs](https://rustup.rs/).
- A Telegram bot token. You can create a bot and get a token by talking to [BotFather](https://t.me/botfather) on Telegram.

## Setup

1.  [Download Rust](http://rustup.rs/).
2.  Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
3.  Initialise the `TELOXIDE_TOKEN` environmental variable to your token:

```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows command line
$ set TELOXIDE_TOKEN=<Your token here>

# Windows PowerShell
$ $env:TELOXIDE_TOKEN=<Your token here>
```

4.  Make sure that your Rust compiler is up to date (`teloxide` currently requires rustc at least version 1.80):

```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

5. Build and run the project:

   ```bash
   cargo run
   ```

## Usage

- Start the bot on Telegram by searching for your bot's username and sending a message with an image file name (e.g., `example.jpg`).
- The bot will respond with the first two image results it finds.

## Dependencies

- [Teloxide](https://github.com/teloxide/teloxide) for Telegram bot API interaction.
- [Reqwest](https://github.com/seanmonstar/reqwest) for making HTTP requests.
- [Scraper](https://github.com/causal-agent/scraper) for parsing HTML.
- [Regex](https://github.com/rust-lang/regex) for regular expression matching.
- [Base64](https://github.com/marshallpierce/rust-base64) for decoding base64 images.
- [Anyhow](https://github.com/dtolnay/anyhow) for error handling.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.