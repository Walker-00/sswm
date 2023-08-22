<div align="center">

<img width=200px, height=200px, src="https://github.com/Walker-00/sswm/blob/rust/logos/sswm_logo_2.png?raw=true"/>

[![Rust Check](https://github.com/Walker-00/sswm/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/sswm/actions/workflows/rust.yml) 
[![Clippy Analyze](https://github.com/Walker-00/sswm/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Walker-00/sswm/actions/workflows/rust-clippy.yml)

</div>

# SSWM
Minimal, flexible &amp; user-friendly X and Wayland tiling window manager but with rust.
<br>
Feel free to open issues and make pull requests.

## [Overview]

SSWM means `Saffron Spring Window Manager`.
<br>
[Saffron](https://en.wikipedia.org/wiki/Saffron_Revolution) and [Spring](https://en.wikipedia.org/wiki/Myanmar_protests_(2021%E2%80%93present)) are the two well-known revolutions in burma (my home country).
<br>
Main focus of the `SSWM` is to create `Free, User Friendly, Configurable, Fast and Safe` window manager for both X and Wayland.
<br>
`SSWM` is a part of the project `Project SS`.
<br>
<br>
> *Note: Not recommended to use with Wayland as of now.*

## [Features]

- [x] Tiling layout
- [x] Multi-monitor support
- [x] Altering layout (Application switching, changing master size...)
- [x] Fullscreening windows
- [x] Auto Start Commands
- [x] Multiple Layouts
- [x] Full compositor Support
- [x] Hover to select windows
- [x] Configuration
- [ ] Scratchpads
- [ ] Status Bar
- [ ] Window decorations
- [ ] Support Extended Window Manager Hints
- [ ] Sys-Trays for bar

## [RoadMap]

- [x] Make it Work
    - [x] X window manager with penrose
    - [x] Basic Wayland compositor
- [ ] Make it Right
    - [ ] Rewrite X window managr in x11rb
    - [ ] Fix bug from Wayland compositor
    - [ ] Add more features to both X and Wayland
- [x] Make it Fast
    - [x] Optimize with cargo flags and options
    - [ ] Optimize the logic codes

## [Configuration]
SSWM will auto generate config file to `$HOME/.config/sswm/config.yml`
<br>
SSWM uses `.yaml` format for user-friendly configuration.

#### [Default Config File]

```yaml

---
max_main: 1
normal_bordar: 0
focused_bordar: 4278190335
workspace_tags:
  - "1"
  - "2"
  - "3"
  - "4"
  - "5"
  - "6"
  - "7"
  - "8"
ratio: 0.6
ratio_step: 0.1
outer_gaps: 5
inner_gaps: 5
top_gaps: 0
start_up: []
commands:
  - keybind: M-S-Return
    command: alacritty
  - keybind: M-x
    command: firefox
actions:
  - keybind: M-S-x
    action: eXit # case doesn't matter for action 
  - keybind: M-S-q
    action: kiLL
xcommands:
  - keybind: M-A-e
    command: emacs
  - keybind: M-C-v
    command: vim
xactions:
  - keybind: M-j
    action: focusNext
  - keybind: M-f
    action: ToggleFullScreen
  - keybind: M-k
    action: focusPrevious
  - keybind: M-S-j
    action: SwapDown
  - keybind: M-S-k
    action: SwapUp
  - keybind: M-S-f
    action: floatfocused
  - keybind: M-Tab
    action: ToggleTag
  - keybind: M-bracketright
    action: FocusNextScreen
  - keybind: M-bracketleft
    action: FocusPreviousScreen
  - keybind: M-grave
    action: NextLayout
  - keybind: M-S-grave
    action: PreviousLayout
  - keybind: M-S-Up
    action: IncMain
  - keybind: M-S-Down
    action: DecMain
  - keybind: M-S-Right
    action: ExpandMain
  - keybind: M-S-Up
    action: ShrinkMain
wcommands:
  - keybind: M-S-Return
    command: kitty
wactions: []
```
<br>

### [Config Options]

| Option          | Format                 | Description                                                                     |
|-----------------|------------------------|---------------------------------------------------------------------------------|
| max_main        | Integer                | Maximum number of windows in the main area of the layout.                       |
| normal_border   | Integer                | Border color for normal (unfocused) windows.                                    |
| focused_border  | Integer                | Border color for focused windows.                                               |
| workspace_tags  | List of Strings        | List of workspace tags.                                                         |
| ratio           | Float                  | Initial ratio of the main area to the whole screen.                             |
| ratio_step      | Float                  | Step size for adjusting the main area ratio.                                    |
| outer_gaps      | Integer                | Outer gaps (spacing) between windows and the screen edges.                      |
| inner_gaps      | Integer                | Inner gaps (spacing) between windows.                                           |
| top_gaps        | Integer                | Gaps at the top of the screen for panels or other elements.                     |
| start_up        | List of Strings        | List of commands to run on startup.                                             |
| commands        | List of Command Objects| List of keybindings and associated commands to execute For Both X and Wayland.  |
| actions         | List of Action Objects | List of keybindings and associated actions to perform For Both X and Wayland.   |
| xcommands       | List of Command Objects| List of keybindings and associated commands to execute For X11.                 |
| xactions        | List of Action Objects | List of keybindings and associated actions to perform For X11.                  |
| wcommands       | List of Command Objects| List of keybindings and associated commands to execute For Wayland.             |
| wactions        | List of Action Objects | List of keybindings and associated actions to perform For Wayland.              |

If x and w specified commands and actions(e.g. xactions, wcommands), they might overwrite the commands and actions if they are in conflict.
<br>
<br>

### [Actions]

| Actions             | Description                                   |
|---------------------|-----------------------------------------------|
| Kill                | Kill the focused window.                      |
| Exit                | Exit the window manager.                      |
| FocusNext           | Focus the next window.                        |
| FocusPrevious       | Focus the previous window.                    |
| SwapDown            | Swap the focused window with the one below.   |
| SwapUp              | Swap the focused window with the one above.   |
| ToggleFullScreen    | Toggle fullscreen mode for the focused window.|
| FloatFocused        | Float the currently focused window.           |
| ToggleTag           | Toggle the tag of the focused window.         |
| FocusNextScreen     | Focus the next screen.                        |
| FocusPreviousScreen | Focus the previous screen.                    |
| NextLayout          | Switch to the next layout.                    |
| PreviousLayout      | Switch to the previous layout.                |
| IncMain             | Increase the size of the main area.           |
| DecMain             | Decrease the size of the main area.           |
| ExpandMain          | Expand the main area.                         |
| ShrinkMain          | Shrink the main area.                         |

Case doesn't matter for writing Actions name.
<br>
`kill`, `Kill`, `KiLL` all work.
<br>

## [Required Packages]
- rustup
- xorg
- libwayland
- libxkbcommon
- libudev
- libinput
- libgbm
- libseat

## [Installation]
```sh
git clone https://github.com/Walker-00/sswm
cd sswm
cargo install --path .
sudo cp sswm.desktop sswm_way.desktop /usr/share/xsessions/.
```

## [ScreenShot]
![sswm_3](https://github.com/Walker-00/sswm/assets/85013114/e15db47c-a014-4945-ace0-f3aba30e8595)

## [Voices From Burma]
Currently, Burma is under the Military rule and as a burmese, living under the dictatorship is extremely chellenging.
<br>
But we stayed strong and are fighting against Military Junta's bullets and violence.
<br>
We demand Justice and Democracy.
<br>
And we kindly want to ask for the International support and attention.
