crate::ix!();

pub trait Resized {

    /**
      | Called when this component's size has
      | been changed.
      | 
      | A component can implement this method
      | to do things such as laying out its child
      | components when its width or height
      | changes.
      | 
      | The method is called synchronously
      | as a result of the setBounds or setSize
      | methods, so repeatedly changing a components
      | size will repeatedly call its resized
      | method (unlike things like repainting,
      | where multiple calls to repaint are
      | coalesced together).
      | 
      | If the component is a top-level window
      | on the desktop, its size could also be
      | changed by operating-system factors
      | beyond the application's control.
      | 
      | @see moved, setSize
      |
      */
    fn resized(&mut self);
}

pub trait Moved {

    /**
      | Called when this component's position
      | has been changed.
      | 
      | This is called when the position relative
      | to its parent changes, not when its absolute
      | position on the screen changes (so it
      | won't be called for all child components
      | when a parent component is moved).
      | 
      | The method is called synchronously
      | as a result of the setBounds, setTopLeftPosition
      | or any of the other repositioning methods,
      | and like resized(), it will be called
      | each time those methods are called.
      | 
      | If the component is a top-level window
      | on the desktop, its position could also
      | be changed by operating-system factors
      | beyond the application's control.
      | 
      | @see resized, setBounds
      |
      */
    fn moved(&mut self);
}
