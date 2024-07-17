crate::ix!();

pub fn remove_ellipsis(path: &String) -> String {
    
    todo!();
    /*
        // This will quickly find both /../ and /./ at the expense of a minor
        // false-positive performance hit when path elements end in a dot.
       #if ALOE_WINDOWS
        if (path.contains (".\\"))
       #else
        if (path.contains ("./"))
       #endif
        {
            StringArray toks;
            toks.addTokens (path, File::getSeparatorString(), {});
            bool anythingChanged = false;

            for (int i = 1; i < toks.size(); ++i)
            {
                auto& t = toks[i];

                if (t == ".." && toks[i - 1] != "..")
                {
                    anythingChanged = true;
                    toks.removeRange (i - 1, 2);
                    i = jmax (0, i - 2);
                }
                else if (t == ".")
                {
                    anythingChanged = true;
                    toks.remove (i--);
                }
            }

            if (anythingChanged)
                return toks.joinIntoString (File::getSeparatorString());
        }

        return path;
    */
}

pub fn normalise_separators(path: &String) -> String {
    
    todo!();
    /*
        auto normalisedPath = path;

        String separator (File::getSeparatorString());
        String doubleSeparator (separator + separator);

        auto uncPath = normalisedPath.startsWith (doubleSeparator)
                      && ! normalisedPath.fromFirstOccurrenceOf (doubleSeparator, false, false).startsWith (separator);

        if (uncPath)
            normalisedPath = normalisedPath.fromFirstOccurrenceOf (doubleSeparator, false, false);

        while (normalisedPath.contains (doubleSeparator))
             normalisedPath = normalisedPath.replace (doubleSeparator, separator);

        return uncPath ? doubleSeparator + normalisedPath
                       : normalisedPath;
    */
}

pub fn compare_filenames(
        name1: &String,
        name2: &String) -> i32 {
    
    todo!();
    /*
        #if NAMES_ARE_CASE_SENSITIVE
        return name1.compare (name2);
       #else
        return name1.compareIgnoreCase (name2);
       #endif
    */
}

pub fn count_number_of_separators(s: CharPointerType) -> i32 {
    
    todo!();
    /*
        int num = 0;

        for (;;)
        {
            auto c = s.getAndAdvance();

            if (c == 0)
                break;

            if (c == File::getSeparatorChar())
                ++num;
        }

        return num;
    */
}

#[cfg(any(target_os="linux",target_os="bsd"))]
pub const NAMES_ARE_CASE_SENSITIVE: usize = 1;

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_File.h]

#[cfg(any(target_os="macos",target_os="ios"))] 
#[cfg(__LP64__)]      
pub type OSType = u32;

#[cfg(any(target_os="macos",target_os="ios"))] 
#[cfg(not(__LP64__))] 
pub type OSType = u64;

/**
  | Represents a local file or directory.
  | 
  | This class encapsulates the absolute
  | pathname of a file or directory, and
  | has methods for finding out about the
  | file and changing its properties.
  | 
  | To read or write to the file, there are
  | methods for returning an input or output
  | stream.
  | 
  | @see FileInputStream, FileOutputStream
  | 
  | @tags{Core}
  |
  */
pub struct File {
    full_path: String,
}

impl Default for File {
    
    /**
      | Creates an (invalid) file object.
      | 
      | The file is initially set to an empty
      | path, so getFullPathName() will return
      | an empty string.
      | 
      | You can use its operator= method to point
      | it at a proper file.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl PartialEq<File> for File {
    
    /**
      | Compares the pathnames for two files.
      |
      */
    #[inline] fn eq(&self, other: &File) -> bool {
        todo!();
        /*
            return compareFilenames (fullPath, other.fullPath) == 0;
        */
    }
}

impl Eq for File {}

impl Ord for File {
    
    /**
      | Compares the pathnames for two files.
      |
      */
    #[inline] fn cmp(&self, other: &File) -> std::cmp::Ordering {
        todo!();
        /*
            return compareFilenames (fullPath, other.fullPath) <  0;
        */
    }
}

impl PartialOrd<File> for File {
    #[inline] fn partial_cmp(&self, other: &File) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

///--------------------------------
pub mod file {

    use super::*;

    /**
      | Comparator for files
      |
      */
    pub struct NaturalFileComparator {
        folders_first: bool,
    }

    impl NaturalFileComparator {

        pub fn new(should_put_folders_first: bool) -> Self {
        
            todo!();
            /*
            : folders_first(shouldPutFoldersFirst),
            */
        }
        
        pub fn compare_elements(&self, 
            first_file:  &File,
            second_file: &File) -> i32 {
            
            todo!();
            /*
                if (foldersFirst && (firstFile.isDirectory() != secondFile.isDirectory()))
                        return firstFile.isDirectory() ? -1 : 1;

                   #if NAMES_ARE_CASE_SENSITIVE
                    return firstFile.getFullPathName().compareNatural (secondFile.getFullPathName(), true);
                   #else
                    return firstFile.getFullPathName().compareNatural (secondFile.getFullPathName(), false);
                   #endif
            */
        }
    }

    /**
      | Used in file searching, to specify whether
      | to return files, directories, or both.
      |
      */
    pub enum TypesOfFileToFind
    {
        /**
          Use this flag to indicate that you want to
          find directories.
          */
        findDirectories             = 1,    

        /**
          Use this flag to indicate that you want to
          find files.
          */
        findFiles                   = 2,    

        /**
          Use this flag to indicate that you want to
          find both files and directories.
          */
        findFilesAndDirectories     = 3,    

        /**
          Add this flag to avoid returning any hidden
          files in the results.
          */
        ignoreHiddenFiles           = 4,     
    }

    /**
      | A set of types of location that can be
      | passed to the getSpecialLocation()
      | method.
      |
      */
    pub enum SpecialLocationType
    {
        /**
          | The user's home folder. This is the same
          | as using File ("~").
          |
          */
        userHomeDirectory,

        /**
          | The user's default documents folder.
          | On Windows, this might be the user's
          | "My Documents" folder. On the Mac it'll
          | be their "Documents" folder. Linux
          | doesn't tend to have one of these, so
          | it might just return their home folder.
          |
          */
        userDocumentsDirectory,

        /**
          | The folder that contains the user's
          | desktop objects.
          |
          */
        userDesktopDirectory,

        /**
          | The most likely place where a user might
          | store their music files.
          |
          */
        userMusicDirectory,

        /**
          | The most likely place where a user might
          | store their movie files.
          |
          */
        userMoviesDirectory,

        /**
          | The most likely place where a user might
          | store their picture files.
          |
          */
        userPicturesDirectory,

        /**
          | The folder in which applications store
          | their persistent user-specific settings.
          | On Windows, this might be "\Documents
          | and Settings\username\Application
          | Data".
          | 
          | On the Mac, it might be "~/Library".
          | If you're going to store your settings
          | in here, always create your own sub-folder
          | to put them in, to avoid making a mess.
          | 
          | On GNU/Linux it is "~/.config".
          |
          */
        userApplicationDataDirectory,

        /**
          | An equivalent of the userApplicationDataDirectory
          | folder that is shared by all users of
          | the computer, rather than just the current
          | user.
          | 
          | On the Mac it'll be "/Library", on Windows,
          | it could be something like "\Documents
          | and Settings\All Users\Application
          | Data".
          | 
          | On GNU/Linux it is "/opt".
          | 
          | Depending on the setup, this folder
          | may be read-only.
          |
          */
        commonApplicationDataDirectory,

        /**
          | A place to put documents which are shared
          | by all users of the machine.
          | 
          | On Windows this may be somewhere like
          | "C:\Users\Public\Documents", on
          | OSX it will be something like "/Users/Shared".
          | Other OSes may have no such concept though,
          | so be careful.
          |
          */
        commonDocumentsDirectory,

        /**
          | The folder that should be used for temporary
          | files.
          | 
          | Always delete them when you're finished,
          | to keep the user's computer tidy!
          |
          */
        tempDirectory,

        /**
          | Returns this application's executable
          | file.
          | 
          | If running as a plug-in or DLL, this will
          | (where possible) be the DLL rather than
          | the host app.
          | 
          | On the mac this will return the unix binary,
          | not the package folder - see currentApplicationFile
          | for that.
          | 
          | See also invokedExecutableFile, which
          | is similar, but if the exe was launched
          | from a file link, invokedExecutableFile
          | will return the name of the link.
          |
          */
        currentExecutableFile,

        /**
          | Returns this application's location.
          | 
          | If running as a plug-in or DLL, this will
          | (where possible) be the DLL rather than
          | the host app.
          | 
          | On the mac this will return the package
          | folder (if it's in one), not the unix
          | binary that's inside it - compare with
          | currentExecutableFile.
          |
          */
        currentApplicationFile,

        /**
          | Returns the file that was invoked to
          | launch this executable.
          | 
          | This may differ from currentExecutableFile
          | if the app was started from e.g. a link
          | - this will return the name of the link
          | that was used, whereas currentExecutableFile
          | will return the actual location of the
          | target executable.
          |
          */
        invokedExecutableFile,

        /**
          | In a plugin, this will return the path
          | of the host executable.
          |
          */
        hostApplicationPath,

        /**
          | On a Windows machine, returns the location
          | of the Windows/System32 folder.
          |
          */
        #[cfg(target_os="windows")]
        windowsSystemDirectory,

        /**
          | The directory in which applications
          | normally get installed.
          | 
          | So on windows, this would be something
          | like "C:\Program Files", on the Mac
          | "/Applications", or "/usr" on linux.
          |
          */
        globalApplicationsDirectory,

        /**
          | On a Windows machine, returns the directory
          | in which 32 bit applications normally
          | get installed.
          | 
          | On a 64 bit machine this would be something
          | like "C:\Program Files (x86)", whereas
          | for 32 bit machines this would match
          | globalApplicationsDirectory and
          | be something like "C:\Program Files".
          | 
          | @see globalApplicationsDirectory
          |
          */
        #[cfg(target_os="windows")]
        globalApplicationsDirectoryX86
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_File.cpp]
impl File {

    /**
      | Creates a file from an absolute path.
      | 
      | If the path supplied is a relative path,
      | it is taken to be relative to the current
      | working directory (see File::getCurrentWorkingDirectory()),
      | but this isn't a recommended way of creating
      | a file, because you never know what the
      | CWD is going to be.
      | 
      | On the Mac/Linux, the path can include
      | "~" notation for referring to user home
      | directories.
      |
      */
    pub fn new_from_abs_path(absolute_path: &String) -> Self {
    
        todo!();
        /*

        
        */
    }

    /**
      | Checks whether the file actually exists.
      | 
      | -----------
      | @return
      | 
      | true if the file exists, either as a file
      | or a directory. @see existsAsFile,
      | isDirectory
      |
      */
    pub fn exists(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Checks whether the file exists and is
      | a file rather than a directory.
      | 
      | -----------
      | @return
      | 
      | true only if this is a real file, false
      | if it's a directory or doesn't exist
      | @see exists, isDirectory
      |
      */
    pub fn exists_as_file(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Checks whether the file is a directory
      | that exists.
      | 
      | -----------
      | @return
      | 
      | true only if the file is a directory which
      | actually exists, so false if it's a file
      | or doesn't exist at all @see exists,
      | existsAsFile
      |
      */
    pub fn is_directory(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the size of the file in bytes.
      | 
      | -----------
      | @return
      | 
      | the number of bytes in the file, or 0 if
      | it doesn't exist.
      |
      */
    pub fn get_size(&self) -> i64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the complete, absolute path
      | of this file.
      | 
      | This includes the filename and all its
      | parent folders. On Windows it'll also
      | include the drive letter prefix; on
      | Mac or Linux it'll be a complete path
      | starting from the root folder.
      | 
      | If you just want the file's name, you
      | should use getFileName() or getFileNameWithoutExtension().
      | 
      | @see getFileName, getRelativePathFrom
      |
      */
    pub fn get_full_path_name(&self) -> &String {
        
        todo!();
        /*
            return fullPath;
        */
    }

    /**
      | Checks whether a file can be created
      | or written to.
      | 
      | -----------
      | @return
      | 
      | true if it's possible to create and write
      | to this file. If the file doesn't already
      | exist, this will check its parent directory
      | to see if writing is allowed. @see setReadOnly
      |
      */
    pub fn has_write_access(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this file is a hidden
      | or system file.
      | 
      | The criteria for deciding whether a
      | file is hidden are platform-dependent.
      |
      */
    pub fn is_hidden(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns a unique identifier for the
      | file, if one is available.
      | 
      | Depending on the OS and file-system,
      | this may be a unix inode number or a win32
      | file identifier, or 0 if it fails to find
      | one. The number will be unique on the
      | filesystem, but not globally.
      |
      */
    pub fn get_file_identifier(&self) -> u64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | If possible, this will try to create
      | a version string for the given file.
      | 
      | The OS may be able to look at the file and
      | give a version for it - e.g. with executables,
      | bundles, dlls, etc. If no version is
      | available, this will return an empty
      | string.
      |
      */
    pub fn get_version(&self) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Deletes a file.
      | 
      | If this file is actually a directory,
      | it may not be deleted correctly if it
      | contains files. See deleteRecursively()
      | as a better way of deleting directories.
      | 
      | If this file is a symlink, then the symlink
      | will be deleted and not the target of
      | the symlink.
      | 
      | -----------
      | @return
      | 
      | true if the file has been successfully
      | deleted (or if it didn't exist to begin
      | with). @see deleteRecursively
      |
      */
    pub fn delete_file(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Moves this file or folder to the trash.
      | 
      | -----------
      | @return
      | 
      | true if the operation succeeded. It
      | could fail if the trash is full, or if
      | the file is write-protected, so you
      | should check the return value and act
      | appropriately.
      |
      */
    pub fn move_to_trash(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Creates a set of files to represent each
      | file root.
      | 
      | e.g. on Windows this will create files
      | for "c:\", "d:\" etc according to which
      | ones are available. On the Mac/Linux,
      | this will probably just add a single
      | entry for "/".
      |
      */
    pub fn find_file_system_roots(results: &mut Vec<File>)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Finds the name of the drive on which this
      | file lives.
      | 
      | -----------
      | @return
      | 
      | the volume label of the drive, or an empty
      | string if this isn't possible
      |
      */
    pub fn get_volume_label(&self) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the serial number of the volume
      | on which this file lives.
      | 
      | -----------
      | @return
      | 
      | the serial number, or zero if there's
      | a problem doing this
      |
      */
    pub fn get_volume_serial_number(&self) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the number of bytes free on the
      | drive that this file lives on.
      | 
      | -----------
      | @return
      | 
      | the number of bytes free, or 0 if there's
      | a problem finding this out @see getVolumeTotalSize
      |
      */
    pub fn get_bytes_free_on_volume(&self) -> i64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the total size of the drive that
      | contains this file.
      | 
      | -----------
      | @return
      | 
      | the total number of bytes that the volume
      | can hold @see getBytesFreeOnVolume
      |
      */
    pub fn get_volume_total_size(&self) -> i64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this file is on a CD or
      | DVD drive.
      |
      */
    pub fn is_on_cd_rom_drive(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this file is on a hard
      | disk.
      | 
      | This will fail if it's a network drive,
      | but will still be true for removable
      | hard-disks.
      |
      */
    pub fn is_on_hard_disk(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this file is on a removable
      | disk drive.
      | 
      | This might be a usb-drive, a CD-rom,
      | or maybe a network drive.
      |
      */
    pub fn is_on_removable_drive(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Opens Finder, Explorer, or whatever
      | the OS uses, to show the user this file's
      | location. @see startAsProcess
      |
      */
    pub fn reveal_to_user(&self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Finds the location of a special type
      | of file or directory, such as a home folder
      | or documents folder.
      | 
      | @see SpecialLocationType
      |
      */
    pub fn get_special_location(ty: file::SpecialLocationType) -> File {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the current working directory.
      | @see setAsCurrentWorkingDirectory
      |
      */
    pub fn get_current_working_directory() -> File {
        
        todo!();
        /*
        
        */
    }

    /**
      | Sets the current working directory
      | to be this file.
      | 
      | For this to work the file must point to
      | a valid directory.
      | 
      | -----------
      | @return
      | 
      | true if the current directory has been
      | changed. @see getCurrentWorkingDirectory
      |
      */
    pub fn set_as_current_working_directory(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | The system-specific file separator
      | character.
      | 
      | On Windows, this will be '\', on Mac/Linux,
      | it'll be '/'
      |
      */
    pub fn get_separator_char() -> wchar_t {
        
        todo!();
        /*
        
        */
    }

    /**
      | The system-specific file separator
      | character, as a string.
      | 
      | On Windows, this will be '\', on Mac/Linux,
      | it'll be '/'
      |
      */
    pub fn get_separator_string() -> &'static str {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns true if this file is a link or
      | alias that can be followed using getLinkedTarget().
      |
      */
    pub fn is_symbolic_link(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | This returns the native path that the
      | symbolic link points to.
      | 
      | The returned path is a native path of
      | the current OS and can be a relative,
      | absolute or special path.
      |
      */
    pub fn get_native_linked_target(&self) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Windows ONLY - Creates a win32 .LNK shortcut
      | file that links to this file.
      |
      */
    #[cfg(target_os="windows")]
    pub fn create_shortcut(&self, 
        description:         &String,
        link_file_to_create: &File) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Windows ONLY - Returns true if this is
      | a win32 .LNK file.
      |
      */
    #[cfg(target_os="windows")]
    pub fn is_shortcut(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | OSX ONLY - Finds the OSType of a file from
      | the its resources.
      |
      */
    #[cfg(any(target_os="macos",target_os="ios"))]
    pub fn get_mac_os_type(&self) -> OSType {
        
        todo!();
        /*
        
        */
    }

    /**
      | OSX ONLY - Returns true if this file is
      | actually a bundle.
      |
      */
    #[cfg(any(target_os="macos",target_os="ios"))]
    pub fn is_bundle(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | OSX ONLY - Adds this file to the OSX dock
      |
      */
    #[cfg(target_os="macos")]
    pub fn add_to_dock(&self)  {
        
        todo!();
        /*
        
        */
    }

    pub fn create_directory_internal(&self, _0: &String) -> Result<(),()> {
        
        todo!();
        /*
        
        */
    }
    
    pub fn copy_internal(&self, _0: &File) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn move_internal(&self, _0: &File) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn replace_internal(&self, _0: &File) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_file_times_internal(&self, 
        m: i64,
        a: i64,
        c: i64) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_file_times_internal(&self, 
        m: &mut i64,
        a: &mut i64,
        c: &mut i64)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_file_read_only_internal(&self, _0: bool) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_file_executable_internal(&self, _0: bool) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new_from_full_path(full_path_name: &String) -> Self {
    
        todo!();
        /*
        : full_path(parseAbsolutePath (fullPathName)),

        
        */
    }
    
    /**
      | Creates a file that simply contains
      | this string, without doing the sanity-checking
      | that the normal constructors do.
      | 
      | Best to avoid this unless you really
      | know what you're doing.
      |
      */
    pub fn create_file_without_checking_path(&mut self, path: &String) -> File {
        
        todo!();
        /*
            File f;
        f.fullPath = path;
        return f;
        */
    }
    
    /**
      | Creates a copy of another file object.
      |
      */
    pub fn new_from_other(other: &File) -> Self {
    
        todo!();
        /*
        : full_path(other.fullPath),

        
        */
    }
    
    /**
      | Sets the file based on an absolute pathname.
      | 
      | If the path supplied is a relative path,
      | it is taken to be relative to the current
      | working directory (see File::getCurrentWorkingDirectory()),
      | but this isn't a recommended way of creating
      | a file, because you never know what the
      | CWD is going to be.
      | 
      | On the Mac/Linux, the path can include
      | "~" notation for referring to user home
      | directories.
      |
      */
    pub fn assign_from_new_path(&mut self, new_path: &String) -> &mut File {
        
        todo!();
        /*
            fullPath = parseAbsolutePath (newPath);
        return *this;
        */
    }
    
    /**
      | Copies from another file object.
      |
      */
    pub fn assign_from_other_ref(&mut self, other: &File) -> &mut File {
        
        todo!();
        /*
            fullPath = other.fullPath;
        return *this;
        */
    }
    
    pub fn new_from_other_file(other: File) -> Self {
    
        todo!();
        /*
        : full_path(std::move (other.fullPath)),

        
        */
    }
    
    pub fn assign_from_other(&mut self, other: File) -> &mut File {
        
        todo!();
        /*
            fullPath = std::move (other.fullPath);
        return *this;
        */
    }
    
    /**
      | Checks whether the path of this file
      | represents the root of a file system,
      | irrespective of its existence.
      | 
      | This will return true for "C:", "D:",
      | etc on Windows and "/" on other platforms.
      |
      */
    pub fn is_root(&self) -> bool {
        
        todo!();
        /*
            return fullPath.isNotEmpty() && *this == getParentDirectory();
        */
    }
    
    pub fn parse_absolute_path(&mut self, p: &String) -> String {
        
        todo!();
        /*
            if (p.isEmpty())
            return {};

       #if ALOE_WINDOWS
        // Windows..
        auto path = normaliseSeparators (removeEllipsis (p.replaceCharacter ('/', '\\')));

        if (path.startsWithChar (getSeparatorChar()))
        {
            if (path[1] != getSeparatorChar())
            {
                /*  When you supply a raw string to the File object constructor, it must be an absolute path.
                    If you're trying to parse a string that may be either a relative path or an absolute path,
                    you MUST provide a context against which the partial path can be evaluated - you can do
                    this by simply using File::getChildFile() instead of the File constructor. E.g. saying
                    "File::getCurrentWorkingDirectory().getChildFile (myUnknownPath)" would return an absolute
                    path if that's what was supplied, or would evaluate a partial path relative to the CWD.
                */
                jassertfalse;

                path = File::getCurrentWorkingDirectory().getFullPathName().substring (0, 2) + path;
            }
        }
        else if (! path.containsChar (':'))
        {
            /*  When you supply a raw string to the File object constructor, it must be an absolute path.
                If you're trying to parse a string that may be either a relative path or an absolute path,
                you MUST provide a context against which the partial path can be evaluated - you can do
                this by simply using File::getChildFile() instead of the File constructor. E.g. saying
                "File::getCurrentWorkingDirectory().getChildFile (myUnknownPath)" would return an absolute
                path if that's what was supplied, or would evaluate a partial path relative to the CWD.
            */
            jassertfalse;

            return File::getCurrentWorkingDirectory().getChildFile (path).getFullPathName();
        }
       #else
        // Mac or Linux..

        // Yes, I know it's legal for a unix pathname to contain a backslash, but this assertion is here
        // to catch anyone who's trying to run code that was written on Windows with hard-coded path names.
        // If that's why you've ended up here, use File::getChildFile() to build your paths instead.
        jassert ((! p.containsChar ('\\')) || (p.indexOfChar ('/') >= 0 && p.indexOfChar ('/') < p.indexOfChar ('\\')));

        auto path = normaliseSeparators (removeEllipsis (p));

        if (path.startsWithChar ('~'))
        {
            if (path[1] == getSeparatorChar() || path[1] == 0)
            {
                // expand a name of the form "~/abc"
                path = File::getSpecialLocation (File::userHomeDirectory).getFullPathName()
                        + path.substring (1);
            }
            else
            {
                // expand a name of type "~dave/abc"
                auto userName = path.substring (1).upToFirstOccurrenceOf ("/", false, false);

                if (auto* pw = getpwnam (userName.toUTF8()))
                    path = addTrailingSeparator (pw->pw_dir) + path.fromFirstOccurrenceOf ("/", false, false);
            }
        }
        else if (! path.startsWithChar (getSeparatorChar()))
        {
           #if ALOE_DEBUG || ALOE_LOG_ASSERTIONS
            if (! (path.startsWith ("./") || path.startsWith ("../")))
            {
                /*  When you supply a raw string to the File object constructor, it must be an absolute path.
                    If you're trying to parse a string that may be either a relative path or an absolute path,
                    you MUST provide a context against which the partial path can be evaluated - you can do
                    this by simply using File::getChildFile() instead of the File constructor. E.g. saying
                    "File::getCurrentWorkingDirectory().getChildFile (myUnknownPath)" would return an absolute
                    path if that's what was supplied, or would evaluate a partial path relative to the CWD.
                */
                jassertfalse;

               #if ALOE_LOG_ASSERTIONS
                Logger::writeToLog ("Illegal absolute path: " + path);
               #endif
            }
           #endif

            return File::getCurrentWorkingDirectory().getChildFile (path).getFullPathName();
        }
       #endif

        while (path.endsWithChar (getSeparatorChar()) && path != getSeparatorString()) // careful not to turn a single "/" into an empty string.
            path = path.dropLastCharacters (1);

        return path;
        */
    }
    
    /**
      | Adds a separator character to the end
      | of a path if it doesn't already have one.
      |
      */
    pub fn add_trailing_separator(&mut self, path: &String) -> String {
        
        todo!();
        /*
            return path.endsWithChar (getSeparatorChar()) ? path
                                                      : path + getSeparatorChar();
        */
    }
    
    /**
      | Indicates whether filenames are case-sensitive
      | on the current operating system.
      |
      */
    pub fn are_file_names_case_sensitive(&mut self) -> bool {
        
        todo!();
        /*
            #if NAMES_ARE_CASE_SENSITIVE
        return true;
       #else
        return false;
       #endif
        */
    }
    
    /**
      | Changes the write-permission of a file
      | or directory.
      | 
      | -----------
      | @param shouldBeReadOnly
      | 
      | whether to add or remove write-permission
      | ----------
      | @param applyRecursively
      | 
      | if the file is a directory and this is
      | true, it will recurse through all the
      | subfolders changing the permissions
      | of all files
      | 
      | -----------
      | @return
      | 
      | true if it manages to change the file's
      | permissions. @see hasWriteAccess
      |
      */
    pub fn set_read_only(
        &self, 
        should_be_read_only: bool,
        apply_recursively:   Option<bool>) -> bool {

        let apply_recursively: bool = apply_recursively.unwrap_or(false);
        
        todo!();
        /*
            bool worked = true;

        if (applyRecursively && isDirectory())
            for (auto& f : findChildFiles (File::findFilesAndDirectories, false))
                worked = f.setReadOnly (shouldBeReadOnly, true) && worked;

        return setFileReadOnlyInternal (shouldBeReadOnly) && worked;
        */
    }
    
    /**
      | Changes the execute-permissions of
      | a file.
      | 
      | -----------
      | @param shouldBeExecutable
      | 
      | whether to add or remove execute-permission
      | 
      | -----------
      | @return
      | 
      | true if it manages to change the file's
      | permissions.
      |
      */
    pub fn set_execute_permission(&self, should_be_executable: bool) -> bool {
        
        todo!();
        /*
            return setFileExecutableInternal (shouldBeExecutable);
        */
    }
    
    /**
      | Deletes a file or directory and all its
      | subdirectories.
      | 
      | If this file is a directory, this will
      | try to delete it and all its subfolders.
      | If it's just a file, it will just try to
      | delete the file.
      | 
      | -----------
      | @param followSymlinks
      | 
      | If true, then any symlink pointing to
      | a directory will also recursively delete
      | the contents of that directory
      | 
      | -----------
      | @return
      | 
      | true if the file and all its subfolders
      | have been successfully deleted (or
      | if it didn't exist to begin with). @see
      | deleteFile
      |
      */
    pub fn delete_recursively(&self, follow_symlinks: Option<bool>) -> bool {

        let follow_symlinks: bool = follow_symlinks.unwrap_or(false);
        
        todo!();
        /*
            bool worked = true;

        if (isDirectory() && (followSymlinks || ! isSymbolicLink()))
            for (auto& f : findChildFiles (File::findFilesAndDirectories, false))
                worked = f.deleteRecursively (followSymlinks) && worked;

        return deleteFile() && worked;
        */
    }
    
    /**
      | Moves or renames a file.
      | 
      | Tries to move a file to a different location.
      | If the target file already exists, this
      | will attempt to delete it first, and
      | will fail if this can't be done.
      | 
      | Note that the destination file isn't
      | the directory to put it in, it's the actual
      | filename that you want the new file to
      | have.
      | 
      | Also note that on some OSes (e.g. Windows),
      | moving files between different volumes
      | may not be possible.
      | 
      | -----------
      | @return
      | 
      | true if the operation succeeds
      |
      */
    pub fn move_file_to(&self, new_file: &File) -> bool {
        
        todo!();
        /*
            if (newFile.fullPath == fullPath)
            return true;

        if (! exists())
            return false;

       #if ! NAMES_ARE_CASE_SENSITIVE
        if (*this != newFile)
       #endif
            if (! newFile.deleteFile())
                return false;

        return moveInternal (newFile);
        */
    }
    
    /**
      | Copies a file.
      | 
      | Tries to copy a file to a different location.
      | 
      | If the target file already exists, this
      | will attempt to delete it first, and
      | will fail if this can't be done.
      | 
      | -----------
      | @return
      | 
      | true if the operation succeeds
      |
      */
    pub fn copy_file_to(&self, new_file: &File) -> bool {
        
        todo!();
        /*
            return (*this == newFile)
                || (exists() && newFile.deleteFile() && copyInternal (newFile));
        */
    }
    
    /**
      | Replaces a file.
      | 
      | Replace the file in the given location,
      | assuming the replaced files identity.
      | 
      | Depending on the file system this will
      | preserve file attributes such as creation
      | date, short file name, etc.
      | 
      | If replacement succeeds the original
      | file is deleted.
      | 
      | -----------
      | @return
      | 
      | true if the operation succeeds
      |
      */
    pub fn replace_file_in(&self, new_file: &File) -> bool {
        
        todo!();
        /*
            if (newFile.fullPath == fullPath)
            return true;

        if (! newFile.exists())
            return moveFileTo (newFile);

        if (! replaceInternal (newFile))
            return false;

        deleteFile();
        return true;
        */
    }
    
    /**
      | Copies a directory.
      | 
      | Tries to copy an entire directory, recursively.
      | 
      | If this file isn't a directory or if any
      | target files can't be created, this
      | will return false.
      | 
      | -----------
      | @param newDirectory
      | 
      | the directory that this one should be
      | copied to. Note that this is the name
      | of the actual directory to create, not
      | the directory into which the new one
      | should be placed, so there must be enough
      | write privileges to create it if it doesn't
      | exist. Any files inside it will be overwritten
      | by similarly named ones that are copied.
      |
      */
    pub fn copy_directory_to(&self, new_directory: &File) -> bool {
        
        todo!();
        /*
            if (isDirectory() && newDirectory.createDirectory())
        {
            for (auto& f : findChildFiles (File::findFiles, false))
                if (! f.copyFileTo (newDirectory.getChildFile (f.getFileName())))
                    return false;

            for (auto& f : findChildFiles (File::findDirectories, false))
                if (! f.copyDirectoryTo (newDirectory.getChildFile (f.getFileName())))
                    return false;

            return true;
        }

        return false;
        */
    }
    
    pub fn get_path_up_to_last_slash(&self) -> String {
        
        todo!();
        /*
            auto lastSlash = fullPath.lastIndexOfChar (getSeparatorChar());

        if (lastSlash > 0)
            return fullPath.substring (0, lastSlash);

        if (lastSlash == 0)
            return getSeparatorString();

        return fullPath;
        */
    }
    
    /**
      | Returns the directory that contains
      | this file or directory.
      | 
      | e.g. for "/moose/fish/foo.txt" this
      | will return "/moose/fish".
      | 
      | If you are already at the root directory
      | ("/" or "C:") then this method will return
      | the root directory.
      |
      */
    pub fn get_parent_directory(&self) -> File {
        
        todo!();
        /*
            return createFileWithoutCheckingPath (getPathUpToLastSlash());
        */
    }
    
    /**
      | Returns the last section of the pathname.
      | 
      | Returns just the final part of the path
      | - e.g. if the whole path is "/moose/fish/foo.txt"
      | this will return "foo.txt".
      | 
      | For a directory, it returns the final
      | part of the path - e.g. for the directory
      | "/moose/fish" it'll return "fish".
      | 
      | If the filename begins with a dot, it'll
      | return the whole filename, e.g. for
      | "/moose/.fish", it'll return ".fish"
      | 
      | @see getFullPathName, getFileNameWithoutExtension
      |
      */
    pub fn get_file_name(&self) -> String {
        
        todo!();
        /*
            return fullPath.substring (fullPath.lastIndexOfChar (getSeparatorChar()) + 1);
        */
    }
    
    /**
      | Returns the last part of the filename,
      | without its file extension.
      | 
      | e.g. for "/moose/fish/foo.txt" this
      | will return "foo".
      | 
      | @see getFileName, getFileExtension,
      | hasFileExtension, withFileExtension
      |
      */
    pub fn get_file_name_without_extension(&self) -> String {
        
        todo!();
        /*
            auto lastSlash = fullPath.lastIndexOfChar (getSeparatorChar()) + 1;
        auto lastDot   = fullPath.lastIndexOfChar ('.');

        if (lastDot > lastSlash)
            return fullPath.substring (lastSlash, lastDot);

        return fullPath.substring (lastSlash);
        */
    }
    
    /**
      | Checks whether a file is somewhere inside
      | a directory.
      | 
      | Returns true if this file is somewhere
      | inside a subdirectory of the directory
      | that is passed in. Neither file actually
      | has to exist, because the function just
      | checks the paths for similarities.
      | 
      | e.g. File ("/moose/fish/foo.txt").isAChildOf
      | ("/moose") is true.
      | 
      | File ("/moose/fish/foo.txt").isAChildOf
      | ("/moose/fish") is also true.
      |
      */
    pub fn is_achild_of(&self, potential_parent: &File) -> bool {
        
        todo!();
        /*
            if (potentialParent.fullPath.isEmpty())
            return false;

        auto ourPath = getPathUpToLastSlash();

        if (compareFilenames (potentialParent.fullPath, ourPath) == 0)
            return true;

        if (potentialParent.fullPath.length() >= ourPath.length())
            return false;

        return getParentDirectory().isAChildOf (potentialParent);
        */
    }
    
    /**
      | Returns a 32-bit hash-code that identifies
      | this file.
      | 
      | This is based on the filename. Obviously
      | it's possible, although unlikely,
      | that two files will have the same hash-code.
      |
      */
    pub fn hash_code(&self) -> i32 {
        
        todo!();
        /*
            return fullPath.hashCode();
        */
    }
    
    /**
      | Returns a 64-bit hash-code that identifies
      | this file.
      | 
      | This is based on the filename. Obviously
      | it's possible, although unlikely,
      | that two files will have the same hash-code.
      |
      */
    pub fn hash_code64(&self) -> i64 {
        
        todo!();
        /*
            return fullPath.hashCode64();
        */
    }
    
    /**
      | Returns true if the string seems to be
      | a fully-specified absolute path.
      |
      */
    pub fn is_absolute_path(&mut self, path: &str) -> bool {
        
        todo!();
        /*
            auto firstChar = *(path.text);

        return firstChar == getSeparatorChar()
               #if ALOE_WINDOWS
                || (firstChar != 0 && path.text[1] == ':');
               #else
                || firstChar == '~';
               #endif
        */
    }
    
    /**
      | Returns a file that represents a relative
      | (or absolute) sub-path of the current
      | one.
      | 
      | This will find a child file or directory
      | of the current object.
      | 
      | e.g.
      | 
      | File ("/moose/fish").getChildFile
      | ("foo.txt") will produce "/moose/fish/foo.txt".
      | 
      | File ("/moose/fish").getChildFile
      | ("haddock/foo.txt") will produce
      | "/moose/fish/haddock/foo.txt".
      | 
      | File ("/moose/fish").getChildFile
      | ("../foo.txt") will produce "/moose/foo.txt".
      | 
      | If the string is actually an absolute
      | path, it will be treated as such, e.g.
      | 
      | File ("/moose/fish").getChildFile
      | ("/foo.txt") will produce "/foo.txt"
      | 
      | @see getSiblingFile, getParentDirectory,
      | getRelativePathFrom, isAChildOf
      |
      */
    pub fn get_child_file(&self, relative_path: &str) -> File {
        
        todo!();
        /*
            auto r = relativePath.text;

        if (isAbsolutePath (r))
            return File (String (r));

       #if ALOE_WINDOWS
        if (r.indexOf ((aloe_wchar) '/') >= 0)
            return getChildFile (String (r).replaceCharacter ('/', '\\'));
       #endif

        auto path = fullPath;
        auto separatorChar = getSeparatorChar();

        while (*r == '.')
        {
            auto lastPos = r;
            auto secondChar = *++r;

            if (secondChar == '.') // remove "../"
            {
                auto thirdChar = *++r;

                if (thirdChar == separatorChar || thirdChar == 0)
                {
                    auto lastSlash = path.lastIndexOfChar (separatorChar);

                    if (lastSlash >= 0)
                        path = path.substring (0, lastSlash);

                    while (*r == separatorChar) // ignore duplicate slashes
                        ++r;
                }
                else
                {
                    r = lastPos;
                    break;
                }
            }
            else if (secondChar == separatorChar || secondChar == 0)  // remove "./"
            {
                while (*r == separatorChar) // ignore duplicate slashes
                    ++r;
            }
            else
            {
                r = lastPos;
                break;
            }
        }

        path = addTrailingSeparator (path);
        path.appendCharPointer (r);
        return File (path);
        */
    }
    
    /**
      | Returns a file which is in the same directory
      | as this one.
      | 
      | This is equivalent to getParentDirectory().getChildFile
      | (name).
      | 
      | @see getChildFile, getParentDirectory
      |
      */
    pub fn get_sibling_file(&self, file_name: &str) -> File {
        
        todo!();
        /*
            return getParentDirectory().getChildFile (fileName);
        */
    }
    
    /**
      | Utility function to convert a file size
      | in bytes to a neat string description.
      | 
      | So for example 100 would return "100
      | bytes", 2000 would return "2 KB", 2000000
      | would produce "2 MB", etc.
      |
      */
    pub fn description_of_size_in_bytes(&mut self, bytes: i64) -> String {
        
        todo!();
        /*
            const char* suffix;
        double divisor = 0;

        if (bytes == 1)                       { suffix = " byte"; }
        else if (bytes < 1024)                { suffix = " bytes"; }
        else if (bytes < 1024 * 1024)         { suffix = " KB"; divisor = 1024.0; }
        else if (bytes < 1024 * 1024 * 1024)  { suffix = " MB"; divisor = 1024.0 * 1024.0; }
        else                                  { suffix = " GB"; divisor = 1024.0 * 1024.0 * 1024.0; }

        return (divisor > 0 ? String ((double) bytes / divisor, 1) : String (bytes)) + suffix;
        */
    }
    
    /**
      | Creates an empty file if it doesn't already
      | exist.
      | 
      | If the file that this object refers to
      | doesn't exist, this will create a file
      | of zero size.
      | 
      | If it already exists or is a directory,
      | this method will do nothing.
      | 
      | If the parent directories of the File
      | do not exist then this method will recursively
      | create the parent directories.
      | 
      | -----------
      | @return
      | 
      | a result to indicate whether the file
      | was created successfully, or an error
      | message if it failed. @see createDirectory
      |
      */
    pub fn create(&self) -> Result<(),()> {
        
        todo!();
        /*
            if (exists())
            return Result::ok();

        auto parentDir = getParentDirectory();

        if (parentDir == *this)
            return Result::fail ("Cannot create parent directory");

        auto r = parentDir.createDirectory();

        if (r.wasOk())
        {
            FileOutputStream fo (*this, 8);
            r = fo.getStatus();
        }

        return r;
        */
    }
    
    /**
      | Creates a new directory for this filename.
      | 
      | This will try to create the file as a directory,
      | and will also create any parent directories
      | it needs in order to complete the operation.
      | 
      | -----------
      | @return
      | 
      | a result to indicate whether the directory
      | was created successfully, or an error
      | message if it failed. @see create
      |
      */
    pub fn create_directory(&self) -> Result<(),()> {
        
        todo!();
        /*
            if (isDirectory())
            return Result::ok();

        auto parentDir = getParentDirectory();

        if (parentDir == *this)
            return Result::fail ("Cannot create parent directory");

        auto r = parentDir.createDirectory();

        if (r.wasOk())
            r = createDirectoryInternal (fullPath.trimCharactersAtEnd (getSeparatorString()));

        return r;
        */
    }
    
    /**
      | Returns the last modification time
      | of this file.
      | 
      | -----------
      | @return
      | 
      | the time, or an invalid time if the file
      | doesn't exist. @see setLastModificationTime,
      | getLastAccessTime, getCreationTime
      |
      */
    pub fn get_last_modification_time(&self) -> Time {
        
        todo!();
        /*
            int64 m, a, c; getFileTimesInternal (m, a, c); return Time (m);
        */
    }
    
    /**
      | Returns the last time this file was accessed.
      | 
      | -----------
      | @return
      | 
      | the time, or an invalid time if the file
      | doesn't exist. @see setLastAccessTime,
      | getLastModificationTime, getCreationTime
      |
      */
    pub fn get_last_access_time(&self) -> Time {
        
        todo!();
        /*
            int64 m, a, c; getFileTimesInternal (m, a, c); return Time (a);
        */
    }
    
    /**
      | Returns the time that this file was created.
      | 
      | -----------
      | @return
      | 
      | the time, or an invalid time if the file
      | doesn't exist. @see getLastModificationTime,
      | getLastAccessTime
      |
      */
    pub fn get_creation_time(&self) -> Time {
        
        todo!();
        /*
            int64 m, a, c; getFileTimesInternal (m, a, c); return Time (c);
        */
    }
    
    /**
      | Changes the modification time for this
      | file.
      | 
      | -----------
      | @param newTime
      | 
      | the time to apply to the file
      | 
      | -----------
      | @return
      | 
      | true if it manages to change the file's
      | time. @see getLastModificationTime,
      | setLastAccessTime, setCreationTime
      |
      */
    pub fn set_last_modification_time(&self, t: Time) -> bool {
        
        todo!();
        /*
            return setFileTimesInternal (t.toMilliseconds(), 0, 0);
        */
    }
    
    /**
      | Changes the last-access time for this
      | file.
      | 
      | -----------
      | @param newTime
      | 
      | the time to apply to the file
      | 
      | -----------
      | @return
      | 
      | true if it manages to change the file's
      | time. @see getLastAccessTime, setLastModificationTime,
      | setCreationTime
      |
      */
    pub fn set_last_access_time(&self, t: Time) -> bool {
        
        todo!();
        /*
            return setFileTimesInternal (0, t.toMilliseconds(), 0);
        */
    }
    
    /**
      | Changes the creation date for this file.
      | 
      | -----------
      | @param newTime
      | 
      | the time to apply to the file
      | 
      | -----------
      | @return
      | 
      | true if it manages to change the file's
      | time. @see getCreationTime, setLastModificationTime,
      | setLastAccessTime
      |
      */
    pub fn set_creation_time(&self, t: Time) -> bool {
        
        todo!();
        /*
            return setFileTimesInternal (0, 0, t.toMilliseconds());
        */
    }
    
    /**
      | Loads a file's contents into memory
      | as a block of binary data.
      | 
      | Of course, trying to load a very large
      | file into memory will blow up, so it's
      | better to check first.
      | 
      | -----------
      | @param result
      | 
      | the data block to which the file's contents
      | should be appended - note that if the
      | memory block might already contain
      | some data, you might want to clear it
      | first
      | 
      | -----------
      | @return
      | 
      | true if the file could all be read into
      | memory
      |
      */
    pub fn load_file_as_data(&self, dest_block: &mut MemoryBlock) -> bool {
        
        todo!();
        /*
            if (! existsAsFile())
            return false;

        FileInputStream in (*this);
        return in.openedOk() && getSize() == (int64) in.readIntoMemoryBlock (destBlock);
        */
    }
    
    /**
      | Reads a file into memory as a string.
      | 
      | Attempts to load the entire file as a
      | zero-terminated string.
      | 
      | This makes use of InputStream::readEntireStreamAsString,
      | which can read either UTF-16 or UTF-8
      | file formats.
      |
      */
    pub fn load_file_as_string(&self) -> String {
        
        todo!();
        /*
            if (! existsAsFile())
            return {};

        FileInputStream in (*this);
        return in.openedOk() ? in.readEntireStreamAsString()
                             : String();
        */
    }
    
    /**
      | Reads the contents of this file as text
      | and splits it into lines, which are appended
      | to the given StringArray.
      |
      */
    pub fn read_lines(&self, dest_lines: &mut StringArray)  {
        
        todo!();
        /*
            destLines.addLines (loadFileAsString());
        */
    }
    
    /**
      | Searches this directory for files matching
      | a wildcard pattern.
      | 
      | Assuming that this file is a directory,
      | this method will search it for either
      | files or subdirectories whose names
      | match a filename pattern.
      | 
      | -----------
      | @note
      | 
      | the order in which files are returned
      | is completely undefined!
      | 
      | -----------
      | @param whatToLookFor
      | 
      | a value from the TypesOfFileToFind
      | enum, specifying whether to return
      | files, directories, or both. If the
      | ignoreHiddenFiles flag is also added
      | to this value, hidden files won't be
      | returned
      | ----------
      | @param searchRecursively
      | 
      | if true, all subdirectories will be
      | recursed into to do an exhaustive search
      | ----------
      | @param wildCardPattern
      | 
      | the filename pattern to search for,
      | e.g. "*.txt"
      | 
      | -----------
      | @return
      | 
      | the set of files that were found
      | 
      | @see getNumberOfChildFiles, RangedDirectoryIterator
      |
      */
    pub fn find_child_files(&self, 
        what_to_look_for:   i32,
        search_recursively: bool,
        wildcard:           Option<&str>) -> Vec<File> {

        let wildcard = wildcard.unwrap_or("*");
        
        todo!();
        /*
            Vec<File> results;
        findChildFiles (results, whatToLookFor, searchRecursively, wildcard);
        return results;
        */
    }
    
    /**
      | Searches inside a directory for files
      | matching a wildcard pattern.
      | 
      | -----------
      | @note
      | 
      | there's a newer, better version of this
      | method which returns the results array,
      | and in almost all cases, you should use
      | that one instead! This one is kept around
      | mainly for legacy code to use.
      |
      */
    pub fn find_child_files_into(&self, 
        results:            &mut Vec<File>,
        what_to_look_for:   i32,
        search_recursively: bool,
        wildcard:           Option<&str>) -> i32 {

        let wildcard = wildcard.unwrap_or("*");
        
        todo!();
        /*
            int total = 0;

        for (const auto& di : RangedDirectoryIterator (*this, searchRecursively, wildcard, whatToLookFor))
        {
            results.add (di.getFile());
            ++total;
        }

        return total;
        */
    }
    
    /**
      | Searches inside a directory and counts
      | how many files match a wildcard pattern.
      | 
      | Assuming that this file is a directory,
      | this method will search it for either
      | files or subdirectories whose names
      | match a filename pattern, and will return
      | the number of matches found.
      | 
      | This isn't a recursive call, and will
      | only search this directory, not its
      | children.
      | 
      | -----------
      | @param whatToLookFor
      | 
      | a value from the TypesOfFileToFind
      | enum, specifying whether to count files,
      | directories, or both. If the ignoreHiddenFiles
      | flag is also added to this value, hidden
      | files won't be counted
      | ----------
      | @param wildCardPattern
      | 
      | the filename pattern to search for,
      | e.g. "*.txt"
      | 
      | -----------
      | @return
      | 
      | the number of matches found
      | 
      | @see findChildFiles, RangedDirectoryIterator
      |
      */
    pub fn get_number_of_child_files(
        &self, 
        what_to_look_for:  i32,
        wild_card_pattern: Option<&str>) -> i32 
    {
        let wild_card_pattern = wild_card_pattern.unwrap_or("*");
        
        todo!();
        /*
            return std::accumulate (RangedDirectoryIterator (*this, false, wildCardPattern, whatToLookFor),
                                RangedDirectoryIterator(),
                                0,
                                [] (int acc, const DirectoryEntry&) { return acc + 1; });
        */
    }
    
    /**
      | Returns true if this file is a directory
      | that contains one or more subdirectories.
      | @see isDirectory, findChildFiles
      |
      */
    pub fn contains_sub_directories(&self) -> bool {
        
        todo!();
        /*
            if (! isDirectory())
            return false;

        return RangedDirectoryIterator (*this, false, "*", findDirectories) != RangedDirectoryIterator();
        */
    }
    
    /**
      | Chooses a filename relative to this
      | one that doesn't already exist.
      | 
      | If this file is a directory, this will
      | return a child file of this directory
      | that doesn't exist, by adding numbers
      | to a prefix and suffix until it finds
      | one that isn't already there.
      | 
      | If the prefix + the suffix doesn't exist,
      | it won't bother adding a number.
      | 
      | e.g. File ("/moose/fish").getNonexistentChildFile
      | ("foo", ".txt", true) might return
      | "/moose/fish/foo(2).txt" if there's
      | already a file called "foo.txt".
      | 
      | -----------
      | @param prefix
      | 
      | the string to use for the filename before
      | the number
      | ----------
      | @param suffix
      | 
      | the string to add to the filename after
      | the number
      | ----------
      | @param putNumbersInBrackets
      | 
      | if true, this will create filenames
      | in the format "prefix(number)suffix",
      | if false, it will leave the brackets
      | out.
      |
      */
    pub fn get_nonexistent_child_file(
        &self, 
        suggested_prefix:        &String,
        suffix:                  &String,
        put_numbers_in_brackets: Option<bool>) -> File {

        let put_numbers_in_brackets: bool = put_numbers_in_brackets.unwrap_or(true);
        
        todo!();
        /*
            auto f = getChildFile (suggestedPrefix + suffix);

        if (f.exists())
        {
            int number = 1;
            auto prefix = suggestedPrefix;

            // remove any bracketed numbers that may already be on the end..
            if (prefix.trim().endsWithChar (')'))
            {
                putNumbersInBrackets = true;

                auto openBracks  = prefix.lastIndexOfChar ('(');
                auto closeBracks = prefix.lastIndexOfChar (')');

                if (openBracks > 0
                     && closeBracks > openBracks
                     && prefix.substring (openBracks + 1, closeBracks).containsOnly ("0123456789"))
                {
                    number = prefix.substring (openBracks + 1, closeBracks).getIntValue();
                    prefix = prefix.substring (0, openBracks);
                }
            }

            do
            {
                auto newName = prefix;

                if (putNumbersInBrackets)
                {
                    newName << '(' << ++number << ')';
                }
                else
                {
                    if (CharacterFunctions::isDigit (prefix.getLastCharacter()))
                        newName << '_'; // pad with an underscore if the name already ends in a digit

                    newName << ++number;
                }

                f = getChildFile (newName + suffix);

            } while (f.exists());
        }

        return f;
        */
    }
    
    /**
      | Chooses a filename for a sibling file
      | to this one that doesn't already exist.
      | 
      | If this file doesn't exist, this will
      | just return itself, otherwise it will
      | return an appropriate sibling that
      | doesn't exist, e.g. if a file "/moose/fish/foo.txt"
      | exists, this might return "/moose/fish/foo(2).txt".
      | 
      | -----------
      | @param putNumbersInBrackets
      | 
      | whether to add brackets around the numbers
      | that get appended to the new filename.
      |
      */
    pub fn get_nonexistent_sibling(&self, put_numbers_in_brackets: Option<bool>) -> File {

        let put_numbers_in_brackets: bool = put_numbers_in_brackets.unwrap_or(true);
        
        todo!();
        /*
            if (! exists())
            return *this;

        return getParentDirectory().getNonexistentChildFile (getFileNameWithoutExtension(),
                                                             getFileExtension(),
                                                             putNumbersInBrackets);
        */
    }
    
    /**
      | Returns the file's extension.
      | 
      | Returns the file extension of this file,
      | also including the dot.
      | 
      | e.g. "/moose/fish/foo.txt" would
      | return ".txt"
      | 
      | @see hasFileExtension, withFileExtension,
      | getFileNameWithoutExtension
      |
      */
    pub fn get_file_extension(&self) -> String {
        
        todo!();
        /*
            auto indexOfDot = fullPath.lastIndexOfChar ('.');

        if (indexOfDot > fullPath.lastIndexOfChar (getSeparatorChar()))
            return fullPath.substring (indexOfDot);

        return {};
        */
    }
    
    /**
      | Checks whether the file has a given extension.
      | 
      | -----------
      | @param extensionToTest
      | 
      | the extension to look for - it doesn't
      | matter whether or not this string has
      | a dot at the start, so ".wav" and "wav"
      | will have the same effect. To compare
      | with multiple extensions, this parameter
      | can contain multiple strings, separated
      | by semi-colons - so, for example: hasFileExtension
      | (".jpeg;png;gif") would return true
      | if the file has any of those three extensions.
      | 
      | @see getFileExtension, withFileExtension,
      | getFileNameWithoutExtension
      |
      */
    pub fn has_file_extension(&self, possible_suffix: &str) -> bool {
        
        todo!();
        /*
            if (possibleSuffix.isEmpty())
            return fullPath.lastIndexOfChar ('.') <= fullPath.lastIndexOfChar (getSeparatorChar());

        auto semicolon = possibleSuffix.text.indexOf ((aloe_wchar) ';');

        if (semicolon >= 0)
            return hasFileExtension (String (possibleSuffix.text).substring (0, semicolon).trimEnd())
                    || hasFileExtension ((possibleSuffix.text + (semicolon + 1)).findEndOfWhitespace());

        if (fullPath.endsWithIgnoreCase (possibleSuffix))
        {
            if (possibleSuffix.text[0] == '.')
                return true;

            auto dotPos = fullPath.length() - possibleSuffix.length() - 1;

            if (dotPos >= 0)
                return fullPath[dotPos] == '.';
        }

        return false;
        */
    }
    
    /**
      | Returns a version of this file with a
      | different file extension.
      | 
      | e.g. File ("/moose/fish/foo.txt").withFileExtension
      | ("html") returns "/moose/fish/foo.html"
      | 
      | -----------
      | @param newExtension
      | 
      | the new extension, either with or without
      | a dot at the start (this doesn't make
      | any difference). To get remove a file's
      | extension altogether, pass an empty
      | string into this function.
      | 
      | @see getFileName, getFileExtension,
      | hasFileExtension, getFileNameWithoutExtension
      |
      */
    pub fn with_file_extension(&self, new_extension: &str) -> File {
        
        todo!();
        /*
            if (fullPath.isEmpty())
            return {};

        auto filePart = getFileName();

        auto lastDot = filePart.lastIndexOfChar ('.');

        if (lastDot >= 0)
            filePart = filePart.substring (0, lastDot);

        if (newExtension.isNotEmpty() && newExtension.text[0] != '.')
            filePart << '.';

        return getSiblingFile (filePart + newExtension);
        */
    }
    
    /**
      | Launches the file as a process.
      | 
      | - if the file is executable, this will
      | run it.
      | 
      | - if it's a document of some kind, it will
      | launch the document with its default
      | viewer application.
      | 
      | - if it's a folder, it will be opened in
      | Explorer, Finder, or equivalent.
      | 
      | @see revealToUser
      |
      */
    pub fn start_as_process(&self, parameters: Option<&str>) -> bool {

        let parameters = parameters.unwrap_or("");
        
        todo!();
        /*
            return exists() && Process::openDocument (fullPath, parameters);
        */
    }
    
    /**
      | Creates a stream to read from this file.
      | 
      | -----------
      | @note
      | 
      | this is an old method, and actually it's
      | usually best to avoid it and instead
      | use an RAII pattern with an FileInputStream
      | directly, e.g.
      | 
      | -----------
      | @code
      | 
      | FileInputStream input (fileToOpen);
      | 
      | if (input.openedOk())
      | {
      |     input.read (etc...
      | }
      | 
      | -----------
      | @return
      | 
      | a stream that will read from this file
      | (initially positioned at the start
      | of the file), or nullptr if the file can't
      | be opened for some reason @see createOutputStream,
      | loadFileAsData
      |
      */
    pub fn create_input_stream(&self) -> Box<FileInputStream> {
        
        todo!();
        /*
            auto fin = std::make_unique<FileInputStream> (*this);

        if (fin->openedOk())
            return fin;

        return nullptr;
        */
    }

    /**
      | Creates a stream to write to this file.
      | 
      | Note that this is an old method, and actually
      | it's usually best to avoid it and instead
      | use an RAII pattern with an FileOutputStream
      | directly, e.g.
      | 
      | -----------
      | @code
      | 
      | FileOutputStream output (fileToOpen);
      | 
      | if (output.openedOk())
      | {
      |     output.read etc...
      | }
      | 
      | If the file exists, the stream that is
      | returned will be positioned ready for
      | writing at the end of the file. If you
      | want to write to the start of the file,
      | replacing the existing content, then
      | you can do the following:
      | ----------
      | @code
      | 
      | FileOutputStream output (fileToOverwrite);
      | 
      | if (output.openedOk())
      | {
      |     output.setPosition (0);
      |     output.truncate();
      |     ...
      | }
      | 
      | -----------
      | @return
      | 
      | a stream that will write to this file
      | (initially positioned at the end of
      | the file), or nullptr if the file can't
      | be opened for some reason @see createInputStream,
      | appendData, appendText
      |
      */
    pub fn create_output_stream(&self, buffer_size: Option<usize>) -> Box<FileOutputStream> {

        let buffer_size: usize = buffer_size.unwrap_or(0x8000);
        
        todo!();
        /*
            auto fout = std::make_unique<FileOutputStream> (*this, bufferSize);

        if (fout->openedOk())
            return fout;

        return nullptr;
        */
    }
    
    /**
      | Appends a block of binary data to the
      | end of the file.
      | 
      | This will try to write the given buffer
      | to the end of the file.
      | 
      | -----------
      | @return
      | 
      | false if it can't write to the file for
      | some reason
      |
      */
    pub fn append_data(&self, 
        data_to_append:  *const c_void,
        number_of_bytes: usize) -> bool {
        
        todo!();
        /*
            jassert (((ssize_t) numberOfBytes) >= 0);

        if (numberOfBytes == 0)
            return true;

        FileOutputStream fout (*this, 8192);
        return fout.openedOk() && fout.write (dataToAppend, numberOfBytes);
        */
    }
    
    /**
      | Replaces this file's contents with
      | a given block of data.
      | 
      | This will delete the file and replace
      | it with the given data.
      | 
      | A nice feature of this method is that
      | it's safe - instead of deleting the file
      | first and then re-writing it, it creates
      | a new temporary file, writes the data
      | to that, and then moves the new file to
      | replace the existing file. This means
      | that if the power gets pulled out or something
      | crashes, you're a lot less likely to
      | end up with a corrupted or unfinished
      | file..
      | 
      | Returns true if the operation succeeds,
      | or false if it fails.
      | 
      | @see appendText
      |
      */
    pub fn replace_with_data(&self, 
        data_to_write:   *const c_void,
        number_of_bytes: usize) -> bool {
        
        todo!();
        /*
            if (numberOfBytes == 0)
            return deleteFile();

        TemporaryFile tempFile (*this, TemporaryFile::useHiddenFile);
        tempFile.getFile().appendData (dataToWrite, numberOfBytes);
        return tempFile.overwriteTargetFileWithTemporary();
        */
    }
    
    /**
      | Appends a string to the end of the file.
      | 
      | This will try to append a text string
      | to the file, as either 16-bit unicode
      | or 8-bit characters in the default system
      | encoding.
      | 
      | It can also write the 'ff fe' unicode
      | header bytes before the text to indicate
      | the endianness of the file.
      | 
      | If lineEndings is nullptr, then line
      | endings in the text won't be modified.
      | If you pass "\\n" or "\\r\\n" then this
      | function will replace any existing
      | line feeds.
      | 
      | @see replaceWithText
      |
      */
    pub fn append_text(&self, 
        text:               &String,
        as_unicode:         Option<bool>,
        write_header_bytes: Option<bool>,
        line_feed:          Option<&str>) -> bool {

        let as_unicode:         bool = as_unicode.unwrap_or(false);
        let write_header_bytes: bool = write_header_bytes.unwrap_or(false);

        let line_feed = line_feed.unwrap_or("\r\n");
        
        todo!();
        /*
            FileOutputStream fout (*this);

        if (fout.failedToOpen())
            return false;

        return fout.writeText (text, asUnicode, writeHeaderBytes, lineFeed);
        */
    }
    
    /**
      | Replaces this file's contents with
      | a given text string.
      | 
      | This will delete the file and replace
      | it with the given text.
      | 
      | A nice feature of this method is that
      | it's safe - instead of deleting the file
      | first and then re-writing it, it creates
      | a new temporary file, writes the text
      | to that, and then moves the new file to
      | replace the existing file. This means
      | that if the power gets pulled out or something
      | crashes, you're a lot less likely to
      | end up with an empty file..
      | 
      | For an explanation of the parameters
      | here, see the appendText() method.
      | 
      | Returns true if the operation succeeds,
      | or false if it fails.
      | 
      | @see appendText
      |
      */
    pub fn replace_with_text(
        &self, 
        text_to_write:      &String,
        as_unicode:         Option<bool>,
        write_header_bytes: Option<bool>,
        line_feed:          Option<&str>) -> bool {

        let as_unicode:         bool = as_unicode.unwrap_or(false);
        let write_header_bytes: bool = write_header_bytes.unwrap_or(false);

        let line_feed = line_feed.unwrap_or("\r\n");
        
        todo!();
        /*
            TemporaryFile tempFile (*this, TemporaryFile::useHiddenFile);
        tempFile.getFile().appendText (textToWrite, asUnicode, writeHeaderBytes, lineFeed);
        return tempFile.overwriteTargetFileWithTemporary();
        */
    }
    
    /**
      | Attempts to scan the contents of this
      | file and compare it to another file,
      | returning true if this is possible and
      | they match byte-for-byte.
      |
      */
    pub fn has_identical_content_to(&self, other: &File) -> bool {
        
        todo!();
        /*
            if (other == *this)
            return true;

        if (getSize() == other.getSize() && existsAsFile() && other.existsAsFile())
        {
            FileInputStream in1 (*this), in2 (other);

            if (in1.openedOk() && in2.openedOk())
            {
                const int bufferSize = 4096;
                HeapBlock<char> buffer1 (bufferSize), buffer2 (bufferSize);

                for (;;)
                {
                    auto num1 = in1.read (buffer1, bufferSize);
                    auto num2 = in2.read (buffer2, bufferSize);

                    if (num1 != num2)
                        break;

                    if (num1 <= 0)
                        return true;

                    if (memcmp (buffer1, buffer2, (size_t) num1) != 0)
                        break;
                }
            }
        }

        return false;
        */
    }
    
    /**
      | Returns a version of a path with any illegal
      | characters removed.
      | 
      | Similar to createLegalFileName(),
      | but this won't remove slashes, so can
      | be used on a complete pathname.
      | 
      | @see createLegalFileName
      |
      */
    pub fn create_legal_path_name(&mut self, original: &String) -> String {
        
        todo!();
        /*
            auto s = original;
        String start;

        if (s.isNotEmpty() && s[1] == ':')
        {
            start = s.substring (0, 2);
            s = s.substring (2);
        }

        return start + s.removeCharacters ("\"#@,;:<>*^|?")
                        .substring (0, 1024);
        */
    }
    
    /**
      | Returns a version of a filename with
      | any illegal characters removed.
      | 
      | This will return a copy of the given string
      | after removing characters that are
      | not allowed in a legal filename, and
      | possibly shortening the string if it's
      | too long.
      | 
      | Because this will remove slashes, don't
      | use it on an absolute pathname - use createLegalPathName()
      | for that.
      | 
      | @see createLegalPathName
      |
      */
    pub fn create_legal_file_name(&mut self, original: &String) -> String {
        
        todo!();
        /*
            auto s = original.removeCharacters ("\"#@,;:<>*^|?\\/");

        const int maxLength = 128; // only the length of the filename, not the whole path
        auto len = s.length();

        if (len > maxLength)
        {
            auto lastDot = s.lastIndexOfChar ('.');

            if (lastDot > jmax (0, len - 12))
            {
                s = s.substring (0, maxLength - (len - lastDot))
                     + s.substring (lastDot);
            }
            else
            {
                s = s.substring (0, maxLength);
            }
        }

        return s;
        */
    }
    
    /**
      | Creates a relative path that refers
      | to a file relatively to a given directory.
      | 
      | e.g. File ("/moose/foo.txt").getRelativePathFrom
      | (File ("/moose/fish/haddock")) would
      | return "../../foo.txt".
      | 
      | If it's not possible to navigate from
      | one file to the other, an absolute path
      | is returned. If the paths are invalid,
      | an empty string may also be returned.
      | 
      | -----------
      | @param directoryToBeRelativeTo
      | 
      | the directory which the resultant string
      | will be relative to. If this is actually
      | a file rather than a directory, its parent
      | directory will be used instead. If it
      | doesn't exist, it's assumed to be a directory.
      | @see getChildFile, isAbsolutePath
      |
      */
    pub fn get_relative_path_from(&self, dir: &File) -> String {
        
        todo!();
        /*
            if (dir == *this)
            return ".";

        auto thisPath = fullPath;

        while (thisPath.endsWithChar (getSeparatorChar()))
            thisPath = thisPath.dropLastCharacters (1);

        auto dirPath = addTrailingSeparator (dir.existsAsFile() ? dir.getParentDirectory().getFullPathName()
                                                                : dir.fullPath);

        int commonBitLength = 0;
        auto thisPathAfterCommon = thisPath.getCharPointer();
        auto dirPathAfterCommon  = dirPath.getCharPointer();

        {
            auto thisPathIter = thisPath.getCharPointer();
            auto dirPathIter = dirPath.getCharPointer();

            for (int i = 0;;)
            {
                auto c1 = thisPathIter.getAndAdvance();
                auto c2 = dirPathIter.getAndAdvance();

               #if NAMES_ARE_CASE_SENSITIVE
                if (c1 != c2
               #else
                if ((c1 != c2 && CharacterFunctions::toLowerCase (c1) != CharacterFunctions::toLowerCase (c2))
               #endif
                     || c1 == 0)
                    break;

                ++i;

                if (c1 == getSeparatorChar())
                {
                    thisPathAfterCommon = thisPathIter;
                    dirPathAfterCommon  = dirPathIter;
                    commonBitLength = i;
                }
            }
        }

        // if the only common bit is the root, then just return the full path..
        if (commonBitLength == 0 || (commonBitLength == 1 && thisPath[1] == getSeparatorChar()))
            return fullPath;

        auto numUpDirectoriesNeeded = countNumberOfSeparators (dirPathAfterCommon);

        if (numUpDirectoriesNeeded == 0)
            return thisPathAfterCommon;

       #if ALOE_WINDOWS
        auto s = String::repeatedString ("..\\", numUpDirectoriesNeeded);
       #else
        auto s = String::repeatedString ("../",  numUpDirectoriesNeeded);
       #endif
        s.appendCharPointer (thisPathAfterCommon);
        return s;
        */
    }
    
    /**
      | Returns a temporary file in the system's
      | temp directory.
      | 
      | This will try to return the name of a non-existent
      | temp file.
      | 
      | To get the temp folder, you can use getSpecialLocation
      | (File::tempDirectory).
      |
      */
    pub fn create_temp_file(&mut self, file_name_ending: &str) -> File {
        
        todo!();
        /*
            auto tempFile = getSpecialLocation (tempDirectory)
                          .getChildFile ("temp_" + String::toHexString (Random::getSystemRandom().nextInt()))
                          .withFileExtension (fileNameEnding);

        if (tempFile.exists())
            return createTempFile (fileNameEnding);

        return tempFile;
        */
    }
    
    /**
      | Create a symbolic link to a native path
      | and return a boolean to indicate success.
      | 
      | Use this method if you want to create
      | a link to a relative path or a special
      | native file path (such as a device file
      | on Windows).
      |
      */
    pub fn create_symbolic_link_native(&mut self, 
        link_file_to_create:   &File,
        native_path_of_target: &String,
        overwrite_existing:    bool) -> bool {
        
        todo!();
        /*
            if (linkFileToCreate.exists())
        {
            if (! linkFileToCreate.isSymbolicLink())
            {
                // user has specified an existing file / directory as the link
                // this is bad! the user could end up unintentionally destroying data
                jassertfalse;
                return false;
            }

            if (overwriteExisting)
                linkFileToCreate.deleteFile();
        }

       #if ALOE_MAC || ALOE_LINUX
        // one common reason for getting an error here is that the file already exists
        if (symlink (nativePathOfTarget.toRawUTF8(), linkFileToCreate.getFullPathName().toRawUTF8()) == -1)
        {
            jassertfalse;
            return false;
        }

        return true;
       #elif ALOE_MSVC
        File targetFile (linkFileToCreate.getSiblingFile (nativePathOfTarget));

        return CreateSymbolicLink (linkFileToCreate.getFullPathName().toWideCharPointer(),
                                   nativePathOfTarget.toWideCharPointer(),
                                   targetFile.isDirectory() ? SYMBOLIC_LINK_FLAG_DIRECTORY : 0) != FALSE;
       #else
        ignoreUnused (nativePathOfTarget);
        jassertfalse; // symbolic links not supported on this platform!
        return false;
       #endif
        */
    }
    
    /**
      | Tries to create a symbolic link and returns
      | a boolean to indicate success
      |
      */
    pub fn create_symbolic_link(&self, 
        link_file_to_create: &File,
        overwrite_existing:  bool) -> bool {
        
        todo!();
        /*
            return createSymbolicLink (linkFileToCreate, getFullPathName(), overwriteExisting);
        */
    }

    /**
      | If this file is a link or alias, this returns
      | the file that it points to.
      | 
      | If the file isn't actually link, it'll
      | just return itself.
      |
      */
    #[cfg(not(target_os="windows"))]
    pub fn get_linked_target(&self) -> File {
        
        todo!();
        /*
            if (isSymbolicLink())
            return getSiblingFile (getNativeLinkedTarget());

        return *this;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct FileTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for FileTests {
    
    fn default() -> Self {
        todo!();
        /*
        : UnitTest ("Files", UnitTestCategories::files
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl FileTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Reading");

            const File home (File::getSpecialLocation (File::userHomeDirectory));
            const File temp (File::getSpecialLocation (File::tempDirectory));

            expect (! File().exists());
            expect (! File().existsAsFile());
            expect (! File().isDirectory());
           #if ! ALOE_WINDOWS
            expect (File("/").isDirectory());
           #endif
            expect (home.isDirectory());
            expect (home.exists());
            expect (! home.existsAsFile());
            expect (File::getSpecialLocation (File::userApplicationDataDirectory).isDirectory());
            expect (File::getSpecialLocation (File::currentExecutableFile).exists());
            expect (File::getSpecialLocation (File::currentApplicationFile).exists());
            expect (File::getSpecialLocation (File::invokedExecutableFile).exists());
            expect (home.getVolumeTotalSize() > 1024 * 1024);
            expect (home.getBytesFreeOnVolume() > 0);
            expect (! home.isHidden());
            expect (home.isOnHardDisk());
            expect (! home.isOnCDRomDrive());
            expect (File::getCurrentWorkingDirectory().exists());
            expect (home.setAsCurrentWorkingDirectory());

            {
                auto homeParent = home;
                bool noSymlinks = true;

                while (! homeParent.isRoot())
                {
                    if (homeParent.isSymbolicLink())
                    {
                        noSymlinks = false;
                        break;
                    }

                    homeParent = homeParent.getParentDirectory();
                }

                if (noSymlinks)
                    expect (File::getCurrentWorkingDirectory() == home);
            }

            {
                Vec<File> roots;
                File::findFileSystemRoots (roots);
                expect (roots.size() > 0);

                int numRootsExisting = 0;
                for (int i = 0; i < roots.size(); ++i)
                    if (roots[i].exists())
                        ++numRootsExisting;

                // (on windows, some of the drives may not contain media, so as long as at least one is ok..)
                expect (numRootsExisting > 0);
            }

            beginTest ("Writing");

            auto random = getRandom();
            const auto tempFolderName = "Aloe UnitTests Temp Folder "
                                      + String::toHexString (random.nextInt())
                                      + ".folder";
            File demoFolder (temp.getChildFile (tempFolderName));
            expect (demoFolder.deleteRecursively());
            expect (demoFolder.createDirectory());
            expect (demoFolder.isDirectory());
            expect (demoFolder.getParentDirectory() == temp);
            expect (temp.isDirectory());
            expect (temp.findChildFiles (File::findFilesAndDirectories, false, "*").contains (demoFolder));
            expect (temp.findChildFiles (File::findDirectories, true, "*.folder").contains (demoFolder));

            File tempFile (demoFolder.getNonexistentChildFile ("test", ".txt", false));

            expect (tempFile.getFileExtension() == ".txt");
            expect (tempFile.hasFileExtension (".txt"));
            expect (tempFile.hasFileExtension ("txt"));
            expect (tempFile.withFileExtension ("xyz").hasFileExtension (".xyz"));
            expect (tempFile.withFileExtension ("xyz").hasFileExtension ("abc;xyz;foo"));
            expect (tempFile.withFileExtension ("xyz").hasFileExtension ("xyz;foo"));
            expect (! tempFile.withFileExtension ("h").hasFileExtension ("bar;foo;xx"));
            expect (tempFile.getSiblingFile ("foo").isAChildOf (temp));
            expect (tempFile.hasWriteAccess());

            expect (home.getChildFile (".") == home);
            expect (home.getChildFile ("..") == home.getParentDirectory());
            expect (home.getChildFile (".xyz").getFileName() == ".xyz");
            expect (home.getChildFile ("..xyz").getFileName() == "..xyz");
            expect (home.getChildFile ("...xyz").getFileName() == "...xyz");
            expect (home.getChildFile ("./xyz") == home.getChildFile ("xyz"));
            expect (home.getChildFile ("././xyz") == home.getChildFile ("xyz"));
            expect (home.getChildFile ("../xyz") == home.getParentDirectory().getChildFile ("xyz"));
            expect (home.getChildFile (".././xyz") == home.getParentDirectory().getChildFile ("xyz"));
            expect (home.getChildFile (".././xyz/./abc") == home.getParentDirectory().getChildFile ("xyz/abc"));
            expect (home.getChildFile ("./../xyz") == home.getParentDirectory().getChildFile ("xyz"));
            expect (home.getChildFile ("a1/a2/a3/./../../a4") == home.getChildFile ("a1/a4"));

            {
                FileOutputStream fo (tempFile);
                fo.write ("0123456789", 10);
            }

            expect (tempFile.exists());
            expect (tempFile.getSize() == 10);
            expect (std::abs ((int) (tempFile.getLastModificationTime().toMilliseconds() - Time::getCurrentTime().toMilliseconds())) < 3000);
            expectEquals (tempFile.loadFileAsString(), String ("0123456789"));
            expect (! demoFolder.containsSubDirectories());

            expectEquals (tempFile.getRelativePathFrom (demoFolder.getParentDirectory()), demoFolder.getFileName() + File::getSeparatorString() + tempFile.getFileName());
            expectEquals (demoFolder.getParentDirectory().getRelativePathFrom (tempFile), ".." + File::getSeparatorString() + ".." + File::getSeparatorString() + demoFolder.getParentDirectory().getFileName());

            expect (demoFolder.getNumberOfChildFiles (File::findFiles) == 1);
            expect (demoFolder.getNumberOfChildFiles (File::findFilesAndDirectories) == 1);
            expect (demoFolder.getNumberOfChildFiles (File::findDirectories) == 0);
            demoFolder.getNonexistentChildFile ("tempFolder", "", false).createDirectory();
            expect (demoFolder.getNumberOfChildFiles (File::findDirectories) == 1);
            expect (demoFolder.getNumberOfChildFiles (File::findFilesAndDirectories) == 2);
            expect (demoFolder.containsSubDirectories());

            expect (tempFile.hasWriteAccess());
            tempFile.setReadOnly (true);
            expect (! tempFile.hasWriteAccess());
            tempFile.setReadOnly (false);
            expect (tempFile.hasWriteAccess());

            Time t (Time::getCurrentTime());
            tempFile.setLastModificationTime (t);
            Time t2 = tempFile.getLastModificationTime();
            expect (std::abs ((int) (t2.toMilliseconds() - t.toMilliseconds())) <= 1000);

            {
                MemoryBlock mb;
                tempFile.loadFileAsData (mb);
                expect (mb.getSize() == 10);
                expect (mb[0] == '0');
            }

            {
                expect (tempFile.getSize() == 10);
                FileOutputStream fo (tempFile);
                expect (fo.openedOk());

                expect (fo.setPosition  (7));
                expect (fo.truncate().wasOk());
                expect (tempFile.getSize() == 7);
                fo.write ("789", 3);
                fo.flush();
                expect (tempFile.getSize() == 10);
            }

            beginTest ("Memory-mapped files");

            {
                MemoryMappedFile mmf (tempFile, MemoryMappedFile::readOnly);
                expect (mmf.getSize() == 10);
                expect (mmf.getData() != nullptr);
                expect (memcmp (mmf.getData(), "0123456789", 10) == 0);
            }

            {
                const File tempFile2 (tempFile.getNonexistentSibling (false));
                expect (tempFile2.create());
                expect (tempFile2.appendData ("xxxxxxxxxx", 10));

                {
                    MemoryMappedFile mmf (tempFile2, MemoryMappedFile::readWrite);
                    expect (mmf.getSize() == 10);
                    expect (mmf.getData() != nullptr);
                    memcpy (mmf.getData(), "abcdefghij", 10);
                }

                {
                    MemoryMappedFile mmf (tempFile2, MemoryMappedFile::readWrite);
                    expect (mmf.getSize() == 10);
                    expect (mmf.getData() != nullptr);
                    expect (memcmp (mmf.getData(), "abcdefghij", 10) == 0);
                }

                expect (tempFile2.deleteFile());
            }

            beginTest ("More writing");

            expect (tempFile.appendData ("abcdefghij", 10));
            expect (tempFile.getSize() == 20);
            expect (tempFile.replaceWithData ("abcdefghij", 10));
            expect (tempFile.getSize() == 10);

            File tempFile2 (tempFile.getNonexistentSibling (false));
            expect (tempFile.copyFileTo (tempFile2));
            expect (tempFile2.exists());
            expect (tempFile2.hasIdenticalContentTo (tempFile));
            expect (tempFile.deleteFile());
            expect (! tempFile.exists());
            expect (tempFile2.moveFileTo (tempFile));
            expect (tempFile.exists());
            expect (! tempFile2.exists());

            expect (demoFolder.deleteRecursively());
            expect (! demoFolder.exists());

            {
                Url url ("https://audio.dev/foo/bar/");
                expectEquals (url.toString (false), String ("https://audio.dev/foo/bar/"));
                expectEquals (url.getChildURL ("x").toString (false), String ("https://audio.dev/foo/bar/x"));
                expectEquals (url.getParentURL().toString (false), String ("https://audio.dev/foo"));
                expectEquals (url.getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/foo/x"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/x"));
            }

            {
                Url url ("https://audio.dev/foo/bar");
                expectEquals (url.toString (false), String ("https://audio.dev/foo/bar"));
                expectEquals (url.getChildURL ("x").toString (false), String ("https://audio.dev/foo/bar/x"));
                expectEquals (url.getParentURL().toString (false), String ("https://audio.dev/foo"));
                expectEquals (url.getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/foo/x"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/x"));
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static FileTests fileUnitTests;
    */
}
