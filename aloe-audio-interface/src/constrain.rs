crate::ix!();

pub trait GetConstrainer {

    /**
      | Returns the bounds constrainer object
      | that this window is using.
      | 
      | You can access this to change its properties.
      |
      */
    fn get_constrainer(&mut self) -> *mut ComponentBoundsConstrainer
    {
        todo!();
        /*
            return constrainer;
        */
    }
}

pub trait SetConstrainer {

    /**
      | Sets the bounds-constrainer object
      | to use for resizing and dragging this
      | window.
      | 
      | A pointer to the object you pass in will
      | be kept, but it won't be deleted by this
      | object, so it's the caller's responsibility
      | to manage it.
      | 
      | If you pass a nullptr, then no contraints
      | will be placed on the positioning of
      | the window.
      |
      */
    fn set_constrainer(&mut self, new_constrainer: *mut ComponentBoundsConstrainer) {

        todo!();
        /*
            if (constrainer != newConstrainer)
        {
            attachConstrainer (newConstrainer);

            if (constrainer != nullptr)
                resizableByHost = (newConstrainer->getMinimumWidth() != newConstrainer->getMaximumWidth()
                                    || newConstrainer->getMinimumHeight() != newConstrainer->getMaximumHeight());

            if (resizableCorner != nullptr)
                attachResizableCornerComponent();
        }
        */
    }
}

pub trait AttachConstrainer {

    fn attach_constrainer(&mut self, new_constrainer: *mut ComponentBoundsConstrainer) {
        todo!();
        /*
            if (constrainer != newConstrainer)
        {
            constrainer = newConstrainer;
            updatePeer();
        }
        */
    }
}

pub trait SetBoundsConstrained {

    /**
      | Calls the window's setBounds method,
      | after first checking these bounds with
      | the current constrainer.
      | 
      | @see setConstrainer
      |
      */
    fn set_bounds_constrained(&mut self, new_bounds: Rectangle<i32>) {

        todo!();
        /*
            if (constrainer == nullptr)
        {
            setBounds (newBounds);
            return;
        }

        auto currentBounds = getBounds();

        constrainer->setBoundsForComponent (this,
                                            newBounds,
                                            newBounds.getY() != currentBounds.getY() && newBounds.getBottom() == currentBounds.getBottom(),
                                            newBounds.getX() != currentBounds.getX() && newBounds.getRight()  == currentBounds.getRight(),
                                            newBounds.getY() == currentBounds.getY() && newBounds.getBottom() != currentBounds.getBottom(),
                                            newBounds.getX() == currentBounds.getX() && newBounds.getRight()  != currentBounds.getRight());
        */
    }
}
