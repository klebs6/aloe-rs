crate::ix!();

pub trait Initialise {

    fn initialise(&mut self)
    {
        todo!();
        /*
            /*
          ==========================================================================
           In accordance with the terms of the Aloe 6 End-Use License Agreement, the
           Aloe Code in SECTION A cannot be removed, changed or otherwise rendered
           ineffective unless you have a Aloe Indie or Pro license, or are using
           Aloe under the GPL v3 license.

           End User License Agreement: www.aloe.com/aloe-6-licence
          ==========================================================================
        */

        // BEGIN SECTION A

        splashScreen = new ALOESplashScreen (*this);

        // END SECTION A

        attachConstrainer (&defaultConstrainer);
        resizeListener.reset (new AudioProcessorEditorListener (*this));
        addComponentListener (resizeListener.get());
        */
    }
}
