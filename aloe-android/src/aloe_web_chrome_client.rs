crate::ix!();

lazy_static!{
    /*
    WebBrowserComponent::WebBrowserComponentPimpl::AloeWebChromeClient_Class WebBrowserComponent::WebBrowserComponentPimpl::AloeWebChromeClient;
    */
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
              METHOD (constructor, "<init>",      "(J)V") 
              CALLBACK (webViewCloseWindowRequest,  "webViewCloseWindowRequest",  "(JLandroid/webkit/WebView;)V") 
              CALLBACK (webViewCreateWindowRequest, "webViewCreateWindowRequest", "(JLandroid/webkit/WebView;)V") 
        */
    }
}

declare_jni_class!{
    AloeWebChromeClient, 
    "com/rmsl/aloe/AloeWebView$ChromeClient"
}
