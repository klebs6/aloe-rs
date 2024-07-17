crate::ix!();

/**
  | Contains information about one of the
  | entries in a ZipFile.
  | 
  | @see ZipFile::getEntry
  |
  */
pub struct ZipEntry
{
    /**
      | The name of the file, which may also include
      | a partial pathname.
      |
      */
    filename:          String,

    /**
      | The file's original size
      |
      */
    uncompressed_size: i64,

    /**
      | The last time the file was modified
      |
      */
    file_time:         Time,

    /**
      | True if the zip entry is a symbolic link.
      |
      */
    is_symbolic_link:  bool,

    /**
      | Platform specific data. Depending
      | on how the zip file was created this may
      | contain macOS and Linux file types,
      | permissions and setuid/setgid/sticky
      | bits.
      |
      */
    external_file_attributes: u32,
}
