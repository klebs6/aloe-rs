crate::ix!();

pub trait GetLookAndFeel {

    fn get_look_and_feel(&self) -> &mut LookAndFeel;
}

pub trait SetLookAndFeel {

    fn set_look_and_feel(&mut self, new_look_and_feel: *mut LookAndFeel);
}

impl<'a> GetLookAndFeel for Component<'a> {

    /**
      | Finds the appropriate look-and-feel
      | to use for this component.
      | 
      | If the component hasn't had a look-and-feel
      | explicitly set, this will return the
      | parent's look-and-feel, or just the
      | default one if there's no parent.
      | 
      | @see setLookAndFeel, lookAndFeelChanged
      |
      */
    fn get_look_and_feel(&self) -> &mut LookAndFeel {
        
        todo!();
        /*
            for (auto* c = this; c != nullptr; c = c->parentComponent)
            if (auto lf = c->lookAndFeel.get())
                return *lf;

        return LookAndFeel::getDefaultLookAndFeel();
        */
    }
}

impl<'a> SetLookAndFeel for Component<'a> {

    /**
      | Sets the look and feel to use for this
      | component.
      | 
      | This will also change the look and feel
      | for any child components that haven't
      | had their look set explicitly.
      | 
      | The object passed in will not be deleted
      | by the component, so it's the caller's
      | responsibility to manage it. It may
      | be used at any time until this component
      | has been deleted.
      | 
      | Calling this method will also invoke
      | the sendLookAndFeelChange() method.
      | 
      | @see getLookAndFeel, lookAndFeelChanged
      |
      */
    fn set_look_and_feel(&mut self, new_look_and_feel: *mut LookAndFeel)  {
        
        todo!();
        /*
        if (lookAndFeel != newLookAndFeel)
        {
            lookAndFeel = newLookAndFeel;
            sendLookAndFeelChange();
        }
        */
    }
}
