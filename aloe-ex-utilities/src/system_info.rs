crate::ix!();

pub fn get_all_system_info() -> String {
    
    todo!();
    /*
        String systemInfo;

        systemInfo
          << "Here are a few system statistics..." << newLine
          << newLine
          << "Time and date:    " << Time::getCurrentTime().toString (true, true) << newLine
          << "System up-time:   " << RelativeTime::milliseconds ((int64) Time::getMillisecondCounterHiRes()).getDescription() << newLine
          << "Compilation date: " << Time::getCompilationDate().toString (true, false) << newLine
          << newLine
          << "Operating system: " << SystemStats::getOperatingSystemName() << newLine
          << "Host name:        " << SystemStats::getComputerName()        << newLine
          << "Device type:      " << SystemStats::getDeviceDescription()   << newLine
          << "Manufacturer:     " << SystemStats::getDeviceManufacturer()  << newLine
          << "User logon name:  " << SystemStats::getLogonName()           << newLine
          << "Full user name:   " << SystemStats::getFullUserName()        << newLine
          << "User region:      " << SystemStats::getUserRegion()          << newLine
          << "User language:    " << SystemStats::getUserLanguage()        << newLine
          << "Display language: " << SystemStats::getDisplayLanguage()     << newLine
          << newLine;

        systemInfo
          << "Number of logical CPUs:  " << SystemStats::getNumCpus() << newLine
          << "Number of physical CPUs: " << SystemStats::getNumPhysicalCpus() << newLine
          << "Memory size:             " << SystemStats::getMemorySizeInMegabytes() << " MB" << newLine
          << "CPU vendor:              " << SystemStats::getCpuVendor() << newLine
          << "CPU model:               " << SystemStats::getCpuModel()  << newLine
          << "CPU speed:               " << SystemStats::getCpuSpeedInMegahertz() << " MHz" << newLine
          << "CPU has MMX:             " << (SystemStats::hasMMX()             ? "yes" : "no") << newLine
          << "CPU has FMA3:            " << (SystemStats::hasFMA3()            ? "yes" : "no") << newLine
          << "CPU has FMA4:            " << (SystemStats::hasFMA4()            ? "yes" : "no") << newLine
          << "CPU has SSE:             " << (SystemStats::hasSSE()             ? "yes" : "no") << newLine
          << "CPU has SSE2:            " << (SystemStats::hasSSE2()            ? "yes" : "no") << newLine
          << "CPU has SSE3:            " << (SystemStats::hasSSE3()            ? "yes" : "no") << newLine
          << "CPU has SSSE3:           " << (SystemStats::hasSSSE3()           ? "yes" : "no") << newLine
          << "CPU has SSE4.1:          " << (SystemStats::hasSSE41()           ? "yes" : "no") << newLine
          << "CPU has SSE4.2:          " << (SystemStats::hasSSE42()           ? "yes" : "no") << newLine
          << "CPU has 3DNOW:           " << (SystemStats::has3DNow()           ? "yes" : "no") << newLine
          << "CPU has AVX:             " << (SystemStats::hasAVX()             ? "yes" : "no") << newLine
          << "CPU has AVX2:            " << (SystemStats::hasAVX2()            ? "yes" : "no") << newLine
          << "CPU has AVX512F:         " << (SystemStats::hasAVX512F()         ? "yes" : "no") << newLine
          << "CPU has AVX512BW:        " << (SystemStats::hasAVX512BW()        ? "yes" : "no") << newLine
          << "CPU has AVX512CD:        " << (SystemStats::hasAVX512CD()        ? "yes" : "no") << newLine
          << "CPU has AVX512DQ:        " << (SystemStats::hasAVX512DQ()        ? "yes" : "no") << newLine
          << "CPU has AVX512ER:        " << (SystemStats::hasAVX512ER()        ? "yes" : "no") << newLine
          << "CPU has AVX512IFMA:      " << (SystemStats::hasAVX512IFMA()      ? "yes" : "no") << newLine
          << "CPU has AVX512PF:        " << (SystemStats::hasAVX512PF()        ? "yes" : "no") << newLine
          << "CPU has AVX512VBMI:      " << (SystemStats::hasAVX512VBMI()      ? "yes" : "no") << newLine
          << "CPU has AVX512VL:        " << (SystemStats::hasAVX512VL()        ? "yes" : "no") << newLine
          << "CPU has AVX512VPOPCNTDQ: " << (SystemStats::hasAVX512VPOPCNTDQ() ? "yes" : "no") << newLine
          << "CPU has Neon:            " << (SystemStats::hasNeon()            ? "yes" : "no") << newLine
          << newLine;

        systemInfo
          << "Current working directory:  " << File::getCurrentWorkingDirectory().getFullPathName() << newLine
          << "Current application file:   " << File::getSpecialLocation (File::currentApplicationFile).getFullPathName() << newLine
          << "Current executable file:    " << File::getSpecialLocation (File::currentExecutableFile) .getFullPathName() << newLine
          << "Invoked executable file:    " << File::getSpecialLocation (File::invokedExecutableFile) .getFullPathName() << newLine
          << newLine;

        systemInfo
          << "User home folder:               " << File::getSpecialLocation (File::userHomeDirectory)             .getFullPathName() << newLine
          << "User desktop folder:            " << File::getSpecialLocation (File::userDesktopDirectory)          .getFullPathName() << newLine
          << "User documents folder:          " << File::getSpecialLocation (File::userDocumentsDirectory)        .getFullPathName() << newLine
          << "User application data folder:   " << File::getSpecialLocation (File::userApplicationDataDirectory)  .getFullPathName() << newLine
          << "User music folder:              " << File::getSpecialLocation (File::userMusicDirectory)            .getFullPathName() << newLine
          << "User movies folder:             " << File::getSpecialLocation (File::userMoviesDirectory)           .getFullPathName() << newLine
          << "User pictures folder:           " << File::getSpecialLocation (File::userPicturesDirectory)         .getFullPathName() << newLine
          << "Common application data folder: " << File::getSpecialLocation (File::commonApplicationDataDirectory).getFullPathName() << newLine
          << "Common documents folder:        " << File::getSpecialLocation (File::commonDocumentsDirectory)      .getFullPathName() << newLine
          << "Local temp folder:              " << File::getSpecialLocation (File::tempDirectory)                 .getFullPathName() << newLine
          << newLine;

        systemInfo
          << "File System roots: "          << getFileSystemRoots() << newLine
          << "Free space in home folder: "  << File::descriptionOfSizeInBytes (File::getSpecialLocation (File::userHomeDirectory)
                                                                                    .getBytesFreeOnVolume()) << newLine
          << newLine
          << getDisplayInfo() << newLine
          << "Network IP addresses: "       << newLine << getIPAddressList()  << newLine
          << "Network card MAC addresses: " << newLine << getMacAddressList() << newLine;

        DBG (systemInfo);
        return systemInfo;
    */
}
