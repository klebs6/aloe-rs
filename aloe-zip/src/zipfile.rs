crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/aloe_ZipFile.h]

/**
  | Decodes a ZIP file from a stream.
  | 
  | This can enumerate the items in a ZIP
  | file and can create suitable stream
  | objects to read each one.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ZipFile {
    entries:          Vec<Box<ZipEntryHolder>>,
    lock:             CriticalSection,
    input_stream:     *mut dyn Read, // default = nullptr
    stream_to_delete: Box<dyn Read>,
    input_source:     Box<dyn InputSource>,

    #[cfg(ALOE_DEBUG)]
    stream_counter:   zip_file::OpenStreamCounter,
}

impl Drop for ZipFile {

    fn drop(&mut self) {
        todo!();
        /* 
        entries.clear();
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/aloe_ZipFile.cpp]
impl ZipFile {

    /**
      | Creates a ZipFile for a given stream.
      | 
      | -----------
      | @param inputStream
      | 
      | the stream to read from
      | ----------
      | @param deleteStreamWhenDestroyed
      | 
      | if set to true, the object passed-in
      | will be deleted when this ZipFile object
      | is deleted
      |
      */
    pub fn new_and_set_deleter(
        input_stream:                 *mut dyn Read,
        delete_stream_when_destroyed: bool

    ) -> Self {
    
        todo!();
        /*


        
        */
    }

    /**
      | Creates a ZipFile for a given stream.
      | 
      | The stream will not be owned or deleted
      | by this class - if you want the ZipFile
      | to manage the stream's lifetime, use
      | the other constructor.
      |
      */
    pub fn new_from_input_stream(input_stream: &mut dyn Read) -> Self {
    
        todo!();
        /*


        
        */
    }

    /**
      | Creates a ZipFile for an input source.
      | 
      | The inputSource object will be owned
      | by the zip file, which will delete it
      | later when not needed.
      |
      */
    pub fn new_from_input_source(input_source: *mut dyn InputSource) -> Self {
    
        todo!();
        /*


        
        */
    }

    /**
      | Returns a structure that describes
      | one of the entries in the zip file.
      | 
      | This may return a nullptr if the index
      | is out of range. @see ZipFile::ZipEntry
      |
      */
    pub fn get_entry_with_index(&self, index: i32) -> *const ZipEntry {
        
        todo!();
        /*
            if (auto* zei = entries[index])
            return &(zei->entry);

        return nullptr;
        */
    }

    /**
      | Returns the index of the first entry
      | with a given filename.
      | 
      | This uses a case-sensitive comparison
      | to look for a filename in the list of entries.
      | It might return -1 if no match is found.
      | 
      | @see ZipFile::ZipEntry
      |
      */
    pub fn get_index_of_file_name(
        &self, 
        file_name:   &String,
        ignore_case: Option<bool>

    ) -> i32 {

        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            for (int i = 0; i < entries.size(); ++i)
        {
            auto& entryFilename = entries.getUnchecked (i)->entry.filename;

            if (ignoreCase ? entryFilename.equalsIgnoreCase (fileName)
                           : entryFilename == fileName)
                return i;
        }

        return -1;
        */
    }

    /**
      | Returns a structure that describes
      | one of the entries in the zip file.
      | 
      | This uses a case-sensitive comparison
      | to look for a filename in the list of entries.
      | It might return 0 if no match is found.
      | 
      | @see ZipFile::ZipEntry
      |
      */
    pub fn get_entry(
        &self, 
        file_name:   &String,
        ignore_case: Option<bool>

    ) -> *const ZipEntry {

        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            return getEntry (getIndexOfFileName (fileName, ignoreCase));
        */
    }

    /**
      | Sorts the list of entries, based on the
      | filename.
      |
      */
    pub fn sort_entries_by_filename(&mut self)  {
        
        todo!();
        /*
            std::sort (entries.begin(), entries.end(),
                   [] (const ZipEntryHolder* e1, const ZipEntryHolder* e2) { return e1->entry.filename < e2->entry.filename; });
        */
    }
    
    /**
      | Creates a stream that can read from one
      | of the zip file's entries.
      | 
      | The stream that is returned must be deleted
      | by the caller (and a nullptr might be
      | returned if a stream can't be opened
      | for some reason).
      | 
      | The stream must not be used after the
      | ZipFile object that created has been
      | deleted.
      | 
      | -----------
      | @note
      | 
      | if the ZipFile was created with a user-supplied
      | InputStream object, then all the streams
      | which are created by this method will
      | by trying to share the same source stream,
      | so cannot be safely used on multiple
      | threads! (But if you create the ZipFile
      | from a File or InputSource, then it is
      | safe to do this).
      |
      */
    pub fn create_stream_for_entry_with_index(&mut self, index: i32) -> Box<dyn Read> {
        
        todo!();
        /*
            InputStream* stream = nullptr;

        if (auto* zei = entries[index])
        {
            stream = new ZipInputStream (*this, *zei);

            if (zei->isCompressed)
            {
                stream = new GZIPDecompressorInputStream (stream, true,
                                                          GZIPDecompressorInputStream::deflateFormat,
                                                          zei->entry.uncompressedSize);

                // (much faster to unzip in big blocks using a buffer..)
                stream = new BufferedInputStream (stream, 32768, true);
            }
        }

        return stream;
        */
    }

    /**
      | Creates a stream that can read from one
      | of the zip file's entries.
      | 
      | The stream that is returned must be deleted
      | by the caller (and a nullptr might be
      | returned if a stream can't be opened
      | for some reason).
      | 
      | The stream must not be used after the
      | ZipFile object that created has been
      | deleted.
      | 
      | -----------
      | @note
      | 
      | if the ZipFile was created with a user-supplied
      | InputStream object, then all the streams
      | which are created by this method will
      | by trying to share the same source stream,
      | so cannot be safely used on multiple
      | threads! (But if you create the ZipFile
      | from a File or InputSource, then it is
      | safe to do this).
      |
      */
    pub fn create_stream_for_entry(&mut self, entry: &ZipEntry) -> Box<dyn Read> {
        
        todo!();
        /*
            for (int i = 0; i < entries.size(); ++i)
            if (&entries.getUnchecked (i)->entry == &entry)
                return createStreamForEntry (i);

        return nullptr;
        */
    }

    /**
      | Uncompresses all of the files in the
      | zip file.
      | 
      | This will expand all the entries into
      | a target directory. The relative paths
      | of the entries are used.
      | 
      | -----------
      | @param targetDirectory
      | 
      | the root folder to uncompress to
      | ----------
      | @param shouldOverwriteFiles
      | 
      | whether to overwrite existing files
      | with similarly-named ones
      | 
      | -----------
      | @return
      | 
      | success if the file is successfully
      | unzipped
      |
      */
    pub fn uncompress_to(&mut self, 
        target_directory:       &File,
        should_overwrite_files: Option<bool>) -> Result<(),&'static str> {

        let should_overwrite_files: bool = should_overwrite_files.unwrap_or(true);
        
        todo!();
        /*
            for (int i = 0; i < entries.size(); ++i)
        {
            auto result = uncompressEntry (i, targetDirectory, shouldOverwriteFiles);

            if (result.failed())
                return result;
        }

        return Result::ok();
        */
    }

    /**
      | Uncompresses one of the entries from
      | the zip file.
      | 
      | This will expand the entry and write
      | it in a target directory. The entry's
      | path is used to determine which subfolder
      | of the target should contain the new
      | file.
      | 
      | -----------
      | @param index
      | 
      | the index of the entry to uncompress
      | - this must be a valid index between 0
      | and (getNumEntries() - 1).
      | ----------
      | @param targetDirectory
      | 
      | the root folder to uncompress into
      | ----------
      | @param shouldOverwriteFiles
      | 
      | whether to overwrite existing files
      | with similarly-named ones
      | 
      | -----------
      | @return
      | 
      | success if all the files are successfully
      | unzipped
      |
      */
    pub fn uncompress_entry(&mut self, 
        index:                  i32,
        target_directory:       &File,
        should_overwrite_files: Option<bool>) -> Result<(),&'static str> {

        let should_overwrite_files: bool = should_overwrite_files.unwrap_or(true);
        
        todo!();
        /*
            auto* zei = entries.getUnchecked (index);

       #if ALOE_WINDOWS
        auto entryPath = zei->entry.filename;
       #else
        auto entryPath = zei->entry.filename.replaceCharacter ('\\', '/');
       #endif

        if (entryPath.isEmpty())
            return Result::ok();

        auto targetFile = targetDirectory.getChildFile (entryPath);

        if (entryPath.endsWithChar ('/') || entryPath.endsWithChar ('\\'))
            return targetFile.createDirectory(); // (entry is a directory, not a file)

        std::unique_ptr<InputStream> in (createStreamForEntry (index));

        if (in == nullptr)
            return Result::fail ("Failed to open the zip file for reading");

        if (targetFile.exists())
        {
            if (! shouldOverwriteFiles)
                return Result::ok();

            if (! targetFile.deleteFile())
                return Result::fail ("Failed to write to target file: " + targetFile.getFullPathName());
        }

        if (! targetFile.getParentDirectory().createDirectory())
            return Result::fail ("Failed to create target folder: " + targetFile.getParentDirectory().getFullPathName());

        if (zei->entry.isSymbolicLink)
        {
            String originalFilePath (in->readEntireStreamAsString()
                                        .replaceCharacter (L'/', File::getSeparatorChar()));

            if (! File::createSymbolicLink (targetFile, originalFilePath, true))
                return Result::fail ("Failed to create symbolic link: " + originalFilePath);
        }
        else
        {
            FileOutputStream out (targetFile);

            if (out.failedToOpen())
                return Result::fail ("Failed to write to target file: " + targetFile.getFullPathName());

            out << *in;
        }

        targetFile.setCreationTime (zei->entry.fileTime);
        targetFile.setLastModificationTime (zei->entry.fileTime);
        targetFile.setLastAccessTime (zei->entry.fileTime);

        return Result::ok();
        */
    }

    pub fn new(
        stream:                       *mut dyn Read,
        delete_stream_when_destroyed: bool

    ) -> Self {
    
        todo!();
        /*
        : input_stream(stream),

            if (deleteStreamWhenDestroyed)
            streamToDelete.reset (inputStream);

        init();
        */
    }

    pub fn new_from_steram(stream: &mut dyn Read) -> Self {
    
        todo!();
        /*
        : input_stream(&stream),

            init();
        */
    }
    
    /**
      | Creates a ZipFile to read a specific
      | file.
      |
      */
    pub fn new_from_file(file: &File) -> Self {
    
        todo!();
        /*


            : inputSource (new FileInputSource (file))
        init();
        */
    }
    
    pub fn new_from_source(source: *mut dyn InputSource) -> Self {
    
        todo!();
        /*
        : input_source(source),

            init();
        */
    }

    /**
      | Returns the number of items in the zip
      | file.
      |
      */
    pub fn get_num_entries(&self) -> i32 {
        
        todo!();
        /*
            return entries.size();
        */
    }

    pub fn init(&mut self)  {
        
        todo!();
        /*
            std::unique_ptr<InputStream> toDelete;
        InputStream* in = inputStream;

        if (inputSource != nullptr)
        {
            in = inputSource->createInputStream();
            toDelete.reset (in);
        }

        if (in != nullptr)
        {
            int numEntries = 0;
            auto centralDirectoryPos = findCentralDirectoryFileHeader (*in, numEntries);

            if (centralDirectoryPos >= 0 && centralDirectoryPos < in->getTotalLength())
            {
                auto size = (size_t) (in->getTotalLength() - centralDirectoryPos);

                in->setPosition (centralDirectoryPos);
                MemoryBlock headerData;

                if (in->readIntoMemoryBlock (headerData, (ssize_t) size) == size)
                {
                    size_t pos = 0;

                    for (int i = 0; i < numEntries; ++i)
                    {
                        if (pos + 46 > size)
                            break;

                        auto* buffer = static_cast<const char*> (headerData.getData()) + pos;
                        auto fileNameLen = readUnalignedLittleEndianShort (buffer + 28u);

                        if (pos + 46 + fileNameLen > size)
                            break;

                        entries.add (new ZipEntryHolder (buffer, fileNameLen));

                        pos += 46u + fileNameLen
                                + readUnalignedLittleEndianShort (buffer + 30u)
                                + readUnalignedLittleEndianShort (buffer + 32u);
                    }
                }
            }
        }
        */
    }
}

