crate::ix!();

#[macro_export] macro_rules! inline_uid_of {
    ($ClassName:ty) => {
        /*
                ClassName##_iid
        */
    }
}

#[macro_export] macro_rules! inline_uid_from_fuid {
    ($x:ident) => {
        /*
            INLINE_UID (x.getLong1 (), x.getLong2 (), x.getLong3 (), x.getLong4 ())
        */
    }
}

/* ----------- Unique Identifier macros  ----------- */

#[cfg(COM_COMPATIBLE)]
#[macro_export] macro_rules! inline_uid {
    ($l1:ident, 
     $l2:ident, 
     $l3:ident, 
     $l4:ident) => {
        /*
        
        { 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0x000000FF)      ), (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0x0000FF00) >>  8), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0x00FF0000) >> 16), (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0xFF000000) >> 24), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0x00FF0000) >> 16), (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0xFF000000) >> 24), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0x000000FF)      ), (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0x0000FF00) >>  8), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0xFF000000) >> 24), (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0x00FF0000) >> 16), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0x0000FF00) >>  8), (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0x000000FF)      ), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0xFF000000) >> 24), (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0x00FF0000) >> 16), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0x0000FF00) >>  8), (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0x000000FF)      )  
        }
        */
    }
}

#[cfg(not(COM_COMPATIBLE))]
#[macro_export] macro_rules! inline_uid {
    ($l1:ident, 
     $l2:ident, 
     $l3:ident, 
     $l4:ident) => {
        /*
        
        { 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0xFF000000) >> 24), (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0x00FF0000) >> 16), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0x0000FF00) >>  8), (typename Steinberg::int8)(((typename Steinberg::uint32)(l1) & 0x000000FF)      ), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0xFF000000) >> 24), (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0x00FF0000) >> 16), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0x0000FF00) >>  8), (typename Steinberg::int8)(((typename Steinberg::uint32)(l2) & 0x000000FF)      ), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0xFF000000) >> 24), (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0x00FF0000) >> 16), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0x0000FF00) >>  8), (typename Steinberg::int8)(((typename Steinberg::uint32)(l3) & 0x000000FF)      ), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0xFF000000) >> 24), (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0x00FF0000) >> 16), 
            (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0x0000FF00) >>  8), (typename Steinberg::int8)(((typename Steinberg::uint32)(l4) & 0x000000FF)      )  
        }
        */
    }
}

///-----------------------------------
#[macro_export] macro_rules! declare_uid {
    ($name:ident, 
     $l1:ident, 
     $l2:ident, 
     $l3:ident, 
     $l4:ident) => {
        /*
                typename Steinberg::TUID name = INLINE_UID (l1, l2, l3, l4);
        */
    }
}

#[macro_export] macro_rules! extern_uid {
    ($name:ident) => {
        /*
                extern const typename Steinberg::TUID name;
        */
    }
}
