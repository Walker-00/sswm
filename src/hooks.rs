use penrose::{core::hooks::StateHook, util::spawn, x::XConn};

pub struct Startup {
    path: String,
}

impl Startup {
    pub fn new(s: impl Into<String>) -> Self {
        Self { path: s.into() }
    }
}

impl<X: XConn> StateHook<X> for Startup {
    fn call(&mut self, _: &mut penrose::core::State<X>, x: &X) -> penrose::Result<()> {
        spawn(&self.path)
    }
}
