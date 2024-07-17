crate::ix!();

/* ------------ FUID class declaration  ------------ */

/**
  | plain UID type
  |
  */
pub type TUID = [i8; 16];

/**
  | Handling 16 Byte Globally Unique Identifiers.
  | \ingroup pluginBase
  | 
  | Each interface declares its identifier
  | as static member inside the interface
  | namespace (e.g. FUnknown::iid).
  |
  */
pub struct FUID {
    data: TUID,
}

impl PartialEq<FUID> for FUID {
    
    #[inline] fn eq(&self, other: &FUID) -> bool {
        todo!();
        /*
            return typename Steinberg::FUnknownPrivate::iidEqual (data, f.data);
        */
    }
}

impl Eq for FUID {}

impl Ord for FUID {
    
    #[inline] fn cmp(&self, other: &FUID) -> std::cmp::Ordering {
        todo!();
        /*
            return memcmp (data, f.data, sizeof (TUID)) < 0;
        */
    }
}

impl PartialOrd<FUID> for FUID {
    #[inline] fn partial_cmp(&self, other: &FUID) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<TUID> for FUID {
    
    fn into(self) -> TUID {
        todo!();
        /*
            return data;
        */
    }
}

#[cfg(SMTG_CPP11_STDLIBSUPPORT)]
impl<T> PartialEq<T> for FUID {
    
    fn eq(&self, other: &T) -> bool {
        todo!();
        /*
            static_assert (
            std::is_same<typename std::remove_cv<T>::type, FUID>::value,
            "Do not compare a FUID with a TUID directly. Either convert the TUID to a FUID and compare them or use FUnknownPrivate::iidEqual");
        return f1.operator== (f2);
        */
    }
}

pub type fuid_string = [u8; 33];

pub enum FuidUIDPrintStyle
{
    /**
      INLINE_UID (
      0x00000000, 0x00000000,
      0x00000000, 0x00000000)
      */
    InlineUid,  

    /**
      "DECLARE_UID (
      0x00000000, 0x00000000,
      0x00000000, 0x00000000)"
      */
    DeclareUid, 

    /**
      "FUID (
      0x00000000, 0x00000000,
      0x00000000, 0x00000000)"
      */
    Fuid,        

    /**
      "DECLARE_CLASS_IID ( Interface,
      0x00000000, 0x00000000, 0x00000000,
      0x00000000)"
      */
    ClassUid    
}

impl Default for FUID {

    fn default() -> Self {
    
        todo!();
        /*
            memset (data, 0, sizeof (TUID));
        */
    }
}
    
impl FUID {

    pub fn new_from_uid<const N: usize>(uid: &[i8; N]) -> Self {
    
        todo!();
        /*


            #if SMTG_CPP11_STDLIBSUPPORT
            static_assert (N == sizeof (TUID), "only TUID allowed");
            #endif

            memcpy (data, uid, sizeof (TUID));
        */
    }
    
    #[inline] pub fn totuid_with(&self, result: TUID)  {
        
        todo!();
        /*
            memcpy (result, data, sizeof (TUID));
        */
    }
    
    #[inline] pub fn totuid(&self) -> &TUID {
        
        todo!();
        /*
            return data;
        */
    }
    
    pub fn fromtuid(uid: TUID) -> FUID {
        
        todo!();
        /*
            FUID res;
            if (uid)
                memcpy (res.data, uid, sizeof (TUID));
            return res;
        */
    }
    
    pub fn new(
        l1: u32,
        l2: u32,
        l3: u32,
        l4: u32) -> Self {
    
        todo!();
        /*
            from4Int (l1, l2, l3, l4);
        */
    }
    
    pub fn new_from_other_ref(f: &FUID) -> Self {
    
        todo!();
        /*
            memcpy (data, f.data, sizeof (TUID));
        */
    }
    
    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    pub fn new_from_other(other: FUID) -> Self {
    
        todo!();
        /*
            memcpy (data, other.data, sizeof (TUID));
        */
    }
    
    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    pub fn assign_from_other(&mut self, other: FUID) -> &mut FUID {
        
        todo!();
        /*
        memcpy (data, other.data, sizeof (TUID));
        return *this;
        */
    }
    
    /**
      | Generates a new Unique Identifier (UID).
      | 
      | Will return true for success. If the
      | return value is false, either no
      | 
      | UID is generated or the UID is not guaranteed
      | to be unique worldwide.
      |
      */
    pub fn generate(&mut self) -> bool {
        
        todo!();
        /*
            #if SMTG_OS_WINDOWS
    #if defined(_M_ARM64) || defined(_M_ARM)
        //#warning implement me!
        return false;
    #else
        GUID guid;
        HRESULT hr = CoCreateGuid (&guid);
        switch (hr)
        {
            case RPC_S_OK: memcpy (data, (char*)&guid, sizeof (TUID)); return true;

            case (HRESULT)RPC_S_UUID_LOCAL_ONLY:
            default: return false;
        }
    #endif

    #elif SMTG_OS_MACOS
        CFUUIDRef uuid = CFUUIDCreate (kCFAllocatorDefault);
        if (uuid)
        {
            CFUUIDBytes bytes = CFUUIDGetUUIDBytes (uuid);
            memcpy (data, (char*)&bytes, sizeof (TUID));
            CFRelease (uuid);
            return true;
        }
        return false;

    #elif SMTG_OS_LINUX
        srand ((size_t)this);
        for (int32 i = 0; i < 16; i++)
            data[i] = static_cast<unsigned char>(rand ());
        return true;
    #else
    #warning implement me!
        return false;
    #endif
        */
    }
    
    /**
      | Checks if the UID data is valid.
      | 
      | The default constructor initializes
      | the memory with zeros.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            TUID nulluid = {0};

        return memcmp (data, nulluid, sizeof (TUID)) != 0;
        */
    }
    
    pub fn assign_from_other_ref(&mut self, f: &FUID) -> &mut FUID {
        
        todo!();
        /*
            memcpy (data, f.data, sizeof (TUID));
        return *this;
        */
    }
    
    pub fn from_4int(&mut self, 
        l1: u32,
        l2: u32,
        l3: u32,
        l4: u32)  {
        
        todo!();
        /*
            #if COM_COMPATIBLE
        data [0]  = (char)((l1 & 0x000000FF)      );
        data [1]  = (char)((l1 & 0x0000FF00) >>  8);
        data [2]  = (char)((l1 & 0x00FF0000) >> 16);
        data [3]  = (char)((l1 & 0xFF000000) >> 24);
        data [4]  = (char)((l2 & 0x00FF0000) >> 16);
        data [5]  = (char)((l2 & 0xFF000000) >> 24);
        data [6]  = (char)((l2 & 0x000000FF)      );
        data [7]  = (char)((l2 & 0x0000FF00) >>  8);
        data [8]  = (char)((l3 & 0xFF000000) >> 24);
        data [9]  = (char)((l3 & 0x00FF0000) >> 16);
        data [10] = (char)((l3 & 0x0000FF00) >>  8);
        data [11] = (char)((l3 & 0x000000FF)      );
        data [12] = (char)((l4 & 0xFF000000) >> 24);
        data [13] = (char)((l4 & 0x00FF0000) >> 16);
        data [14] = (char)((l4 & 0x0000FF00) >>  8);
        data [15] = (char)((l4 & 0x000000FF)      );
    #else
        data [0]  = (char)((l1 & 0xFF000000) >> 24);
        data [1]  = (char)((l1 & 0x00FF0000) >> 16);
        data [2]  = (char)((l1 & 0x0000FF00) >>  8);
        data [3]  = (char)((l1 & 0x000000FF)      );
        data [4]  = (char)((l2 & 0xFF000000) >> 24);
        data [5]  = (char)((l2 & 0x00FF0000) >> 16);
        data [6]  = (char)((l2 & 0x0000FF00) >>  8);
        data [7]  = (char)((l2 & 0x000000FF)      );
        data [8]  = (char)((l3 & 0xFF000000) >> 24);
        data [9]  = (char)((l3 & 0x00FF0000) >> 16);
        data [10] = (char)((l3 & 0x0000FF00) >>  8);
        data [11] = (char)((l3 & 0x000000FF)      );
        data [12] = (char)((l4 & 0xFF000000) >> 24);
        data [13] = (char)((l4 & 0x00FF0000) >> 16);
        data [14] = (char)((l4 & 0x0000FF00) >>  8);
        data [15] = (char)((l4 & 0x000000FF)      );
    #endif
        */
    }
    
    pub fn to_4int(&self, 
        d1: &mut u32,
        d2: &mut u32,
        d3: &mut u32,
        d4: &mut u32)  {
        
        todo!();
        /*
            d1 = getLong1 ();
        d2 = getLong2 ();
        d3 = getLong3 ();
        d4 = getLong4 ();
        */
    }
    
    pub fn get_long1(&self) -> u32 {
        
        todo!();
        /*
            #if COM_COMPATIBLE
        return makeLong (data[3], data[2], data[1], data[0]);
    #else
        return makeLong (data[0], data[1], data[2], data[3]);
    #endif
        */
    }
    
    pub fn get_long2(&self) -> u32 {
        
        todo!();
        /*
            #if COM_COMPATIBLE
        return makeLong (data[5], data[4], data[7], data[6]);
    #else
        return makeLong (data[4], data[5], data[6], data[7]);
    #endif
        */
    }
    
    pub fn get_long3(&self) -> u32 {
        
        todo!();
        /*
            #if COM_COMPATIBLE
        return makeLong (data[8], data[9], data[10], data[11]);
    #else
        return makeLong (data[8], data[9], data[10], data[11]);
    #endif
        */
    }
    
    pub fn get_long4(&self) -> u32 {
        
        todo!();
        /*
            #if COM_COMPATIBLE
        return makeLong (data[12], data[13], data[14], data[15]);
    #else
        return makeLong (data[12], data[13], data[14], data[15]);
    #endif
        */
    }
    
    /**
      | Converts UID to a string.
      | 
      | The string will be 32 characters long,
      | representing the hexadecimal values
      | of each data byte (e.g. "9127BE30160E4BB69966670AA6087880").
      | 
      | Typical use-case is:
      | 
      | -----------
      | @code
      | 
      | {.cpp}
      |         char8[33] strUID = {0};
      |         FUID uid;
      |         if (uid.generate ())
      |             uid.toString (strUID);
      |
      */
    pub fn to_string(&self, string: *mut u8)  {
        
        todo!();
        /*
            if (!string)
            return;

    #if COM_COMPATIBLE
        auto* g = (GuidStruct*)data;

        char8 s[17];
        typename Steinberg::toString8 (s, data, 8, 16);

        sprintf (string, "%08X%04X%04X%s", g->Data1, g->Data2, g->Data3, s);
    #else
        typename Steinberg::toString8 (string, data, 0, 16);
    #endif
        */
    }
    
    /**
      | Sets the UID data from a string.
      | 
      | The string has to be 32 characters long,
      | where each character-pair is the ASCII-encoded
      | hexadecimal value of the corresponding
      | data byte.
      |
      */
    pub fn from_string(&mut self, string: *const u8) -> bool {
        
        todo!();
        /*
            if (!string || !*string)
            return false;
        if (strlen (string) != 32)
            return false;

    #if COM_COMPATIBLE
        GuidStruct g;
        char s[33];

        strcpy (s, string);
        s[8] = 0;
        sscanf (s, "%x", &g.Data1);
        strcpy (s, string + 8);
        s[4] = 0;
        sscanf (s, "%hx", &g.Data2);
        strcpy (s, string + 12);
        s[4] = 0;
        sscanf (s, "%hx", &g.Data3);

        memcpy (data, &g, 8);
        typename Steinberg::fromString8 (string + 16, data, 8, 16);
    #else
        typename Steinberg::fromString8 (string, data, 0, 16);
    #endif

        return true;
        */
    }
    
    /**
      | Sets the UID data from a string in Microsoft(R)
      | OLE format.
      |
      */
    pub fn from_registry_string(&mut self, string: *const u8) -> bool {
        
        todo!();
        /*
            if (!string || !*string)
            return false;
        if (strlen (string) != 38)
            return false;

    // e.g. {c200e360-38c5-11ce-ae62-08002b2b79ef}

    #if COM_COMPATIBLE
        GuidStruct g;
        char8 s[10];

        strncpy (s, string + 1, 8);
        s[8] = 0;
        sscanf (s, "%x", &g.Data1);
        strncpy (s, string + 10, 4);
        s[4] = 0;
        sscanf (s, "%hx", &g.Data2);
        strncpy (s, string + 15, 4);
        s[4] = 0;
        sscanf (s, "%hx", &g.Data3);
        memcpy (data, &g, 8);

        typename Steinberg::fromString8 (string + 20, data, 8, 10);
        typename Steinberg::fromString8 (string + 25, data, 10, 16);
    #else
        typename Steinberg::fromString8 (string + 1, data, 0, 4);
        typename Steinberg::fromString8 (string + 10, data, 4, 6);
        typename Steinberg::fromString8 (string + 15, data, 6, 8);
        typename Steinberg::fromString8 (string + 20, data, 8, 10);
        typename Steinberg::fromString8 (string + 25, data, 10, 16);
    #endif

        return true;
        */
    }
    
    /**
      | Converts UID to a string in Microsoft(R)
      | OLE format. (e.g. "{c200e360-38c5-11ce-ae62-08002b2b79ef}")
      |
      */
    pub fn to_registry_string(&self, string: *mut u8)  {
        
        todo!();
        /*
            // e.g. {c200e360-38c5-11ce-ae62-08002b2b79ef}

    #if COM_COMPATIBLE
        auto* g = (GuidStruct*)data;

        char8 s1[5];
        typename Steinberg::toString8 (s1, data, 8, 10);

        char8 s2[13];
        typename Steinberg::toString8 (s2, data, 10, 16);

        sprintf (string, "{%08X-%04X-%04X-%s-%s}", g->Data1, g->Data2, g->Data3, s1, s2);
    #else
        char8 s1[9];
        typename Steinberg::toString8 (s1, data, 0, 4);
        char8 s2[5];
        typename Steinberg::toString8 (s2, data, 4, 6);
        char8 s3[5];
        typename Steinberg::toString8 (s3, data, 6, 8);
        char8 s4[5];
        typename Steinberg::toString8 (s4, data, 8, 10);
        char8 s5[13];
        typename Steinberg::toString8 (s5, data, 10, 16);

        sprintf (string, "{%s-%s-%s-%s-%s}", s1, s2, s3, s4, s5);
    #endif
        */
    }
    
    /**
      | Prints the UID to a string (or debug output
      | if string is NULL).
      | 
      | -----------
      | @param string
      | 
      | is the output string if not NULL.
      | ----------
      | @param style
      | 
      | can be chosen from the FUID::UIDPrintStyle
      | enumeration.
      |
      */
    pub fn print(
        &self, 
        string: *mut u8,
        style:  Option<FuidUIDPrintStyle>

    ) {

        let style = style.unwrap_or(FuidUIDPrintStyle::InlineUid);
        
        todo!();
        /*
            if (!string) // no string: debug output
        {
            char8 str[128];
            print (str, style);

    #if SMTG_OS_WINDOWS
            OutputDebugStringA (str);
            OutputDebugStringA ("\n");
    #else
            fprintf (stdout, "%s\n", str);
    #endif
            return;
        }

        uint32 l1, l2, l3, l4;
        to4Int (l1, l2, l3, l4);

        switch (style)
        {
            case InlineUid:
                sprintf (string, "INLINE_UID (0x%08X, 0x%08X, 0x%08X, 0x%08X)", l1, l2, l3, l4);
                break;

            case DeclareUid:
                sprintf (string, "DECLARE_UID (0x%08X, 0x%08X, 0x%08X, 0x%08X)", l1, l2, l3, l4);
                break;

            case Fuid:
                sprintf (string, "FUID (0x%08X, 0x%08X, 0x%08X, 0x%08X)", l1, l2, l3, l4);
                break;

            case ClassUid:
            default:
                sprintf (string, "DECLARE_CLASS_IID (Interface, 0x%08X, 0x%08X, 0x%08X, 0x%08X)", l1,
                         l2, l3, l4);
                break;
        }
        */
    }
}

