crate::ix!();

#[cfg(target_os="android")]
pub struct SingleMediaScanner {
    base: MediaScannerConnectionClient,
    msc:  GlobalRef,
    file: String,
}

#[cfg(target_os="android")]
impl SingleMediaScanner {

    pub fn new(filename: &String) -> Self {
    
        todo!();
        /*


            : msc (LocalRef<jobject> (getEnv()->NewObject (MediaScannerConnection,
                                                           MediaScannerConnection.constructor,
                                                           getAppContext().get(),
                                                           CreateJavaInterface (this, "android/media/MediaScannerConnection$MediaScannerConnectionClient").get()))),
              file (filename)

            getEnv()->CallVoidMethod (msc.get(), MediaScannerConnection.connect);
        */
    }
    
    pub fn on_media_scanner_connected(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

            env->CallVoidMethod (msc.get(), MediaScannerConnection.scanFile, javaString (file).get(), 0);
        */
    }
    
    pub fn on_scan_completed(&mut self)  {
        
        todo!();
        /*
            getEnv()->CallVoidMethod (msc.get(), MediaScannerConnection.disconnect);
        */
    }
}
