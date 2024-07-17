crate::ix!();

/**
  | Structure describing properties file
  | options
  |
  */
pub struct PropertiesFileOptions {

    /**
      | The name of your application - this is
      | used to help generate the path and filename
      | at which the properties file will be
      | stored.
      |
      */
    application_name: String,

    /**
      | The suffix to use for your properties
      | file. It doesn't really matter what
      | this is - you may want to use ".settings"
      | or ".properties" or something. If the
      | suffix includes the prefixing dot (for
      | example ".settings") then the suffix
      | of applicationName will be replaced
      | with your suffix ("MyApp.exe" -> "MyApp.settings").
      | If your filenameSuffix does NOT include
      | the dot, then the suffix will be appended
      | to the applicationName ("MyApp.exe"
      | -> "MyApp.exe.settings").
      |
      */
    filename_suffix: String,

    /**
      | The name of a subfolder in which you'd
      | like your properties file to live. See
      | the getDefaultFile() method for more
      | details about how this is used.
      |
      */
    folder_name: String,

    /**
      | If you're using properties files on
      | a Mac, you must set this value - failure
      | to do so will cause a runtime assertion.
      | 
      | The PropertiesFile class always used
      | to put its settings files in "Library/Preferences",
      | but Apple have changed their advice,
      | and now stipulate that settings should
      | go in "Library/Application Support".
      | 
      | Because older apps would be broken by
      | a silent change in this class's behaviour,
      | you must now explicitly set the osxLibrarySubFolder
      | value to indicate which path you want
      | to use.
      | 
      | In newer apps, you should always set
      | this to "Application Support" or "Application
      | Support/YourSubFolderName".
      | 
      | If your app needs to load settings files
      | that were created by older versions
      | of aloe and you want to maintain backwards-compatibility,
      | then you can set this to "Preferences".
      | But.. for better Apple-compliance,
      | the recommended approach would be to
      | write some code that finds your old settings
      | files in ~/Library/Preferences, moves
      | them to ~/Library/Application Support,
      | and then uses the new path.
      |
      */
    osx_library_sub_folder: String,

    /**
      | If true, the file will be created in a
      | location that's shared between users.
      | 
      | The default constructor initialises
      | this value to false.
      |
      */
    common_to_all_users: bool,

    /**
      | If true, this means that property names
      | are matched in a case-insensitive manner.
      | 
      | See the PropertySet constructor for
      | more info.
      | 
      | The default constructor initialises
      | this value to false.
      |
      */
    ignore_case_of_key_names: bool,

    /**
      | If set to true, this prevents the file
      | from being written to disk.
      |
      */
    do_not_save: bool,

    /**
      | If this is zero or greater, then after
      | a value is changed, the object will wait
      | for this amount of time and then save
      | the file. If this zero, the file will
      | be written to disk immediately on being
      | changed (which might be slow, as it'll
      | re-write synchronously each time a
      | value-change method is called). If
      | it is less than zero, the file won't be
      | saved until save() or saveIfNeeded()
      | are explicitly called. The default
      | constructor sets this to a reasonable
      | value of a few seconds, so you only need
      | to change it if you need a special case.
      |
      */
    milliseconds_before_saving: i32,

    /**
      | Specifies whether the file should be
      | written as XML, binary, etc. The default
      | constructor sets this to storeAsXML,
      | so you only need to set it explicitly
      | if you want to use a different format.
      |
      */
    storage_format: PropertiesFileStorageFormat,

    /**
      | An optional InterprocessLock object
      | that will be used to prevent multiple
      | threads or processes from writing to
      | the file at the same time. The PropertiesFile
      | will keep a pointer to this object but
      | will not take ownership of it - the caller
      | is responsible for making sure that
      | the lock doesn't get deleted before
      | the PropertiesFile has been deleted.
      | The default constructor initialises
      | this value to nullptr, so you don't need
      | to touch it unless you want to use a lock.
      |
      */
    process_lock: *mut InterProcessLock,
}

impl Default for PropertiesFileOptions {
    
    /**
      | Creates an empty PropertiesFileOptions structure.
      | 
      | You'll need to fill-in the data members
      | appropriately before using this structure.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            : commonToAllUsers (false),
          ignoreCaseOfKeyNames (false),
          doNotSave (false),
          millisecondsBeforeSaving (3000),
          storageFormat (PropertiesFile::storeAsXML),
          processLock (nullptr)
        */
    }
}

impl PropertiesFileOptions {
    
    /**
      | This can be called to suggest a file that
      | should be used, based on the values in
      | this structure.
      | 
      | So on a Mac, this will return a file called:
      | ~/Library/[osxLibrarySubFolder]/[folderName]/[applicationName].[filenameSuffix]
      | 
      | On Windows it'll return something like:
      | C:\\Documents and Settings\\username\\Application
      | Data\\[folderName]\\[applicationName].[filenameSuffix]
      | 
      | On Linux it'll return ~/[folderName]/[applicationName].[filenameSuffix]
      | 
      | If the folderName variable is empty,
      | it'll use the app name for this (or omit
      | the folder name on the Mac).
      | 
      | The paths will also vary depending on
      | whether commonToAllUsers is true.
      |
      */
    pub fn get_default_file(&self) -> File {
        
        todo!();
        /*
            // mustn't have illegal characters in this name..
        jassert (applicationName == File::createLegalFileName (applicationName));

       #if ALOE_MAC || ALOE_IOS
        File dir (commonToAllUsers ?  "/Library/"
                                   : "~/Library/");

        if (osxLibrarySubFolder != "Preferences"
            && ! osxLibrarySubFolder.startsWith ("Application Support")
            && ! osxLibrarySubFolder.startsWith ("Containers"))
        {
            /* The PropertiesFile class always used to put its settings files in "Library/Preferences", but Apple
               have changed their advice, and now stipulate that settings should go in "Library/Application Support",
               or Library/Containers/[app_bundle_id] for a sandboxed app.

               Because older apps would be broken by a silent change in this class's behaviour, you must now
               explicitly set the osxLibrarySubFolder value to indicate which path you want to use.

               In newer apps, you should always set this to "Application Support"
               or "Application Support/YourSubFolderName".

               If your app needs to load settings files that were created by older versions of aloe and
               you want to maintain backwards-compatibility, then you can set this to "Preferences".
               But.. for better Apple-compliance, the recommended approach would be to write some code that
               finds your old settings files in ~/Library/Preferences, moves them to ~/Library/Application Support,
               and then uses the new path.
            */
            jassertfalse;

            dir = dir.getChildFile ("Application Support");
        }
        else
        {
            dir = dir.getChildFile (osxLibrarySubFolder);
        }

        if (folderName.isNotEmpty())
            dir = dir.getChildFile (folderName);

       #elif ALOE_LINUX || ALOE_BSD || ALOE_ANDROID
        auto dir = File (commonToAllUsers ? "/var" : "~")
                          .getChildFile (folderName.isNotEmpty() ? folderName
                                                                 : ("." + applicationName));

       #elif ALOE_WINDOWS
        auto dir = File::getSpecialLocation (commonToAllUsers ? File::commonApplicationDataDirectory
                                                              : File::userApplicationDataDirectory);

        if (dir == File())
            return {};

        dir = dir.getChildFile (folderName.isNotEmpty() ? folderName
                                                        : applicationName);
       #endif

        return (filenameSuffix.startsWithChar (L'.')
                   ? dir.getChildFile (applicationName).withFileExtension (filenameSuffix)
                   : dir.getChildFile (applicationName + "." + filenameSuffix));
        */
    }
}
