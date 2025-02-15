crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/aloe_linux_FileChooser.cpp]

pub fn exe_is_available(executable: String) -> bool {
    
    todo!();
    /*
        ChildProcess child;

        if (child.start ("which " + executable))
        {
            child.waitForProcessToFinish (60 * 1000);
            return (child.getExitCode() == 0);
        }

        return false;
    */
}

impl<'a> FileChooser<'a> {
    
    pub fn is_platform_dialog_available(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_DISABLE_NATIVE_FILECHOOSERS
        return false;
       #else
        static bool canUseNativeBox = exeIsAvailable ("zenity") || exeIsAvailable ("kdialog");
        return canUseNativeBox;
       #endif
        */
    }
    
    pub fn show_platform_dialog(
        &mut self, 
        owner: &mut FileChooser,
        flags: i32,
        _2:    *mut FilePreviewComponent

    ) -> Arc<dyn FileChooserImpl> {
        
        todo!();
        /*
            return std::make_shared<FileChooserNative> (owner, flags);
        */
    }
}
