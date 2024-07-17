crate::ix!();

bitflags!{

    pub struct MBCodePage: u32
    {
        /**
          | Default ANSI codepage.
          |
          */
        const CP_ANSI = 0;           

        /**
          | Default Mac codepage.
          |
          */
        const CP_MAC_ROMAN = 2;      

        /**
          | West European Latin Encoding.
          |
          */
        const CP_ANSI_WEL = 1252;    

        /**
          | Mac Central European Encoding.
          |
          */
        const CP_MAC_CEE = 10029;    

        /**
          | UTF8 Encoding.
          |
          */
        const CP_Utf8 = 65001;       

        /**
          | Shifted Japan Industrial Standard
          | Encoding.
          |
          */
        const CP_ShiftJIS = 932;     

        /**
          | US-ASCII (7-bit).
          |
          */
        const CP_US_ASCII = 20127;   

        /**
          | Default ANSI codepage.
          |
          */
        const CP_Default = Self::CP_ANSI.bits();
    }
}

#[cfg(SMTG_OS_MACOS)]
pub fn mb_code_page_to_cf_string_encoding(code_page: u32) -> CFStringEncoding {
    
    todo!();
        /*
            switch (codePage)
        {
            case kCP_ANSI:      return kDefaultSystemEncoding; // MacRoman or JIS
            case kCP_MAC_ROMAN: return kCFStringEncodingMacRoman;
            case kCP_ANSI_WEL:  return kCFStringEncodingWindowsLatin1;
            case kCP_MAC_CEE:   return kCFStringEncodingMacCentralEurRoman;
            case kCP_Utf8:      return kCFStringEncodingUTF8;
            case kCP_ShiftJIS:  return kCFStringEncodingShiftJIS_X0213_00;
            case kCP_US_ASCII:  return kCFStringEncodingASCII;
        }
        return kCFStringEncodingASCII;
        */
}
