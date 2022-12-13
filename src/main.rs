mod data;
mod delegate;
mod theme;
mod ui;

use data::ViewImageData;
use delegate::ViewImageDelegate;
use druid::{AppLauncher, WindowDesc};

fn main() -> Result<(), druid::PlatformError> {
    let window = WindowDesc::new(ui::main_view).title("View Image");

    let data = ViewImageData::new();

    AppLauncher::with_window(window)
        .delegate(ViewImageDelegate)
        .use_simple_logger()
        .launch(data)
}
