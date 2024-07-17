crate::ix!();

impl<'a> Clicked      for Button<'a> { }
impl<'a> TriggerClick for Button<'a> { }
impl<'a> ClickedWith  for Button<'a> { type Modifiers = ModifierKeys; }

impl<'a> GetImageBounds for DrawableButton<'a> { 

    fn get_image_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            auto r = getLocalBounds();

        if (style != ImageStretched)
        {
            auto indentX = jmin (edgeIndent, proportionOfWidth  (0.3f));
            auto indentY = jmin (edgeIndent, proportionOfHeight (0.3f));

            if (shouldDrawButtonBackground())
            {
                indentX = jmax (getWidth()  / 4, indentX);
                indentY = jmax (getHeight() / 4, indentY);
            }
            else if (style == ImageAboveTextLabel)
            {
                r = r.withTrimmedBottom (jmin (16, proportionOfHeight (0.25f)));
            }

            r = r.reduced (indentX, indentY);
        }

        return r.toFloat();
        */
    }
}

pub trait GetImageBounds {

    /**
      | Can be overridden to specify a custom
      | position for the image within the button.
      |
      */
    fn get_image_bounds(&self) -> Rectangle<f32>;
}

pub trait TriggerClick {

    /**
      | Causes the button to act as if it's been
      | clicked.
      | 
      | This will asynchronously make the button
      | draw itself going down and up, and will
      | then call back the clicked() method
      | as if mouse was clicked on it.
      | 
      | @see clicked
      |
      */
    fn trigger_click(&mut self) {}
}

pub trait Clicked {

    /**
      | This method is called when the button
      | has been clicked.
      | 
      | Subclasses can override this to perform
      | whatever actions they need to do. In
      | general, you wouldn't use this method
      | to receive clicks, but should get your
      | callbacks by attaching a std::function
      | to the onClick callback, or adding a
      | Button::ButtonListener. @see triggerClick,
      | onClick
      |
      */
    fn clicked(&mut self) {}
}

pub trait ClickedWith {

    type Modifiers;

    /**
      | This method is called when the button
      | has been clicked.
      | 
      | By default it just calls clicked(),
      | but you might want to override it to handle
      | things like clicking when a modifier
      | key is pressed, etc.
      | 
      | @see ModifierKeys
      |
      */
    fn clicked_with(&mut self, modifiers: Self::Modifiers) {}
}

pub trait PaintButton {

    /**
      | Subclasses should override this to
      | actually paint the button's contents.
      | 
      | It's better to use this than the paint
      | method, because it gives you information
      | about the over/down state of the button.
      | 
      | -----------
      | @param g
      | 
      | the graphics context to use
      | ----------
      | @param shouldDrawButtonAsHighlighted
      | 
      | true if the button is either in the 'over'
      | or 'down' state
      | ----------
      | @param shouldDrawButtonAsDown
      | 
      | true if the button should be drawn in
      | the 'down' position
      |
      */
    fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool);
}

pub trait ButtonStateChanged {

    /**
      | Called when the button's up/down/over
      | state changes.
      | 
      | Subclasses can override this if they
      | need to do something special when the
      | button goes up or down.
      | 
      | @see isDown, isOver
      |
      */
    fn button_state_changed(&mut self);
}

pub trait InternalClickCallback {

    fn internal_click_callback(&mut self, _0: &ModifierKeys);
}

pub trait ChangeWidthToFitText {

    fn change_width_to_fit_text(&mut self);

    fn change_width_to_fit_text_with_height(&mut self, new_height: i32);
}

pub trait GetBestWidthForHeight {

    fn get_best_width_for_height(&mut self, button_height: i32) -> i32;
}
