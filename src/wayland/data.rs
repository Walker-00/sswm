use crate::wayland::state::State;
use smithay::{
    reexports::wayland_server::{backend, Display},
    wayland::compositor::CompositorClientState,
};

pub struct Data {
    pub display: Display<State>,
    pub state: State,
}

#[derive(Default)]
pub struct ClientData {
    pub compositor_state: CompositorClientState,
}

impl backend::ClientData for ClientData {}
