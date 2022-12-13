use druid::{widget::Flex, FileDialogOptions, UnitPoint, Widget, WidgetExt};

use crate::data::ViewImageData;

use super::button::button;

pub fn tool_navbar() -> impl Widget<ViewImageData> {
    Flex::column()
        .with_child(button("Open Image").on_click(|ctx, _data, _| {
            let dialog_file = FileDialogOptions::new().show_hidden();
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(dialog_file.clone()));
        }))
        .align_horizontal(UnitPoint::TOP)
}
