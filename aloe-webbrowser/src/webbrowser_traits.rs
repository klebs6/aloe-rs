crate::ix!();

pub trait WebBrowserComponentInterface:
    PageAboutToLoad
    + PageFinishedLoading
    + PageLoadHadNetworkError
    + WindowCloseRequest
    + NewWindowAttemptingToLoad {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_WebBrowserComponent.h]

pub trait PageAboutToLoad {

    /**
      | This callback is called when the browser
      | is about to navigate to a new location.
      | 
      | You can override this method to perform
      | some action when the user tries to go
      | to a particular Url. To allow the operation
      | to carry on, return true, or return false
      | to stop the navigation happening.
      |
      */
    fn page_about_to_load(&mut self, newurl: &String) -> bool {
        
        todo!();
        /*
            ignoreUnused (newURL); return true;
        */
    }
}

pub trait PageFinishedLoading {

    /**
      | This callback happens when the browser
      | has finished loading a page.
      |
      */
    fn page_finished_loading(&mut self, url: &String)  {
        
        todo!();
        /*
            ignoreUnused (url);
        */
    }
}
    
pub trait PageLoadHadNetworkError {

    /**
      | This callback happens when a network
      | error was encountered while trying
      | to load a page.
      | 
      | You can override this method to show
      | some other error page by calling goToURL.
      | Return true to allow the browser to carry
      | on to the internal browser error page.
      | 
      | The errorInfo contains some platform
      | dependent string describing the error.
      |
      */
    fn page_load_had_network_error(&mut self, error_info: &String) -> bool {
        
        todo!();
        /*
            ignoreUnused (errorInfo); return true;
        */
    }
}

pub trait WindowCloseRequest {

    /**
      | This callback occurs when a script or
      | other activity in the browser asks for
      | the window to be closed.
      |
      */
    fn window_close_request(&mut self)  { }
}

pub trait NewWindowAttemptingToLoad {

    /**
      | This callback occurs when the browser
      | attempts to load a Url in a new window.
      | 
      | This won't actually load the window
      | but gives you a chance to either launch
      | a new window yourself or just load the
      | Url into the current window with goToURL().
      |
      */
    fn new_window_attempting_to_load(&mut self, newurl: &String)  {
        
        todo!();
        /*
            ignoreUnused (newURL);
        */
    }
}
