crate::ix!();

pub enum PClassInfoClassCardinality
{
    kManyInstances = 0x7FFFFFFF
}

pub const PCLASS_INFO_CATEGORY_SIZE: usize = 32;
pub const PCLASS_INFO_NAME_SIZE:     usize = 64;

/**
  | Basic Information about a class provided
  | by the plug-in. \ingroup pluginBase
  |
  */
pub struct PClassInfo {

    /**
      | Class ID 16 Byte class GUID
      |
      */
    cid:         TUID,

    /**
      | cardinality of the class, set to kManyInstances
      | (see \ref PClassInfoClassCardinality)
      |
      */
    cardinality: i32,

    /**
      | class category, host uses this to categorize
      | interfaces
      |
      */
    category:    [u8; PCLASS_INFO_CATEGORY_SIZE],

    /**
      | class name, visible to the user
      |
      */
    name:        [u8; PCLASS_INFO_NAME_SIZE],
}

impl PClassInfo {
    
    pub fn new(
        cid:         TUID,
        cardinality: i32,
        category:    *const u8,
        name:        *const u8) -> Self {
    
        todo!();
        /*


            memset (this, 0, sizeof (PClassInfo));
            memcpy (cid, _cid, sizeof (TUID));
            if (_category)
                strncpy8 (category, _category, kCategorySize);
            if (_name)
                strncpy8 (name, _name, kNameSize);
            cardinality = _cardinality;
        */
    }
}

impl Default for PClassInfo {

    #[cfg(SMTG_CPP11)]
    fn default() -> Self {
    
        todo!();
        /*
        : cid(),
        : cardinality(),
        : category(),
        : name(),

        
        */
    }

    #[cfg(not(SMTG_CPP11))]
    fn default() -> Self {
        todo!();
        /*


            memset (this, 0, sizeof (PClassInfo));
        */
    }
}
