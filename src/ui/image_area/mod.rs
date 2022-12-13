mod view;

use druid::{
    widget::SizedBox, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetId,
};

use crate::data::ViewImageData;

pub struct ImageArea {
    inner: Box<dyn Widget<ViewImageData>>,
}

impl ImageArea {
    pub fn new() -> ImageArea {
        ImageArea {
            inner: SizedBox::empty().boxed(),
        }
    }

    pub fn rebuild_inner(&mut self, data: &ViewImageData) {
        self.inner = view::view_image(data);
    }
}

impl Widget<ViewImageData> for ImageArea {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut ViewImageData, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &ViewImageData,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &ViewImageData,
        data: &ViewImageData,
        _env: &Env,
    ) {
        if !old_data.same(data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &ViewImageData,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ViewImageData, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}
