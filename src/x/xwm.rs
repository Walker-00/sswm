use penrose::{
    builtin::{
        actions::{
            exit, floating::float_focused, log_current_state, modify_with, send_layout_message,
            spawn,
        },
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReflectHorizontal, ReserveTop},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::{
        actions::toggle_fullscreen,
        hooks::{
            add_ewmh_hooks, add_named_scratchpads, manage::FloatingCentered, NamedScratchPad,
            ToggleNamedScratchPad,
        },
    },
    map, stack,
    x::query::ClassName,
    x11rb::RustConn,
    Result,
};
use penrose_ui::{status_bar, Position, TextStyle};
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::{util::SubscriberInitExt, FmtSubscriber};

const FONT: &str = "ProFontIIx Nerd Font";
const BLACK: u32 = 0x282828ff;
const WHITE: u32 = 0xebdbb2ff;
const GREY: u32 = 0x3c3836ff;
const BLUE: u32 = 0x458588ff;

const MAX_MAIN: u32 = 1;
const RATIO: f32 = 0.6;
const RATIO_STEP: f32 = 0.1;
const OUTER_PX: u32 = 5;
const INNER_PX: u32 = 5;
const BAR_HEIGHT_PX: u32 = 18;

fn raw_key_bindings(
    tg_1: ToggleNamedScratchPad,
    tg_2: ToggleNamedScratchPad,
) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.into();

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
        "M-Up" => send_layout_message(|| IncMain(1)),
        "M-Down" => send_layout_message(|| IncMain(-1)),
        "M-Right" => send_layout_message(|| ExpandMain),
        "M-Left" => send_layout_message(|| ShrinkMain),
        "M-semicolon" => spawn("dmenu_run"),
        "M-S-s" => log_current_state(),
        "M-S-Return" => spawn("alacritty"),
        "M-Escape" => exit(),
        "M-slash" => Box::new(tg_1),
        "M-f" => toggle_fullscreen(),
        "M-S-f" => float_focused(),
    };

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

fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        ReflectHorizontal::wrap(MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP)),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX))
}

pub fn xrun() -> Result<()> {
    FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish()
        .init();

    let (nsp_1, toggle_1) = NamedScratchPad::new(
        "terminal",
        "alacritty",
        ClassName("Alacritty"),
        FloatingCentered::new(0.6, 0.6),
        true,
    );
    let (nsp_2, toggle_2) = NamedScratchPad::new(
        "fire-browser",
        "firefox-dev",
        ClassName("Firefox"),
        FloatingCentered::new(0.6, 0.6),
        true,
    );

    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings(toggle_1, toggle_2))?;
    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2, 2),
    };

    let bar = status_bar(BAR_HEIGHT_PX, FONT, 8, style, BLUE, GREY, Position::Top).unwrap();
    let wm = add_named_scratchpads(
        WindowManager::new(config, key_bindings, HashMap::new(), conn)?,
        vec![nsp_1, nsp_2],
    );

    let wm = bar.add_to(wm);

    wm.run().unwrap();
    Ok(())
}
