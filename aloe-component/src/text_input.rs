crate::ix!();

pub trait TextInputRequired {

    /**
      | Tells the window that text input may
      | be required at the given position.
      | 
      | This may cause things like a virtual
      | on-screen keyboard to appear, depending
      | on the OS.
      |
      */
    fn text_input_required(
        &mut self, 
        position: Point<i32>,
        _1:       &mut dyn TextInputTarget
    );
}

pub trait DismissPendingTextInput {

    /**
      | If there's some kind of OS input-method
      | in progress, this should dismiss it.
      |
      */
    fn dismiss_pending_text_input(&mut self);
}
