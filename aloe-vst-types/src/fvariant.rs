crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/fvariant.h]

pub union FVariantU {
    int_value:   i64,
    float_value: f64,
    string8:     *const u8,
    string16:    *const u16,
    object:      *mut dyn FUnknown,
}

pub const FVARIANT_EMPTY:    usize = 0;
pub const FVARIANT_INTEGER:  usize = 1 << 0;
pub const FVARIANT_FLOAT:    usize = 1 << 1;
pub const FVARIANT_STRING8:  usize = 1 << 2;
pub const FVARIANT_OBJECT:   usize = 1 << 3;
pub const FVARIANT_OWNER:    usize = 1 << 4;
pub const FVARIANT_STRING16: usize = 1 << 5;

/**
  | A Value of variable type. \ingroup pluginBase
  |
  */
pub struct FVariant {
    ty: u16,
    u:  fvariant::FVariantU,
}

impl Drop for FVariant {

    fn drop(&mut self) {
        todo!();
        /*
            empty ();
        */
    }
}

impl PartialEq<FVariant> for FVariant {
    
    fn eq(&self, other: &FVariant) -> bool {
        todo!();
        /*
            #if SMTG_PLATFORM_64
        return v1.type == v2.type && v1.intValue == v2.intValue;
    #else
        if (v1.type != v2.type)
            return false;
        if (v1.type & (FVariant::kString8 | FVariant::kString16 | FVariant::kObject))
            return v1.string8 == v2.string8; // pointer type comparisons
        return v1.intValue == v2.intValue; // intValue & double comparison

    #endif
        */
    }
}

impl Eq for FVariant {}

impl Default for FVariant {

    fn default() -> Self {
    
        todo!();
        /*


            memset (this, 0, sizeof (FVariant));
        */
    }
}

impl FVariant {
    
    pub fn new_from_bool(b: bool) -> Self {
    
        todo!();
        /*
        : ty(kInteger),
        : int_value(b),

        
        */
    }
    
    pub fn new_from_u32(v: u32) -> Self {
    
        todo!();
        /*
        : ty(kInteger),
        : int_value(v),

        
        */
    }
    
    pub fn new_from_i64(v: i64) -> Self {
    
        todo!();
        /*
        : ty(kInteger),
        : int_value(v),

        
        */
    }
    
    pub fn new_from_f64(v: f64) -> Self {
    
        todo!();
        /*
        : ty(kFloat),
        : float_value(v),

        
        */
    }
    
    pub fn new_from_raw_u8_ptr(str_: *const u8) -> Self {
    
        todo!();
        /*
        : ty(kString8),
        : string8(str),

        
        */
    }
    
    pub fn new_from_raw_u16_ptr(str_: *const u16) -> Self {
    
        todo!();
        /*
        : ty(kString16),
        : string16(str),

        
        */
    }
    
    pub fn new(
        obj:   *mut dyn FUnknown,
        owner: Option<bool>

    ) -> Self {

        let owner: bool = owner.unwrap_or(false);
        todo!();
        /*
        : ty(kObject),
        : object(obj),

            setOwner (owner);
        */
    }
    
    #[inline] pub fn set(&mut self, b: bool)  {
        
        todo!();
        /*
            setInt (b);
        */
    }
    
    #[inline] pub fn set_u32(&mut self, v: u32)  {
        
        todo!();
        /*
            setInt (v);
        */
    }
    
    #[inline] pub fn set_i64(&mut self, v: i64)  {
        
        todo!();
        /*
            setInt (v);
        */
    }
    
    #[inline] pub fn set_f64(&mut self, v: f64)  {
        
        todo!();
        /*
            setFloat (v);
        */
    }
    
    #[inline] pub fn set_u8_raw_ptr(&mut self, c: *const u8)  {
        
        todo!();
        /*
            setString8 (c);
        */
    }
    
    #[inline] pub fn set_u16_raw_ptr(&mut self, c: *const u16)  {
        
        todo!();
        /*
            setString16 (c);
        */
    }
    
    #[inline] pub fn set_int(&mut self, v: i64)  {
        
        todo!();
        /*
            empty ();
            type = kInteger;
            intValue = v;
        */
    }
    
    #[inline] pub fn set_float(&mut self, v: f64)  {
        
        todo!();
        /*
            empty ();
            type = kFloat;
            floatValue = v;
        */
    }
    
    #[inline] pub fn set_string8(&mut self, v: *const u8)  {
        
        todo!();
        /*
            empty ();
            type = kString8;
            string8 = v;
        */
    }
    
    #[inline] pub fn set_string16(&mut self, v: *const u16)  {
        
        todo!();
        /*
            empty ();
            type = kString16;
            string16 = v;
        */
    }
    
    #[inline] pub fn set_object(&mut self, obj: *mut dyn FUnknown)  {
        
        todo!();
        /*
            empty ();
            type = kObject;
            object = obj;
        */
    }
    
    #[inline] pub fn get<T>(&self) -> T {
    
        todo!();
        /*
        
        */
    }
    
    #[inline] pub fn get_int(&self) -> i64 {
        
        todo!();
        /*
            return (type & kInteger) ? intValue : 0;
        */
    }
    
    #[inline] pub fn get_float_f64(&self) -> f64 {
        
        todo!();
        /*
            return (type & kFloat) ? floatValue : 0.;
        */
    }
    
    #[inline] pub fn get_number(&self) -> f64 {
        
        todo!();
        /*
            return (type & kInteger) ? static_cast<double> (intValue) : (type & kFloat) ? floatValue :
                                                                                          0.;
        */
    }
    
    #[inline] pub fn get_string8(&self) -> *const u8 {
        
        todo!();
        /*
            return (type & kString8) ? string8 : nullptr;
        */
    }
    
    #[inline] pub fn get_string16(&self) -> *const u16 {
        
        todo!();
        /*
            return (type & kString16) ? string16 : nullptr;
        */
    }
    
    #[inline] pub fn get_object(&self) -> *mut dyn FUnknown {
        
        todo!();
        /*
            return (type & kObject) ? object : nullptr;
        */
    }
    
    #[inline] pub fn get_type(&self) -> u16 {
        
        todo!();
        /*
            return static_cast<uint16> (type & ~(kOwner));
        */
    }
    
    #[inline] pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return getType () == kEmpty;
        */
    }
    
    #[inline] pub fn is_owner(&self) -> bool {
        
        todo!();
        /*
            return (type & kOwner) != 0;
        */
    }
    
    #[inline] pub fn is_string(&self) -> bool {
        
        todo!();
        /*
            return (type & (kString8 | kString16)) != 0;
        */
    }
    
    #[inline] pub fn set_owner(&mut self, state: bool)  {
        
        todo!();
        /*
            if (state)
                type |= kOwner;
            else
                type &= ~kOwner;
        */
    }
    
    #[inline] pub fn get_bool(&self) -> bool {
        
        todo!();
        /*
            return getInt () != 0;
        */
    }
    
    #[inline] pub fn get_uint32(&self) -> u32 {
        
        todo!();
        /*
            return static_cast<uint32> (getInt ());
        */
    }
    
    #[inline] pub fn get_int32(&self) -> i32 {
        
        todo!();
        /*
            return static_cast<int32> (getInt ());
        */
    }
    
    #[inline] pub fn get_int64(&self) -> i64 {
        
        todo!();
        /*
            return static_cast<int64> (getInt ());
        */
    }
    
    #[inline] pub fn get_float(&self) -> f32 {
        
        todo!();
        /*
            return static_cast<float> (getFloat ());
        */
    }
    
    #[inline] pub fn get_double(&self) -> f64 {
        
        todo!();
        /*
            return getFloat ();
        */
    }
    
    #[inline] pub fn get_const_char8_ptr(&self) -> *const u8 {
        
        todo!();
        /*
            return getString8 ();
        */
    }
    
    #[inline] pub fn get_const_char16_ptr(&self) -> *const u16 {
        
        todo!();
        /*
            return getString16 ();
        */
    }
    
    #[inline] pub fn get_funknown_ptr(&self) -> *mut dyn FUnknown {
        
        todo!();
        /*
            return getObject ();
        */
    }
    
    pub fn new_from_variant(variant: &FVariant) -> Self {
    
        todo!();
        /*
        : ty(kEmpty),

            *this = variant;
        */
    }
    
    #[inline] pub fn empty(&mut self)  {
        
        todo!();
        /*
            if (type & kOwner)
        {
            if ((type & kString8) && string8)
                delete[] string8;
            else if ((type & kString16) && string16)
                delete[] string16;

            else if ((type & kObject) && object)
                object->release ();
        }
        memset (this, 0, sizeof (FVariant));
        */
    }
    
    #[inline] pub fn assign_from(&mut self, variant: &FVariant) -> &mut FVariant {
        
        todo!();
        /*
            empty ();

        type = variant.type;

        if ((type & kString8) && variant.string8)
        {
            string8 = new char8[strlen (variant.string8) + 1];
            strcpy (const_cast<char8*> (string8), variant.string8);
            type |= kOwner;
        }
        else if ((type & kString16) && variant.string16)
        {
            auto len = static_cast<size_t> (strlen16 (variant.string16));
            string16 = new char16[len + 1];
            char16* tmp = const_cast<char16*> (string16);
            memcpy (tmp, variant.string16, len * sizeof (char16));
            tmp[len] = 0;
            type |= kOwner;
        }
        else if ((type & kObject) && variant.object)
        {
            object = variant.object;
            object->addRef ();
            type |= kOwner;
        }
        else
            intValue = variant.intValue; // copy memory

        return *this;
        */
    }
}
