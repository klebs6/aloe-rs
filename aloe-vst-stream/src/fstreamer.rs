/*!
  | Description : Extract of typed stream
  | i/o methods from FStream
  |
  */

crate::ix!();

/**
  | @name Implementing class must override.
  |
  */
pub trait FStreamerInterface {

    /**
      | Read one buffer of size.
      |
      */
    fn read_raw(&mut self, 
        _0: *mut c_void,
        _1: TSize) -> TSize;

    /**
      | Write one buffer of size.
      |
      */
    fn write_raw(&mut self, 
        _0: *const c_void,
        _1: TSize) -> TSize;

    /**
      | Set file position for stream.
      |
      */
    fn seek(&mut self, 
        _0: i64,
        _1: FSeekMode) -> i64;

    /**
      | Return current file position.
      |
      */
    fn tell(&mut self) -> i64;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fstreamer.h]

pub enum FSeekMode
{
    kSeekSet,
    kSeekCurrent,
    kSeekEnd
}

/**
  | Byteorder-aware base class for typed
  | stream i/o.
  |
  */
pub struct FStreamer {
    byte_order: i16,
}

impl FStreamer {
    
    pub fn new(byte_order: Option<i16>) -> Self {

        let byte_order: i16 = byte_order.unwrap_or(byteorder![] as i16);
    
        todo!();
        /*
        : byte_order(_byteOrder),

        
        */
    }
    
    /**
      | @name read and write int8 and char.
      |
      */
    pub fn write_char8(&mut self, c: u8) -> bool {
        
        todo!();
        /*
            return writeRaw ((void*)&c, sizeof (char8)) == sizeof (char8);
        */
    }
    
    pub fn read_char8(&mut self, c: &mut u8) -> bool {
        
        todo!();
        /*
            return readRaw ((void*)&c, sizeof (char8)) == sizeof (char8);
        */
    }
    
    pub fn write_uchar8(&mut self, c: u8) -> bool {
        
        todo!();
        /*
            return writeRaw ((void*)&c, sizeof (unsigned char)) == sizeof (unsigned char);
        */
    }
    
    pub fn read_uchar8(&mut self, c: &mut u8) -> bool {
        
        todo!();
        /*
            return readRaw ((void*)&c, sizeof (unsigned char)) == sizeof (unsigned char);
        */
    }
    
    pub fn write_char16(&mut self, c: u16) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_16 (c);
        return writeRaw ((void*)&c, sizeof (char16)) == sizeof (char16);
        */
    }
    
    pub fn read_char16(&mut self, c: &mut u16) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&c, sizeof (char16)) == sizeof (char16))
        {
            if (BYTEORDER != byteOrder)
                SWAP_16 (c);
            return true;
        }
        c = 0;
        return false;
        */
    }

    // int16 -----------------------------------------------------------------

    /**
      | @name read and write int16.
      |
      */
    pub fn write_int16(&mut self, i: i16) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_16 (i);
        return writeRaw ((void*)&i, sizeof (int16)) == sizeof (int16);
        */
    }
    
    pub fn read_int16(&mut self, i: &mut i16) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&i, sizeof (int16)) == sizeof (int16))
        {
            if (BYTEORDER != byteOrder)
                SWAP_16 (i);
            return true;
        }
        i = 0;
        return false;
        */
    }
    
    pub fn write_int_16array(&mut self, 
        array: *const i16,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeInt16 (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_int_16array(&mut self, 
        array: *mut i16,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readInt16 (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn write_int_16u(&mut self, i: u16) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_16 (i);
        return writeRaw ((void*)&i, sizeof (uint16)) == sizeof (uint16);
        */
    }
    
    pub fn read_int_16u(&mut self, i: &mut u16) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&i, sizeof (uint16)) == sizeof (uint16))
        {
            if (BYTEORDER != byteOrder)
                SWAP_16 (i);
            return true;
        }
        i = 0;
        return false;
        */
    }
    
    pub fn write_int_16u_array(&mut self, 
        array: *const u16,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeInt16u (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_int_16u_array(&mut self, 
        array: *mut u16,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readInt16u (array[i]))
                return false;
        }
        return true;
        */
    }

    // int32 -----------------------------------------------------------------
    
    /**
      | @name read and write int32.
      |
      */
    pub fn write_int32(&mut self, i: i32) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_32 (i);
        return writeRaw ((void*)&i, sizeof (int32)) == sizeof (int32);
        */
    }
    
    pub fn read_int32(&mut self, i: &mut i32) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&i, sizeof (int32)) == sizeof (int32))
        {
            if (BYTEORDER != byteOrder)
                SWAP_32 (i);
            return true;
        }
        i = 0;
        return false;
        */
    }
    
    pub fn write_int_32array(&mut self, 
        array: *const i32,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeInt32 (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_int_32array(&mut self, 
        array: *mut i32,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readInt32 (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn write_int_32u(&mut self, i: u32) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_32 (i);
        return writeRaw ((void*)&i, sizeof (uint32)) == sizeof (uint32);
        */
    }
    
    pub fn read_int_32u(&mut self, i: &mut u32) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&i, sizeof (uint32)) == sizeof (uint32))
        {
            if (BYTEORDER != byteOrder)
                SWAP_32 (i);
            return true;
        }
        i = 0;
        return false;
        */
    }
    
    pub fn write_int_32u_array(&mut self, 
        array: *const u32,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeInt32u (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_int_32u_array(&mut self, 
        array: *mut u32,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readInt32u (array[i]))
                return false;
        }
        return true;
        */
    }

    // int64 -----------------------------------------------------------------

    /**
      | @name read and write int64.
      |
      */
    pub fn write_int64(&mut self, i: i64) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_64 (i);
        return writeRaw ((void*)&i, sizeof (int64)) == sizeof (int64);
        */
    }
    
    pub fn read_int64(&mut self, i: &mut i64) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&i, sizeof (int64)) == sizeof (int64))
        {
            if (BYTEORDER != byteOrder)
                SWAP_64 (i);
            return true;
        }
        i = 0;
        return false;
        */
    }
    
    pub fn write_int_64array(&mut self, 
        array: *const i64,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeInt64 (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_int_64array(&mut self, 
        array: *mut i64,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readInt64 (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn write_int_64u(&mut self, i: u64) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_64 (i);
        return writeRaw ((void*)&i, sizeof (uint64)) == sizeof (uint64);
        */
    }
    
    pub fn read_int_64u(&mut self, i: &mut u64) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&i, sizeof (uint64)) == sizeof (uint64))
        {
            if (BYTEORDER != byteOrder)
                SWAP_64 (i);
            return true;
        }
        i = 0;
        return false;
        */
    }
    
    pub fn write_int_64u_array(&mut self, 
        array: *const u64,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeInt64u (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_int_64u_array(&mut self, 
        array: *mut u64,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readInt64u (array[i]))
                return false;
        }
        return true;
        */
    }

    // float / double --------------------------------------------------------

    /**
      | @name read and write float and float
      | array.
      |
      */
    pub fn write_float(&mut self, f: f32) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_32 (f);
        return writeRaw ((void*)&f, sizeof (float)) == sizeof (float);
        */
    }
    
    pub fn read_float(&mut self, f: &mut f32) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&f, sizeof (float)) == sizeof (float))
        {
            if (BYTEORDER != byteOrder)
                SWAP_32 (f);
            return true;
        }
        f = 0.f;
        return false;
        */
    }
    
    pub fn write_float_array(&mut self, 
        array: *const f32,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeFloat (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_float_array(&mut self, 
        array: *mut f32,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readFloat (array[i]))
                return false;
        }
        return true;
        */
    }
    
    /**
      | @name read and write double and double
      | array.
      |
      */
    pub fn write_double(&mut self, d: f64) -> bool {
        
        todo!();
        /*
            if (BYTEORDER != byteOrder)
            SWAP_64 (d);
        return writeRaw ((void*)&d, sizeof (double)) == sizeof (double);
        */
    }
    
    pub fn read_double(&mut self, d: &mut f64) -> bool {
        
        todo!();
        /*
            if (readRaw ((void*)&d, sizeof (double)) == sizeof (double))
        {
            if (BYTEORDER != byteOrder)
                SWAP_64 (d);
            return true;
        }
        d = 0.0;
        return false;
        */
    }
    
    pub fn write_double_array(&mut self, 
        array: *const f64,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!writeDouble (array[i]))
                return false;
        }
        return true;
        */
    }
    
    pub fn read_double_array(&mut self, 
        array: *mut f64,
        count: i32) -> bool {
        
        todo!();
        /*
            for (int32 i = 0; i < count; i++)
        {
            if (!readDouble (array[i]))
                return false;
        }
        return true;
        */
    }
    
    /**
      | Read one bool.
      |
      */
    pub fn read_bool(&mut self, b: &mut bool) -> bool {
        
        todo!();
        /*
            int16 v = 0;
        bool res = readInt16 (v);
        b = (v != 0);
        return res;
        */
    }
    
    /**
      | Write one boolean
      |
      */
    pub fn write_bool(&mut self, b: bool) -> bool {
        
        todo!();
        /*
            return writeInt16 ((int16)b);
        */
    }
    
    /**
      | a direct output function writing only
      | one string (ascii 8bit)
      |
      */
    pub fn write_string8(
        &mut self, 
        ptr:       *const u8,
        terminate: Option<bool>

    ) -> TSize {

        let terminate: bool = terminate.unwrap_or(false);
        
        todo!();
        /*
            TSize size = strlen (ptr);
        if (terminate) // write \0
            size++;

        return writeRaw ((void*)ptr, size);
        */
    }
    
    /**
      | a direct input function reading only
      | one string (ascii) (ended by a \n or \0
      | or eof)
      |
      */
    pub fn read_string8(&mut self, 
        ptr:  *mut u8,
        size: TSize) -> TSize {
        
        todo!();
        /*
            TSize i = 0;
        char8 c = 0;
        while (i < size)
        {
            if (readRaw ((void*)&c, sizeof (char)) != sizeof (char))
                break;
            ptr[i] = c;
            i++;
            if (c == '\n' || c == '\0')
                break;
        }
        if (c == '\n' && ptr[i - 2] == '\r')
            ptr[i - 2] = 0;
        if (i < size)
            ptr[i] = 0;
        else
            ptr[size - 1] = 0;

        return strlen (ptr);
        */
    }
    
    /**
      | always terminated, converts to utf8
      | if non ascii characters are in string
      |
      */
    pub fn write_string_utf8(&mut self, ptr: *const u16) -> bool {
        
        todo!();
        /*
            bool isUtf8 = false;

        String str (ptr);
        if (str.isAsciiString () == false) 
        {
            str.toMultiByte (kCP_Utf8);
            isUtf8 = true;
        }
        else
        {
            str.toMultiByte ();
        }

        if (isUtf8) 
            if (writeRaw (kBomUtf8, kBomUtf8Length) != kBomUtf8Length)
                return false;

        TSize size = str.length () + 1;
        if (writeRaw (str.text8 (), size) != size)
            return false;

        return true;
        */
    }
    
    /**
      | read a UTF8 string
      |
      */
    pub fn read_string_utf8(&mut self, 
        ptr:     *mut u16,
        n_chars: i32) -> i32 {
        
        todo!();
        /*
            char8 c = 0;

        ptr [0] = 0;

        Buffer tmp;
        tmp.setDelta (1024);

        while (true)
        {
            if (readRaw ((void*)&c, sizeof (char)) != sizeof (char))
                break;
            tmp.put (c);
            if (c == '\0')
                break;
        }

        char8* source = tmp.int8Ptr ();
        uint32 codePage = kCP_Default; // for legacy take default page if no utf8 bom is present...
        if (tmp.getFillSize () > 2)
        {
            if (memcmp (source, kBomUtf8, kBomUtf8Length) == 0)
            {
                codePage = kCP_Utf8;
                source += 3;
            }
        }

        if (tmp.getFillSize () > 1)
        {
    #ifdef UNICODE
            ConstString::multiByteToWideString (ptr, source, nChars, codePage);
    #else
            if (codePage == kCP_Utf8)
            {
                Buffer wideBuffer (tmp.getFillSize () * 3);
                ConstString::multiByteToWideString (wideBuffer.wcharPtr (), source, wideBuffer.getSize () / 2, kCP_Utf8);
                ConstString::wideStringToMultiByte (ptr, wideBuffer.wcharPtr (), nChars);       
            }
            else
            {
                memcpy (ptr, source, Min<TSize> (nChars, tmp.getFillSize ()));
            }
    #endif
        }

        ptr[nChars - 1] = 0;
        return ConstString (ptr).length ();
        */
    }
    
    /**
      | write a string length (strlen) and string
      | itself
      |
      */
    pub fn write_str8(&mut self, s: *const u8) -> bool {
        
        todo!();
        /*
            int32 length = (s) ? (int32) strlen (s) + 1 : 0;
        if (!writeInt32 (length))
            return false;

        if (length > 0)
            return writeRaw (s, sizeof (char8) * length) == static_cast<TSize>(sizeof (char8) * length);

        return true;
        */
    }
    
    /**
      | returns the size of a saved string
      |
      */
    pub fn get_str_8size(&mut self, s: *const u8) -> i32 {
        
        todo!();
        /*
            return sizeof (int32) + (int32)strlen (s) + 1;
        */
    }
    
    /**
      | read a string length and string text (The
      | return string must be deleted when use is
      | finished)
      */
    pub fn read_str8(&mut self) -> *mut u8 {
        
        todo!();
        /*
            int32 length;
        if (!readInt32 (length))
            return nullptr;
        
        // check corruption
        if (length > 262144)
            return nullptr;

        char8* s = (length > 0) ? NEWVEC char8[length] : nullptr;
        if (s)
            readRaw (s, length * sizeof (char8));
        return s;
        */
    }
    
    pub fn skip(&mut self, bytes: u32) -> bool {
        
        todo!();
        /*
            int8 tmp;
        while (bytes-- > 0) 
        {
            if (readInt8 (tmp) == false)
                return false;   
        }
        return true;
        */
    }
    
    pub fn pad(&mut self, bytes: u32) -> bool {
        
        todo!();
        /*
            while (bytes-- > 0) 
        {
            if (writeInt8 (0) == false)
                return false;   
        }
        return true;
        */
    }
}
