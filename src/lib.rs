use iced_core::{
    Clipboard, Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget,
    layout::{Limits, Node},
    mouse::{Cursor, Interaction},
    overlay,
    renderer::Style,
    widget::{self, Operation},
};
use std::cell::{RefCell, RefMut};

#[must_use]
pub fn persistent<'a, Message, Theme, Renderer>(
    child: impl Into<Element<'a, Message, Theme, Renderer>>,
    tree: &'a Tree,
) -> Persistent<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    Persistent::new(child, tree)
}

#[derive(Debug)]
pub struct Tree(RefCell<widget::Tree>);

impl Tree {
    #[must_use]
    pub fn empty() -> Self {
        Self(RefCell::new(widget::Tree::empty()))
    }
}

pub struct Persistent<'a, Message, Theme, Renderer> {
    child: Element<'a, Message, Theme, Renderer>,
    tree: RefMut<'a, widget::Tree>,
}

impl<'a, Message, Theme, Renderer> Persistent<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    #[must_use]
    pub fn new(child: impl Into<Element<'a, Message, Theme, Renderer>>, tree: &'a Tree) -> Self {
        let child = child.into();
        let tree = tree.0.borrow_mut();

        Self { child, tree }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Persistent<'_, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.child.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.child.as_widget().size_hint()
    }

    fn layout(&mut self, _: &mut widget::Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.tree.diff(&self.child);
        self.child
            .as_widget_mut()
            .layout(&mut self.tree, renderer, limits)
    }

    fn draw(
        &self,
        _: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.child
            .as_widget()
            .draw(&self.tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn operate(
        &mut self,
        _: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.child
            .as_widget_mut()
            .operate(&mut self.tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        _: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.child.as_widget_mut().update(
            &mut self.tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _: &widget::Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> Interaction {
        self.child
            .as_widget()
            .mouse_interaction(&self.tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        _: &'a mut widget::Tree,
        layout: Layout<'a>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> {
        self.child
            .as_widget_mut()
            .overlay(&mut self.tree, layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<Persistent<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(value: Persistent<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
