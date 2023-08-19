<div align="center">

<img width=200px, height=200px, src="https://github.com/Walker-00/sswm/blob/rust/logos/sswm_logo_2.png?raw=true"/>

[![Rust Check](https://github.com/Walker-00/sswm/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/sswm/actions/workflows/rust.yml) 
[![Clippy Analyze](https://github.com/Walker-00/sswm/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Walker-00/sswm/actions/workflows/rust-clippy.yml)

</div>

# SSWM
Minimal, flexible &amp; user-friendly X and Wayland tiling window manager but with rust.
<br>
Feel Free to Open Issues and Pull Requests

## [Overview]

SSWM mean `Saffron Spring Window Manager`.
<br>
Which [Saffron](https://en.wikipedia.org/wiki/Saffron_Revolution) and [Spring](https://en.wikipedia.org/wiki/Myanmar_protests_(2021%E2%80%93present)) are two most popular revolutions of burma ( which is my country )
<br>
You can call it as `Swim`, If `SSWM` is a little wired for you. (Which I got idea from [u/camguywhataguy](https://www.reddit.com/user/camguywhataguy))
<br>
Main Focus of the `SSWM` is to create `Freedom, User Friendly, Configurable, Fast and Safe` window manager for `Both X and Wayland`. 
<br>
Not Recommend to use For wayland, it's a little buggy.
<br>
This `SSWM` is a part of the project `Project SS`.

## [Features]

- [x] Tiling layout
- [x] Multi-monitor support
- [x] Altering layout (Application switching, changing master size...)
- [x] Fullscreening windows
- [x] Scratchpads
- [x] Auto Start Commands
- [x] Multiple Layouts
- [x] Full compositor Support
- [x] Hover to select windows
- [x] Configuration
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
SSWM use `.yaml` format for user-friendly configuration.

### [Config File]

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

If x and w specified commands and actions might be overrided to commands and actions if they were conflit.

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

Case doesn't matter when writing Actions name.
<br>
`kill`, `Kill`, `KiLL` all work.

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
    action: eXit
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

## [Voices From Burma]
As burmeses living in dictatorship is fucking hard.
<br>
We all burmeses are asking for Justice, but all we got are bullets.
<br>
We all burmeses are want our democracy and our leaders back.
<br>
We all burmeses are fighting back to the Military Junta and Injustice.
<br>
As burmeses we really want other countries to join us, but also we still fighting our own.
