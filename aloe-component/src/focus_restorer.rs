crate::ix!();

#[no_copy]
pub struct FocusRestorer<'a> {
    last_focus: WeakReference<Component<'a>>,
}

impl<'a> Default for FocusRestorer<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            : lastFocus (Component::getCurrentlyFocusedComponent()
        */
    }
}

impl<'a> Drop for FocusRestorer<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            if (lastFocus != nullptr
                 && lastFocus->isShowing()
                 && ! lastFocus->isCurrentlyBlockedByAnotherModalComponent())
                lastFocus->grabKeyboardFocus();
         */
    }
}
