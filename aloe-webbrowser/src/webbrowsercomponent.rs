crate::ix!();

/**
  | A component that displays an embedded
  | web browser.
  | 
  | The browser itself will be platform-dependent.
  | On Mac and iOS it will be
  | 
  | WebKit, on Android it will be Chrome,
  | and on Linux it will be WebKit.
  | 
  | On Windows it will be IE, but if ALOE_USE_WIN_WEBVIEW2
  | is enabled then using the WindowsWebView2WebBrowserComponent
  | wrapper instead of this class directly
  | will attempt to use the Microsoft Edge
  | (Chromium) WebView2. See the documentation
  | of that class for more information about
  | its requirements.
  | 
  | @tags{GUI}
  |
  */
//#[cfg(ALOE_WEB_BROWSER)]
#[no_copy]
#[leak_detector]
pub struct WebBrowserComponent<'a> {
    base:                    Component<'a>,
    browser:                 Box<WebBrowserComponentImpl<'a>>,
    blank_page_shown:        bool, // default = false
    unload_page_when_hidden: bool,
    lasturl:                 String,
    last_headers:            StringArray,
    last_post_data:          MemoryBlock,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_linux_X11_WebBrowserComponent.cpp]
//#[cfg(ALOE_WEB_BROWSER)]
impl<'a> WebBrowserComponent<'a> {
    
    /**
      | Creates a WebBrowserComponent.
      | 
      | Once it's created and visible, send
      | the browser to a Url using goToURL().
      | 
      | -----------
      | @param unloadPageWhenBrowserIsHidden
      | 
      | if this is true, then when the browser
      | component is taken offscreen, it'll
      | clear the current page and replace it
      | with a blank page - this can be handy to
      | stop the browser using resources in
      | the background when it's not actually
      | being used.
      |
      */
    pub fn new(unload_when_hidden: Option<bool>) -> Self {

        let unload_when_hidden = unload_when_hidden.unwrap_or(true);
    
        todo!();
        /*


            : browser (new WebBrowserComponentImpl (*this)),
          unloadPageWhenHidden (unloadWhenHidden)

        ignoreUnused (blankPageShown);
        ignoreUnused (unloadPageWhenHidden);

        setOpaque (true);

        browser->init();
        */
    }
    
    pub fn new_with_files(
        unload_when_hidden: bool,
        _1:                 &File,
        _2:                 &File

    ) -> Self {
    
        todo!();
        /*
        : web_browser_component(unloadWhenHidden),

        
        */
    }
    
    /**
      | Sends the browser to a particular Url.
      | 
      | -----------
      | @param url
      | 
      | the Url to go to.
      | ----------
      | @param headers
      | 
      | an optional set of parameters to put
      | in the HTTP header. If you supply this,
      | it should be a set of string in the form
      | "HeaderKey: HeaderValue"
      | ----------
      | @param postData
      | 
      | an optional block of data that will be
      | attached to the HTTP
      | 
      | POST request
      |
      */
    pub fn go_tourl(&mut self, 
        url:       &String,
        headers:   *const StringArray,
        post_data: *const MemoryBlock)  {
        
        todo!();
        /*
            lastURL = url;

        if (headers != nullptr)
            lastHeaders = *headers;
        else
            lastHeaders.clear();

        if (postData != nullptr)
            lastPostData = *postData;
        else
            lastPostData.reset();

        browser->goToURL (url, headers, postData);
        */
    }
    
    /**
      | Stops the current page loading.
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            browser->stop();
        */
    }
    
    /**
      | Sends the browser back one page.
      |
      */
    pub fn go_back(&mut self)  {
        
        todo!();
        /*
            lastURL.clear();

        browser->goBack();
        */
    }
    
    /**
      | Sends the browser forward one page.
      |
      */
    pub fn go_forward(&mut self)  {
        
        todo!();
        /*
            lastURL.clear();
        browser->goForward();
        */
    }
    
    /**
      | Refreshes the browser.
      |
      */
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            browser->refresh();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::white);
        */
    }
    
    pub fn check_window_association(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn reload_lasturl(&mut self)  {
        
        todo!();
        /*
            if (lastURL.isNotEmpty())
        {
            goToURL (lastURL, &lastHeaders, &lastPostData);
            lastURL.clear();
        }
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            checkWindowAssociation();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (browser != nullptr)
            browser->resized();
        */
    }
    
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            checkWindowAssociation();
        */
    }
    
    pub fn focus_gained(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Clear cookies that the OS has stored
      | for the WebComponents of this application
      |
      */
    pub fn clear_cookies(&mut self)  {
        
        todo!();
        /*
            // Currently not implemented on linux as WebBrowserComponent currently does not
        // store cookies on linux
        jassertfalse;
        */
    }
}
