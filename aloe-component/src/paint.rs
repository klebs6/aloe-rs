crate::ix!();

pub trait PerformAnyPendingRepaintsNow {

    /**
      | This can be called (from the message
      | thread) to cause the immediate redrawing
      | of any areas of this window that need
      | repainting.
      | 
      | You shouldn't ever really need to use
      | this, it's mainly for special purposes
      | like supporting audio plugins where
      | the host's event loop is out of our control.
      |
      */
    fn perform_any_pending_repaints_now(&mut self);
}

pub trait Repaint {

    /**
      | Invalidates a region of the window to
      | be repainted asynchronously.
      |
      */
    fn repaint(&mut self, area: &Rectangle<i32>);
}

pub trait Paint {

    /**
      | Components can override this method
      | to draw their content.
      | 
      | The paint() method gets called when
      | a region of a component needs redrawing,
      | either because the component's repaint()
      | method has been called, or because something
      | has happened on the screen that means
      | a section of a window needs to be redrawn.
      | 
      | Any child components will draw themselves
      | over whatever this method draws. If
      | you need to paint over the top of your
      | child components, you can also implement
      | the paintOverChildren() method to
      | do this.
      | 
      | If you want to cause a component to redraw
      | itself, this is done asynchronously
      | - calling the repaint() method marks
      | a region of the component as "dirty",
      | and the paint() method will automatically
      | be called sometime later, by the message
      | thread, to paint any bits that need refreshing.
      | In Aloe (and almost all modern UI frameworks),
      | you never redraw something synchronously.
      | 
      | You should never need to call this method
      | directly - to take a snapshot of the component
      | you could use createComponentSnapshot()
      | or paintEntireComponent().
      | 
      | -----------
      | @param g
      | 
      | the graphics context that must be used
      | to do the drawing operations. @see repaint,
      | paintOverChildren, Graphics
      |
      */
    fn paint(&mut self, g: &mut Graphics);
}

pub trait PaintOverChildren {

    /**
      | Components can override this method
      | to draw over the top of their children.
      | 
      | For most drawing operations, it's better
      | to use the normal paint() method, but
      | if you need to overlay something on top
      | of the children, this can be used.
      | 
      | @see paint, Graphics
      |
      */
    fn paint_over_children(&mut self, g: &mut Graphics);
}
