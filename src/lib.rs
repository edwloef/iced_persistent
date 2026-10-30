use iced_core::{
    Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget,
    layout::Limits,
    mouse::{Cursor, Interaction},
    overlay,
    renderer::Style,
    widget::{self, Meta, Operation},
};
use std::cell::{RefCell, RefMut};

#[must_use]
pub fn persistent<W>(child: W, tree: &Tree) -> Persistent<'_, W> {
    Persistent::new(child, tree)
}

#[derive(Debug)]
pub struct Tree(RefCell<widget::Tree>);

impl Default for Tree {
    fn default() -> Self {
        Self(RefCell::new(widget::Tree::empty()))
    }
}

pub struct Persistent<'a, W> {
    child: W,
    tree: RefMut<'a, widget::Tree>,
}

impl<'a, W> Persistent<'a, W> {
    #[must_use]
    pub fn new(child: W, tree: &'a Tree) -> Self {
        let tree = tree.0.borrow_mut();
        Self { child, tree }
    }
}

impl<W> Meta for Persistent<'_, W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Persistent<'_, W>
where
    Renderer: iced_core::Renderer,
    W: Widget<Message, Theme, Renderer>,
{
    fn size(&self) -> Size<Length> {
        self.child.size()
    }

    fn layout(&mut self, _: &mut widget::Tree, renderer: &Renderer, limits: &Limits) {
        self.child.layout(&mut self.tree, renderer, limits);
    }

    fn draw(
        &self,
        _: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.child
            .draw(&self.tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn diff(&mut self, _: &mut widget::Tree) {
        self.tree.diff(&mut self.child);
    }

    fn operate(
        &mut self,
        _: &mut widget::Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.child
            .operate(&mut self.tree, layout, viewport, renderer, operation);
    }

    fn update(
        &mut self,
        _: &mut widget::Tree,
        event: &Event,
        layout: Layout,
        cursor: Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.child.update(
            &mut self.tree,
            event,
            layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _: &widget::Tree,
        layout: Layout,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> Interaction {
        self.child
            .mouse_interaction(&self.tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        _: &'a mut widget::Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'a, Message, Theme, Renderer>> {
        self.child.overlay(
            &mut self.tree,
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}
