crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_android_SystemStats.cpp]

pub mod android_stats_helpers {
    use super::*;

    lazy_static!{
        /*
        #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
             STATICMETHOD (getProperty, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;")

            DECLARE_JNI_CLASS (SystemClass, "java/lang/System")
            #undef JNI_CLASS_MEMBERS

            #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
             STATICMETHOD (getDefault, "getDefault", "()Ljava/util/Locale;") \
             METHOD (getCountry, "getCountry", "()Ljava/lang/String;") \
             METHOD (getLanguage, "getLanguage", "()Ljava/lang/String;")

            DECLARE_JNI_CLASS (JavaLocale, "java/util/Locale")
            #undef JNI_CLASS_MEMBERS
        */
    }

    pub fn get_system_property(name: &String) -> String {
        
        todo!();
        /*
            return aloeString (LocalRef<jstring> ((jstring) getEnv()->CallStaticObjectMethod (SystemClass,
                                                                                                  SystemClass.getProperty,
                                                                                                  javaString (name).get())));
        */
    }

    pub fn get_locale_value(is_region: bool) -> String {
        
        todo!();
        /*
            auto* env = getEnv();
                LocalRef<jobject> locale (env->CallStaticObjectMethod (JavaLocale, JavaLocale.getDefault));

                auto stringResult = isRegion ? env->CallObjectMethod (locale.get(), JavaLocale.getCountry)
                                             : env->CallObjectMethod (locale.get(), JavaLocale.getLanguage);

                return aloeString (LocalRef<jstring> ((jstring) stringResult));
        */
    }

    pub fn get_android_os_build_value(field_name: *const u8) -> String {
        
        todo!();
        /*
            return aloeString (LocalRef<jstring> ((jstring) getEnv()->GetStaticObjectField (
                                    AndroidBuild, getEnv()->GetStaticFieldID (AndroidBuild, fieldName, "Ljava/lang/String;"))));
        */
    }
}

#[cfg(target_os="android")]
impl SystemStats {
    
    pub fn get_operating_system_type(&mut self) -> system_stats::OperatingSystemType {
        
        todo!();
        /*
            return Android;
        */
    }
    
    pub fn get_operating_system_name(&mut self) -> String {
        
        todo!();
        /*
            return "Android " + AndroidStatsHelpers::getSystemProperty ("os.version");
        */
    }
    
    pub fn get_device_description(&mut self) -> String {
        
        todo!();
        /*
            return AndroidStatsHelpers::getAndroidOsBuildValue ("MODEL")
                + "-" + AndroidStatsHelpers::getAndroidOsBuildValue ("SERIAL");
        */
    }
    
    pub fn get_device_manufacturer(&mut self) -> String {
        
        todo!();
        /*
            return AndroidStatsHelpers::getAndroidOsBuildValue ("MANUFACTURER");
        */
    }
    
    pub fn is_operating_system_64bit(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_64BIT
        return true;
       #else
        return false;
       #endif
        */
    }
    
    pub fn get_cpu_vendor(&mut self) -> String {
        
        todo!();
        /*
            return AndroidStatsHelpers::getSystemProperty ("os.arch");
        */
    }
    
    pub fn get_cpu_model(&mut self) -> String {
        
        todo!();
        /*
            return readPosixConfigFileValue ("/proc/cpuinfo", "Hardware");
        */
    }
    
    pub fn get_cpu_speed_in_megahertz(&mut self) -> i32 {
        
        todo!();
        /*
            int maxFreqKHz = 0;

        for (int i = 0; i < getNumCpus(); ++i)
        {
            int freqKHz = File ("/sys/devices/system/cpu/cpu" + String(i) + "/cpufreq/cpuinfo_max_freq")
                            .loadFileAsString()
                            .getIntValue();

            maxFreqKHz = jmax (freqKHz, maxFreqKHz);
        }

        return maxFreqKHz / 1000;
        */
    }
    
    pub fn get_memory_size_in_megabytes(&mut self) -> i32 {
        
        todo!();
        /*
            #if __ANDROID_API__ >= 9
        struct sysinfo sysi;

        if (sysinfo (&sysi) == 0)
            return static_cast<int> ((sysi.totalram * sysi.mem_unit) / (1024 * 1024));
       #endif

        return 0;
        */
    }
    
    pub fn get_page_size(&mut self) -> i32 {
        
        todo!();
        /*
            return static_cast<int> (sysconf (_SC_PAGESIZE));
        */
    }
    
    pub fn get_logon_name(&mut self) -> String {
        
        todo!();
        /*
            if (const char* user = getenv ("USER"))
            return CharPointer_UTF8 (user);

        if (struct passwd* const pw = getpwuid (getuid()))
            return CharPointer_UTF8 (pw->pw_name);

        return {};
        */
    }
    
    pub fn get_full_user_name(&mut self) -> String {
        
        todo!();
        /*
            return getLogonName();
        */
    }
    
    pub fn get_computer_name(&mut self) -> String {
        
        todo!();
        /*
            char name [256] = { 0 };
        if (gethostname (name, sizeof (name) - 1) == 0)
            return name;

        return {};
        */
    }
    
    pub fn get_user_language(&mut self) -> String {
        
        todo!();
        /*
            return AndroidStatsHelpers::getLocaleValue (false);
        */
    }
    
    pub fn get_user_region(&mut self) -> String {
        
        todo!();
        /*
            return AndroidStatsHelpers::getLocaleValue (true);
        */
    }
    
    pub fn get_display_language(&mut self) -> String {
        
        todo!();
        /*
            return getUserLanguage() + "-" + getUserRegion();
        */
    }
}

