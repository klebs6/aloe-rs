crate::ix!();

pub fn create_temp_file(
        parent_directory: &File,
        name:             String,
        suffix:           &String,
        option_flags:     i32) -> File {
    
    todo!();
    /*
        if ((optionFlags & TemporaryFile::useHiddenFile) != 0)
            name = "." + name;

        return parentDirectory.getNonexistentChildFile (name, suffix, (optionFlags & TemporaryFile::putNumbersInBrackets) != 0);
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_TemporaryFile.h]

/**
  | Manages a temporary file, which will
  | be deleted when this object is deleted.
  | 
  | This object is intended to be used as
  | a stack based object, using its scope
  | to make sure the temporary file isn't
  | left lying around.
  | 
  | For example:
  | 
  | 
  | -----------
  | @code
  | 
  | {
  |     File myTargetFile ("~/myfile.txt");
  | 
  |     // this will choose a file called something like "~/myfile_temp239348.txt"
  |     // which definitely doesn't exist at the time the constructor is called.
  |     TemporaryFile temp (myTargetFile);
  | 
  |     // create a stream to the temporary file, and write some data to it...
  |     if (auto out = std::unique_ptr<FileOutputStream> (temp.getFile().createOutputStream()))
  |     {
  |         out->write ( ...etc )
  |         out.reset(); // (deletes the stream)
  | 
  |         // ..now we've finished writing, this will rename the temp file to
  |         // make it replace the target file we specified above.
  |         bool succeeded = temp.overwriteTargetFileWithTemporary();
  |     }
  | 
  |     // ..and even if something went wrong and our overwrite failed,
  |     // as the TemporaryFile object goes out of scope here, it'll make sure
  |     // that the temp file gets deleted.
  | }
  | 
  | @see File, FileOutputStream
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TemporaryFile {
    temporary_file: File,
    target_file:    File,
}

pub mod temporary_file {
    use super::*;

    pub enum OptionFlags {
        /**
          | Indicates that the temporary file should
          | be hidden - i.e. its name should start
          | with a dot.
          |
          */
        useHiddenFile = 1,          

        /**
          | Indicates that when numbers are appended
          | to make sure the file is unique, they
          | should go in brackets rather than just
          | being appended (see File::getNonexistentSibling())
          |
          */
        putNumbersInBrackets = 2    
  }
}

impl Drop for TemporaryFile {

    /**
      | Destructor.
      | 
      | When this object is deleted it will make
      | sure that its temporary file is also
      | deleted! If the operation fails, it'll
      | throw an assertion in debug mode.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        if (! deleteTemporaryFile())
        {
            /* Failed to delete our temporary file! The most likely reason for this would be
               that you've not closed an output stream that was being used to write to file.

               If you find that something beyond your control is changing permissions on
               your temporary files and preventing them from being deleted, you may want to
               call TemporaryFile::deleteTemporaryFile() to detect those error cases and
               handle them appropriately.
            */
            jassertfalse;
        }
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_TemporaryFile.cpp]
impl TemporaryFile {
    
    /**
      | Returns the temporary file.
      |
      */
    pub fn get_file(&self) -> &File {
        
        todo!();
        /*
            return temporaryFile;
        */
    }

    /**
      | Returns the target file that was specified
      | in the constructor.
      |
      */
    pub fn get_target_file(&self) -> &File {
        
        todo!();
        /*
            return targetFile;
        */
    }

    /**
      | Creates a randomly-named temporary
      | file in the default temp directory.
      | 
      | -----------
      | @param suffix
      | 
      | a file suffix to use for the file
      | ----------
      | @param optionFlags
      | 
      | a combination of the values listed in
      | the OptionFlags enum
      | 
      | The file will not be created until you
      | write to it. And remember that when this
      | object is deleted, the file will also
      | be deleted!
      |
      */
    pub fn new_from_suffix_and_option_flags(
        suffix:       Option<&str>,
        option_flags: Option<i32>) -> Self {
    
        let suffix = suffix.unwrap_or("");

        let option_flags: i32 = option_flags.unwrap_or(0);

        todo!();
        /*


            : temporaryFile (createTempFile (File::getSpecialLocation (File::tempDirectory),
                                         "temp_" + String::toHexString (Random::getSystemRandom().nextInt()),
                                         suffix, optionFlags)),
          targetFile()
        */
    }
    
    /**
      | Creates a temporary file in the same
      | directory as a specified file.
      | 
      | This is useful if you have a file that
      | you want to overwrite, but don't want
      | to harm the original file if the write
      | operation fails. You can use this to
      | create a temporary file next to the target
      | file, then write to the temporary file,
      | and finally use overwriteTargetFileWithTemporary()
      | to replace the target file with the one
      | you've just written.
      | 
      | This class won't create any files until
      | you actually write to them. And remember
      | that when this object is deleted, the
      | temporary file will also be deleted!
      | 
      | -----------
      | @param targetFile
      | 
      | the file that you intend to overwrite
      | - the temporary file will be created
      | in the same directory as this @param
      | optionFlags a combination of the values
      | listed in the OptionFlags enum
      |
      */
    pub fn new_from_target_and_option_flags(
        target:       &File,
        option_flags: Option<i32>) -> Self {

        let option_flags: i32 = option_flags.unwrap_or(0);
    
        todo!();
        /*


            : temporaryFile (createTempFile (target.getParentDirectory(),
                                         target.getFileNameWithoutExtension()
                                           + "_temp" + String::toHexString (Random::getSystemRandom().nextInt()),
                                         target.getFileExtension(), optionFlags)),
          targetFile (target)

        // If you use this constructor, you need to give it a valid target file!
        jassert (targetFile != File());
        */
    }
    
    /**
      | Creates a temporary file using an explicit
      | filename.
      | 
      | The other constructors are a better
      | choice than this one, unless for some
      | reason you need to explicitly specify
      | the temporary file you want to use.
      | 
      | -----------
      | @param targetFile
      | 
      | the file that you intend to overwrite
      | ----------
      | @param temporaryFile
      | 
      | the temporary file to be used
      |
      */
    pub fn new_from_target_and_temporary(
        target:    &File,
        temporary: &File) -> Self {
    
        todo!();
        /*


            : temporaryFile (temporary), targetFile (target)
        */
    }
    
    /**
      | Tries to move the temporary file to overwrite
      | the target file that was specified in
      | the constructor.
      | 
      | If you used the constructor that specified
      | a target file, this will attempt to replace
      | that file with the temporary one.
      | 
      | Before calling this, make sure:
      | 
      | - that you've actually written to the
      | temporary file
      | 
      | - that you've closed any open streams
      | that you were using to write to it
      | 
      | - and that you don't have any streams
      | open to the target file, which would
      | prevent it being overwritten
      | 
      | If the file move succeeds, this returns
      | true, and the temporary file will have
      | disappeared. If it fails, the temporary
      | file will probably still exist, but
      | will be deleted when this object is destroyed.
      |
      */
    pub fn overwrite_target_file_with_temporary(&self) -> bool {
        
        todo!();
        /*
            // This method only works if you created this object with the constructor
        // that takes a target file!
        jassert (targetFile != File());

        if (temporaryFile.exists())
        {
            // Have a few attempts at overwriting the file before giving up..
            for (int i = 5; --i >= 0;)
            {
                if (temporaryFile.replaceFileIn (targetFile))
                    return true;

                Thread::sleep (100);
            }
        }
        else
        {
            // There's no temporary file to use. If your write failed, you should
            // probably check, and not bother calling this method.
            jassertfalse;
        }

        return false;
        */
    }
    
    /**
      | Attempts to delete the temporary file,
      | if it exists.
      | 
      | -----------
      | @return
      | 
      | true if the file is successfully deleted
      | (or if it didn't exist).
      |
      */
    pub fn delete_temporary_file(&self) -> bool {
        
        todo!();
        /*
            // Have a few attempts at deleting the file before giving up..
        for (int i = 5; --i >= 0;)
        {
            if (temporaryFile.deleteFile())
                return true;

            Thread::sleep (50);
        }

        return false;
        */
    }
}
