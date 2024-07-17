crate::ix!();

#[cfg(INIT_CLASS_IID)]
#[macro_export] 
macro_rules! declare_class_iid {
    ($ClassName:ty, 
     $l1:expr, 
     $l2:expr, 
     $l3:expr, 
     $l4:expr) => {
        /*
        
            static const typename Steinberg::TUID ClassName##_iid = INLINE_UID (l1, l2, l3, l4); 
            
        const typename Steinberg::FUID ClassName::iid (ClassName##_iid);
        */
    }
}

#[cfg(not(INIT_CLASS_IID))]
#[macro_export] 
macro_rules! declare_class_iid {
    ($ClassName:ty, 
     $l1:expr, 
     $l2:expr, 
     $l3:expr, 
     $l4:expr) => {
        /*
            static const typename Steinberg::TUID ClassName##_iid = INLINE_UID (l1, l2, l3, l4);
        */
    }
}

#[macro_export] macro_rules! def_class_iid {
    ($ClassName:ty) => {
        /*
                const typename Steinberg::FUID ClassName::iid (ClassName##_iid);
        */
    }
}

