crate::ix!();

pub trait CreateScrollBarComponent {

    /**
      | Creates the Scrollbar components that
      | will be added to the Viewport.
      | 
      | Subclasses can override this if they
      | need to customise the scrollbars in
      | some way.
      |
      */
    fn create_scroll_bar_component(&mut self, is_vertical: bool) -> *mut ScrollBar;
}

pub trait VisibleAreaChanged {

    /**
      | Callback method that is called when
      | the visible area changes.
      | 
      | This will be called when the visible
      | area is moved either be scrolling or
      | by calls to setViewPosition(), etc.
      |
      */
    fn visible_area_changed(&mut self, new_visible_area: &Rectangle<i32>);
}

pub trait ViewedComponentChanged {

    /**
      | Callback method that is called when
      | the viewed component is added, removed
      | or swapped.
      |
      */
    fn viewed_component_changed(&mut self, new_component: *mut Component);
}
