crate::ix!();

const VENDOR_SIZE:        usize = 64;
const VERSION_SIZE:       usize = 64;
const SUBCATEGORIES_SIZE: usize = 128;

/**
  | Version 2 of Basic Information about
  | a class provided by the plug-in. \ingroup
  | pluginBase
  |
  */
pub struct PClassInfo2 {

    /**
      | Class ID 16 Byte class GUID
      |
      */
    cid:            TUID,

    /**
      | cardinality of the class, set to kManyInstances
      | (see \ref PClassInfo::ClassCardinality)
      |
      */
    cardinality:    i32,

    /**
      | class category, host uses this to categorize
      | interfaces
      |
      */
    category:       [u8; PCLASS_INFO_CATEGORY_SIZE],

    /**
      | class name, visible to the user
      |
      */
    name:           [u8; PCLASS_INFO_NAME_SIZE],

    /**
      | flags used for a specific category,
      | must be defined where category is defined
      |
      */
    class_flags:    u32,

    /**
      | module specific subcategories, can
      | be more than one, logically added by
      | the \c OR operator
      |
      */
    sub_categories: [u8; SUBCATEGORIES_SIZE],

    /**
      | overwrite vendor information from
      | factory info
      |
      */
    vendor:         [u8; VENDOR_SIZE],

    /**
      | Version string (e.g. "1.0.0.512" with
      | Major.Minor.Subversion.Build)
      |
      */
    version:        [u8; VERSION_SIZE],

    /**
      | SDK version used to build this class
      | (e.g. "VST 3.0")
      |
      */
    sdk_version:    [u8; VERSION_SIZE],

}

impl PClassInfo2 {

    pub fn new(
        cid:            TUID,
        cardinality:    i32,
        category:       *const u8,
        name:           *const u8,
        class_flags:    i32,
        sub_categories: *const u8,
        vendor:         *const u8,
        version:        *const u8,
        sdk_version:    *const u8) -> Self {
    
        todo!();
        /*


            memset (this, 0, sizeof (PClassInfo2));
            memcpy (cid, _cid, sizeof (TUID));
            cardinality = _cardinality;
            if (_category)
                strncpy8 (category, _category, PClassInfo::kCategorySize);
            if (_name)
                strncpy8 (name, _name, PClassInfo::kNameSize);
            classFlags = static_cast<uint32> (_classFlags);
            if (_subCategories)
                strncpy8 (subCategories, _subCategories, kSubCategoriesSize);
            if (_vendor)
                strncpy8 (vendor, _vendor, kVendorSize);
            if (_version)
                strncpy8 (version, _version, kVersionSize);
            if (_sdkVersion)
                strncpy8 (sdkVersion, _sdkVersion, kVersionSize);
        */
    }
}

impl Default for PClassInfo2 {

    #[cfg(SMTG_CPP11)]
    fn default() -> Self {
    
        todo!();
        /*
        : cid(),
        : cardinality(),
        : category(),
        : name(),
        : class_flags(),
        : sub_categories(),
        : vendor(),
        : version(),
        : sdk_version(),

        
        */
    }

    #[cfg(not(SMTG_CPP11))]
    fn default() -> Self {
        todo!();
        /*


            memset (this, 0, sizeof (PClassInfo2));
        */
    }
}
