use druid::{
    widget::{Label, Painter},
    Color, RenderContext, Widget, WidgetExt,
};

use crate::{data::ViewImageData, theme};

pub fn button(label: impl Into<String>) -> impl Widget<ViewImageData> {
    let painter_label = Painter::new(|ctx, _data, _env| {
        let size_rect = ctx.size().to_rect();
        ctx.fill(size_rect, &theme::PRIMARY_DARK);

        if ctx.is_hot() {
            ctx.stroke(size_rect, &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(size_rect, &theme::PRIMARY_LIGHT);
        }
    });

    Label::new(label.into())
        .with_text_size(24.)
        .center()
        .padding(10.)
        .background(painter_label)
}
