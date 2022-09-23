# Hop

Hop is a [TUI](https://github.com/fdehau/tui-rs)-based project viewer. Inpsired by [Projectile for Emacs](https://github.com/bbatsov/projectile), although nowhere near as feature-complete.

https://user-images.githubusercontent.com/54992484/191871726-57dd6f16-7a2b-4d91-bbfd-f856b139b6f2.mp4

## Installation

1. Download the release. 
2. Store the binary in a location of your choosing.
3. Add the binary to your PATH. 

## Usage

Hop launches in "insert mode" by default. To exit insert mode, hit the `TAB` key. To re-enter insert mode, press `/`.

Navigation is Vim-based (`j` to go down, `k` to go up).

Press the `RETURN` key on a project to open it in your favourite editor.

## Configuration

Hop expects your configration file to be stored at `$HOME/.config/hop/hop.yml`. See [hop.yml](https://github.com/ben-maclaurin/hop/blob/main/hop.yml) for an example configuration.

The following options are available:

| Option            | Description                                                                                                                                | Default     |   |   |
|-------------------|--------------------------------------------------------------------------------------------------------------------------------------------|-------------|---|---|
| `directory`       | Your projects directory                                                                                                                    | `Developer` |   |   |
| `icons`           | Toggles icons. Requires a [Nerd Font](https://www.nerdfonts.com/font-downloads) and [Linguist](https://github.com/github/linguist) to work | `false`     |   |   |
| `editor`          | The launch command for your chosen editor (e.g. `code` for VSCode)                                                                         | `code`      |   |   |
| `include_files`   | If enabled, includes files as projects                                                                                                     | `false`     |   |   |
| `show_search`     | When enabled, displays a search bar below the project list                                                                                 | `true`      |   |   |
| `start_in_insert` | Determines if Hop should start in `insert mode` (search bar focused)                                                                       | `true`      |   |   |

## Themes

## Contributing

## License
[MIT](https://choosealicense.com/licenses/mit/)
