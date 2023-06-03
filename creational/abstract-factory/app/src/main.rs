use crate::render::render;
use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;
mod render;

fn main() {
    let windows = true;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}
