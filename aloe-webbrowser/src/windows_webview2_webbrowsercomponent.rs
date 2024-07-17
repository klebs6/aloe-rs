crate::ix!();

/**
  | If you have enabled the ALOE_USE_WIN_WEBVIEW2
  | flag then this wrapper will attempt
  | to use the Microsoft Edge (Chromium)
  | WebView2 control instead of IE on Windows.
  | It will behave the same as WebBrowserComponent
  | on all other platforms and will fall
  | back to
  | 
  | IE on Windows if the WebView2 requirements
  | are not met.
  | 
  | This requires Microsoft Edge (minimum
  | version 82.0.488.0) to be installed
  | at runtime.
  | 
  | Currently this also requires that WebView2Loader.dll,
  | which can be found in the
  | 
  | Microsoft.Web.WebView package, is
  | installed at runtime. As this is not
  | a standard system DLL, we can't rely
  | on it being found via the normal system
  | DLL search paths.
  | 
  | Therefore in order to use WebView2 you
  | need to ensure that WebView2Loader.dll
  | is installed either to a location covered
  | by the Windows DLL system search paths
  | or to the folder specified in the constructor
  | of this class.
  | 
  | @tags{GUI}
  |
  */
//#[cfg(ALOE_WEB_BROWSER)]
pub struct WindowsWebView2WebBrowserComponent<'a> {
    base: WebBrowserComponent<'a>,
}

//#[cfg(ALOE_WEB_BROWSER)]
impl<'a> WindowsWebView2WebBrowserComponent<'a> {

    /**
      | Creates a WebBrowserComponent that
      | is compatible with the WebView2 control
      | on Windows.
      | 
      | This allows you to specify a custom location
      | for the WebView2Loader.dll as well
      | as a non-default location for storing
      | user data for the browser instance.
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
      | ----------
      | @param dllLocation
      | 
      | the path to WebView2Loader.dll, if
      | this is empty then the default system
      | DLL search paths will be used
      | ----------
      | @param userDataFolder
      | 
      | a directory in which the WebView2 user
      | data will be stored, if this is empty
      | then a directory will be created next
      | to the executable
      |
      */
    pub fn new(
        unload_page_when_browser_is_hidden: Option<bool>,
        dll_location:                       &File,
        user_data_folder:                   &File

    ) -> Self {

        let unload_page_when_browser_is_hidden: bool =
                 unload_page_when_browser_is_hidden.unwrap_or(true);
        todo!();
        /*


            : WebBrowserComponent (unloadPageWhenBrowserIsHidden, dllLocation, userDataFolder)
        */
    }
}
