crate::ix!();

#[cfg(target_os="android")]
impl Font {

    pub fn get_default_typeface_for_font(&mut self, font: &Font) -> TypefacePtr {
        
        todo!();
        /*
            static DefaultFontNames defaultNames;

        Font f (font);
        f.setTypefaceName (defaultNames.getRealFontName (font.getTypefaceName()));
        return Typeface::createSystemTypefaceFor (f);
        */
    }

    #[cfg(ALOE_USE_FREETYPE)]
    pub fn find_all_typeface_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return FTTypefaceList::getInstance()->findAllFamilyNames();
        */
    }
    
    #[cfg(ALOE_USE_FREETYPE)]
    pub fn find_all_typeface_styles(&mut self, family: &String) -> Vec<String> {
        
        todo!();
        /*
            return FTTypefaceList::getInstance()->findAllTypefaceStyles (family);
        */
    }

    #[cfg(not(ALOE_USE_FREETYPE))]
    pub fn find_all_typeface_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> results;

        for (auto& f : File ("/system/fonts").findChildFiles (File::findFiles, false, "*.ttf"))
            results.addIfNotAlreadyThere (f.getFileNameWithoutExtension().upToLastOccurrenceOf ("-", false, false));

        return results;
        */
    }
    
    #[cfg(not(ALOE_USE_FREETYPE))]
    pub fn find_all_typeface_styles(&mut self, family: &String) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> results ("Regular");

        for (auto& f : File ("/system/fonts").findChildFiles (File::findFiles, false, family + "-*.ttf"))
            results.addIfNotAlreadyThere (f.getFileNameWithoutExtension().fromLastOccurrenceOf ("-", false, false));

        return results;
        */
    }
}
