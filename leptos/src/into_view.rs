use tachys::{
    html::attribute::Attribute,
    hydration::Cursor,
    renderer::dom::Dom,
    ssr::StreamBuilder,
    view::{add_attr::AddAnyAttr, Position, PositionState, Render, RenderHtml},
};

pub struct View<T>(T)
where
    T: Sized;

impl<T> View<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub trait IntoView
where
    Self: Sized + Render<Dom> + RenderHtml<Dom> + Send,
{
    fn into_view(self) -> View<Self>;
}

impl<T> IntoView for T
where
    T: Sized + Render<Dom> + RenderHtml<Dom> + Send, //+ AddAnyAttr<Dom>,
{
    fn into_view(self) -> View<Self> {
        View(self)
    }
}

impl<T: IntoView> Render<Dom> for View<T> {
    type State = T::State;

    fn build(self) -> Self::State {
        self.0.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }
}

impl<T: IntoView> RenderHtml<Dom> for View<T> {
    type AsyncOutput = T::AsyncOutput;

    const MIN_LENGTH: usize = <T as RenderHtml<Dom>>::MIN_LENGTH;

    async fn resolve(self) -> Self::AsyncOutput {
        self.0.resolve().await
    }

    fn dry_resolve(&mut self) {
        self.0.dry_resolve();
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) {
        self.0
            .to_html_with_buf(buf, position, escape, mark_branches);
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) where
        Self: Sized,
    {
        self.0.to_html_async_with_buf::<OUT_OF_ORDER>(
            buf,
            position,
            escape,
            mark_branches,
        )
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<Dom>,
        position: &PositionState,
    ) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(cursor, position)
    }
}

impl<T: IntoView> AddAnyAttr<Dom> for View<T> {
    type Output<SomeNewAttr: Attribute<Dom>> =
        <T as AddAnyAttr<Dom>>::Output<SomeNewAttr>;

    fn add_any_attr<NewAttr: Attribute<Dom>>(
        self,
        attr: NewAttr,
    ) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml<Dom>,
    {
        self.0.add_any_attr(attr)
    }
}

pub trait CollectView {
    type View: IntoView;

    fn collect_view(self) -> Vec<Self::View>;
}

impl<It, V> CollectView for It
where
    It: IntoIterator<Item = V>,
    V: IntoView,
{
    type View = V;

    fn collect_view(self) -> Vec<Self::View> {
        self.into_iter().collect()
    }
}
