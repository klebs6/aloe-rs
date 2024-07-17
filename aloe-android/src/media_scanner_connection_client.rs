crate::ix!();

pub trait MediaScannerConnectionClientInterface {

    fn on_media_scanner_connected(&mut self);
    fn on_scan_completed(&mut self);
}

///-----------------------
#[cfg(target_os="android")]
pub struct MediaScannerConnectionClient {
    base: AndroidInterfaceImplementer,
}

#[cfg(target_os="android")]
impl MediaScannerConnectionClient {

    pub fn invoke(&mut self, 
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject {
        
        todo!();
        /*
            auto* env = getEnv();

            auto methodName = aloeString ((jstring) env->CallObjectMethod (method, JavaMethod.getName));

            if (methodName == "onMediaScannerConnected")
            {
                onMediaScannerConnected();
                return nullptr;
            }
            else if (methodName == "onScanCompleted")
            {
                onScanCompleted();
                return nullptr;
            }

            return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
