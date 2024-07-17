crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatManager.h]

/**
  | A class for keeping a list of available
  | audio formats, and for deciding which
  | one to use to open a given file.
  | 
  | After creating an AudioFormatManager
  | object, you should call registerFormat()
  | or registerBasicFormats() to give
  | it a list of format types that it can use.
  | 
  | @see AudioFormat
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioFormatManager {
    known_formats:        Vec<Box<AudioFormat>>,
    default_format_index: i32, // default = 0
}

impl Default for AudioFormatManager {
    
    /**
      | Creates an empty format manager.
      | 
      | Before it'll be any use, you'll need
      | to call registerFormat() with all the
      | formats you want it to be able to recognise.
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatManager.cpp]
impl AudioFormatManager {

    /**
      | Iterator access to the list of known
      | formats.
      |
      */
    pub fn begin_mut(&mut self) -> *mut *mut AudioFormat {
        
        todo!();
        /*
            return knownFormats.begin();
        */
    }

    /**
      | Iterator access to the list of known
      | formats.
      |
      */
    pub fn begin(&self) -> *const *const AudioFormat {
        
        todo!();
        /*
            return knownFormats.begin();
        */
    }

    /**
      | Iterator access to the list of known
      | formats.
      |
      */
    pub fn end_mut(&mut self) -> *mut *mut AudioFormat {
        
        todo!();
        /*
            return knownFormats.end();
        */
    }

    /**
      | Iterator access to the list of known
      | formats.
      |
      */
    pub fn end(&self) -> *const *const AudioFormat {
        
        todo!();
        /*
            return knownFormats.end();
        */
    }

    /**
      | Adds a format to the manager's list of
      | available file types.
      | 
      | The object passed-in will be deleted
      | by this object, so don't keep a pointer
      | to it!
      | 
      | If makeThisTheDefaultFormat is true,
      | then the getDefaultFormat() method
      | will return this one when called.
      |
      */
    pub fn register_format(&mut self, 
        new_format:                   *mut AudioFormat,
        make_this_the_default_format: bool)  {
        
        todo!();
        /*
            jassert (newFormat != nullptr);

        if (newFormat != nullptr)
        {
           #if ALOE_DEBUG
            for (auto* af : knownFormats)
            {
                if (af->getFormatName() == newFormat->getFormatName())
                    jassertfalse; // trying to add the same format twice!
            }
           #endif

            if (makeThisTheDefaultFormat)
                defaultFormatIndex = getNumKnownFormats();

            knownFormats.add (newFormat);
        }
        */
    }
    
    /**
      | Handy method to make it easy to register
      | the formats that come with Aloe.
      | 
      | This will add WAV and AIFF to the list,
      | along with any other formats enabled
      | in either the Proaloer or your application's
      | preprocessor definitions.
      |
      */
    pub fn register_basic_formats(&mut self)  {
        
        todo!();
        /*
            registerFormat (new WavAudioFormat(), true);
        registerFormat (new AiffAudioFormat(), false);

       #if ALOE_USE_FLAC
        registerFormat (new FlacAudioFormat(), false);
       #endif

       #if ALOE_USE_OGGVORBIS
        registerFormat (new OggVorbisAudioFormat(), false);
       #endif

       #if ALOE_MAC || ALOE_IOS
        registerFormat (new CoreAudioFormat(), false);
       #endif

       #if ALOE_USE_MP3AUDIOFORMAT
        registerFormat (new MP3AudioFormat(), false);
       #endif

       #if ALOE_USE_WINDOWS_MEDIA_FORMAT
        registerFormat (new WindowsMediaAudioFormat(), false);
       #endif
        */
    }
    
    /**
      | Clears the list of known formats.
      |
      */
    pub fn clear_formats(&mut self)  {
        
        todo!();
        /*
            knownFormats.clear();
        defaultFormatIndex = 0;
        */
    }
    
    /**
      | Returns the number of currently registered
      | file formats.
      |
      */
    pub fn get_num_known_formats(&self) -> i32 {
        
        todo!();
        /*
            return knownFormats.size();
        */
    }
    
    /**
      | Returns one of the registered file formats.
      |
      */
    pub fn get_known_format(&self, index: i32) -> *mut AudioFormat {
        
        todo!();
        /*
            return knownFormats[index];
        */
    }
    
    /**
      | Returns the format which has been set
      | as the default one.
      | 
      | You can set a format as being the default
      | when it is registered. It's useful when
      | you want to write to a file, because the
      | best format may change between platforms,
      | e.g. AIFF is preferred on the Mac, WAV
      | on Windows.
      | 
      | If none has been set as the default, this
      | method will just return the first one
      | in the list.
      |
      */
    pub fn get_default_format(&self) -> *mut AudioFormat {
        
        todo!();
        /*
            return getKnownFormat (defaultFormatIndex);
        */
    }
    
    /**
      | Looks for which of the known formats
      | is listed as being for a given file extension.
      | 
      | The extension may have a dot before it,
      | so e.g. ".wav" or "wav" are both ok.
      |
      */
    pub fn find_format_for_file_extension(&self, file_extension: &String) -> *mut AudioFormat {
        
        todo!();
        /*
            if (! fileExtension.startsWithChar ('.'))
            return findFormatForFileExtension ("." + fileExtension);

        for (auto* af : knownFormats)
            if (af->getFileExtensions().contains (fileExtension, true))
                return af;

        return nullptr;
        */
    }
    
    /**
      | Returns a set of wildcards for file-matching
      | that contains the extensions for all
      | known formats.
      | 
      | E.g. if might return "*.wav;*.aiff"
      | if it just knows about wavs and aiffs.
      |
      */
    pub fn get_wildcard_for_all_formats(&self) -> String {
        
        todo!();
        /*
            StringArray extensions;

        for (auto* af : knownFormats)
            extensions.addArray (af->getFileExtensions());

        extensions.trim();
        extensions.removeEmptyStrings();

        for (auto& e : extensions)
            e = (e.startsWithChar ('.') ? "*" : "*.") + e;

        extensions.removeDuplicates (true);
        return extensions.joinIntoString (";");
        */
    }
    
    /**
      | Searches through the known formats
      | to try to create a suitable reader for
      | this file.
      | 
      | If none of the registered formats can
      | open the file, it'll return nullptr.
      | 
      | It's the caller's responsibility to
      | delete the reader that is returned.
      |
      */
    pub fn create_reader_for<'a>(&mut self, file: &File) -> *mut AudioFormatReader<'a> {
        
        todo!();
        /*
            // you need to actually register some formats before the manager can
        // use them to open a file!
        jassert (getNumKnownFormats() > 0);

        for (auto* af : knownFormats)
            if (af->canHandleFile (file))
                if (auto in = file.createInputStream())
                    if (auto* r = af->createReaderFor (in.release(), true))
                        return r;

        return nullptr;
        */
    }
    
    /**
      | Searches through the known formats
      | to try to create a suitable reader for
      | this stream.
      | 
      | The stream object that is passed-in
      | will be deleted by this method or by the
      | reader that is returned, so the caller
      | should not keep any references to it.
      | 
      | The stream that is passed-in must be
      | capable of being repositioned so that
      | all the formats can have a go at opening
      | it.
      | 
      | If none of the registered formats can
      | open the stream, it'll return nullptr.
      | 
      | If it returns a reader, it's the caller's
      | responsibility to delete the reader.
      |
      */
    pub fn create_reader_for_with_file_stream<'a>(&mut self, audio_file_stream: Box<dyn Read>) -> *mut AudioFormatReader<'a> {
        
        todo!();
        /*
            // you need to actually register some formats before the manager can
        // use them to open a file!
        jassert (getNumKnownFormats() > 0);

        if (audioFileStream != nullptr)
        {
            auto originalStreamPos = audioFileStream->getPosition();

            for (auto* af : knownFormats)
            {
                if (auto* r = af->createReaderFor (audioFileStream.get(), false))
                {
                    audioFileStream.release();
                    return r;
                }

                audioFileStream->setPosition (originalStreamPos);

                // the stream that is passed-in must be capable of being repositioned so
                // that all the formats can have a go at opening it.
                jassert (audioFileStream->getPosition() == originalStreamPos);
            }
        }

        return nullptr;
        */
    }
}

