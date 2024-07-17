crate::ix!();

#[cfg(target_os="android")]
impl Typeface {

    #[cfg(ALOE_USE_FREETYPE)]
    pub fn create_system_typeface_for(&mut self, font: &Font) -> Typeface::Ptr {
        
        todo!();
        /*
            return new FreeTypeTypeface (font);
        */
    }
    
    #[cfg(ALOE_USE_FREETYPE)]
    pub fn scan_folder_for_fonts(&mut self, folder: &File)  {
        
        todo!();
        /*
            FTTypefaceList::getInstance()->scanFontPaths (Vec<String> (folder.getFullPathName()));
        */
    }

    #[cfg(not(ALOE_USE_FREETYPE))]
    pub fn create_system_typeface_for(&mut self, font: &Font) -> TypefacePtr {
        
        todo!();
        /*
            return new AndroidTypeface (font);
        */
    }
    
    #[cfg(not(ALOE_USE_FREETYPE))]
    pub fn create_system_typeface_for(&mut self, 
        data: *const c_void,
        size: usize) -> TypefacePtr {
        
        todo!();
        /*
            return new AndroidTypeface (data, size);
        */
    }
    
    #[cfg(not(ALOE_USE_FREETYPE))]
    pub fn scan_folder_for_fonts(&mut self, _0: &File)  {
        
        todo!();
        /*
            jassertfalse; // not available unless using FreeType
        */
    }
    
    #[cfg(not(ALOE_USE_FREETYPE))]
    pub fn create_native_layout(&mut self, _0: &AttributedString) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
