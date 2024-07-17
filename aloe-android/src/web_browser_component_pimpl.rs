crate::ix!();

#[cfg(target_os="android")]
pub struct WebBrowserComponentPimpl<'a> {
    base:                   AndroidViewComponent<'a>,
    base2:                  AsyncUpdater<'a>,
    owner:                  &'a mut WebBrowserComponent<'a>,
    aloe_web_chrome_client: GlobalRef,
    aloe_web_view_client:   GlobalRef,
    connection_thread:      Box<ConnectionThread<'a>>,
    response_ready_event:   WaitableEvent,
    master_reference:       WeakReferenceMaster<WebBrowserComponentPimpl<'a>>,
}

#[cfg(target_os="android")]
impl<'a> Drop for WebBrowserComponentPimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            auto* env = getEnv();

            env->CallVoidMethod ((jobject) getView(), AndroidWebView.stopLoading);

            auto defaultChromeClient = LocalRef<jobject> (env->NewObject (AndroidWebChromeClient, AndroidWebChromeClient.constructor));
            auto defaultViewClient   = LocalRef<jobject> (env->NewObject (AndroidWebViewClient,   AndroidWebViewClient  .constructor));

            env->CallVoidMethod ((jobject) getView(), AndroidWebView.setWebChromeClient, defaultChromeClient.get());
            env->CallVoidMethod ((jobject) getView(), AndroidWebView.setWebViewClient,   defaultViewClient  .get());

            masterReference.clear();

            // if other Java thread is waiting for us to respond to page load request
            // wake it up immediately (false answer will be sent), so that it releases
            // the lock we need when calling hostDeleted.
            responseReadyEvent.signal();

            env->CallVoidMethod (aloeWebViewClient, getAndroidSDKVersion() >= 21 ? AloeWebViewClient21.hostDeleted
                                                                                 : AloeWebViewClient16.hostDeleted);
        */
    }
}

#[cfg(target_os="android")]
impl<'a> WebBrowserComponentPimpl<'a> {
    
    pub fn new(o: &mut WebBrowserComponent) -> Self {
    
        todo!();
        /*
        : owner(o),

            auto* env = getEnv();

            setView (env->NewObject (AndroidWebView, AndroidWebView.constructor, getMainActivity().get()));

            auto settings = LocalRef<jobject> (env->CallObjectMethod ((jobject) getView(), AndroidWebView.getSettings));
            env->CallVoidMethod (settings, WebSettings.setJavaScriptEnabled, true);
            env->CallVoidMethod (settings, WebSettings.setBuiltInZoomControls, true);
            env->CallVoidMethod (settings, WebSettings.setDisplayZoomControls, false);
            env->CallVoidMethod (settings, WebSettings.setSupportMultipleWindows, true);

            aloeWebChromeClient = GlobalRef (LocalRef<jobject> (env->NewObject (AloeWebChromeClient, AloeWebChromeClient.constructor,
                                                                                reinterpret_cast<jlong> (this))));
            env->CallVoidMethod ((jobject) getView(), AndroidWebView.setWebChromeClient, aloeWebChromeClient.get());

            auto sdkVersion = getAndroidSDKVersion();

            if (sdkVersion >= 21)
                aloeWebViewClient = GlobalRef (LocalRef<jobject> (env->NewObject (AloeWebViewClient21, AloeWebViewClient21.constructor,
                                                                                  reinterpret_cast<jlong> (this))));
            else
                aloeWebViewClient = GlobalRef (LocalRef<jobject> (env->NewObject (AloeWebViewClient16, AloeWebViewClient16.constructor,
                                                                                  reinterpret_cast<jlong> (this))));

            env->CallVoidMethod ((jobject) getView(), AndroidWebView.setWebViewClient, aloeWebViewClient.get());
        */
    }
    
    pub fn go_tourl(&mut self, 
        url:       &String,
        headers:   *const Vec<String>,
        post_data: *const MemoryBlock)  {
        
        todo!();
        /*
            auto* env = getEnv();

            if (headers == nullptr && postData == nullptr)
            {
                env->CallVoidMethod ((jobject) getView(), AndroidWebView.loadUrl, javaString (url).get(), 0);
            }
            else if (headers != nullptr && postData == nullptr)
            {
                auto headersMap = LocalRef<jobject> (env->NewObject (JavaHashMap,
                                                                     JavaHashMap.constructorWithCapacity,
                                                                     headers->size()));

                for (const auto& header : *headers)
                {
                    auto name  = header.upToFirstOccurrenceOf (":", false, false).trim();
                    auto value = header.fromFirstOccurrenceOf (":", false, false).trim();

                    env->CallObjectMethod (headersMap, JavaMap.put,
                                           javaString (name).get(),
                                           javaString (value).get());
                }

                env->CallVoidMethod ((jobject) getView(), AndroidWebView.loadUrl,
                                     javaString (url).get(), headersMap.get());
            }
            else if (headers == nullptr && postData != nullptr)
            {
                auto dataStringAloe = postData->toString();
                auto dataStringJava = javaString (dataStringAloe);
                auto encodingString = LocalRef<jobject> (env->CallStaticObjectMethod (URLEncoder, URLEncoder.encode,
                                                                                      dataStringJava.get(), javaString ("utf-8").get()));

                auto bytes = LocalRef<jbyteArray> ((jbyteArray) env->CallObjectMethod (encodingString, JavaString.getBytes));

                env->CallVoidMethod ((jobject) getView(), AndroidWebView.postUrl,
                                     javaString (url).get(), bytes.get());
            }
            else if (headers != nullptr && postData != nullptr)
            {
                // There is no support for both extra headers and post data in Android WebView, so
                // we need to open Url manually.

                Url urlToUse = Url (url).withPOSTData (*postData);
                connectionThread.reset (new ConnectionThread (*this, urlToUse, *headers));
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            connectionThread = nullptr;

            getEnv()->CallVoidMethod ((jobject) getView(), AndroidWebView.stopLoading);
        */
    }
    
    pub fn go_back(&mut self)  {
        
        todo!();
        /*
            connectionThread = nullptr;

            auto* env = getEnv();
            auto view = (jobject) getView();

            if (env->CallBooleanMethod (view, AndroidWebView.canGoBack))
                env->CallVoidMethod (view, AndroidWebView.goBack);
            else
                owner.reloadLastURL();
        */
    }
    
    pub fn go_forward(&mut self)  {
        
        todo!();
        /*
            connectionThread = nullptr;

            getEnv()->CallVoidMethod ((jobject) getView(), AndroidWebView.goForward);
        */
    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            connectionThread = nullptr;

            getEnv()->CallVoidMethod ((jobject) getView(), AndroidWebView.reload);
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            jassert (connectionThread != nullptr);

            if (connectionThread == nullptr)
                return;

            auto& result = connectionThread->getResult();

            if (result.statusCode >= 200 && result.statusCode < 300)
            {
                auto url = javaString (result.url);
                auto data = javaString (result.data);
                auto mimeType = javaString ("text/html");
                auto encoding = javaString ("utf-8");

                getEnv()->CallVoidMethod ((jobject) getView(), AndroidWebView.loadDataWithBaseURL,
                                          url.get(), data.get(), mimeType.get(),
                                          encoding.get(), 0);
            }
            else
            {
                owner.pageLoadHadNetworkError (result.description);
            }
        */
    }
    
    pub fn handle_page_about_to_load(&mut self, url: &String) -> bool {
        
        todo!();
        /*
            if (MessageManager::getInstance()->isThisTheMessageThread())
                return owner.pageAboutToLoad (url);

            WeakReference<WebBrowserComponentPimpl> weakRef (this);

            if (weakRef == nullptr)
                return false;

            responseReadyEvent.reset();

            bool shouldLoad = false;

            MessageManager::callAsync ([weakRef, url, &shouldLoad]
            {
                if (weakRef == nullptr)
                    return;

                shouldLoad = weakRef->owner.pageAboutToLoad (url);

                weakRef->responseReadyEvent.signal();
            });

            responseReadyEvent.wait (-1);

            return shouldLoad;
        */
    }

    #[JNICALL]
    pub fn web_view_page_load_started(
        _0:       *mut JNIEnv,
        activity: jobject,
        host:     i64,
        web_view: jobject,
        url:      jstring) -> bool {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<WebBrowserComponent::WebBrowserComponentPimpl*> (host))
                return myself->handlePageAboutToLoad (aloeString (url));

            return 0;
        */
    }
    
    #[JNICALL]
    pub fn web_view_page_load_finished(
        _0:       *mut JNIEnv,
        activity: jobject,
        host:     i64,
        web_view: jobject,
        url:      jstring)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<WebBrowserComponent::WebBrowserComponentPimpl*> (host))
                myself->owner.pageFinishedLoading (aloeString (url));
        */
    }
    
    #[JNICALL]
    pub fn web_view_received_http_error(
        _0:             *mut JNIEnv,
        activity:       jobject,
        host:           i64,
        web_view:       jobject,
        request:        jobject,
        error_response: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<WebBrowserComponent::WebBrowserComponentPimpl*> (host))
                myself->webReceivedHttpError (errorResponse);
        */
    }
    
    #[JNICALL]
    pub fn web_view_received_ssl_error(
        _0:                *mut JNIEnv,
        activity:          jobject,
        host:              i64,
        web_view:          jobject,
        ssl_error_handler: jobject,
        ssl_error:         jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();

            if (auto* myself = reinterpret_cast<WebBrowserComponent::WebBrowserComponentPimpl*> (host))
            {
                auto errorString = LocalRef<jstring> ((jstring) env->CallObjectMethod (sslError, SslError.toString));

                myself->owner.pageLoadHadNetworkError (aloeString (errorString));
            }
        */
    }
    
    #[JNICALL]
    pub fn web_view_close_window_request(
        _0:       *mut JNIEnv,
        activity: jobject,
        host:     i64,
        web_view: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<WebBrowserComponent::WebBrowserComponentPimpl*> (host))
                myself->owner.windowCloseRequest();
        */
    }
    
    #[JNICALL]
    pub fn web_view_create_window_request(
        _0:       *mut JNIEnv,
        activity: jobject,
        host:     i64,
        web_view: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<WebBrowserComponent::WebBrowserComponentPimpl*> (host))
                myself->owner.newWindowAttemptingToLoad ({});
        */
    }
    
    pub fn web_received_http_error(&mut self, error_response: jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();

            LocalRef<jclass> responseClass (env->FindClass ("android/webkit/WebResourceResponse"));

            if (responseClass != nullptr)
            {
                jmethodID method = env->GetMethodID (responseClass, "getReasonPhrase", "()Ljava/lang/String;");

                if (method != nullptr)
                {
                    auto errorString = LocalRef<jstring> ((jstring) env->CallObjectMethod (errorResponse, method));

                    owner.pageLoadHadNetworkError (aloeString (errorString));
                    return;
                }
            }

            // Should never get here!
            jassertfalse;
            owner.pageLoadHadNetworkError ({});
        */
    }
}
