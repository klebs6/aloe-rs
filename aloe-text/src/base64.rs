crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_Base64.h]

/**
  | Contains some static methods for converting
  | between binary and the standard base-64
  | encoding format.
  | 
  | @tags{Core}
  |
  */
pub struct Base64 {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_Base64.cpp]
impl Base64 {

    /**
      | Converts a binary block of data into
      | a base-64 string.
      | 
      | This will write the resulting string
      | data to the given stream.
      | 
      | If a write error occurs with the stream,
      | the method will terminate and return
      | false.
      |
      */
    pub fn convert_to_base64<W: Write>(
        &mut self, 
        base_64result:    &mut W,
        source_data:      *const c_void,
        source_data_size: usize) -> bool {
        
        todo!();
        /*
            static const char lookup[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        auto* source = static_cast<const uint8*> (sourceData);

        while (sourceDataSize > 0)
        {
            char frame[4];
            auto byte0 = *source++;
            frame[0] = lookup [(byte0 & 0xfcu) >> 2];
            uint32 bits = (byte0 & 0x03u) << 4;

            if (sourceDataSize > 1)
            {
                auto byte1 = *source++;
                frame[1] = lookup[bits | ((byte1 & 0xf0u) >> 4)];
                bits = (byte1 & 0x0fu) << 2;

                if (sourceDataSize > 2)
                {
                    auto byte2 = *source++;
                    frame[2] = lookup[bits | ((byte2 & 0xc0u) >> 6)];
                    frame[3] = lookup[byte2 & 0x3fu];
                    sourceDataSize -= 3;
                }
                else
                {
                    frame[2] = lookup[bits];
                    frame[3] = '=';
                    sourceDataSize = 0;
                }
            }
            else
            {
                frame[1] = lookup[bits];
                frame[2] = '=';
                frame[3] = '=';
                sourceDataSize = 0;
            }

            if (! base64Result.write (frame, 4))
                return false;
        }

        return true;
        */
    }
    
    /**
      | Converts a base-64 string back to its
      | binary representation.
      | 
      | This will write the decoded binary data
      | to the given stream.
      | 
      | If the string is not valid base-64, the
      | method will terminate and return false.
      |
      */
    pub fn convert_from_base64<W: Write>(
        &mut self, 
        binary_output:     &mut W,
        base_64text_input: &str) -> bool {
        
        todo!();
        /*
            for (auto s = base64TextInput.text; ! s.isEmpty();)
        {
            uint8 data[4];

            for (int i = 0; i < 4; ++i)
            {
                auto c = (uint32) s.getAndAdvance();

                if (c >= 'A' && c <= 'Z')         c -= 'A';
                else if (c >= 'a' && c <= 'z')    c -= 'a' - 26;
                else if (c >= '0' && c <= '9')    c += 52 - '0';
                else if (c == '+')                c = 62;
                else if (c == '/')                c = 63;
                else if (c == '=')                { c = 64; if (i <= 1) return false; }
                else                              return false;

                data[i] = (uint8) c;
            }

            binaryOutput.writeByte ((char) ((data[0] << 2) | (data[1] >> 4)));

            if (data[2] < 64)
            {
                binaryOutput.writeByte ((char) ((data[1] << 4) | (data[2] >> 2)));

                if (data[3] < 64)
                    binaryOutput.writeByte ((char) ((data[2] << 6) | data[3]));
            }
        }

        return true;
        */
    }
    
    /**
      | Converts a block of binary data to a base-64
      | string.
      |
      */
    pub fn to_base64_from_raw(
        &mut self, 
        source_data:      *const c_void,
        source_data_size: usize) -> String {
        
        todo!();
        /*
            MemoryOutputStream m ((sourceDataSize * 4) / 3 + 3);
        bool ok = convertToBase64 (m, sourceData, sourceDataSize);
        jassertquiet (ok); // should always succeed for this simple case
        return m.toString();
        */
    }
    
    /**
      | Converts a string's UTF-8 representation
      | to a base-64 string.
      |
      */
    pub fn to_base64(&mut self, text: &String) -> String {
        
        todo!();
        /*
            return toBase64 (text.toRawUTF8(), strlen (text.toRawUTF8()));
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct Base64Tests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for Base64Tests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Base64 class", UnitTestCategories::text
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl Base64Tests {
    
    pub fn create_random_data(r: &mut Random) -> MemoryBlock {
        
        todo!();
        /*
            MemoryOutputStream m;

            for (int i = r.nextInt (400); --i >= 0;)
                m.writeByte ((char) r.nextInt (256));

            return m.getMemoryBlock();
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Base64");

            auto r = getRandom();

            for (int i = 1000; --i >= 0;)
            {
                auto original = createRandomData (r);
                auto asBase64 = Base64::toBase64 (original.getData(), original.getSize());
                MemoryOutputStream out;
                expect (Base64::convertFromBase64 (out, asBase64));
                auto result = out.getMemoryBlock();
                expect (result == original);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static Base64Tests base64Tests;
    */
}
