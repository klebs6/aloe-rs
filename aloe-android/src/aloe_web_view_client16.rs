crate::ix!();

lazy_static!{
    /*
    WebBrowserComponent::WebBrowserComponentImpl::AloeWebViewClient16_Class   WebBrowserComponent::WebBrowserComponentImpl::AloeWebViewClient16;
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
             METHOD (hostDeleted, "hostDeleted", "()V") 
             CALLBACK (webViewPageLoadStarted, "webViewPageLoadStarted", "(JLandroid/webkit/WebView;Ljava/lang/String;)Z") 
             CALLBACK (webViewPageLoadFinished, "webViewPageLoadFinished", "(JLandroid/webkit/WebView;Ljava/lang/String;)V") 
             CALLBACK (webViewReceivedSslError, "webViewReceivedSslError", "(JLandroid/webkit/WebView;Landroid/webkit/SslErrorHandler;Landroid/net/http/SslError;)V") 
        */
    }
}

declare_jni_class_with_bytecode!{
    AloeWebViewClient16, 
    "com/rmsl/aloe/AloeWebView$Client", 
    16, 
    AloeWebView16ByteCode, 
    sizeof (AloeWebView16ByteCode)
}
