crate::ix!();

pub struct TabbedButtonBarTabInfo<'a>
{
    button: Box<TabBarButton<'a>>,
    name:   String,
    colour: Colour,
}
