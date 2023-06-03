#[warn(unused_imports)]
use gui::GuiFactoryDynamic;
use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;
use crate::render::render;

mod render;

fn main() {
    let window = false;

    let factory: &dyn GuiFactoryDynamic = if window{
        &WindowsFactory
    } else {
        &MacFactory
    };

    let button = factory.create_button();
    button.press();

    render(factory);
}
