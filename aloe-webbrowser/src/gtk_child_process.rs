crate::ix!();

#[cfg(target_os="linux")]
pub type WebKitWebView = webkit2gtk::WebView;

#[cfg(target_os="linux")]
pub struct GtkChildProcess {
    out_channel: i32, // default = 0
    receiver:    CommandReceiver,
    webview:     *mut WebKitWebView, // default = nullptr
    decisions:   Vec<*mut WebKitPolicyDecision>,
}

#[cfg(target_os="linux")]
impl CommandReceiverResponder for GtkChildProcess {

}

#[cfg(target_os="linux")]
impl GtkChildProcess {
    
    pub fn new(
        in_channel:         i32,
        out_channel_to_use: i32) -> Self {
    
        todo!();
        /*
        : out_channel(outChannelToUse),
        : receiver(this, inChannel),

        
        */
    }
    
    pub fn entry(&mut self) -> i32 {
        
        todo!();
        /*
            CommandReceiver::setBlocking (outChannel, true);

            WebKitSymbols::getInstance()->aloe_gtk_init (nullptr, nullptr);

            auto* settings = WebKitSymbols::getInstance()->aloe_webkit_settings_new();
            WebKitSymbols::getInstance()->aloe_webkit_settings_set_hardware_acceleration_policy (settings,
                                                                                                 /* WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER */ 2);

            auto* plug      = WebKitSymbols::getInstance()->aloe_gtk_plug_new (0);
            auto* container = WebKitSymbols::getInstance()->aloe_gtk_scrolled_window_new (nullptr, nullptr);

            auto* webviewWidget = WebKitSymbols::getInstance()->aloe_webkit_web_view_new_with_settings (settings);
            webview = (WebKitWebView*) webviewWidget;

            WebKitSymbols::getInstance()->aloe_gtk_container_add ((GtkContainer*) container, webviewWidget);
            WebKitSymbols::getInstance()->aloe_gtk_container_add ((GtkContainer*) plug,      container);

            WebKitSymbols::getInstance()->aloe_webkit_web_view_load_uri (webview, "about:blank");

            aloe_g_signal_connect (webview, "decide-policy",
                                   (GCallback) decidePolicyCallback, this);

            aloe_g_signal_connect (webview, "load-changed",
                                   (GCallback) loadChangedCallback, this);

            aloe_g_signal_connect (webview, "load-failed",
                                   (GCallback) loadFailedCallback, this);

            WebKitSymbols::getInstance()->aloe_gtk_widget_show_all (plug);
            auto wID = (unsigned long) WebKitSymbols::getInstance()->aloe_gtk_plug_get_id ((GtkPlug*) plug);

            ssize_t ret;

            for (;;)
            {
                ret = write (outChannel, &wID, sizeof (wID));

                if (ret != -1 || errno != EINTR)
                    break;
            }

            WebKitSymbols::getInstance()->aloe_g_unix_fd_add (receiver.getFd(), G_IO_IN, pipeReadyStatic, this);
            receiver.tryNextRead();

            WebKitSymbols::getInstance()->aloe_gtk_main();

            WebKitSymbols::getInstance()->deleteInstance();
            return 0;
        */
    }
    
    pub fn go_tourl(&mut self, params: &Var)  {
        
        todo!();
        /*
            static Identifier urlIdentifier ("url");
            auto url = params.getProperty (urlIdentifier, var()).toString();

            WebKitSymbols::getInstance()->aloe_webkit_web_view_load_uri (webview, url.toRawUTF8());
        */
    }
    
    pub fn handle_decision_response(&mut self, params: &Var)  {
        
        todo!();
        /*
            auto* decision = (WebKitPolicyDecision*) ((int64) params.getProperty ("decision_id", var (0)));
            bool allow = params.getProperty ("allow", var (false));

            if (decision != nullptr && decisions.contains (decision))
            {
                if (allow)
                    WebKitSymbols::getInstance()->aloe_webkit_policy_decision_use (decision);
                else
                    WebKitSymbols::getInstance()->aloe_webkit_policy_decision_ignore (decision);

                decisions.removeAllInstancesOf (decision);
                WebKitSymbols::getInstance()->aloe_g_object_unref (decision);
            }
        */
    }
    
    pub fn handle_command(
        &mut self, 
        cmd:    &String,
        params: &Var

    )  {
        
        todo!();
        /*
            if      (cmd == "quit")      quit();
            else if (cmd == "goToURL")   goToURL (params);
            else if (cmd == "goBack")    WebKitSymbols::getInstance()->aloe_webkit_web_view_go_back      (webview);
            else if (cmd == "goForward") WebKitSymbols::getInstance()->aloe_webkit_web_view_go_forward   (webview);
            else if (cmd == "refresh")   WebKitSymbols::getInstance()->aloe_webkit_web_view_reload       (webview);
            else if (cmd == "stop")      WebKitSymbols::getInstance()->aloe_webkit_web_view_stop_loading (webview);
            else if (cmd == "decision")  handleDecisionResponse (params);
        */
    }
    
    pub fn receiver_had_error(&mut self)  {
        
        todo!();
        /*
            exit (-1);
        */
    }
    
    pub fn pipe_ready(
        &mut self, 
        fd: i32,
        _1: GIOCondition

    ) -> bool {
        
        todo!();
        /*
            if (fd == receiver.getFd())
            {
                receiver.tryNextRead();
                return true;
            }

            return false;
        */
    }
    
    pub fn quit(&mut self)  {
        
        todo!();
        /*
            WebKitSymbols::getInstance()->aloe_gtk_main_quit();
        */
    }
    
    pub fn get_uri_string_for_action(&mut self, action: *mut WebKitNavigationAction) -> String {
        
        todo!();
        /*
            auto* request = WebKitSymbols::getInstance()->aloe_webkit_navigation_action_get_request (action);
            return WebKitSymbols::getInstance()->aloe_webkit_uri_request_get_uri (request);
        */
    }
    
    pub fn on_navigation(&mut self, 
        frame_name: String,
        action:     *mut WebKitNavigationAction,
        decision:   *mut WebKitPolicyDecision) -> bool {
        
        todo!();
        /*
            if (decision != nullptr && frameName.isEmpty())
            {
                WebKitSymbols::getInstance()->aloe_g_object_ref (decision);
                decisions.add (decision);

                DynamicObject::Ptr params = new DynamicObject;

                params->setProperty ("url", getURIStringForAction (action));
                params->setProperty ("decision_id", (int64) decision);
                CommandReceiver::sendCommand (outChannel, "pageAboutToLoad", var (params.get()));

                return true;
            }

            return false;
        */
    }
    
    pub fn on_new_window(&mut self, 
        frame_name: String,
        action:     *mut WebKitNavigationAction,
        decision:   *mut WebKitPolicyDecision) -> bool {
        
        todo!();
        /*
            if (decision != nullptr)
            {
                DynamicObject::Ptr params = new DynamicObject;

                params->setProperty ("url", getURIStringForAction (action));
                CommandReceiver::sendCommand (outChannel, "newWindowAttemptingToLoad", var (params.get()));

                // never allow new windows
                WebKitSymbols::getInstance()->aloe_webkit_policy_decision_ignore (decision);

                return true;
            }

            return false;
        */
    }
    
    pub fn on_load_changed(&mut self, load_event: WebKitLoadEvent)  {
        
        todo!();
        /*
            if (loadEvent == WEBKIT_LOAD_FINISHED)
            {
                DynamicObject::Ptr params = new DynamicObject;

                params->setProperty ("url", String (WebKitSymbols::getInstance()->aloe_webkit_web_view_get_uri (webview)));
                CommandReceiver::sendCommand (outChannel, "pageFinishedLoading", var (params.get()));
            }
        */
    }
    
    pub fn on_decide_policy(&mut self, 
        decision:      *mut WebKitPolicyDecision,
        decision_type: WebKitPolicyDecisionType) -> bool {
        
        todo!();
        /*
            switch (decisionType)
            {
            case WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION:
                {
                    auto* navigationDecision = (WebKitNavigationPolicyDecision*) decision;
                    auto* frameName = WebKitSymbols::getInstance()->aloe_webkit_navigation_policy_decision_get_frame_name (navigationDecision);

                    return onNavigation (String (frameName != nullptr ? frameName : ""),
                                         WebKitSymbols::getInstance()->aloe_webkit_navigation_policy_decision_get_navigation_action (navigationDecision),
                                         decision);
                }
                break;
            case WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION:
                {
                    auto* navigationDecision = (WebKitNavigationPolicyDecision*) decision;
                    auto* frameName = WebKitSymbols::getInstance()->aloe_webkit_navigation_policy_decision_get_frame_name (navigationDecision);

                    return onNewWindow  (String (frameName != nullptr ? frameName : ""),
                                         WebKitSymbols::getInstance()->aloe_webkit_navigation_policy_decision_get_navigation_action (navigationDecision),
                                         decision);
                }
                break;
            case WEBKIT_POLICY_DECISION_TYPE_RESPONSE:
                {
                    auto* response = (WebKitNavigationPolicyDecision*) decision;

                    // for now just always allow response requests
                    ignoreUnused (response);
                    WebKitSymbols::getInstance()->aloe_webkit_policy_decision_use (decision);
                    return true;
                }
                break;
            default:
                break;
            }

            return false;
        */
    }
    
    pub fn on_load_failed(&mut self, error: *mut GError)  {
        
        todo!();
        /*
            DynamicObject::Ptr params = new DynamicObject;

            params->setProperty ("error", String (error != nullptr ? error->message : "unknown error"));
            CommandReceiver::sendCommand (outChannel, "pageLoadHadNetworkError", var (params.get()));
        */
    }
    
    pub fn pipe_ready_static(
        fd:        i32,
        condition: GIOCondition,
        user:      gpointer) -> bool {
        
        todo!();
        /*
            return (reinterpret_cast<GtkChildProcess*> (user)->pipeReady (fd, condition) ? TRUE : FALSE);
        */
    }
    
    pub fn decide_policy_callback(
        _0:            *mut WebKitWebView,
        decision:      *mut WebKitPolicyDecision,
        decision_type: WebKitPolicyDecisionType,
        user:          gpointer) -> bool {
        
        todo!();
        /*
            auto& owner = *reinterpret_cast<GtkChildProcess*> (user);
            return (owner.onDecidePolicy (decision, decisionType) ? TRUE : FALSE);
        */
    }
    
    pub fn load_changed_callback(
        _0:         *mut WebKitWebView,
        load_event: WebKitLoadEvent,
        user:       gpointer)  {
        
        todo!();
        /*
            auto& owner = *reinterpret_cast<GtkChildProcess*> (user);
            owner.onLoadChanged (loadEvent);
        */
    }
    
    pub fn load_failed_callback(
        _0:          *mut WebKitWebView,
        load_event:  WebKitLoadEvent,
        failing_uri: *mut u8,
        error:       *mut GError,
        user:        gpointer)  {
        
        todo!();
        /*
            auto& owner = *reinterpret_cast<GtkChildProcess*> (user);
            owner.onLoadFailed (error);
        */
    }
}
