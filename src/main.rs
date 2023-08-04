use confy::load;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use penrose::{
    builtin::{
        actions::{exit, key_handler, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        Config, WindowManager,
    },
    extensions::hooks::{
        add_named_scratchpads, manage::FloatingCentered, NamedScratchPad, ToggleNamedScratchPad,
    },
    map, util,
    x::query::ClassName,
    x11rb::RustConn,
    Result,
};
use tracing_subscriber::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Default)]
struct WMConfig {
    spawn: HashMap<String, String>,
    command: HashMap<String, String>,
}

fn raw_key_bindings(
    toggle_1: ToggleNamedScratchPad,
    toggle_2: ToggleNamedScratchPad,
) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-S-q" => modify_with(|cs| cs.kill_focused()),
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-grave" => modify_with(|cs| cs.next_layout()),
        "M-S-grave" => modify_with(|cs| cs.previous_layout()),
        "M-S-Up" => send_layout_message(|| IncMain(1)),
        "M-S-Down" => send_layout_message(|| IncMain(-1)),
        "M-S-Right" => send_layout_message(|| ExpandMain),
        "M-S-Left" => send_layout_message(|| ShrinkMain),
        "M-semicolon" => spawn("dmenu_run"),
        "M-S-Return" => spawn("alacritty"),
        "M-A-Escape" => exit(),

        "M-slash" => Box::new(toggle_1),
        "M-p" => Box::new(toggle_2),
    };

    if let Ok(cfg) = load::<WMConfig>("sswm", Some("config")) {
        for i in cfg.spawn {
            let r = i.1.clone();
            raw_bindings.insert(i.0, key_handler(move |_, _| util::spawn(r.clone())));
        }

        for i in cfg.command {
            let cmd_match: Option<Box<dyn KeyEventHandler<RustConn>>> =
                match i.1.to_lowercase().as_str() {
                    "killf" => Some(modify_with(|cs| cs.kill_focused())),
                    "focdw" => Some(modify_with(|cs| cs.focus_down())),
                    "focup" => Some(modify_with(|cs| cs.focus_up())),
                    "swpdw" => Some(modify_with(|cs| cs.swap_down())),
                    "swpup" => Some(modify_with(|cs| cs.swap_up())),
                    "togtg" => Some(modify_with(|cs| cs.toggle_tag())),
                    "nexsc" => Some(modify_with(|cs| cs.next_screen())),
                    "presc" => Some(modify_with(|cs| cs.previous_screen())),
                    "nexly" => Some(modify_with(|cs| cs.next_layout())),
                    "prely" => Some(modify_with(|cs| cs.previous_layout())),
                    "incmn" => Some(send_layout_message(|| IncMain(1))),
                    "decmn" => Some(send_layout_message(|| IncMain(-1))),
                    "expmn" => Some(send_layout_message(|| ExpandMain)),
                    "shkmn" => Some(send_layout_message(|| ShrinkMain)),
                    "exits" => Some(exit()),
                    _ => None,
                };

            if let Some(cmd) = cmd_match {
                raw_bindings.insert(i.0, cmd);
            }
        }
    }

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let (nsp_1, toggle_1) = NamedScratchPad::new(
        "terminal",
        "alacritty",
        ClassName("Alacritty"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );
    let (nsp_2, toggle_2) = NamedScratchPad::new(
        "qt-console",
        "jupyter-qtconsole",
        ClassName("jupyter-qtconsole"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings(toggle_1, toggle_2))?;

    let wm = add_named_scratchpads(
        WindowManager::new(Config::default(), key_bindings, HashMap::new(), conn)?,
        vec![nsp_1, nsp_2],
    );

    wm.run()
}
