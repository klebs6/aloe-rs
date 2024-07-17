crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ListBoxMouseMoveSelector<'a> {
    owner: &'a mut ListBox<'a>,
}

impl<'a> MouseListener    for ListBoxMouseMoveSelector<'a> { }
impl<'a> MouseMagnify     for ListBoxMouseMoveSelector<'a> {}
impl<'a> MouseWheelMove   for ListBoxMouseMoveSelector<'a> {}
impl<'a> MouseDoubleClick for ListBoxMouseMoveSelector<'a> {}
impl<'a> MouseUp          for ListBoxMouseMoveSelector<'a> {}
impl<'a> MouseDrag        for ListBoxMouseMoveSelector<'a> {}
impl<'a> MouseDown        for ListBoxMouseMoveSelector<'a> {}
impl<'a> MouseEnter       for ListBoxMouseMoveSelector<'a> {}

impl<'a> MouseExit for ListBoxMouseMoveSelector<'a> {

    fn mouse_exit(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseMove (e);
        */
    }
}

impl<'a> MouseMove for ListBoxMouseMoveSelector<'a> {

    fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto pos = e.getEventRelativeTo (&owner).position.toInt();
            owner.selectRow (owner.getRowContainingPosition (pos.x, pos.y), true);
        */
    }
}

impl<'a> Drop for ListBoxMouseMoveSelector<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            owner.removeMouseListener (this);
        */
    }
}

impl<'a> ListBoxMouseMoveSelector<'a> {

    pub fn new(lb: &mut ListBox) -> Self {
    
        todo!();
        /*
        : owner(lb),

            owner.addMouseListener (this, true);
        */
    }
}
