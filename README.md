# Hop

Hop is a very minimal [TUI](https://github.com/fdehau/tui-rs)-based project viewer. Loosely inpsired by [Projectile for Emacs](https://github.com/bbatsov/projectile), although nowhere near as feature-complete.

https://user-images.githubusercontent.com/54992484/191971020-05de3fa3-1f0d-4cfc-a0eb-61538e2c48af.mp4

## Features

- Interactive TUI for viewing and opening projects from the command line.
- Uses Vim-based scrolling and list navigation.
- Search for projects by language (e.g. `rust` or `go`).
- Support for language tagging and icons (see: [Languages and Icons](https://github.com/ben-maclaurin/hop#languages-and-icons))

## Installation

1. Download the release. 
2. Store the binary in a location of your choosing.
3. Add the binary to your PATH. 

## Usage

Hop launches in "insert mode" by default. To exit insert mode, hit the `TAB` key. To re-enter insert mode, press `/`.

Navigation is Vim-based (`j` to go down, `k` to go up).

Press the `RETURN` key on a project to open it in your favourite editor.

Running Hop with the `sync` flag (e.g. `hop -s`) will initiate a sync (Hop will re-index all of your projects). Read more in [Languages and Icons](https://github.com/ben-maclaurin/hop#languages-and-icons).

Use the `ESCAPE` key to exit Hop in any mode.

## Configuration

Hop expects your configration file to be stored at `$HOME/.config/hop/hop.yml`. See [hop.yml](https://github.com/ben-maclaurin/hop/blob/main/hop.yml) for an example configuration.

The following options are available:

| Option            | Description                                                                                                                                | Default     |
|-------------------|--------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| `directory`       | Your projects directory                                                                                                                    | `Developer` |
| `icons`           | Toggles icons. Requires a [Nerd Font](https://www.nerdfonts.com/font-downloads) and [Linguist](https://github.com/github/linguist) to work | `false`     |
| `editor`          | The launch command for your chosen editor (e.g. `code` for VSCode)                                                                         | `code`      |
| `include_files`   | If enabled, includes files as projects                                                                                                     | `false`     |
| `show_search`     | When enabled, displays a search bar below the project list                                                                                 | `true`      |
| `start_in_insert` | Determines if Hop should start in `insert mode` (search bar focused)                                                                       | `true`      |
| `highlight_symbol`| Modifies the highlight symbol                                                                                                              | `""`        |

## Languages and Icons

To display languages and icons, your terminal or emulator font should be a [Nerd Font](https://www.nerdfonts.com/font-downloads). Hop uses [Linguist](https://github.com/github/linguist) to detect project languages.

If you have `icons` enabled, the first time you run Hop it may take a few minutes to index all of your projects. Hop keeps track of previously indexed projects in a `.json` file. This reduces start times for future executions. 

If the dominant language of a project changes, you can update this manually in `$HOME/.config/hop/projects.json` or resync your project list with `hop -s`.

Currently supported languages:

- Rust
- TypeScript
- JavaScript
- Swift
- Elixir
- Ruby
- Markdown
- HTML
- Python
- Java
- EmacsLisp
- Go
- Lua
- C++
- CSS

## Themes

The existing theme is based on [Kanagawa](https://github.com/rebelot/kanagawa.nvim). I plan to add additional themes and/or support for customising. See [theme.rs](https://github.com/ben-maclaurin/hop/blob/main/src/interface/theme.rs) for an example of theme implementation.

## Credits

- [tui-rs](https://github.com/fdehau/tui-rs)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [config-rs](https://github.com/mehcode/config-rs)
- [directories-rs](https://github.com/dirs-dev/directories-rs)
- [Nerd Fonts](https://www.nerdfonts.com/)
- [GitHub Linguist](https://github.com/github/linguist)

## Contributing

Contributions are welcome (I am new to Rust).

## License
[MIT](https://choosealicense.com/licenses/mit/)
