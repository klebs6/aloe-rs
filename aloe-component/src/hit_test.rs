crate::ix!();

pub trait ContainsPoint {

    /**
      | Checks if a point is in the window.
      | 
      | The position is relative to the top-left
      | of this window, in unscaled peer coordinates.
      | 
      | If trueIfInAChildWindow is false,
      | then this returns false if the point
      | is actually inside a child of this window.
      |
      */
    fn contains(&self, 
        local_pos:                Point<i32>,
        true_if_in_achild_window: bool) -> bool;
}

pub trait HitTest {

    /**
      | Tests whether a given point is inside
      | the component.
      | 
      | Overriding this method allows you to
      | create components which only intercept
      | mouse-clicks within a user-defined
      | area.
      | 
      | This is called to find out whether a particular
      | x, y coordinate is considered to be inside
      | the component or not, and is used by methods
      | such as contains() and getComponentAt()
      | to work out which component the mouse
      | is clicked on.
      | 
      | Components with custom shapes will
      | probably want to override it to perform
      | some more complex hit-testing.
      | 
      | The default implementation of this
      | method returns either true or false,
      | depending on the value that was set by
      | calling setInterceptsMouseClicks()
      | (true is the default return value).
      | 
      | -----------
      | @note
      | 
      | the hit-test region is not related to
      | the opacity with which areas of a component
      | are painted.
      | 
      | Applications should never call hitTest()
      | directly - instead use the contains()
      | method, because this will also test
      | for occlusion by the component's parent.
      | ----------
      | @note
      | 
      | for components on the desktop, this
      | method will be ignored, because it's
      | not always possible to implement this
      | behaviour on all platforms.
      | 
      | -----------
      | @param x
      | 
      | the x coordinate to test, relative to
      | the left hand edge of this component.
      | This value is guaranteed to be greater
      | than or equal to zero, and less than the
      | component's width
      | ----------
      | @param y
      | 
      | the y coordinate to test, relative to
      | the top edge of this component. This
      | value is guaranteed to be greater than
      | or equal to zero, and less than the component's
      | height
      | 
      | -----------
      | @return
      | 
      | true if the click is considered to be
      | inside the component @see setInterceptsMouseClicks,
      | contains
      |
      */
    fn hit_test(&mut self, x: i32, y: i32) -> bool;
}
