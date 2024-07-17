crate::ix!();

pub const PCLASS_INFOW_VENDOR_SIZE:        usize = 64;
pub const PCLASS_INFOW_VERSION_SIZE:       usize = 64;
pub const PCLASS_INFOW_SUBCATEGORIES_SIZE: usize = 128;

/**
  | Unicode Version of Basic Information
  | about a class provided by the plug-in
  |
  */
pub struct PClassInfoW {

    /**
      | see \ref PClassInfo
      |
      */
    cid:            TUID,

    /**
      | see \ref PClassInfo
      |
      */
    cardinality:    i32,

    /**
      | see \ref PClassInfo
      |
      */
    category:       [u8; PCLASS_INFO_CATEGORY_SIZE],

    /**
      | see \ref PClassInfo
      |
      */
    name:           [u16; PCLASS_INFO_NAME_SIZE],

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
    sub_categories: [u8; PCLASS_INFOW_SUBCATEGORIES_SIZE],

    /**
      | overwrite vendor information from
      | factory info
      |
      */
    vendor:         [u16; PCLASS_INFOW_VENDOR_SIZE],

    /**
      | Version string (e.g. "1.0.0.512" with
      | Major.Minor.Subversion.Build)
      |
      */
    version:        [u16; PCLASS_INFOW_VERSION_SIZE],

    /**
      | SDK version used to build this class
      | (e.g. "VST 3.0")
      |
      */
    sdk_version:    [u16; PCLASS_INFOW_VERSION_SIZE],
}

impl PClassInfoW {

    pub fn new(
        cid:            TUID,
        cardinality:    i32,
        category:       *const u8,
        name:           *const u16,
        class_flags:    i32,
        sub_categories: *const u8,
        vendor:         *const u16,
        version:        *const u16,
        sdk_version:    *const u16) -> Self {
    
        todo!();
        /*


            memset (this, 0, sizeof (PClassInfoW));
            memcpy (cid, _cid, sizeof (TUID));
            cardinality = _cardinality;
            if (_category)
                strncpy8 (category, _category, PClassInfo::kCategorySize);
            if (_name)
                strncpy16 (name, _name, PClassInfo::kNameSize);
            classFlags = static_cast<uint32> (_classFlags);
            if (_subCategories)
                strncpy8 (subCategories, _subCategories, kSubCategoriesSize);
            if (_vendor)
                strncpy16 (vendor, _vendor, kVendorSize);
            if (_version)
                strncpy16 (version, _version, kVersionSize);
            if (_sdkVersion)
                strncpy16 (sdkVersion, _sdkVersion, kVersionSize);
        */
    }

    pub fn from_ascii(&mut self, ci2: &PClassInfo2)  {
        
        todo!();
        /*
            memcpy (cid, ci2.cid, sizeof (TUID));
            cardinality = ci2.cardinality;
            strncpy8 (category, ci2.category, PClassInfo::kCategorySize);
            str8ToStr16 (name, ci2.name, PClassInfo::kNameSize);
            classFlags = ci2.classFlags;
            strncpy8 (subCategories, ci2.subCategories, kSubCategoriesSize);

            str8ToStr16 (vendor, ci2.vendor, kVendorSize);
            str8ToStr16 (version, ci2.version, kVersionSize);
            str8ToStr16 (sdkVersion, ci2.sdkVersion, kVersionSize);
        */
    }
}

impl Default for PClassInfoW {

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


            memset (this, 0, sizeof (PClassInfoW));
        */
    }
}
