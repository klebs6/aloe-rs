crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_linux_SystemStats.cpp]

extern "C"  {

    #[cfg(target_os="linux")]
    #[cfg(ALOE_BELA)]
    pub fn cobalt_thread_mode() -> i32 {
        
        todo!();
        /*
        
        */
    }
}

#[cfg(target_os="linux")]
#[cfg(not(target_os="bsd"))]
pub fn get_locale_value(key: nl_item) -> String {
    
    todo!();
    /*
        auto oldLocale = ::setlocale (LC_ALL, "");
        auto result = String::fromUTF8 (nl_langinfo (key));
        ::setlocale (LC_ALL, oldLocale);
        return result;
    */
}
