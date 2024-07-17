crate::ix!();

pub trait SetRepresentedFile {

    /**
      | If this type of window is capable of indicating
      | that it represents a file, then this
      | lets you set the file.
      | 
      | E.g. in OSX it'll show an icon for the
      | file in the title bar.
      |
      */
    fn set_represented_file(&mut self, _0: &File);
}
