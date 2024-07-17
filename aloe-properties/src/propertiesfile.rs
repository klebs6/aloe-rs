crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/app_properties/aloe_PropertiesFile.h]

/**
  | Wrapper on a file that stores a list of
  | key/value data pairs.
  | 
  | Useful for storing application settings,
  | etc. See the PropertySet class for the
  | interfaces that read and write values.
  | 
  | Not designed for very large amounts
  | of data, as it keeps all the values in
  | memory and writes them out to disk lazily
  | when they are changed.
  | 
  | Because this class derives from ChangeBroadcaster,
  | ChangeListeners can be registered
  | with it, and these will be signalled
  | when a value changes.
  | 
  | @see PropertySet
  | 
  | @tags{DataStructures}
  |
  */
#[leak_detector]
#[no_copy]
pub struct PropertiesFile<'a> {
    base:          PropertySet,
    base2:         ChangeBroadcaster<'a>,
    base3:         Timer,
    file:          File,
    options:       PropertiesFileOptions,
    loaded_ok:     bool, // default = false
    needs_writing: bool, // default = false
}

pub enum PropertiesFileStorageFormat
{
    storeAsBinary,
    storeAsCompressedBinary,
    storeAsXML
}

pub type PropertiesFileProcessScopedLock<'a> = Box<ScopedLockType<'a>>;

pub const PROPERTIES_FILE_MAGIC_NUMBER:            i32 = ByteOrder::make_int32('P' as u8, 'R' as u8, 'O' as u8, 'P' as u8) as i32;
pub const PROPERTIES_FILE_MAGIC_NUMBER_COMPRESSED: i32 = ByteOrder::make_int32('C' as u8, 'P' as u8, 'R' as u8, 'P' as u8) as i32;

pub const PROPERTIES_FILE_FILE_TAG:        &'static str = "PROPERTIES";
pub const PROPERTIES_FILE_VALUE_TAG:       &'static str = "VALUE";
pub const PROPERTIES_FILE_NAME_ATTRIBUTE:  &'static str = "name";
pub const PROPERTIES_FILE_VALUE_ATTRIBUTE: &'static str = "val";

impl<'a> Drop for PropertiesFile<'a> {

    /**
      | Destructor.
      | 
      | When deleted, the file will first call
      | saveIfNeeded() to flush any changes
      | to disk.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       
                 saveIfNeeded();
                 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/app_properties/aloe_PropertiesFile.cpp]

impl<'a> PropertiesFile<'a> {

    /**
      | Returns true if this file was created
      | from a valid (or non-existent) file.
      | 
      | If the file failed to load correctly
      | because it was corrupt or had insufficient
      | access, this will be false.
      |
      */
    pub fn is_valid_file(&self) -> bool {
        
        todo!();
        /*
            return loadedOk;
        */
    }

    /**
      | Returns the file that's being used.
      |
      */
    pub fn get_file(&self) -> &File {
        
        todo!();
        /*
            return file;
        */
    }

    /**
      | Creates a PropertiesFile object.
      | 
      | Unlike the other constructor, this
      | one allows you to explicitly set the
      | file that you want to be used, rather
      | than using the default one.
      |
      */
    pub fn new_with_file_and_properties_file_options(
        f: &File,
        o: &PropertiesFileOptions

    ) -> Self {
    
        todo!();
        /*


            : PropertySet (o.ignoreCaseOfKeyNames),
          file (f), options (o)

        reload();
        */
    }
    
    /**
      | Creates a PropertiesFile object.
      | 
      | The file used will be chosen by calling
      | PropertiesFile::PropertiesFileOptions::getDefaultFile()
      | for the options provided. To set the
      | file explicitly, use the other constructor.
      |
      */
    pub fn new_from_properties_file_options(o: &PropertiesFileOptions) -> Self {
    
        todo!();
        /*


            : PropertySet (o.ignoreCaseOfKeyNames),
          file (o.getDefaultFile()), options (o)

        reload();
        */
    }
    
    /**
      | Attempts to reload the settings from
      | the file.
      |
      */
    pub fn reload(&mut self) -> bool {
        
        todo!();
        /*
            ProcessScopedLock pl (createProcessLock());

        if (pl != nullptr && ! pl->isLocked())
            return false; // locking failure..

        loadedOk = (! file.exists()) || loadAsBinary() || loadAsXml();
        return loadedOk;
        */
    }
    
    pub fn create_process_lock(&self) -> *mut ScopedLockType {
        
        todo!();
        /*
            return options.processLock != nullptr ? new InterProcessLock::ScopedLockType (*options.processLock) : nullptr;
        */
    }

    /**
      | This will flush all the values to disk
      | if they've changed since the last time
      | they were saved.
      | 
      | Returns false if it fails to write to
      | the file for some reason (maybe because
      | it's read-only or the directory doesn't
      | exist or something).
      | 
      | @see save
      |
      */
    pub fn save_if_needed(&mut self) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (getLock());
        return (! needsWriting) || save();
        */
    }

    /**
      | Returns true if the properties have
      | been altered since the last time they
      | were saved.
      | 
      | The file is flagged as needing to be saved
      | when you change a value, but you can explicitly
      | set this flag with setNeedsToBeSaved().
      |
      */
    pub fn needs_to_be_saved(&self) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (getLock());
        return needsWriting;
        */
    }

    /**
      | Explicitly sets the flag to indicate
      | whether the file needs saving or not.
      | @see needsToBeSaved
      |
      */
    pub fn set_needs_to_be_saved(&mut self, needs_to_be_saved: bool)  {
        
        todo!();
        /*
            const ScopedLock sl (getLock());
        needsWriting = needsToBeSaved_;
        */
    }

    /**
      | This will force a write-to-disk of the
      | current values, regardless of whether
      | anything has changed since the last
      | save.
      | 
      | Returns false if it fails to write to
      | the file for some reason (maybe because
      | it's read-only or the directory doesn't
      | exist or something).
      | 
      | @see saveIfNeeded
      |
      */
    pub fn save(&mut self) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (getLock());

        stopTimer();

        if (options.doNotSave
             || file == File()
             || file.isDirectory()
             || ! file.getParentDirectory().createDirectory())
            return false;

        if (options.storageFormat == storeAsXML)
            return saveAsXml();

        return saveAsBinary();
        */
    }

    pub fn load_as_xml(&mut self) -> bool {
        
        todo!();
        /*
            if (auto doc = parseXMLIfTagMatches (file, PropertyFileConstants::fileTag))
        {
            for (auto* e : doc->getChildWithTagNameIterator (PropertyFileConstants::valueTag))
            {
                auto name = e->getStringAttribute (PropertyFileConstants::nameAttribute);

                if (name.isNotEmpty())
                    getAllProperties().set (name,
                                            e->getFirstChildElement() != nullptr
                                                ? e->getFirstChildElement()->toString (XmlElement::TextFormat().singleLine().withoutHeader())
                                                : e->getStringAttribute (PropertyFileConstants::valueAttribute));
            }

            return true;
        }

        return false;
        */
    }

    pub fn save_as_xml(&mut self) -> bool {
        
        todo!();
        /*
            XmlElement doc (PropertyFileConstants::fileTag);
        auto& props = getAllProperties();

        for (int i = 0; i < props.size(); ++i)
        {
            auto* e = doc.createNewChildElement (PropertyFileConstants::valueTag);
            e->setAttribute (PropertyFileConstants::nameAttribute, props.getAllKeys() [i]);

            // if the value seems to contain xml, store it as such..
            if (auto childElement = parseXML (props.getAllValues() [i]))
                e->addChildElement (childElement.release());
            else
                e->setAttribute (PropertyFileConstants::valueAttribute, props.getAllValues() [i]);
        }

        ProcessScopedLock pl (createProcessLock());

        if (pl != nullptr && ! pl->isLocked())
            return false; // locking failure..

        if (doc.writeTo (file, {}))
        {
            needsWriting = false;
            return true;
        }

        return false;
        */
    }

    pub fn load_as_binary(&mut self) -> bool {
        
        todo!();
        /*
            FileInputStream fileStream (file);

        if (fileStream.openedOk())
        {
            auto magicNumber = fileStream.readInt();

            if (magicNumber == PropertyFileConstants::magicNumberCompressed)
            {
                SubregionStream subStream (&fileStream, 4, -1, false);
                GZIPDecompressorInputStream gzip (subStream);
                return loadAsBinary (gzip);
            }

            if (magicNumber == PropertyFileConstants::magicNumber)
                return loadAsBinary (fileStream);
        }

        return false;
        */
    }

    pub fn load_as_binary_from_input<R: Read>(&mut self, input: &mut R) -> bool {
        
        todo!();
        /*
            BufferedInputStream in (input, 2048);

        int numValues = in.readInt();

        while (--numValues >= 0 && ! in.isExhausted())
        {
            auto key = in.readString();
            auto value = in.readString();
            jassert (key.isNotEmpty());

            if (key.isNotEmpty())
                getAllProperties().set (key, value);
        }

        return true;
        */
    }

    pub fn save_as_binary(&mut self) -> bool {
        
        todo!();
        /*
            ProcessScopedLock pl (createProcessLock());

        if (pl != nullptr && ! pl->isLocked())
            return false; // locking failure..

        TemporaryFile tempFile (file);

        {
            FileOutputStream out (tempFile.getFile());

            if (! out.openedOk())
                return false;

            if (options.storageFormat == storeAsCompressedBinary)
            {
                out.writeInt (PropertyFileConstants::magicNumberCompressed);
                out.flush();

                GZIPCompressorOutputStream zipped (out, 9);

                if (! writeToStream (zipped))
                    return false;
            }
            else
            {
                // have you set up the storage option flags correctly?
                jassert (options.storageFormat == storeAsBinary);

                out.writeInt (PropertyFileConstants::magicNumber);

                if (! writeToStream (out))
                    return false;
            }
        }

        if (! tempFile.overwriteTargetFileWithTemporary())
            return false;

        needsWriting = false;
        return true;
        */
    }

    pub fn write_to_stream<W: Write>(
        &mut self, 
        out: &mut W

    ) -> bool {
        
        todo!();
        /*
            auto& props  = getAllProperties();
        auto& keys   = props.getAllKeys();
        auto& values = props.getAllValues();
        auto numProperties = props.size();

        if (! out.writeInt (numProperties))
            return false;

        for (int i = 0; i < numProperties; ++i)
        {
            if (! out.writeString (keys[i]))   return false;
            if (! out.writeString (values[i])) return false;
        }

        return true;
        */
    }

    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            saveIfNeeded();
        */
    }

    pub fn property_changed(&mut self)  {
        
        todo!();
        /*
            sendChangeMessage();
        needsWriting = true;

        if (options.millisecondsBeforeSaving > 0)
            startTimer (options.millisecondsBeforeSaving);
        else if (options.millisecondsBeforeSaving == 0)
            saveIfNeeded();
        */
    }
}
