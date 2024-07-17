crate::ix!();

/**
  | Contains cached information about
  | one of the files in a DirectoryContentsList.
  |
  */
pub struct DirectoryContentsListFileInfo
{
    /**
      | The filename. This isn't a full pathname, it's
      | just the last part of the path, same as you'd get
      | from File::getFileName(). To get the full pathname,
      | use DirectoryContentsList::getDirectory().getChildFile
      | (filename).
      */
    filename:          String,

    /**
      | File size in bytes.
      |
      */
    file_size:         i64,

    /**
      | File modification time. As supplied
      | by File::getLastModificationTime().
      |
      */
    modification_time: Time,

    /**
      | File creation time. As supplied by
      | File::getCreationTime().
      |
      */
    creation_time:     Time,

    /**
      | True if the file is a directory.
      |
      */
    is_directory:      bool,

    /**
      | True if the file is read-only.
      |
      */
    is_read_only:      bool,
}
