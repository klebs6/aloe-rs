crate::ix!();

/**
  | Used to create a new zip file.
  | 
  | Create a ZipFile::ZipFileBuilder object,
  | and call its addFile() method to add
  | some files, then you can write it to a
  | stream with write().
  |
  */
#[no_copy]
#[leak_detector]
#[derive(Default)]
pub struct ZipFileBuilder {

    items: Option<Vec<Box<ZipFileBuilderItem>>>,
}

impl ZipFileBuilder {

    /**
      | Adds a file to the list of items which
      | will be added to the archive.
      | 
      | The file isn't read immediately: the
      | files will be read later when the writeToStream()
      | method is called.
      | 
      | The compressionLevel can be between
      | 0 (no compression), and 9 (maximum compression).
      | 
      | If the storedPathName parameter is
      | specified, you can customise the partial
      | pathname that will be stored for this
      | file.
      |
      */
    pub fn add_file(
        &mut self, 
        file_to_add:       &File,
        compression_level: i32,
        stored_path_name:  Option<&str>
    ) {

        let stored_path_name: &str = stored_path_name.unwrap_or("");

        todo!();
        /*
            items.add (new ZipFileBuilderItem (file, nullptr, compression,
                                 path.isEmpty() ? file.getFileName() : path,
                                 file.getLastModificationTime()));
        */
    }

    /**
      | Adds a stream to the list of items which
      | will be added to the archive.
      | 
      | -----------
      | @param streamToRead
      | 
      | this stream isn't read immediately
      | - a pointer to the stream is stored, then
      | used later when the writeToStream()
      | method is called, and deleted by the
      | ZipFileBuilder object when no longer needed,
      | so be very careful about its lifetime
      | and the lifetime of any objects on which
      | it depends! This must not be null.
      | ----------
      | @param compressionLevel
      | 
      | this can be between 0 (no compression),
      | and 9 (maximum compression).
      | ----------
      | @param storedPathName
      | 
      | the partial pathname that will be stored
      | for this file
      | ----------
      | @param fileModificationTime
      | 
      | the timestamp that will be stored as
      | the last modification time of this entry
      |
      */
    pub fn add_entry(
        &mut self, 
        stream_to_read:         *mut dyn Read,
        compression_level:      i32,
        stored_path_name:       &String,
        file_modification_time: Time

    )  {
        
        todo!();
        /*
            jassert (stream != nullptr); // must not be null!
            jassert (path.isNotEmpty());
            items.add (new ZipFileBuilderItem ({}, stream, compression, path, time));
        */
    }

    /**
      | Generates the zip file, writing it to
      | the specified stream.
      | 
      | If the progress parameter is non-null,
      | it will be updated with an approximate
      | progress status between 0 and 1.0
      |
      */
    pub fn write_to_stream(
        &self, 
        target:   &mut dyn Write,
        progress: *mut f64

    ) -> bool {
        
        todo!();
        /*
            auto fileStart = target.getPosition();

            for (int i = 0; i < items.size(); ++i)
            {
                if (progress != nullptr)
                    *progress = (i + 0.5) / items.size();

                if (! items.getUnchecked (i)->writeData (target, fileStart))
                    return false;
            }

            auto directoryStart = target.getPosition();

            for (auto* item : items)
                if (! item->writeDirectoryEntry (target))
                    return false;

            auto directoryEnd = target.getPosition();

            target.writeInt (0x06054b50);
            target.writeShort (0);
            target.writeShort (0);
            target.writeShort ((short) items.size());
            target.writeShort ((short) items.size());
            target.writeInt ((int) (directoryEnd - directoryStart));
            target.writeInt ((int) (directoryStart - fileStart));
            target.writeShort (0);

            if (progress != nullptr)
                *progress = 1.0;

            return true;
        */
    }
}
