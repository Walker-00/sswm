use std::env::{self, args};
use wayland::waywm::wayrun;
use x::xwm::xrun;

mod wayland;
mod x;

fn main() {
    let arg = args().nth(1);
    if arg.is_some() && arg.unwrap() == "way" {
        wayrun().unwrap();
    } else if env::var("DISPLAY").is_ok() {
        xrun().unwrap();
    }
}
