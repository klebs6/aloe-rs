crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_LocalisedStrings.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_LocalisedStrings.cpp]

/**
  | Used to convert strings to localised
  | foreign-language versions.
  | 
  | This is basically a look-up table of
  | strings and their translated equivalents.
  | It can be loaded from a text file, so that
  | you can supply a set of localised versions
  | of strings that you use in your app.
  | 
  | To use it in your code, simply call the
  | translate() method on each string that
  | might have foreign versions, and if
  | none is found, the method will just return
  | the original string.
  | 
  | The translation file should start with
  | some lines specifying a description
  | of the language it contains, and also
  | a list of ISO country codes where it might
  | be appropriate to use the file. After
  | that, each line of the file should contain
  | a pair of quoted strings with an '=' sign.
  | 
  | E.g. for a french translation, the file
  | might be:
  | 
  | @code
  | 
  | language: French
  | countries: fr be mc ch lu
  | 
  | "hello" = "bonjour"
  | "goodbye" = "au revoir"
  | 
  | @endcode
  | 
  | If the strings need to contain a quote
  | character, they can use '\"' instead,
  | and if the first non-whitespace character
  | on a line isn't a quote, then it's ignored,
  | (you can use this to add comments).
  | 
  | Note that this is a singleton class,
  | so don't create or destroy the object
  | directly. There's also a TRANS(text)
  | macro defined to make it easy to use the
  | this.
  | 
  | E.g. @code
  | 
  | printSomething (TRANS("hello"));
  | 
  | @endcode
  | 
  | This macro is used in the Aloe classes
  | themselves, so your application has
  | a chance to intercept and translate
  | any internal Aloe text strings that
  | might be shown. (You can easily get a
  | list of all the messages by searching
  | for the TRANS() macro in the Aloe source
  | code).
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct LocalisedStrings {
    language_name: AloeString,
    country_codes: StringArray,
    translations:  StringPairArray,
    fallback:      Box<LocalisedStrings>,
}

impl LocalisedStrings {

    /**
      | Attempts to look up a string and return
      | its localised version.
      | 
      | If the string isn't found in the list,
      | the original string will be returned.
      |
      */
    pub fn translate(&self, text: &str) -> AloeString {
        
        todo!();
        /*
            if (fallback != nullptr && ! translations.containsKey (text))
            return fallback->translate (text);

        return translations.getValue (text, text);
        */
    }
    
    /**
      | Returns the name of the language specified
      | in the translation file.
      | 
      | This is specified in the file using a
      | line starting with "language:", e.g.
      | 
      | @code
      | 
      | language: german
      | 
      | @endcode
      |
      */
    pub fn get_language_name(&self) -> AloeString {
        
        todo!();
        /*
            return languageName;
        */
    }

    /**
      | Returns the list of suitable country
      | codes listed in the translation file.
      | 
      | These is specified in the file using
      | a line starting with "countries:",
      | e.g.
      | 
      | @code
      | 
      | countries: fr be mc ch lu
      | 
      | @endcode
      | 
      | The country codes are supposed to be
      | 2-character ISO compliant codes.
      |
      */
    pub fn get_country_codes(&self) -> &StringArray {
        
        todo!();
        /*
            return countryCodes;
        */
    }

    /**
      | Provides access to the actual list of
      | mappings.
      |
      */
    pub fn get_mappings(&self) -> &StringPairArray {
        
        todo!();
        /*
            return translations;
        */
    }
    
    /**
      | Creates a set of translations from the
      | text of a translation file.
      | 
      | When you create one of these, you can
      | call setCurrentMappings() to make
      | it the set of mappings that the system's
      | using.
      |
      */
    pub fn new_with_file_contents(
        file_contents: &AloeString,
        ignore_case:   bool) -> Self {
    
        todo!();
        /*


            loadFromText (fileContents, ignoreCase);
        */
    }
    
    /**
      | Creates a set of translations from a
      | file.
      | 
      | When you create one of these, you can
      | call setCurrentMappings() to make
      | it the set of mappings that the system's
      | using.
      |
      */
    pub fn new_with_file(
        file_to_load: &std::fs::File,
        ignore_case:  bool) -> Self {
    
        todo!();
        /*


            loadFromText (fileToLoad.loadFileAsString(), ignoreCase);
        */
    }
    
    pub fn new_from_other(other: &LocalisedStrings) -> Self {
    
        todo!();
        /*


            : languageName (other.languageName), countryCodes (other.countryCodes),
          translations (other.translations), fallback (createCopyIfNotNull (other.fallback.get()))
        */
    }
    
    pub fn assign_from(&mut self, other: &LocalisedStrings) -> &mut LocalisedStrings {
        
        todo!();
        /*
            languageName = other.languageName;
        countryCodes = other.countryCodes;
        translations = other.translations;
        fallback.reset (createCopyIfNotNull (other.fallback.get()));
        return *this;
        */
    }
    
    /**
      | Attempts to look up a string and return
      | its localised version.
      | 
      | If the string isn't found in the list,
      | the resultIfNotFound string will be
      | returned.
      |
      */
    pub fn translate_text_with_fallback(&self, 
        text:                &str,
        result_if_not_found: &str) -> AloeString {
        
        todo!();
        /*
            if (fallback != nullptr && ! translations.containsKey (text))
            return fallback->translate (text, resultIfNotFound);

        return translations.getValue (text, resultIfNotFound);
        */
    }
    
    pub fn load_from_text(&mut self, 
        file_contents: &AloeString,
        ignore_case:   bool)  {
        
        todo!();
        /*
            translations.setIgnoresCase (ignoreCase);

        StringArray lines;
        lines.addLines (fileContents);

        for (auto& l : lines)
        {
            auto line = l.trim();

            if (line.startsWithChar ('"'))
            {
                auto closeQuote = findCloseQuote (line, 1);
                auto originalText = unescapeString (line.substring (1, closeQuote));

                if (originalText.isNotEmpty())
                {
                    auto openingQuote = findCloseQuote (line, closeQuote + 1);
                    closeQuote = findCloseQuote (line, openingQuote + 1);
                    auto newText = unescapeString (line.substring (openingQuote + 1, closeQuote));

                    if (newText.isNotEmpty())
                        translations.set (originalText, newText);
                }
            }
            else if (line.startsWithIgnoreCase ("language:"))
            {
                languageName = line.substring (9).trim();
            }
            else if (line.startsWithIgnoreCase ("countries:"))
            {
                countryCodes.addTokens (line.substring (10).trim(), true);
                countryCodes.trim();
                countryCodes.removeEmptyStrings();
            }
        }

        translations.minimiseStorageOverheads();
        */
    }
    
    /**
      | Adds and merges another set of translations
      | into this set.
      | 
      | -----------
      | @note
      | 
      | the language name and country codes
      | of the new LocalisedStrings object
      | must match that of this object - an assertion
      | will be thrown if they don't match.
      | 
      | Any existing values will have their
      | mappings overwritten by the new ones.
      |
      */
    pub fn add_strings(&mut self, other: &LocalisedStrings)  {
        
        todo!();
        /*
            jassert (languageName == other.languageName);
        jassert (countryCodes == other.countryCodes);

        translations.addArray (other.translations);
        */
    }
    
    /**
      | Gives this object a set of strings to
      | use as a fallback if a string isn't found.
      | 
      | The object that is passed-in will be
      | owned and deleted by this object when
      | no longer needed. It can be nullptr to
      | clear the existing fallback object.
      |
      */
    pub fn set_fallback(&mut self, f: *mut LocalisedStrings)  {
        
        todo!();
        /*
            fallback.reset (f);
        */
    }
    
    /**
      | Selects the current set of mappings
      | to be used by the system.
      | 
      | The object you pass in will be automatically
      | deleted when no longer needed, so don't
      | keep a pointer to it. You can also pass
      | in nullptr to remove the current mappings.
      | 
      | See also the TRANS() macro, which uses
      | the current set to do its translation.
      | 
      | @see translateWithCurrentMappings
      |
      */
    pub fn set_current_mappings(&mut self, new_translations: *mut LocalisedStrings)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (currentMappingsLock);
        currentMappings.reset (newTranslations);
        */
    }
    
    /**
      | Returns the currently selected set
      | of mappings.
      | 
      | This is the object that was last passed
      | to setCurrentMappings(). It may be
      | nullptr if none has been created.
      |
      */
    pub fn get_current_mappings(&mut self) -> *mut LocalisedStrings {
        
        todo!();
        /*
            return currentMappings.get();
        */
    }
    
    /**
      | Tries to translate a string using the
      | currently selected set of mappings.
      | 
      | If no mapping has been set, or if the mapping
      | doesn't contain a translation for the
      | string, this will just return the original
      | string.
      | 
      | See also the TRANS() macro, which uses
      | this method to do its translation.
      | 
      | @see setCurrentMappings, getCurrentMappings
      |
      */
    pub fn translate_with_current_mappings(&mut self, text: &str) -> AloeString {
        
        todo!();
        /*
            return translate (text);
        */
    }
}


/**
  | Uses the LocalisedStrings class to
  | translate the given string literal.
  | This macro is provided for backwards-compatibility,
  | and just calls the translate() function.
  | In new code, it's recommended that you
  | just call translate() directly instead,
  | and avoid using macros. @see translate(),
  | LocalisedStrings
  |
  */
#[cfg(not(TRANS))]
macro_rules! trans {
    ($stringLiteral:ident) => {
        /*
                translate (stringLiteral)
        */
    }
}

/**
  | A dummy version of the TRANS macro, used
  | to indicate a string literal that should
  | be added to the translation file by source-code
  | scanner tools.
  | 
  | Wrapping a string literal in this macro
  | has no effect, but by using it around
  | strings that your app needs to translate
  | at a later stage, it lets automatic code-scanning
  | tools find this string and add it to the
  | list of strings that need translation.
  |
  */
macro_rules! needs_trans {
    ($stringLiteral:ident) => {
        /*
                (stringLiteral)
        */
    }
}

/**
  | Uses the LocalisedStrings class to
  | translate the given string literal.
  | 
  | @see LocalisedStrings
  |
  */
pub fn translate(text: &str) -> AloeString {
    
    todo!();
    /*
        return translate (text, text);
    */
}

/**
  | Uses the LocalisedStrings class to
  | translate the given string literal.
  | 
  | @see LocalisedStrings
  |
  */
pub fn translate_text_with_fallback(
        text:                &str,
        result_if_not_found: &str) -> AloeString {
    
    todo!();
    /*
        const SpinLock::ScopedLockType sl (currentMappingsLock);

        if (auto* mappings = LocalisedStrings::getCurrentMappings())
            return mappings->translate (text, resultIfNotFound);

        return resultIfNotFound;
    */
}


/**
  | By using this object to force
  | a LocalisedStrings object to be created
  | before the currentMappings object, we can
  | force the static order-of-destruction to
  | delete the currentMappings object first,
  | which avoids a bogus leak warning.  (Oddly,
  | just creating a LocalisedStrings on the
  | stack doesn't work in gcc, it has to be
  | created with 'new' for this to work..)
  */
#[cfg(ALOE_CHECK_MEMORY_LEAKS)]
pub struct LeakAvoidanceTrick {

}

#[cfg(ALOE_CHECK_MEMORY_LEAKS)]
impl Default for LeakAvoidanceTrick {
    
    fn default() -> Self {
        todo!();
        /*


            const std::unique_ptr<LocalisedStrings> dummy (new LocalisedStrings (AloeString(), false))
        */
    }
}

#[cfg(ALOE_CHECK_MEMORY_LEAKS)]
lazy_static!{
    /*
    LeakAvoidanceTrick leakAvoidanceTrick;
    */
}

lazy_static!{
    /*
    SpinLock currentMappingsLock;
        std::unique_ptr<LocalisedStrings> currentMappings;
    */
}

pub fn find_close_quote(
        text:      &AloeString,
        start_pos: i32) -> i32 {
    
    todo!();
    /*
        aloe_wchar lastChar = 0;
            auto t = text.getCharPointer() + startPos;

            for (;;)
            {
                auto c = t.getAndAdvance();

                if (c == 0 || (c == '"' && lastChar != '\\'))
                    break;

                lastChar = c;
                ++startPos;
            }

            return startPos;
    */
}

pub fn unescape_string(s: &AloeString) -> AloeString {
    
    todo!();
    /*
        return s.replace ("\\\"", "\"")
                    .replace ("\\\'", "\'")
                    .replace ("\\t", "\t")
                    .replace ("\\r", "\r")
                    .replace ("\\n", "\n");
    */
}
