crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/WebBrowserDemo.h]

/**
  | We'll use a subclass of WebBrowserComponent
  | to demonstrate how to get callbacks
  | when the browser changes Url. You don't
  | need to do this, you can just also just
  | use the WebBrowserComponent class
  | directly.
  |
  */
#[no_copy]
#[leak_detector]
pub struct DemoBrowserComponent<'a> {
    base:             WebBrowserComponent<'a>,
    address_text_box: &'a mut TextEditor<'a>,
}

impl<'a> DemoBrowserComponent<'a> {

    pub fn new(address_box: &mut TextEditor) -> Self {
    
        todo!();
        /*
        : address_text_box(addressBox),
        */
    }

    /**
      | This method gets called when the browser
      | is about to go to a new Url..
      |
      */
    pub fn page_about_to_load(&mut self, newurl: &String) -> bool {
        
        todo!();
        /*
            // We'll just update our address box to reflect the new location..
            addressTextBox.setText (newURL, false);

            // we could return false here to tell the browser not to go ahead with
            // loading the page.
            return true;
        */
    }

    /**
      | This method gets called when the browser
      | is requested to launch a new window
      |
      */
    pub fn new_window_attempting_to_load(&mut self, newurl: &String)  {
        
        todo!();
        /*
            // We'll just load the Url into the main window
            goToURL (newURL);
        */
    }
}


