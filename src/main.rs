use confy::load;
use penrose::{
    builtin::{
        actions::{exit, key_handler, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        Config, WindowManager,
    },
    map, util,
    x11rb::RustConn,
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Default)]
struct WMConfig {
    bindings: HashMap<String, String>,
}

impl WMConfig {
    fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
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
        };

        if let Ok(cfg) = load::<WMConfig>("sswm", Some("config")) {
            for i in cfg.bindings {
                let r = i.1.clone();
                raw_bindings.insert(i.0, key_handler(move |_, _| util::spawn(r.clone())));
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
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(WMConfig::raw_key_bindings())?;
    let wm = WindowManager::new(Config::default(), key_bindings, HashMap::new(), conn)?;

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
