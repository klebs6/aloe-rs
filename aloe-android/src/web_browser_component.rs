crate::ix!();

#[cfg(target_os="android")]
impl<'a> WebBrowserComponent<'a> {

    pub fn new(unload_when_hidden: bool) -> Self {
    
        todo!();
        /*
        : blank_page_shown(false),
        : unload_page_when_hidden(unloadWhenHidden),

            setOpaque (true);

        browser.reset (new WebBrowserComponentImpl (*this));
        addAndMakeVisible (browser.get());
        */
    }
    
    pub fn new(
        unload_when_hidden: bool,
        _1:                 &File,
        _2:                 &File) -> Self {
    
        todo!();
        /*
        : web_browser_component(unloadWhenHidden),

        
        */
    }
    
    pub fn go_tourl(&mut self, 
        url:       &String,
        headers:   *const Vec<String>,
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

        blankPageShown = false;

        browser->goToURL (url, headers, postData);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            browser->stop();
        */
    }
    
    pub fn go_back(&mut self)  {
        
        todo!();
        /*
            browser->goBack();

        lastURL.clear();
        blankPageShown = false;
        */
    }
    
    pub fn go_forward(&mut self)  {
        
        todo!();
        /*
            lastURL.clear();

        browser->goForward();
        */
    }
    
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
            if (isShowing())
        {
            if (blankPageShown)
                goBack();
        }
        else
        {
            if (unloadPageWhenHidden && ! blankPageShown)
            {
                // when the component becomes invisible, some stuff like flash
                // carries on playing audio, so we need to force it onto a blank
                // page to avoid this, (and send it back when it's made visible again).

                blankPageShown = true;
                browser->goToURL ("about:blank", nullptr, nullptr);
            }
        }
        */
    }
    
    pub fn reload_lasturl(&mut self)  {
        
        todo!();
        /*
            if (lastURL.isNotEmpty())
        {
            goToURL (lastURL, &lastHeaders, lastPostData.isEmpty() ? nullptr : &lastPostData);
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
            browser->setSize (getWidth(), getHeight());
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
    
    pub fn clear_cookies(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

        auto cookieManager = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidCookieManager,
                                                                             AndroidCookieManager.getInstance));

        jmethodID clearCookiesMethod = nullptr;

        if (getAndroidSDKVersion() >= 21)
        {
            clearCookiesMethod = env->GetMethodID (AndroidCookieManager, "removeAllCookies", "(Landroid/webkit/ValueCallback;)V");
            env->CallVoidMethod (cookieManager, clearCookiesMethod, 0);
        }
        else
        {
            clearCookiesMethod = env->GetMethodID (AndroidCookieManager, "removeAllCookie", "()V");
            env->CallVoidMethod (cookieManager, clearCookiesMethod);
        }
        */
    }
}
