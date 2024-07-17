crate::ix!();

pub trait IsResizable {

    /**
      | Returns true if the host is allowed to
      | resize the editor's parent window.
      | 
      | @see setResizable
      |
      */
    fn is_resizable(&self) -> bool {

        todo!();
        /*
            return resizableByHost;
        */
    }
}

pub trait SetResizable {

    /**
      | Sets whether the editor is resizable
      | by the host and/or user.
      | 
      | -----------
      | @param allowHostToResize
      | 
      | whether the editor's parent window
      | can be resized by the host. Even if this
      | is false, you can still resize your window
      | yourself by calling setBounds (for
      | example, when a user clicks on a button
      | in your editor to drop out a panel) which
      | will bypass any resizable/constraints
      | checks.
      | ----------
      | @param useBottomRightCornerResizer
      | 
      | if this is true, a ResizableCornerComponent
      | will be added to the editor's bottom-right
      | to allow the user to resize the editor
      | regardless of the value of `allowHostToResize`.
      | 
      | @see setResizeLimits, isResizable
      |
      */
    fn set_resizable(
        &mut self, 
        allow_host_to_resize:            bool,
        use_bottom_right_corner_resizer: bool
    )
    {
        todo!();
        /*
            resizableByHost = allowHostToResize;

        const auto hasResizableCorner = (resizableCorner.get() != nullptr);

        if (useBottomRightCornerResizer != hasResizableCorner)
        {
            if (useBottomRightCornerResizer)
                attachResizableCornerComponent();
            else
                resizableCorner = nullptr;
        }
        */
    }
}

pub trait SetResizeLimits {

    /**
      | This sets the maximum and minimum sizes
      | for the window.
      | 
      | If the window's current size is outside
      | these limits, it will be resized to make
      | sure it's within them.
      | 
      | If you pass in a different minimum and
      | maximum size, this will mark the editor
      | as resizable by the host.
      | 
      | A direct call to setBounds() will bypass
      | any constraint checks, but when the
      | window is dragged by the user or resized
      | by other indirect means, the constrainer
      | will limit the numbers involved.
      | 
      | -----------
      | @note
      | 
      | if you have set a custom constrainer
      | for this editor then this will have no
      | effect, and if you have removed the constrainer
      | with `setConstrainer (nullptr);`
      | then this will re-add the default constrainer
      | with the new limits.
      | 
      | @see setResizable
      |
      */
    fn set_resize_limits(
        &mut self, 
        new_minimum_width:  i32,
        new_minimum_height: i32,
        new_maximum_width:  i32,
        new_maximum_height: i32
    ) {
        todo!();
        /*
            if (constrainer != nullptr && constrainer != &defaultConstrainer)
        {
            // if you've set up a custom constrainer then these settings won't have any effect..
            jassertfalse;
            return;
        }

        resizableByHost = (newMinimumWidth != newMaximumWidth || newMinimumHeight != newMaximumHeight);

        defaultConstrainer.setSizeLimits (newMinimumWidth, newMinimumHeight,
                                          newMaximumWidth, newMaximumHeight);

        if (constrainer == nullptr)
            setConstrainer (&defaultConstrainer);

        if (resizableCorner != nullptr)
            attachResizableCornerComponent();

        setBoundsConstrained (getBounds());
        */
    }
}

pub trait EditorResized {

    fn editor_resized(&mut self, was_resized: bool) {

        todo!();
        /*
            // The host needs to be able to rescale the plug-in editor and applying your own transform will
        // obliterate it! If you want to scale the whole of your UI use Desktop::setGlobalScaleFactor(),
        // or, for applying other transforms, consider putting the component you want to transform
        // in a child of the editor and transform that instead.
        jassert (getTransform() == hostScaleTransform);

        if (wasResized)
        {
            bool resizerHidden = false;

            if (auto* peer = getPeer())
                resizerHidden = peer->isFullScreen() || peer->isKioskMode();

            if (resizableCorner != nullptr)
            {
                resizableCorner->setVisible (! resizerHidden);

                const int resizerSize = 18;
                resizableCorner->setBounds (getWidth() - resizerSize,
                                            getHeight() - resizerSize,
                                            resizerSize, resizerSize);
            }
        }
        */
    }
}
