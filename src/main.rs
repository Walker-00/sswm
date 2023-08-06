use std::sync::Arc;

use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;
use xcb::x::{Cursor, Window};
use xcb_util::ewmh::Connection;

struct Client {
    window: Window,
    workspace: Option<u8>,
    visible: bool,
    controlled: bool,
    full_screen: bool,
    padding_top: u32,
}

struct Clients {
    conn: Arc<Connection>,
}

struct WM {
    conn: Arc<Connection>,
    cursor: Cursor,
}

fn main() {
    let sub = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    subscriber::set_global_default(sub).expect("Error due to: Setting Default Subscriber Failed!");
}
