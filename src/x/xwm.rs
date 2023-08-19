use penrose::{
    builtin::{
        actions::{exit, floating::float_focused, key_handler, modify_with, send_layout_message},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReflectHorizontal, ReserveTop},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        hooks::StateHook,
        layout::LayoutStack,
        Config as PConfig, State, WindowManager,
    },
    extensions::{actions::toggle_fullscreen, hooks::add_ewmh_hooks},
    stack,
    util::{self, spawn_with_args},
    x::XConn,
    x11rb::RustConn,
    Color, Result,
};
use std::{borrow::Cow, collections::HashMap};
use tracing::Level;
use tracing_subscriber::{util::SubscriberInitExt, FmtSubscriber};

use crate::config::Config;

#[derive(Default)]
struct Xwm {
    max_main: u32,
    ratio: f32,
    ratio_step: f32,
    outer_gaps: u32,
    inner_gaps: u32,
    top_gaps: u32,
}

impl Xwm {
    fn raw_key_bindings(
        &self,
        keybinds: HashMap<String, Box<dyn KeyEventHandler<RustConn>>>,
    ) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
        let mut key_bindings: HashMap<String, Box<dyn KeyEventHandler<RustConn>>> = HashMap::new();
        key_bindings.extend(keybinds);
        for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
            key_bindings.extend([
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
        key_bindings
    }

    fn layouts(&self) -> LayoutStack {
        stack!(
            MainAndStack::side(self.max_main, self.ratio, self.ratio_step),
            ReflectHorizontal::wrap(MainAndStack::side(
                self.max_main,
                self.ratio,
                self.ratio_step
            )),
            MainAndStack::bottom(self.max_main, self.ratio, self.ratio_step),
            Monocle::boxed()
        )
        .map(|layout| {
            ReserveTop::wrap(
                Gaps::wrap(layout, self.outer_gaps, self.inner_gaps),
                self.top_gaps,
            )
        })
    }

    fn match_actions(&self, action_str: &str) -> Option<Box<dyn KeyEventHandler<RustConn>>> {
        match action_str.to_lowercase().as_str() {
            "kill" => Some(modify_with(|cs| cs.kill_focused())),
            "focusnext" => Some(modify_with(|cs| cs.focus_down())),
            "focusprevious" => Some(modify_with(|cs| cs.focus_up())),
            "swapdown" => Some(modify_with(|cs| cs.swap_down())),
            "togglefullscreen" => Some(toggle_fullscreen()),
            "floatfocused" => Some(float_focused()),
            "swapup" => Some(modify_with(|cs| cs.swap_up())),
            "toggletag" => Some(modify_with(|cs| cs.toggle_tag())),
            "focusnextscreen" => Some(modify_with(|cs| cs.next_screen())),
            "focuspreviousscreen" => Some(modify_with(|cs| cs.previous_screen())),
            "nextlayout" => Some(modify_with(|cs| cs.next_layout())),
            "previouslayout" => Some(modify_with(|cs| cs.previous_layout())),
            "incmain" => Some(send_layout_message(|| IncMain(1))),
            "decmain" => Some(send_layout_message(|| IncMain(-1))),
            "expandmain" => Some(send_layout_message(|| ExpandMain)),
            "shrinkmain" => Some(send_layout_message(|| ShrinkMain)),
            "exit" => Some(exit()),
            _ => None,
        }
    }

    fn handle_config(
        &mut self,
        cfg: Config,
    ) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
        let mut keybinds: HashMap<String, Box<dyn KeyEventHandler<RustConn>>> = Default::default();
        self.max_main = cfg.max_main;
        self.ratio = cfg.ratio;
        self.ratio_step = cfg.ratio_step;
        self.outer_gaps = cfg.outer_gaps;
        self.inner_gaps = cfg.inner_gaps;
        self.top_gaps = cfg.top_gaps;

        for i in cfg.commands {
            keybinds.insert(
                i.keybind,
                key_handler(move |_, _| util::spawn(i.command.as_str())),
            );
        }

        for i in cfg.actions {
            let action_match: Option<Box<dyn KeyEventHandler<RustConn>>> =
                self.match_actions(&i.action);
            if let Some(action) = action_match {
                keybinds.insert(i.keybind, action);
            }
        }

        for i in cfg.xcommands {
            keybinds.insert(
                i.keybind,
                key_handler(move |_, _| util::spawn(i.command.as_str())),
            );
        }

        for i in cfg.xactions {
            let action_match: Option<Box<dyn KeyEventHandler<RustConn>>> =
                self.match_actions(&i.action);
            if let Some(action) = action_match {
                keybinds.insert(i.keybind, action);
            }
        }

        keybinds
    }
}

pub fn xrun(cfg: Config) -> Result<()> {
    FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish()
        .init();

    let mut startup_hook1: String = "".into();

    if !cfg.start_up.is_empty() {
        for (e, i) in cfg.start_up.iter().enumerate() {
            startup_hook1.push_str(i);
            if cfg.start_up.len() > 1 && e + 1 < cfg.start_up.len() {
                startup_hook1.push_str(" && ");
            }
        }
    }

    let mut startup_hook: Option<Box<dyn StateHook<RustConn>>> = None;

    if !startup_hook1.is_empty() {
        startup_hook = Some(SpawnOnStartupArg::boxed(startup_hook1));
    }

    let mut xwm = Xwm::default();
    let keybinds = xwm.handle_config(cfg.clone());

    let config = add_ewmh_hooks(PConfig {
        default_layouts: xwm.layouts(),
        normal_border: Color::new_from_hex(cfg.normal_bordar),
        focused_border: Color::new_from_hex(cfg.focused_bordar),
        startup_hook,
        tags: cfg.workspace_tags,
        ..PConfig::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(xwm.raw_key_bindings(keybinds))?;
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    wm.run().unwrap();
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnOnStartupArg {
    prog: Cow<'static, str>,
}

impl SpawnOnStartupArg {
    pub fn boxed<X>(prog: impl Into<Cow<'static, str>>) -> Box<dyn StateHook<X>>
    where
        X: XConn,
    {
        Box::new(Self { prog: prog.into() })
    }
}

impl<X> StateHook<X> for SpawnOnStartupArg
where
    X: XConn,
{
    fn call(&mut self, _state: &mut State<X>, _x: &X) -> Result<()> {
        let args = ["-c", &self.prog];
        spawn_with_args("bash", &args)?;
        Ok(())
    }
}
