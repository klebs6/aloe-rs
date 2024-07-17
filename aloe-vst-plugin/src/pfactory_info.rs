crate::ix!();

pub enum PFactoryInfoFactoryFlags
{
    /**
      | Nothing
      */
    kNoFlags                    = 0,        

    /**
      | The number of exported classes can
      | change each time the Module is
      | loaded. If this flag is set, the host
      | does not cache class information. This
      | leads to a longer startup time because
      | the host always has to load the Module
      | to get the current class information.
      */
    kClassesDiscardable         = 1 << 0,   

    /**
      | Class IDs of components are interpreted
      | as Syncrosoft-License
      | (LICENCE_UID). Loaded in a Steinberg
      | host, the module will not be loaded when
      | the license is not valid
      */
    kLicenseCheck               = 1 << 1,   

    /**
      | Component will not be unloaded until
      | process exit
      */
    kComponentNonDiscardable    = 1 << 3,   

    /**
      | Components have entirely unicode encoded
      | strings. (True for VST 3 plug-ins so
      | far)
      */
    kUnicode                    = 1 << 4,    
}

pub const PFACTORY_INFO_URL_SIZE:   usize = 256;
pub const PFACTORY_INFO_EMAIL_SIZE: usize = 128;
pub const PFACTORY_INFO_NAME_SIZE:  usize = 64;

/**
  | Basic Information about the class factory
  | of the plug-in. \ingroup pluginBase
  |
  */
pub struct PFactoryInfo {

    /**
      | e.g. "Steinberg Media Technologies"
      |
      */
    vendor: [u8; PFACTORY_INFO_NAME_SIZE],

    /**
      | e.g. "http://www.steinberg.de"
      |
      */
    url:    [u8; PFACTORY_INFO_URL_SIZE],

    /**
      | e.g. "info@steinberg.de"
      |
      */
    email:  [u8; PFACTORY_INFO_EMAIL_SIZE],

    /**
      | (see above)
      |
      */
    flags:  i32,
}

impl PFactoryInfo {

    pub fn new(
        vendor: *const u8,
        url:    *const u8,
        email:  *const u8,
        flags:  i32) -> Self {
    
        todo!();
        /*


            strncpy8 (vendor, _vendor, kNameSize);
            strncpy8 (url, _url, kURLSize);
            strncpy8 (email, _email, kEmailSize);
            flags = _flags;
    #ifdef UNICODE
            flags |= kUnicode;
    #endif
        */
    }
}

impl Default for PFactoryInfo {

    #[cfg(SMTG_CPP11)]
    fn default() -> Self {
    
        todo!();
        /*
        : vendor(),
        : url(),
        : email(),
        : flags(),

        
        */
    }

    #[cfg(not(SMTG_CPP11))]
    fn default() -> Self {
        todo!();
        /*


            memset (this, 0, sizeof (PFactoryInfo));
        */
    }
}
