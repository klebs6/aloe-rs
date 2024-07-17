crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_posix_SharedCode.h]

#[cfg(feature = "aloe_posix")]
#[cfg(any(any(target_os="macos",target_os="linux"),target_os="bsd"))]
pub struct MaxNumFileHandlesInitialiser {

}

#[cfg(feature = "aloe_posix")]
#[cfg(any(any(target_os="macos",target_os="linux"),target_os="bsd"))]
impl Default for MaxNumFileHandlesInitialiser {
    
    fn default() -> Self {
        todo!();
        /*


            #ifndef ALOE_PREFERRED_MAX_FILE_HANDLES
            enum { ALOE_PREFERRED_MAX_FILE_HANDLES = 8192 };
           #endif

            // Try to give our app a decent number of file handles by default
            if (! Process::setMaxNumberOfFileHandles (0))
            {
                for (int num = ALOE_PREFERRED_MAX_FILE_HANDLES; num > 256; num -= 1024)
                    if (Process::setMaxNumberOfFileHandles (num))
                        break;
            
        */
    }
}

#[cfg(feature = "aloe_posix")]
lazy_static!{
    /*
    #if ALOE_LINUX || (ALOE_IOS && ! __DARWIN_ONLY_64_BIT_INO_T) // (this iOS stuff is to avoid a simulator bug)
    using aloe_statStruct = struct stat64;
    #define ALOE_STAT  stat64
    #else
    using aloe_statStruct = struct stat;
    #define ALOE_STAT  stat
    #endif
    */
}

#[cfg(feature = "aloe_posix")]
pub fn aloe_stat(
    file_name: &String,
    info:      &mut aloe_statStruct) -> bool {

    todo!();
    /*
        return fileName.isNotEmpty()
                 && ALOE_STAT (fileName.toUTF8(), &info) == 0;
    */
}

/**
  | if this file doesn't exist, find a parent
  | of it that does..
  |
  */
#[cfg(feature = "aloe_posix")]
#[cfg(not(ALOE_WASM))]
pub fn aloe_do_statfs(
    f:      File,
    result: &mut StatFs) -> bool {
    
    todo!();
    /*
        for (int i = 5; --i >= 0;)
        {
            if (f.exists())
                break;

            f = f.getParentDirectory();
        }

        return statfs (f.getFullPathName().toUTF8(), &result) == 0;
    */
}

#[cfg(feature = "aloe_posix")]
#[cfg(not(ALOE_WASM))]
#[cfg(any(target_os="macos",target_os="ios"))]
pub fn get_creation_time(s: &aloe_statStruct) -> i64 {
    
    todo!();
    /*
        return (int64) s.st_birthtime;
    */
}

#[cfg(feature = "aloe_posix")]
#[cfg(not(ALOE_WASM))]
#[cfg(not(any(target_os="macos",target_os="ios")))]
pub fn get_creation_time(s: &aloe_statStruct) -> i64 {
    
    todo!();
    /*
        return (int64) s.st_ctime;
    */
}


#[cfg(feature = "aloe_posix")]
pub fn get_result_for_errno() -> Result {
    
    todo!();
    /*
        return Result::fail (String (strerror (errno)));
    */
}

#[cfg(feature = "aloe_posix")]
pub fn get_result_for_return_value(value: i32) -> Result {
    
    todo!();
    /*
        return value == -1 ? getResultForErrno() : Result::ok();
    */
}

///-------------------------

///-------------------------------

#[cfg(feature = "aloe_posix")]
#[cfg(not(target_os="ios"))]
pub fn aloe_run_system_command(command: &String)  {
    
    todo!();
    /*
        int result = system (command.toUTF8());
        ignoreUnused (result);
    */
}

#[cfg(feature = "aloe_posix")]
#[cfg(not(target_os="ios"))]
pub fn aloe_get_output_from_command(command: &String) -> String {
    
    todo!();
    /*
        // slight bodge here, as we just pipe the output into a temp file and read it...
        auto tempFile = File::getSpecialLocation (File::tempDirectory)
                          .getNonexistentChildFile (String::toHexString (Random::getSystemRandom().nextInt()), ".tmp", false);

        aloe_runSystemCommand (command + " > " + tempFile.getFullPathName());

        auto result = tempFile.loadFileAsString();
        tempFile.deleteFile();
        return result;
    */
}

#[cfg(feature = "aloe_posix")]
#[cfg(target_os="android")]
lazy_static!{
    /*
    extern JavaVM* androidJNIJavaVM;
    */
}

#[cfg(feature = "aloe_posix")]
#[cfg(not(ALOE_WASM))]
#[cfg(any(target_os="linux",target_os="android"))]
pub fn read_posix_config_file_value(
        file: *const u8,
        key:  *const u8) -> String {
    
    todo!();
    /*
        StringArray lines;
        File (file).readLines (lines);

        for (int i = lines.size(); --i >= 0;) // (NB - it's important that this runs in reverse order)
            if (lines[i].upToFirstOccurrenceOf (":", false, false).trim().equalsIgnoreCase (key))
                return lines[i].fromFirstOccurrenceOf (":", false, false).trim();

        return {};
    */
}
