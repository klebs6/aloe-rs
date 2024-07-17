crate::ix!();

/**
  | FCast overload 1 - FObject to FObject
  |
  */
#[inline] pub fn cast_fobject<C>(object: *const FObject) -> *mut C {

    todo!();
        /*
            if (object && object->isTypeOf (C::getFClassID (), true))
            return (C*) object;
        return 0;
        */
}

/**
  | FCast overload 2 - FUnknown to FObject
  |
  */
#[inline] pub fn cast_funknown<C>(unknown: *mut dyn FUnknown) -> *mut C {

    todo!();
        /*
            FObject* object = FObject::unknownToObject (unknown);
        return FCast<C> (object);
        */
}

/**
  | FUCast - casting from FUnknown to Interface
  |
  */
#[inline] pub fn fu_cast_fobject<C>(object: *mut FObject) -> *mut C {

    todo!();
        /*
            return FUnknownPtr<C> (object ? object->unknownCast () : nullptr);
        */
}

#[inline] pub fn fu_cast_funknown<C>(object: *mut dyn FUnknown) -> *mut C {

    todo!();
        /*
            return FUnknownPtr<C> (object);
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/funknown.cpp]

pub fn make_long(
        b1: u8,
        b2: u8,
        b3: u8,
        b4: u8) -> u32 {
    
    todo!();
        /*
            return (uint32 (b1) << 24) | (uint32 (b2) << 16) | (uint32 (b3) << 8) | uint32 (b4);
        */
}

pub fn to_string8(
        string: *mut u8,
        data:   *const u8,
        i1:     i32,
        i2:     i32)  {
    
    todo!();
        /*
            *string = 0;
        for (int32 i = i1; i < i2; i++)
        {
            char8 s[3];
            sprintf (s, "%02X", (uint8)data[i]);
            strcat (string, s);
        }
        */
}

pub fn from_string8(
        string: *const u8,
        data:   *mut u8,
        i1:     i32,
        i2:     i32)  {
    
    todo!();
        /*
            for (int32 i = i1; i < i2; i++)
        {
            char8 s[3];
            s[0] = *string++;
            s[1] = *string++;
            s[2] = 0;

            int32 d = 0;
            sscanf (s, "%2x", &d);
            data[i] = (char)d;
        }
        */
}
