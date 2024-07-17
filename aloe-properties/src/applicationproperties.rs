crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/app_properties/aloe_ApplicationProperties.h]

/**
  | Manages a collection of properties.
  | 
  | This is a slightly higher-level wrapper
  | for managing PropertiesFile objects.
  | 
  | It holds two different PropertiesFile
  | objects internally, one for user-specific
  | settings (stored in your user directory),
  | and one for settings that are common
  | to all users (stored in a folder accessible
  | to all users).
  | 
  | The class manages the creation of these
  | files on-demand, allowing access via
  | the getUserSettings() and getCommonSettings()
  | methods.
  | 
  | After creating an instance of an ApplicationProperties
  | object, you should first of all call
  | setStorageParameters() to tell it
  | the parameters to use to create its files.
  | 
  | @see PropertiesFile
  | 
  | @tags{DataStructures}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ApplicationProperties<'a> {
    options:                       PropertiesFileOptions,
    user_props:                    Box<PropertiesFile<'a>>,
    common_props:                  Box<PropertiesFile<'a>>,
    common_settings_are_read_only: i32, // default = 0
}

impl<'a> Default for ApplicationProperties<'a> {
    
    /**
      | Creates an ApplicationProperties
      | object.
      | 
      | Before using it, you must call setStorageParameters()
      | to give it the info it needs to create
      | the property files.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl<'a> Drop for ApplicationProperties<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        closeFiles();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/app_properties/aloe_ApplicationProperties.cpp]
impl<'a> ApplicationProperties<'a> {
    
    /**
      | Returns the current storage parameters.
      | @see setStorageParameters
      |
      */
    pub fn get_storage_parameters(&self) -> &PropertiesFileOptions {
        
        todo!();
        /*
            return options;
        */
    }
    
    /**
      | Gives the object the information it
      | needs to create the appropriate properties
      | files.
      | 
      | See the PropertiesFile::Options class
      | for details about what options you need
      | to set.
      |
      */
    pub fn set_storage_parameters(&mut self, new_options: &PropertiesFileOptions)  {
        
        todo!();
        /*
            options = newOptions;
        */
    }

    pub fn open_files(&mut self)  {
        
        todo!();
        /*
            // You need to call setStorageParameters() before trying to get hold of the properties!
        jassert (options.applicationName.isNotEmpty());

        if (options.applicationName.isNotEmpty())
        {
            PropertiesFile::Options o (options);

            if (userProps == nullptr)
            {
                o.commonToAllUsers = false;
                userProps.reset (new PropertiesFile (o));
            }

            if (commonProps == nullptr)
            {
                o.commonToAllUsers = true;
                commonProps.reset (new PropertiesFile (o));
            }

            userProps->setFallbackPropertySet (commonProps.get());
        }
        */
    }

    /**
      | Returns the user settings file.
      | 
      | The first time this is called, it will
      | create and load the properties file.
      | 
      | Note that when you search the user PropertiesFile
      | for a value that it doesn't contain,
      | the common settings are used as a second-chance
      | place to look. This is done via the PropertySet::setFallbackPropertySet()
      | method - by default the common settings
      | are set to the fallback for the user settings.
      | 
      | @see getCommonSettings
      |
      */
    pub fn get_user_settings(&mut self) -> *mut PropertiesFile {
        
        todo!();
        /*
            if (userProps == nullptr)
            openFiles();

        return userProps.get();
        */
    }

    /**
      | Returns the common settings file.
      | 
      | The first time this is called, it will
      | create and load the properties file.
      | 
      | -----------
      | @param returnUserPropsIfReadOnly
      | 
      | if this is true, and the common properties
      | file is read-only (e.g. because the
      | user doesn't have permission to write
      | to shared files), then this will return
      | the user settings instead, (like getUserSettings()
      | would do).
      | 
      | This is handy if you'd like to write a
      | value to the common settings, but if
      | that's no possible, then you'd rather
      | write to the user settings than none
      | at all.
      | 
      | If returnUserPropsIfReadOnly is false,
      | this method will always return the common
      | settings, even if any changes to them
      | can't be saved. @see getUserSettings
      |
      */
    pub fn get_common_settings(&mut self, return_user_props_if_read_only: bool) -> *mut PropertiesFile {
        
        todo!();
        /*
            if (commonProps == nullptr)
            openFiles();

        if (returnUserPropsIfReadOnly)
        {
            if (commonSettingsAreReadOnly == 0)
                commonSettingsAreReadOnly = commonProps->save() ? -1 : 1;

            if (commonSettingsAreReadOnly > 0)
                return userProps.get();
        }

        return commonProps.get();
        */
    }

    /**
      | Saves both files if they need to be saved.
      | 
      | @see PropertiesFile::saveIfNeeded
      |
      */
    pub fn save_if_needed(&mut self) -> bool {
        
        todo!();
        /*
            return (userProps == nullptr || userProps->saveIfNeeded())
             && (commonProps == nullptr || commonProps->saveIfNeeded());
        */
    }

    /**
      | Flushes and closes both files if they
      | are open.
      | 
      | This flushes any pending changes to
      | disk with PropertiesFile::saveIfNeeded()
      | and closes both files. They will then
      | be re-opened the next time getUserSettings()
      | or getCommonSettings() is called.
      |
      */
    pub fn close_files(&mut self)  {
        
        todo!();
        /*
            userProps.reset();
        commonProps.reset();
        */
    }
}
