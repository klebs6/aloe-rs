crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/conststringtable.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/conststringtable.cpp]

/**
  | Constant unicode string table.
  | 
  | Used for conversion from ASCII string
  | literals to char16.
  |
  */
pub struct ConstStringTable {

}

lazy_static!{
    /*
    static std::map<const char8*, char16*>* stringMap;
    static std::map<const char8, char16>* charMap;
    */
}

impl Drop for ConstStringTable {

    fn drop(&mut self) {
        todo!();
        /*
            // free out allocated strings
        {
            std::map<const char8*, char16*>::iterator iter = stringMap->begin ();
            while (iter != stringMap->end ())
            {
                delete[] iter->second;
                iter++;
            }
        } // delete iterator on map before deleting the map

        delete stringMap;
        delete charMap;
        */
    }
}

pub fn generateutf16(str_: *const u8) -> *mut u16 {
    
    todo!();
        /*
            int32 len = (int32)strlen (str);
        char16* result = new char16[len + 1];
        for (int32 i = 0; i < len; i++)
        {
    #if BYTEORDER == kBigEndian
            char8* pChr = (char8*)&result[i];
            pChr[0] = 0;
            pChr[1] = str[i];
    #else
            result[i] = str[i];
    #endif
        }
        result[len] = 0;
        return result;
        */
}

impl Default for ConstStringTable {
    
    fn default() -> Self {
    
        todo!();
        /*


            stringMap = new std::map<const char8*, char16*>;
        charMap = new std::map<const char8, char16>;
        */
    }
}

impl ConstStringTable {

    pub fn instance(&mut self) -> *mut ConstStringTable {
        
        todo!();
        /*
            static ConstStringTable stringTable;
        return &stringTable;
        */
    }
    
    /**
      | Returns a char16 string of a ASCII string
      | literal
      |
      */
    pub fn get_string_with_raw(&self, str_: *const u8) -> *const u16 {
        
        todo!();
        /*
            std::map<const char8*, char16*>::iterator iter = stringMap->find (str);
        if (iter != stringMap->end ())
            return iter->second;
        char16* uStr = generateUTF16 (str);
        stringMap->insert (std::make_pair (str, uStr));
        return uStr;
        */
    }
    
    /**
      | Returns a char16 character of a ASCII
      | character
      |
      */
    pub fn get_string(&self, str_: u8) -> u16 {
        
        todo!();
        /*
            std::map<const char8, char16>::iterator iter = charMap->find (str);
        if (iter != charMap->end ())
            return iter->second;
        char16 uStr = 0;
    #if BYTEORDER == kBigEndian
        char8* puStr = (char8*)&uStr;
        puStr[1] = str;
    #else
        uStr = str;
    #endif
        charMap->insert (std::make_pair (str, uStr));
        return uStr;
        */
    }
}
