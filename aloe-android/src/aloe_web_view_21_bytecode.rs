crate::ix!();

lazy_static!{
    /*
       WebBrowserComponent::WebBrowserComponentImpl::AloeWebViewClient21_Class   WebBrowserComponent::WebBrowserComponentImpl::AloeWebViewClient21;
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
             CALLBACK (webViewReceivedHttpError, "webViewReceivedHttpError", "(JLandroid/webkit/WebView;Landroid/webkit/WebResourceRequest;Landroid/webkit/WebResourceResponse;)V") 
             CALLBACK (webViewPageLoadStarted, "webViewPageLoadStarted", "(JLandroid/webkit/WebView;Ljava/lang/String;)Z") 
             CALLBACK (webViewPageLoadFinished, "webViewPageLoadFinished", "(JLandroid/webkit/WebView;Ljava/lang/String;)V") 
             CALLBACK (webViewReceivedSslError, "webViewReceivedSslError", "(JLandroid/webkit/WebView;Landroid/webkit/SslErrorHandler;Landroid/net/http/SslError;)V") 
        */
    }
}

declare_jni_class_with_bytecode!{
    AloeWebViewClient21, 
    "com/rmsl/aloe/AloeWebView21$Client", 
    21, 
    AloeWebView21ByteCode, 
    sizeof (AloeWebView21ByteCode)
}

