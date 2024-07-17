crate::ix!();

pub struct DefaultFontNames {
    default_sans:     String,
    default_serif:    String,
    default_fixed:    String,
    default_fallback: String,
}

impl Default for DefaultFontNames {
    
    fn default() -> Self {
        todo!();
        /*


            : defaultSans  ("sans"),
              defaultSerif ("serif"),
              defaultFixed ("monospace"),
              defaultFallback ("sans")
        */
    }
}

impl DefaultFontNames {
    
    pub fn get_real_font_name(&self, face_name: &String) -> String {
        
        todo!();
        /*
            if (faceName == Font::getDefaultSansSerifFontName())    return defaultSans;
            if (faceName == Font::getDefaultSerifFontName())        return defaultSerif;
            if (faceName == Font::getDefaultMonospacedFontName())   return defaultFixed;

            return faceName;
        */
    }
}

