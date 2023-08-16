use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Action {
    pub keybind: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Command {
    pub keybind: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub max_main: u32,
    pub ratio: f32,
    pub ratio_step: f32,
    pub outer_gaps: u32,
    pub inner_gaps: u32,
    pub top_gaps: u32,
    pub commands: Vec<Command>,
    pub actions: Vec<Action>,
    pub xcommands: Vec<Command>,
    pub xactions: Vec<Action>,
    pub wcommands: Vec<Command>,
    pub wactions: Vec<Action>,
}

impl Default for Config {
    fn default() -> Self {
        let cfg = Config {
            max_main: 1,
            ratio: 0.6,
            ratio_step: 0.1,
            outer_gaps: 5,
            inner_gaps: 5,
            top_gaps: 0,
            commands: vec![
                Command {
                    keybind: "M-S-Return".into(),
                    command: "alacritty".into(),
                },
                Command {
                    keybind: "M-x".into(),
                    command: "firefox".into(),
                },
            ],
            actions: vec![
                Action {
                    keybind: "M-S-x".into(),
                    action: "eXit".into(),
                },
                Action {
                    keybind: "M-S-q".into(),
                    action: "kiLL".into(),
                },
            ],
            xcommands: vec![
                Command {
                    keybind: "M-A-e".into(),
                    command: "emacs".into(),
                },
                Command {
                    keybind: "M-C-v".into(),
                    command: "vim".into(),
                },
            ],
            xactions: vec![
                Action {
                    keybind: "M-j".into(),
                    action: "focusNext".into(),
                },
                Action {
                    keybind: "M-f".into(),
                    action: "ToggleFullScreen".into(),
                },
                Action {
                    keybind: "M-k".into(),
                    action: "focusPrevious".into(),
                },
                Action {
                    keybind: "M-S-j".into(),
                    action: "SwapDown".into(),
                },
                Action {
                    keybind: "M-S-k".into(),
                    action: "SwapUp".into(),
                },
                Action {
                    keybind: "M-S-f".into(),
                    action: "floatfocused".into(),
                },
                Action {
                    keybind: "M-Tab".into(),
                    action: "ToggleTag".into(),
                },
                Action {
                    keybind: "M-bracketright".into(),
                    action: "FocusNextScreen".into(),
                },
                Action {
                    keybind: "M-bracketleft".into(),
                    action: "FocusPreviousScreen".into(),
                },
                Action {
                    keybind: "M-grave".into(),
                    action: "NextLayout".into(),
                },
                Action {
                    keybind: "M-S-grave".into(),
                    action: "PreviousLayout".into(),
                },
                Action {
                    keybind: "M-S-Up".into(),
                    action: "IncMain".into(),
                },
                Action {
                    keybind: "M-S-Down".into(),
                    action: "DecMain".into(),
                },
                Action {
                    keybind: "M-S-Right".into(),
                    action: "ExpandMain".into(),
                },
                Action {
                    keybind: "M-S-Up".into(),
                    action: "ShrinkMain".into(),
                },
            ],
            wcommands: {
                vec![Command {
                    keybind: "M-S-Return".into(),
                    command: "kitty".into(),
                }]
            },
            wactions: { vec![] },
        };
        confy::store("sswm", Some("config"), cfg).unwrap();
        confy::load("sswm", Some("config")).unwrap()
    }
}

pub fn get_config() -> Config {
    match confy::load("sswm", Some("config")) {
        Ok(k) => k,
        Err(_) => {
            confy::store("sswm", Some("config"), Config::default()).unwrap();
            confy::load("sswm", Some("config")).unwrap()
        }
    }
}
