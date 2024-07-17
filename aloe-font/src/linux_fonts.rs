crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_linux_Fonts.cpp]

pub fn find_fonts_conf_file() -> Box<XmlElement> {
    
    todo!();
        /*
            static const char* pathsToSearch[] = { "/etc/fonts/fonts.conf",
                                               "/usr/share/fonts/fonts.conf",
                                               "/usr/local/etc/fonts/fonts.conf" };

        for (auto* path : pathsToSearch)
            if (auto xml = parseXML (File (path)))
                return xml;

        return {};
        */
}

impl FTTypefaceList {

    pub fn get_default_font_directories(&mut self) -> Vec<String> {
        
        todo!();
        /*
            StringArray fontDirs;

        fontDirs.addTokens (String (CharPointer_UTF8 (getenv ("ALOE_FONT_PATH"))), ";,", "");
        fontDirs.removeEmptyStrings (true);

        if (fontDirs.isEmpty())
        {
            if (auto fontsInfo = findFontsConfFile())
            {
                for (auto* e : fontsInfo->getChildWithTagNameIterator ("dir"))
                {
                    auto fontPath = e->getAllSubText().trim();

                    if (fontPath.isNotEmpty())
                    {
                        if (e->getStringAttribute ("prefix") == "xdg")
                        {
                            auto xdgDataHome = SystemStats::getEnvironmentVariable ("XDG_DATA_HOME", {});

                            if (xdgDataHome.trimStart().isEmpty())
                                xdgDataHome = "~/.local/share";

                            fontPath = File (xdgDataHome).getChildFile (fontPath).getFullPathName();
                        }

                        fontDirs.add (fontPath);
                    }
                }
            }
        }

        if (fontDirs.isEmpty())
            fontDirs.add ("/usr/X11R6/lib/X11/fonts");

        fontDirs.removeDuplicates (false);
        return fontDirs;
        */
    }
}

impl Typeface {

    /**
      | Creates a new system typeface.
      |
      */
    pub fn create_system_typeface_for_font(&mut self, font: &Font) -> TypefacePtr {
        
        todo!();
        /*
            return new FreeTypeTypeface (font);
        */
    }
    
    /**
      | Attempts to create a font from some raw
      | font file data (e.g. a TTF or OTF file
      | image). The system will take its own
      | internal copy of the data, so you can
      | free the block once this method has returned.
      |
      */
    pub fn create_system_typeface_for(&mut self, 
        data:      *const c_void,
        data_size: usize) -> TypefacePtr {
        
        todo!();
        /*
            return new FreeTypeTypeface (data, dataSize);
        */
    }
    
    /**
      | On some platforms, this allows a specific
      | path to be scanned.
      | 
      | On macOS you can load .ttf and .otf files,
      | otherwise this is only available when
      | using FreeType.
      |
      */
    pub fn scan_folder_for_fonts(&mut self, folder: &File)  {
        
        todo!();
        /*
            FTTypefaceList::getInstance()->scanFontPaths (StringArray (folder.getFullPathName()));
        */
    }
}

impl Font {

    /**
      | Returns a list of all the available typeface
      | font families.
      | 
      | The names returned can be passed into
      | setTypefaceName().
      | 
      | You can use this instead of findFonts()
      | if you only need their font family names,
      | and not font objects.
      |
      */
    pub fn find_all_typeface_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return FTTypefaceList::getInstance()->findAllFamilyNames();
        */
    }
    
    /**
      | Returns a list of all the available typeface
      | font styles.
      | 
      | The names returned can be passed into
      | setTypefaceStyle().
      | 
      | You can use this instead of findFonts()
      | if you only need their styles, and not
      | font objects.
      |
      */
    pub fn find_all_typeface_styles(&mut self, family: &String) -> Vec<String> {
        
        todo!();
        /*
            return FTTypefaceList::getInstance()->findAllTypefaceStyles (family);
        */
    }

    /**
      | Returns the default system typeface
      | for the given font.
      |
      */
    pub fn get_default_typeface_for_font(&mut self, font: &Font) -> TypefacePtr {
        
        todo!();
        /*
            static DefaultFontNames defaultNames;

        Font f (font);
        f.setTypefaceName (defaultNames.getRealFontName (font.getTypefaceName()));
        return Typeface::createSystemTypefaceFor (f);
        */
    }
}

impl TextLayout {

    pub fn create_native_layout(&mut self, _0: &AttributedString) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

///-------------------------
#[no_copy]
pub struct DefaultFontNames {
    default_sans:  String,
    default_serif: String,
    default_fixed: String,
}

impl Default for DefaultFontNames {
    
    fn default() -> Self {
        todo!();
        /*


            : defaultSans  (getDefaultSansSerifFontName()),
              defaultSerif (getDefaultSerifFontName()),
              defaultFixed (getDefaultMonospacedFontName())
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
    
    pub fn pick_best_font(
        names:         &Vec<String>,
        choices_array: *const *const u8) -> String {
        
        todo!();
        /*
            const StringArray choices (choicesArray);

            for (auto& choice : choices)
                if (names.contains (choice, true))
                    return choice;

            for (auto& choice : choices)
                for (auto& name : names)
                    if (name.startsWithIgnoreCase (choice))
                        return name;

            for (auto& choice : choices)
                for (auto& name : names)
                    if (name.containsIgnoreCase (choice))
                        return name;

            return names[0];
        */
    }
    
    pub fn get_default_sans_serif_font_name() -> String {
        
        todo!();
        /*
            StringArray allFonts;
            FTTypefaceList::getInstance()->getSansSerifNames (allFonts);

            static const char* targets[] = { "Verdana", "Bitstream Vera Sans", "Luxi Sans",
                                             "Liberation Sans", "DejaVu Sans", "Sans", nullptr };
            return pickBestFont (allFonts, targets);
        */
    }
    
    pub fn get_default_serif_font_name() -> String {
        
        todo!();
        /*
            StringArray allFonts;
            FTTypefaceList::getInstance()->getSerifNames (allFonts);

            static const char* targets[] = { "Bitstream Vera Serif", "Times", "Nimbus Roman",
                                             "Liberation Serif", "DejaVu Serif", "Serif", nullptr };
            return pickBestFont (allFonts, targets);
        */
    }
    
    pub fn get_default_monospaced_font_name() -> String {
        
        todo!();
        /*
            StringArray allFonts;
            FTTypefaceList::getInstance()->getMonospacedNames (allFonts);

            static const char* targets[] = { "DejaVu Sans Mono", "Bitstream Vera Sans Mono", "Sans Mono",
                                             "Liberation Mono", "Courier", "DejaVu Mono", "Mono", nullptr };
            return pickBestFont (allFonts, targets);
        */
    }
}
