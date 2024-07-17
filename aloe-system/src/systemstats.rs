crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/system/aloe_SystemStats.h]

#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ALOE_VERSION_PRINTING))]
pub struct AloeVersionPrinter {

}

#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ALOE_VERSION_PRINTING))]
impl Default for AloeVersionPrinter {
    
    fn default() -> Self {
        todo!();
        /*


            DBG (SystemStats::getALOEVersion())
        */
    }
}

#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ALOE_VERSION_PRINTING))]
lazy_static!{
    /*
    static AloeVersionPrinter aloeVersionPrinter;
    */
}

/**
  | Contains methods for finding out about
  | the current hardware and OS configuration.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct SystemStats {


}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/system/aloe_SystemStats.cpp]
impl SystemStats {

    /**
      | Returns the type of operating system
      | we're running on.
      | 
      | -----------
      | @return
      | 
      | one of the values from the OperatingSystemType
      | enum. @see getOperatingSystemName
      |
      */
    pub fn get_operating_system_type() -> OperatingSystemType {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the name of the type of operating
      | system we're running on.
      | 
      | -----------
      | @return
      | 
      | a string describing the OS type. @see
      | getOperatingSystemType
      |
      */
    pub fn get_operating_system_name() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if the OS is 64-bit, or false
      | for a 32-bit OS.
      |
      */
    pub fn is_operating_system_64bit() -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns an environment variable.
      | 
      | If the named value isn't set, this will
      | return the defaultValue string instead.
      |
      */
    pub fn get_environment_variable(
        name:          &String,
        default_value: &String) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the current user's name, if
      | available.
      | 
      | @see getFullUserName()
      |
      */
    pub fn get_logon_name() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the current user's full name,
      | if available.
      | 
      | On some OSes, this may just return the
      | same value as getLogonName().
      | 
      | @see getLogonName()
      |
      */
    pub fn get_full_user_name() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the host-name of the computer.
      |
      */
    pub fn get_computer_name() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the language of the user's locale.
      | 
      | The return value is a 2 or 3 letter language
      | code (ISO 639-1 or ISO 639-2)
      |
      */
    pub fn get_user_language() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the region of the user's locale.
      | 
      | The return value is a 2 letter country
      | code (ISO 3166-1 alpha-2).
      |
      */
    pub fn get_user_region() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the user's display language.
      | 
      | The return value is a 2 or 3 letter language
      | code (ISO 639-1 or ISO 639-2).
      | 
      | -----------
      | @note
      | 
      | depending on the OS and region, this
      | may also be followed by a dash and a sub-region
      | code, e.g "en-GB"
      |
      */
    pub fn get_display_language() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | This will attempt to return some kind
      | of string describing the device.
      | 
      | If no description is available, it'll
      | just return an empty string. You may
      | want to use this for things like determining
      | the type of phone/iPad, etc.
      |
      */
    pub fn get_device_description() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | This will attempt to return the manufacturer
      | of the device.
      | 
      | If no description is available, it'll
      | just return an empty string.
      |
      */
    pub fn get_device_manufacturer() -> String {
        
        todo!();
        /*
        
        */
    }

    /* --------- CPU and memory information..   --------- */

    /**
      | Returns the approximate CPU speed.
      | 
      | -----------
      | @return
      | 
      | the speed in megahertz, e.g. 1500, 2500,
      | 32000 (depending on what year you're
      | reading this...)
      |
      */
    pub fn get_cpu_speed_in_megahertz() -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns a string to indicate the CPU
      | vendor.
      | 
      | Might not be known on some systems.
      |
      */
    pub fn get_cpu_vendor() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Attempts to return a string describing
      | the CPU model.
      | 
      | May not be available on some systems.
      |
      */
    pub fn get_cpu_model() -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Finds out how much RAM is in the machine.
      | 
      | -----------
      | @return
      | 
      | the approximate number of megabytes
      | of memory, or zero if something goes
      | wrong when finding out.
      |
      */
    pub fn get_memory_size_in_megabytes() -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the system page-size.
      | 
      | This is only used by programmers with
      | beards.
      |
      */
    pub fn get_page_size() -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the current version of Aloe,
      | See also the ALOE_VERSION, ALOE_MAJOR_VERSION
      | and ALOE_MINOR_VERSION macros.
      |
      */
    pub fn get_aloe_version(&mut self) -> String {
        
        todo!();
        /*
            // Some basic tests, to keep an eye on things and make sure these types work ok
        // on all platforms. Let me know if any of these assertions fail on your system!
        static_assert (sizeof (pointer_sized_int) == sizeof (void*), "Basic sanity test failed: please report!");
        static_assert (sizeof (int8) == 1,                           "Basic sanity test failed: please report!");
        static_assert (sizeof (uint8) == 1,                          "Basic sanity test failed: please report!");
        static_assert (sizeof (int16) == 2,                          "Basic sanity test failed: please report!");
        static_assert (sizeof (uint16) == 2,                         "Basic sanity test failed: please report!");
        static_assert (sizeof (int32) == 4,                          "Basic sanity test failed: please report!");
        static_assert (sizeof (uint32) == 4,                         "Basic sanity test failed: please report!");
        static_assert (sizeof (int64) == 8,                          "Basic sanity test failed: please report!");
        static_assert (sizeof (uint64) == 8,                         "Basic sanity test failed: please report!");

        return "Aloe v" ALOE_STRINGIFY(ALOE_MAJOR_VERSION)
                    "." ALOE_STRINGIFY(ALOE_MINOR_VERSION)
                    "." ALOE_STRINGIFY(ALOE_BUILDNUMBER);
        */
    }
    
    /**
      | This method calculates some IDs to uniquely
      | identify the device.
      | 
      | The first choice for an ID is a filesystem
      | ID for the user's home folder or windows
      | directory. If that fails then this function
      | returns the MAC addresses.
      |
      */
    pub fn get_device_identifiers(&mut self) -> Vec<String> {
        
        todo!();
        /*
            StringArray ids;

       #if ALOE_WINDOWS
        File f (File::getSpecialLocation (File::windowsSystemDirectory));
       #else
        File f ("~");
       #endif
        if (auto num = f.getFileIdentifier())
        {
            ids.add (String::toHexString ((int64) num));
        }
        else
        {
            for (auto& address : MACAddress::getAllAddresses())
                ids.add (address.toString());
        }

        jassert (! ids.isEmpty()); // Failed to create any IDs!
        return ids;
        */
    }
    
    /**
      | Returns the number of logical CPU cores.
      |
      */
    pub fn get_num_cpus(&mut self) -> i32 {
        
        todo!();
        /*
            return getCPUInformation().numLogicalCPUs;
        */
    }
    
    /**
      | Returns the number of physical CPU cores.
      |
      */
    pub fn get_num_physical_cpus(&mut self) -> i32 {
        
        todo!();
        /*
            return getCPUInformation().numPhysicalCPUs;
        */
    }
    
    /**
      | Returns true if Intel MMX instructions
      | are available.
      |
      */
    pub fn hasmmx(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasMMX;
        */
    }
    
    /**
      | Returns true if AMD 3DNOW instructions
      | are available.
      |
      */
    pub fn has3d_now(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().has3DNow;
        */
    }
    
    /**
      | Returns true if AMD FMA3 instructions
      | are available.
      |
      */
    pub fn hasfma3(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasFMA3;
        */
    }
    
    /**
      | Returns true if AMD FMA4 instructions
      | are available.
      |
      */
    pub fn hasfma4(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasFMA4;
        */
    }
    
    /**
      | Returns true if Intel SSE instructions
      | are available.
      |
      */
    pub fn hassse(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasSSE;
        */
    }
    
    /**
       Returns true if Intel SSE2 instructions are
       available.
      */
    pub fn hassse2(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasSSE2;
        */
    }
    
    /**
       Returns true if Intel SSE3 instructions are
       available.
      */
    pub fn hassse3(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasSSE3;
        */
    }
    
    /**
      | Returns true if Intel SSSE3 instructions
      | are available.
      |
      */
    pub fn hasssse3(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasSSSE3;
        */
    }
    
    /**
      | Returns true if Intel SSE4.1 instructions
      | are available.
      |
      */
    pub fn hassse41(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasSSE41;
        */
    }
    
    /**
      | Returns true if Intel SSE4.2 instructions
      | are available.
      |
      */
    pub fn hassse42(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasSSE42;
        */
    }
    
    /**
       Returns true if Intel AVX instructions are
       available.
      */
    pub fn hasavx(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX;
        */
    }
    
    /**
       Returns true if Intel AVX2 instructions are
       available.
      */
    pub fn hasavx2(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX2;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Foundation
       instructions are available.
      */
    pub fn hasavx512f(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512F;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Byte and Word
       instructions are available.
      */
    pub fn hasavx512bw(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512BW;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Conflict
       Detection instructions are available.
      */
    pub fn hasavx512cd(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512CD;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Doubleword
       and Quadword instructions are available.
      */
    pub fn hasavx512dq(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512DQ;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Exponential
       and Reciprocal instructions are available.
      */
    pub fn hasavx512er(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512ER;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Integer Fused
       Multiply-Add instructions are available.
      */
    pub fn hasavx512ifma(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512IFMA;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Prefetch
       instructions are available.
      */
    pub fn hasavx512pf(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512PF;
        */
    }
    
    /**
      | Returns true if Intel AVX-512 Vector
      | Bit Manipulation instructions are available.
      |
      */
    pub fn hasavx512vbmi(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512VBMI;
        */
    }
    
    /**
       Returns true if Intel AVX-512 Vector Length
       instructions are available.
      */
    pub fn hasavx512vl(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512VL;
        */
    }
    
    /**
      | Returns true if Intel AVX-512 Vector
      | 
      | Population Count Double and Quad-word
      | instructions are available.
      |
      */
    pub fn hasavx512vpopcntdq(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasAVX512VPOPCNTDQ;
        */
    }
    
    /**
       Returns true if ARM NEON instructions are
       available.
      */
    pub fn has_neon(&mut self) -> bool {
        
        todo!();
        /*
            return getCPUInformation().hasNeon;
        */
    }
    
    /**
      | Returns a backtrace of the current call-stack.
      | 
      | The usefulness of the result will depend
      | on the level of debug symbols that are
      | available in the executable.
      |
      */
    pub fn get_stack_backtrace(&mut self) -> String {
        
        todo!();
        /*
            String result;

       #if ALOE_ANDROID || ALOE_MINGW || ALOE_WASM
        jassertfalse; // sorry, not implemented yet!

       #elif ALOE_WINDOWS
        HANDLE process = GetCurrentProcess();
        SymInitialize (process, nullptr, TRUE);

        void* stack[128];
        int frames = (int) CaptureStackBackTrace (0, numElementsInArray (stack), stack, nullptr);

        HeapBlock<SYMBOL_INFO> symbol;
        symbol.calloc (sizeof (SYMBOL_INFO) + 256, 1);
        symbol->MaxNameLen = 255;
        symbol->SizeOfStruct = sizeof (SYMBOL_INFO);

        for (int i = 0; i < frames; ++i)
        {
            DWORD64 displacement = 0;

            if (SymFromAddr (process, (DWORD64) stack[i], &displacement, symbol))
            {
                result << i << ": ";

                IMAGEHLP_MODULE64 moduleInfo;
                zerostruct (moduleInfo);
                moduleInfo.SizeOfStruct = sizeof (moduleInfo);

                if (::SymGetModuleInfo64 (process, symbol->ModBase, &moduleInfo))
                    result << moduleInfo.ModuleName << ": ";

                result << symbol->Name << " + 0x" << String::toHexString ((int64) displacement) << newLine;
            }
        }

       #else
        void* stack[128];
        auto frames = backtrace (stack, numElementsInArray (stack));
        char** frameStrings = backtrace_symbols (stack, frames);

        for (int i = 0; i < frames; ++i)
            result << frameStrings[i] << newLine;

        ::free (frameStrings);
       #endif

        return result;
        */
    }
    
    /**
      | Sets up a global callback function that
      | will be called if the application executes
      | some kind of illegal instruction.
      | 
      | You may want to call getStackBacktrace()
      | in your handler function, to find out
      | where the problem happened and log it,
      | etc.
      |
      */
    #[cfg(not(ALOE_WASM))]
    pub fn set_application_crash_handler(
        &mut self, 
        handler: SystemStatsCrashHandlerFunction)
    {
        todo!();
        /*
            jassert (handler != nullptr); // This must be a valid function.
        globalCrashHandler = handler;

       #if ALOE_WINDOWS
        SetUnhandledExceptionFilter (handleCrash);
       #else
        const int signals[] = { SIGFPE, SIGILL, SIGSEGV, SIGBUS, SIGABRT, SIGSYS };

        for (int i = 0; i < numElementsInArray (signals); ++i)
        {
            ::signal (signals[i], handleCrash);
            aloe_siginterrupt (signals[i], 1);
        }
       #endif
        */
    }
    
    /**
      | Returns true if this code is running
      | inside an app extension sandbox.
      | 
      | This function will always return false
      | on windows, linux and android.
      |
      */
    pub fn is_running_in_app_extension_sandbox(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_MAC || ALOE_IOS
        static bool firstQuery = true;
        static bool isRunningInAppSandbox = false;

        if (firstQuery)
        {
            firstQuery = false;

            File bundle = File::getSpecialLocation (File::invokedExecutableFile).getParentDirectory();

           #if ALOE_MAC
            bundle = bundle.getParentDirectory().getParentDirectory();
           #endif

            if (bundle.isDirectory())
                isRunningInAppSandbox = (bundle.getFileExtension() == ".appex");
        }

        return isRunningInAppSandbox;
       #else
        return false;
       #endif
        */
    }
}

impl SystemStats {
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_operating_system_type(&mut self) -> system_stats::OperatingSystemType {
        
        todo!();
        /*
            return WASM;
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_operating_system_name(&mut self) -> String {
        
        todo!();
        /*
            return "WASM";
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn is_operating_system_64bit(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_device_description(&mut self) -> String {
        
        todo!();
        /*
            return "Web-browser";
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_device_manufacturer(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_cpu_vendor(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_cpu_model(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_cpu_speed_in_megahertz(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_memory_size_in_megabytes(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_page_size(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_logon_name(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_full_user_name(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_computer_name(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_user_language(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_user_region(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn get_display_language(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
}

impl SystemStats {
    
    #[cfg(target_os="linux")]
    pub fn get_operating_system_type(&mut self) -> system_stats::OperatingSystemType {
        
        todo!();
        /*
            return Linux;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_operating_system_name(&mut self) -> String {
        
        todo!();
        /*
            return "Linux";
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn is_operating_system_64bit(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_64BIT
        return true;
       #else
        //xxx not sure how to find this out?..
        return false;
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_device_description(&mut self) -> String {
        
        todo!();
        /*
            #if ALOE_BSD
        int mib[] = {
            CTL_HW,
            HW_MACHINE
        };
        size_t machineDescriptionLength = 0;
        auto result = sysctl (mib, numElementsInArray (mib), nullptr, &machineDescriptionLength, nullptr, 0);

        if (result != 0 || machineDescriptionLength == 0)
            return {};

        MemoryBlock machineDescription { machineDescriptionLength };
        result = sysctl (mib, numElementsInArray (mib), machineDescription.getData(), &machineDescriptionLength, nullptr, 0);
        return String::fromUTF8 (result == 0 ? (char*) machineDescription.getData() : "");
       #else
        return getCpuInfo ("Hardware");
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_device_manufacturer(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_cpu_vendor(&mut self) -> String {
        
        todo!();
        /*
            #if ALOE_BSD
        return {};
       #else
        auto v = getCpuInfo ("vendor_id");

        if (v.isEmpty())
            v = getCpuInfo ("model name");

        return v;
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_cpu_model(&mut self) -> String {
        
        todo!();
        /*
            #if ALOE_BSD
        int mib[] = {
            CTL_HW,
            HW_MODEL
        };
        size_t modelLength = 0;
        auto result = sysctl (mib, numElementsInArray (mib), nullptr, &modelLength, nullptr, 0);

        if (result != 0 || modelLength == 0)
            return {};

        MemoryBlock model { modelLength };
        result = sysctl (mib, numElementsInArray (mib), model.getData(), &modelLength, nullptr, 0);
        return String::fromUTF8 (result == 0 ? (char*) model.getData() : "");
       #else
        return getCpuInfo ("model name");
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_cpu_speed_in_megahertz(&mut self) -> i32 {
        
        todo!();
        /*
            #if ALOE_BSD
        int32 clockRate = 0;
        auto clockRateSize = sizeof (clockRate);
        auto result = sysctlbyname ("hw.clockrate", &clockRate, &clockRateSize, nullptr, 0);
        return result == 0 ? clockRate : 0;
       #else
        return roundToInt (getCpuInfo ("cpu MHz").getFloatValue());
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_memory_size_in_megabytes(&mut self) -> i32 {
        
        todo!();
        /*
            #if ALOE_BSD
        int mib[] = {
            CTL_HW,
            HW_PHYSMEM
        };
        int64 memory = 0;
        auto memorySize = sizeof (memory);
        auto result = sysctl (mib, numElementsInArray (mib), &memory, &memorySize, nullptr, 0);
        return result == 0 ? (int) (memory / 1e6) : 0;
       #else
        struct sysinfo sysi;

        if (sysinfo (&sysi) == 0)
            return (int) (sysi.totalram * sysi.mem_unit / (1024 * 1024));

        return 0;
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_page_size(&mut self) -> i32 {
        
        todo!();
        /*
            return (int) sysconf (_SC_PAGESIZE);
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_logon_name(&mut self) -> String {
        
        todo!();
        /*
            if (auto user = getenv ("USER"))
            return String::fromUTF8 (user);

        if (auto pw = getpwuid (getuid()))
            return String::fromUTF8 (pw->pw_name);

        return {};
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_full_user_name(&mut self) -> String {
        
        todo!();
        /*
            return getLogonName();
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_computer_name(&mut self) -> String {
        
        todo!();
        /*
            char name[256] = {};

        if (gethostname (name, sizeof (name) - 1) == 0)
            return name;

        return {};
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_user_language(&mut self) -> String {
        
        todo!();
        /*
            #if ALOE_BSD
        if (auto langEnv = getenv ("LANG"))
            return String::fromUTF8 (langEnv).upToLastOccurrenceOf (".UTF-8", false, true);

        return {};
       #else
        return getLocaleValue (_NL_IDENTIFICATION_LANGUAGE);
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_user_region(&mut self) -> String {
        
        todo!();
        /*
            #if ALOE_BSD
        return {};
       #else
        return getLocaleValue (_NL_IDENTIFICATION_TERRITORY);
       #endif
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_display_language(&mut self) -> String {
        
        todo!();
        /*
            auto result = getUserLanguage();
        auto region = getUserRegion();

        if (region.isNotEmpty())
            result << "-" << region;

        return result;
        */
    }
}

impl SystemStats {
    
    #[cfg(feature = "aloe_posix")]
    pub fn get_environment_variable(&mut self, 
        name:          &String,
        default_value: &String) -> String {
        
        todo!();
        /*
            if (auto s = ::getenv (name.toUTF8()))
            return String::fromUTF8 (s);

        return defaultValue;
        */
    }
}


///------------------------
pub struct CPUInformation {
    num_logical_cp_us:  i32, // default = 0
    num_physical_cp_us: i32, // default = 0
    hasmmx:             bool, // default = false
    hassse:             bool, // default = false
    hassse2:            bool, // default = false
    hassse3:            bool, // default = false
    has3d_now:          bool, // default = false
    hasfma3:            bool, // default = false
    hasfma4:            bool, // default = false
    hasssse3:           bool, // default = false
    hassse41:           bool, // default = false
    hassse42:           bool, // default = false
    hasavx:             bool, // default = false
    hasavx2:            bool, // default = false
    hasavx512f:         bool, // default = false
    hasavx512bw:        bool, // default = false
    hasavx512cd:        bool, // default = false
    hasavx512dq:        bool, // default = false
    hasavx512er:        bool, // default = false
    hasavx512ifma:      bool, // default = false
    hasavx512pf:        bool, // default = false
    hasavx512vbmi:      bool, // default = false
    hasavx512vl:        bool, // default = false
    hasavx512vpopcntdq: bool, // default = false
    has_neon:           bool, // default = false
}

impl Default for CPUInformation {
    
    fn default() -> Self {
        todo!();
        /*


            initialise()
        */
    }
}

impl CPUInformation {
    
    #[cfg(target_arch = "wasm32")]
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            numLogicalCPUs = 1;
        numPhysicalCPUs = 1;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            #if ALOE_BSD
       #if ALOE_INTEL && ! ALOE_NO_INLINE_ASM
        SystemStatsHelpers::getCPUInfo (hasMMX,
                                        hasSSE,
                                        hasSSE2,
                                        has3DNow,
                                        hasSSE3,
                                        hasSSSE3,
                                        hasFMA3,
                                        hasSSE41,
                                        hasSSE42,
                                        hasAVX,
                                        hasFMA4,
                                        hasAVX2,
                                        hasAVX512F,
                                        hasAVX512DQ,
                                        hasAVX512IFMA,
                                        hasAVX512PF,
                                        hasAVX512ER,
                                        hasAVX512CD,
                                        hasAVX512BW,
                                        hasAVX512VL,
                                        hasAVX512VBMI,
                                        hasAVX512VPOPCNTDQ);
       #endif

        numLogicalCPUs = numPhysicalCPUs = []
        {
            int mib[] = {
                CTL_HW,
                HW_NCPU
            };
            int32 numCPUs = 1;
            auto numCPUsSize = sizeof (numCPUs);
            auto result = sysctl (mib, numElementsInArray (mib), &numCPUs, &numCPUsSize, nullptr, 0);
            return result == 0 ? numCPUs : 1;
        }();
      #else
        auto flags = getCpuInfo ("flags");

        hasMMX             = flags.contains ("mmx");
        hasFMA3            = flags.contains ("fma");
        hasFMA4            = flags.contains ("fma4");
        hasSSE             = flags.contains ("sse");
        hasSSE2            = flags.contains ("sse2");
        hasSSE3            = flags.contains ("sse3");
        has3DNow           = flags.contains ("3dnow");
        hasSSSE3           = flags.contains ("ssse3");
        hasSSE41           = flags.contains ("sse4_1");
        hasSSE42           = flags.contains ("sse4_2");
        hasAVX             = flags.contains ("avx");
        hasAVX2            = flags.contains ("avx2");
        hasAVX512F         = flags.contains ("avx512f");
        hasAVX512BW        = flags.contains ("avx512bw");
        hasAVX512CD        = flags.contains ("avx512cd");
        hasAVX512DQ        = flags.contains ("avx512dq");
        hasAVX512ER        = flags.contains ("avx512er");
        hasAVX512IFMA      = flags.contains ("avx512ifma");
        hasAVX512PF        = flags.contains ("avx512pf");
        hasAVX512VBMI      = flags.contains ("avx512vbmi");
        hasAVX512VL        = flags.contains ("avx512vl");
        hasAVX512VPOPCNTDQ = flags.contains ("avx512_vpopcntdq");

        numLogicalCPUs  = getCpuInfo ("processor").getIntValue() + 1;

        // Assume CPUs in all sockets have the same number of cores
        numPhysicalCPUs = getCpuInfo ("cpu cores").getIntValue() * (getCpuInfo ("physical id").getIntValue() + 1);

        if (numPhysicalCPUs <= 0)
            numPhysicalCPUs = numLogicalCPUs;
      #endif
        */
    }
    
    #[cfg(target_os="android")]
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            numPhysicalCPUs = numLogicalCPUs = jmax ((int) 1, (int) android_getCpuCount());

        auto cpuFamily   = android_getCpuFamily();
        auto cpuFeatures = android_getCpuFeatures();

        if (cpuFamily == ANDROID_CPU_FAMILY_X86 || cpuFamily == ANDROID_CPU_FAMILY_X86_64)
        {
            hasMMX = hasSSE = hasSSE2 = (cpuFamily == ANDROID_CPU_FAMILY_X86_64);

            hasSSSE3 = ((cpuFeatures & ANDROID_CPU_X86_FEATURE_SSSE3)  != 0);
            hasSSE41 = ((cpuFeatures & ANDROID_CPU_X86_FEATURE_SSE4_1) != 0);
            hasSSE42 = ((cpuFeatures & ANDROID_CPU_X86_FEATURE_SSE4_2) != 0);
            hasAVX   = ((cpuFeatures & ANDROID_CPU_X86_FEATURE_AVX)    != 0);
            hasAVX2  = ((cpuFeatures & ANDROID_CPU_X86_FEATURE_AVX2)   != 0);

            // Google does not distinguish between MMX, SSE, SSE2, SSE3 and SSSE3. So
            // I assume (and quick Google searches seem to confirm this) that there are
            // only devices out there that either support all of this or none of this.
            if (hasSSSE3)
                hasMMX = hasSSE = hasSSE2 = hasSSE3 = true;
        }
        else if (cpuFamily == ANDROID_CPU_FAMILY_ARM)
        {
            hasNeon = ((cpuFeatures & ANDROID_CPU_ARM_FEATURE_NEON) != 0);
        }
        else if (cpuFamily == ANDROID_CPU_FAMILY_ARM64)
        {
            // all arm 64-bit cpus have neon
            hasNeon = true;
        }
        */
    }
}




impl CPUInformation {
    
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

pub fn get_cpu_information() -> &'static CPUInformation {
    
    todo!();
    /*
        static CPUInformation info;
        return info;
    */
}

#[cfg(target_os="linux")]
#[cfg(not(target_os="bsd"))]
pub fn get_cpu_info(key: *const u8) -> String {
    
    todo!();
    /*
        return readPosixConfigFileValue ("/proc/cpuinfo", key);
    */
}

#[cfg(not(ALOE_WASM))]
lazy_static!{
    /*
    static SystemStats::CrashHandlerFunction globalCrashHandler = nullptr;
    */
}

#[cfg(not(ALOE_WASM))]
#[cfg(target_os="windows")]
pub fn handle_crash(ep: LPEXCEPTION_POINTERS) -> LONG {
    
    todo!();
    /*
        globalCrashHandler (ep);
        return EXCEPTION_EXECUTE_HANDLER;
    */
}

#[cfg(not(ALOE_WASM))]
#[cfg(not(target_os="windows"))]
pub fn handle_crash(signum: i32)  {
    
    todo!();
    /*
        globalCrashHandler ((void*) (pointer_sized_int) signum);
        ::kill (getpid(), SIGKILL);
    */
}

#[cfg(not(ALOE_WASM))]
#[cfg(not(target_os="windows"))]
pub fn aloe_siginterrupt(
        sig:  i32,
        flag: i32) -> i32 {
    
    todo!();
    /*
    
    */
}

/**
  | The unix siginterrupt function is deprecated
  | 
  | - this does the same job.
  |
  */
#[cfg(feature = "aloe_posix")]
pub fn aloe_siginterrupt(
    sig:  i32,
    flag: i32) -> i32 {
    
    todo!();
    /*
        #if ALOE_WASM
        ignoreUnused (sig, flag);
        return 0;
       #else
        #if ALOE_ANDROID
         using aloe_sigactionflags_type = unsigned long;
        #else
         using aloe_sigactionflags_type = int;
        #endif

        struct ::sigaction act;
        (void) ::sigaction (sig, nullptr, &act);

        if (flag != 0)
            act.sa_flags &= static_cast<aloe_sigactionflags_type> (~SA_RESTART);
        else
            act.sa_flags |= static_cast<aloe_sigactionflags_type> (SA_RESTART);

        return ::sigaction (sig, &act, nullptr);
       #endif
    */
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_wasm_SystemStats.cpp]
#[cfg(target_arch = "wasm32")]
pub fn aloe_milliseconds_since_startup() -> u32 {
    
    todo!();
    /*
        return static_cast<uint32> (emscripten_get_now());
    */
}

#[cfg(target_arch = "wasm32")]
pub fn aloe_is_running_under_debugger() -> bool {
    
    todo!();
    /*
        return false;
    */
}

#[cfg(target_os="linux")]
pub fn aloe_is_running_under_debugger() -> bool {
    
    todo!();
    /*
        #if ALOE_BSD
        int mib[] =
        {
            CTL_KERN,
            KERN_PROC,
            KERN_PROC_PID,
            ::getpid()
        };
        struct kinfo_proc info;
        auto infoSize = sizeof (info);
        auto result = sysctl (mib, numElementsInArray (mib), &info, &infoSize, nullptr, 0);
        return result == 0 ? ((info.ki_flag & P_TRACED) != 0) : false;
       #else
        return readPosixConfigFileValue ("/proc/self/status", "TracerPid").getIntValue() > 0;
       #endif
    */
}
