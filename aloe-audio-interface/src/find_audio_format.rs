crate::ix!();

pub trait GetNumKnownFormats {

    /**
      | Returns the number of currently registered
      | file formats.
      |
      */
    fn get_num_known_formats(&self) -> i32;
}

pub trait GetKnownFormat {

    /**
      | Returns one of the registered file formats.
      |
      */
    fn get_known_format(&self, index: i32) -> Rc<RefCell<dyn AudioFormatInterface>>;
}

pub trait GetDefaultFormat {

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
    fn get_default_format(&self) -> Rc<RefCell<dyn AudioFormatInterface>>;
}

pub trait FindFormatForFileExtension {

    /**
      | Looks for which of the known formats
      | is listed as being for a given file extension.
      | 
      | The extension may have a dot before it,
      | so e.g. ".wav" or "wav" are both ok.
      |
      */
    fn find_format_for_file_extension(&self, file_extension: &String) -> Rc<RefCell<dyn AudioFormatInterface>>;
}

pub trait GetWildcardForAllFormats {

    /**
      | Returns a set of wildcards for file-matching
      | that contains the extensions for all
      | known formats.
      | 
      | E.g. if might return "*.wav;*.aiff"
      | if it just knows about wavs and aiffs.
      |
      */
    fn get_wildcard_for_all_formats(&self) -> String;
}
