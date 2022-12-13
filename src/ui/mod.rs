mod button;
mod image_area;
mod tool_navbar;

use self::{image_area::ImageArea, tool_navbar::tool_navbar};
use crate::data::ViewImageData;
use druid::{widget::Flex, Widget};

pub fn main_view() -> impl Widget<ViewImageData> {
    Flex::row()
        .with_child(tool_navbar())
        .with_flex_child(ImageArea::new(), 1.0)
}
