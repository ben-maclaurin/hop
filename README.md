# Jump

Jump is a [TUI](https://github.com/fdehau/tui-rs)-based project viewer. It is built to somewhat replicate Emacs' [Projectile](https://github.com/bbatsov/projectile) tool.

<img width="1532" alt="Screenshot 2022-09-21 at 15 42 56" src="https://user-images.githubusercontent.com/54992484/191535229-29ecef3e-e399-478f-8f71-e08a6703ad50.png">

## Prerequisites

If you want Jump to display project languages, you will need to install
two dependencies.

1. [Linguist](https://github.com/github/linguist)

This is a Ruby tool used by Jump to analyse project languages. You can find installation
instructions [here](https://github.com/github/linguist#installation).

2. [Nerd Fonts](https://www.nerdfonts.com/font-downloads)

Jump uses Nerd Fonts for icon rendering. Your terminal/emulator will need to
be running one of these fonts.

## Installation

Currently (to be improved):

1. Clone the repo.
2. Run `cargo install --path .`

## Usage

First, setup a configuration file at `~/.config/jump/jump.yml`.

## Themes

## Contributing

## License
[MIT](https://choosealicense.com/licenses/mit/)
