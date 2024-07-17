crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/updatehandler.cpp]

pub const NON_EXISTING_DEPENDENCY_CHECK: usize = 0; // not yet

#[cfg(DEVELOPMENT)]      pub const CLASS_NAME_TRACKED: bool = true;
#[cfg(not(DEVELOPMENT))] pub const CLASS_NAME_TRACKED: bool = false;

def_class_iid!{ IUpdateManager }

lazy_static!{
    /*
    bool UpdateHandler::lockUpdates = false;
    */
}

/**
  | must be power of 2 (16 bytes * 256 == 4096)
  |
  */
pub const UPDATE_K_HASH_SIZE: usize = 1 << 8;
pub const UPDATE_K_MAP_SIZE:  usize = 1024 * 10;

#[inline] pub fn update_hash_pointer(p: *mut c_void) -> u32 {
    
    todo!();
        /*
            return (uint32)((uint64 (p) >> 12) & (kHashSize - 1));
        */
}

#[inline] pub fn update_get_unknown_base(unknown: *mut dyn FUnknown) -> IPtr<Box<dyn FUnknown>> {
    
    todo!();
        /*
            FUnknown* result = nullptr;
        if (unknown)
            unknown->queryInterface (FUnknown::iid, (void**)&result);

        return owned (result);
        */
}

pub fn count_entries(map: &mut UpdateHandlerDependentMap) -> i32 {
    
    todo!();
        /*
            int32 total = 0;
        Update::UpdateHandlerDependentMapIterConst iterMap = map.begin ();
        while (iterMap != map.end ())
        {
            const Update::UpdateHandlerDependentList& list = iterMap->second;
            Update::UpdateHandlerDependentListIterConst iterList = list.begin ();
            while (iterList != list.end ())
            {
                total++;
                ++iterList;
            }
            ++iterMap;
        }
        return total;
        */
}

pub fn update_done(
    unknown: *mut dyn FUnknown,
    message: i32
) {

    todo!();
        /*
            if (message != IDependent::kDestroyed)
        {
            FObject* obj = FObject::unknownToObject (unknown);
            if (obj)
                obj->updateDone (message);
        }
        */
}
