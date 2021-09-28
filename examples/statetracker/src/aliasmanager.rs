use std::sync::Arc;

use druid::widget::{Align, Label};
use druid::{lens, Lens, Point, WidgetPod};
use druid::{widget::Flex, Widget};

use libquassel::message::objects::AliasManager;

pub struct AliasManagerWidget {
    inner: WidgetPod<Arc<AliasManager>, Box<dyn Widget<Arc<AliasManager>>>>,
}

impl AliasManagerWidget {
    pub fn new() -> Self {
        let widget = WidgetPod::new(Flex::column()).boxed();

        AliasManagerWidget { inner: widget }
    }
}

impl Widget<Arc<AliasManager>> for AliasManagerWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Arc<AliasManager>,
        env: &druid::Env,
    ) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Arc<AliasManager>,
        env: &druid::Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &Arc<AliasManager>,
        data: &Arc<AliasManager>,
        _env: &druid::Env,
    ) {
        let aliases = lens!(AliasManager, aliases);

        let mut names: Flex<Arc<AliasManager>> = Flex::column();
        let mut sign: Flex<Arc<AliasManager>> = Flex::column();
        let mut expansions: Flex<Arc<AliasManager>> = Flex::column();

        // TODO optimise this whole thing
        aliases.with(data, |aliases| {
            for alias in aliases {
                names.add_child(Align::right(Label::new(alias.name.clone())));
                sign.add_child(Label::new("=>"));
                expansions.add_child(Align::left(Label::new(alias.expansion.clone())));
            }
        });

        let widget: Flex<Arc<AliasManager>> = Flex::row()
            .with_flex_child(names, 1.0)
            .with_flex_child(sign, 1.0)
            .with_flex_child(expansions, 1.0);

        self.inner = WidgetPod::new(widget).boxed();

        ctx.children_changed();
        ctx.request_layout();
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Arc<AliasManager>,
        env: &druid::Env,
    ) -> druid::Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ZERO);
        return size;
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Arc<AliasManager>, env: &druid::Env) {
        self.inner.paint(ctx, data, env)
    }
}
