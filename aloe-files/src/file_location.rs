crate::ix!();

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

