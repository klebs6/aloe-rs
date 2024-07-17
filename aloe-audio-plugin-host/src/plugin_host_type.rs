crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_PluginHostType.h]

/**
  | A useful utility class to determine
  | the host or DAW in which your plugin is
  | loaded.
  | 
  | Declare a PluginHostType object in
  | your class to use it.
  | 
  | @tags{Audio}
  |
  */
pub struct PluginHostType {
    ty: PluginHostKind,
}

lazy_static!{
    /*
    static AudioProcessor::WrapperType aloePlugInClientCurrentWrapperType;
        static std::function<bool (AudioProcessor&)> aloePlugInIsRunningInAudioSuiteFn;
        static String hostIdReportedByWrapper;
    */
}

impl Default for PluginHostType {
    
    fn default() -> Self {
        todo!();
        /*
        : ty(getHostType()),
        */
    }
}

impl PluginHostType {

    /**
      | Returns true if the host is any version
      | of Ableton Live.
      |
      */
    pub fn is_ableton_live(&self) -> bool {
        
        todo!();
        /*
            return type == AbletonLive6 || type == AbletonLive7 || type == AbletonLive8 || type == AbletonLive9 || type == AbletonLive10 || type == AbletonLiveGeneric;
        */
    }

    /**
      | Returns true if the host is Adobe Audition.
      |
      */
    pub fn is_adobe_audition(&self) -> bool {
        
        todo!();
        /*
            return type == AdobeAudition;
        */
    }

    /**
      | Returns true if the host is Ardour.
      |
      */
    pub fn is_ardour(&self) -> bool {
        
        todo!();
        /*
            return type == Ardour;
        */
    }

    /**
      | Returns true if the host is AU Lab.
      |
      */
    pub fn is_au_lab(&self) -> bool {
        
        todo!();
        /*
            return type == AULab;
        */
    }

    /**
      | Returns true if the host is Bitwig Studio.
      |
      */
    pub fn is_bitwig_studio(&self) -> bool {
        
        todo!();
        /*
            return type == BitwigStudio;
        */
    }

    /**
      | Returns true if the host is any version
      | of Steinberg Cubase.
      |
      */
    pub fn is_cubase(&self) -> bool {
        
        todo!();
        /*
            return type == SteinbergCubase4 || type == SteinbergCubase5 || type == SteinbergCubase5Bridged || type == SteinbergCubase6 || type == SteinbergCubase7 || type == SteinbergCubase8 || type == SteinbergCubase8_5 || type == SteinbergCubase9 || type == SteinbergCubase9_5 || type == SteinbergCubase10 || type == SteinbergCubase10_5 || type == SteinbergCubaseGeneric;
        */
    }

    /**
      | Returns true if the host is Steinberg
      | Cubase 7 or later.
      |
      */
    pub fn is_cubase7or_later(&self) -> bool {
        
        todo!();
        /*
            return isCubase() && ! (type == SteinbergCubase4 || type == SteinbergCubase5 || type == SteinbergCubase6);
        */
    }

    /**
      | Returns true if the host is Steinberg
      | Cubase 5 Bridged.
      |
      */
    pub fn is_cubase_bridged(&self) -> bool {
        
        todo!();
        /*
            return type == SteinbergCubase5Bridged;
        */
    }

    /**
      | Returns true if the host is DaVinci Resolve.
      |
      */
    pub fn is_da_vinci_resolve(&self) -> bool {
        
        todo!();
        /*
            return type == DaVinciResolve;
        */
    }

    /**
      | Returns true if the host is Digital Performer.
      |
      */
    pub fn is_digital_performer(&self) -> bool {
        
        todo!();
        /*
            return type == DigitalPerformer;
        */
    }

    /**
      | Returns true if the host is Apple Final
      | Cut Pro.
      |
      */
    pub fn is_final_cut(&self) -> bool {
        
        todo!();
        /*
            return type == FinalCut;
        */
    }

    /**
      | Returns true if the host is Fruity Loops.
      |
      */
    pub fn is_fruity_loops(&self) -> bool {
        
        todo!();
        /*
            return type == FruityLoops;
        */
    }

    /**
      | Returns true if the host is Apple GarageBand.
      |
      */
    pub fn is_garage_band(&self) -> bool {
        
        todo!();
        /*
            return type == AppleGarageBand;
        */
    }

    /**
      | Returns true if the host is the Aloe AudioPluginHost
      |
      */
    pub fn is_aloe_plugin_host(&self) -> bool {
        
        todo!();
        /*
            return type == ALOEPluginHost;
        */
    }

    /**
      | Returns true if the host is Apple Logic
      | Pro.
      |
      */
    pub fn is_logic(&self) -> bool {
        
        todo!();
        /*
            return type == AppleLogic;
        */
    }

    /**
      | Returns true if the host is Apple MainStage.
      |
      */
    pub fn is_main_stage(&self) -> bool {
        
        todo!();
        /*
            return type == AppleMainStage;
        */
    }

    /**
      | Returns true if the host is any version
      | of Steinberg Nuendo.
      |
      */
    pub fn is_nuendo(&self) -> bool {
        
        todo!();
        /*
            return type == SteinbergNuendo3 || type == SteinbergNuendo4  || type == SteinbergNuendo5 ||  type == SteinbergNuendoGeneric;
        */
    }

    /**
      | Returns true if the host is pluginval.
      |
      */
    pub fn is_pluginval(&self) -> bool {
        
        todo!();
        /*
            return type == pluginval;
        */
    }

    /**
      | Returns true if the host is Adobe Premiere
      | Pro.
      |
      */
    pub fn is_premiere(&self) -> bool {
        
        todo!();
        /*
            return type == AdobePremierePro;
        */
    }

    /**
      | Returns true if the host is Avid Pro Tools.
      |
      */
    pub fn is_pro_tools(&self) -> bool {
        
        todo!();
        /*
            return type == AvidProTools;
        */
    }

    /**
      | Returns true if the host is Merging Pyramix.
      |
      */
    pub fn is_pyramix(&self) -> bool {
        
        todo!();
        /*
            return type == MergingPyramix;
        */
    }

    /**
      | Returns true if the host is Muse Receptor.
      |
      */
    pub fn is_receptor(&self) -> bool {
        
        todo!();
        /*
            return type == MuseReceptorGeneric;
        */
    }

    /**
      | Returns true if the host is Cockos Reaper.
      |
      */
    pub fn is_reaper(&self) -> bool {
        
        todo!();
        /*
            return type == Reaper;
        */
    }

    /**
      | Returns true if the host is Reason.
      |
      */
    pub fn is_reason(&self) -> bool {
        
        todo!();
        /*
            return type == Reason;
        */
    }

    /**
      | Returns true if the host is Renoise.
      |
      */
    pub fn is_renoise(&self) -> bool {
        
        todo!();
        /*
            return type == Renoise;
        */
    }

    /**
      | Returns true if the host is SADiE.
      |
      */
    pub fn is_sa_die(&self) -> bool {
        
        todo!();
        /*
            return type == SADiE;
        */
    }

    /**
      | Returns true if the host is Magix Samplitude.
      |
      */
    pub fn is_samplitude(&self) -> bool {
        
        todo!();
        /*
            return type == MagixSamplitude;
        */
    }

    /**
      | Returns true if the host is Magix Sequoia.
      |
      */
    pub fn is_sequoia(&self) -> bool {
        
        todo!();
        /*
            return type == MagixSequoia;
        */
    }

    /**
      | Returns true if the host is any version
      | of Cakewalk Sonar.
      |
      */
    pub fn is_sonar(&self) -> bool {
        
        todo!();
        /*
            return type == CakewalkSonar8 || type == CakewalkSonarGeneric || type == CakewalkByBandlab;
        */
    }

    /**
      | Returns true if the host is Steinberg's
      | Vst3 Test Host.
      |
      */
    pub fn is_steinberg_test_host(&self) -> bool {
        
        todo!();
        /*
            return type == SteinbergTestHost;
        */
    }

    /**
      | Returns true if the host is any product
      | from Steinberg.
      |
      */
    pub fn is_steinberg(&self) -> bool {
        
        todo!();
        /*
            return isCubase() || isNuendo() || isWavelab() || isSteinbergTestHost();
        */
    }

    /**
      | Returns true if the host is PreSonus
      | Studio One.
      |
      */
    pub fn is_studio_one(&self) -> bool {
        
        todo!();
        /*
            return type == StudioOne;
        */
    }

    /**
      | Returns true if the host is any version
      | of Tracktion.
      |
      */
    pub fn is_tracktion(&self) -> bool {
        
        todo!();
        /*
            return type == Tracktion3 || type == TracktionGeneric || isTracktionWaveform();
        */
    }

    /**
      | Returns true if the host is Tracktion
      | Waveform.
      |
      */
    pub fn is_tracktion_waveform(&self) -> bool {
        
        todo!();
        /*
            return type == TracktionWaveform;
        */
    }

    /**
      | Returns true if the host is VB Audio Vst
      | Scanner.
      |
      */
    pub fn is_vbvst_scanner(&self) -> bool {
        
        todo!();
        /*
            return type == VBVstScanner;
        */
    }

    /**
      | Returns true if the host is Vienna Ensemble
      | Pro.
      |
      */
    pub fn is_vienna_ensemble_pro(&self) -> bool {
        
        todo!();
        /*
            return type == ViennaEnsemblePro;
        */
    }

    /**
      | Returns true if the host is Apple WaveBurner.
      |
      */
    pub fn is_wave_burner(&self) -> bool {
        
        todo!();
        /*
            return type == WaveBurner;
        */
    }

    /**
      | Returns true if the host is any version
      | of Steinberg WaveLab.
      |
      */
    pub fn is_wavelab(&self) -> bool {
        
        todo!();
        /*
            return isWavelabLegacy() || type == SteinbergWavelab7 || type == SteinbergWavelab8 || type == SteinbergWavelabGeneric;
        */
    }

    /**
      | Returns true if the host is Steinberg
      | WaveLab 6 or below.
      |
      */
    pub fn is_wavelab_legacy(&self) -> bool {
        
        todo!();
        /*
            return type == SteinbergWavelab5 || type == SteinbergWavelab6;
        */
    }

    /**
      | Returns the complete absolute path
      | of the host application executable.
      |
      */
    pub fn get_host_path() -> String {
        
        todo!();
        /*
            return File::getSpecialLocation (File::hostApplicationPath).getFullPathName();
        */
    }
    
    /**
      | Returns the plug-in format via which
      | the plug-in file was loaded. This value
      | is identical to AudioProcessor::wrapperType
      | of the main audio processor of this plug-in.
      | This function is useful for code that
      | does not have access to the plug-in's
      | main audio processor.
      | 
      | @see AudioProcessor::wrapperType
      |
      */
    pub fn get_plugin_loaded_as() -> AudioProcessorWrapperType {
        
        todo!();
        /*
            return aloePlugInClientCurrentWrapperType;
        */
    }

    /**
      | Returns true if the plugin is connected
      | with Inter-App Audio on iOS.
      |
      */
    pub fn is_inter_app_audio_connected(&self) -> bool {
        
        todo!();
        /*
            #if AloePlugin_Enable_IAA && AloePlugin_Build_Standalone && ALOE_IOS && (! ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP)
        if (getPluginLoadedAs() == AudioProcessor::wrapperType_Standalone)
            return aloe_isInterAppAudioConnected();
       #endif

        return false;
        */
    }
    
    /**
      | Switches to the host application when
      | Inter-App Audio is used on iOS.
      |
      */
    pub fn switch_to_host_application(&self)  {
        
        todo!();
        /*
            #if AloePlugin_Enable_IAA && AloePlugin_Build_Standalone && ALOE_IOS && (! ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP)
        if (getPluginLoadedAs() == AudioProcessor::wrapperType_Standalone)
            aloe_switchToHostApplication();
       #endif
        */
    }
    
    /**
      | Returns true if the AudioProcessor
      | instance is an AAX plug-in running in
      | AudioSuite.
      |
      */
    pub fn is_in_aax_audio_suite(&mut self, processor: &mut AudioProcessor) -> bool {
        
        todo!();
        /*
            #if AloePlugin_Build_AAX
        if (PluginHostType::getPluginLoadedAs() == AudioProcessor::wrapperType_AAX
            && aloePlugInIsRunningInAudioSuiteFn != nullptr)
        {
            return aloePlugInIsRunningInAudioSuiteFn (processor);
        }
       #endif

        ignoreUnused (processor);
        return false;
        */
    }
    
    /**
      | Gets the host app's icon when Inter-App
      | Audio is used on iOS.
      |
      */
    pub fn get_host_icon(&self, size: i32) -> Image {
        
        todo!();
        /*
            ignoreUnused (size);

       #if AloePlugin_Enable_IAA && AloePlugin_Build_Standalone && ALOE_IOS && (! ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP)
        if (isInterAppAudioConnected())
            return aloe_getIAAHostIcon (size);
       #endif

       #if ALOE_MAC
        String bundlePath (getHostPath().upToLastOccurrenceOf (".app", true, true));
        return getIconFromApplication (bundlePath, size);
       #endif

        return Image();
        */
    }
    
    /**
      | Returns a human-readable description
      | of the host.
      |
      */
    pub fn get_host_description(&self) -> *const u8 {
        
        todo!();
        /*
            switch (type)
        {
            case AbletonLive6:             return "Ableton Live 6";
            case AbletonLive7:             return "Ableton Live 7";
            case AbletonLive8:             return "Ableton Live 8";
            case AbletonLive9:             return "Ableton Live 9";
            case AbletonLive10:            return "Ableton Live 10";
            case AbletonLiveGeneric:       return "Ableton Live";
            case AdobeAudition:            return "Adobe Audition";
            case AdobePremierePro:         return "Adobe Premiere";
            case AppleGarageBand:          return "Apple GarageBand";
            case AppleLogic:               return "Apple Logic";
            case AppleMainStage:           return "Apple MainStage";
            case Ardour:                   return "Ardour";
            case AULab:                    return "AU Lab";
            case AvidProTools:             return "ProTools";
            case BitwigStudio:             return "Bitwig Studio";
            case CakewalkSonar8:           return "Cakewalk Sonar 8";
            case CakewalkSonarGeneric:     return "Cakewalk Sonar";
            case CakewalkByBandlab:        return "Cakewalk by Bandlab";
            case DaVinciResolve:           return "DaVinci Resolve";
            case DigitalPerformer:         return "DigitalPerformer";
            case FinalCut:                 return "Final Cut";
            case FruityLoops:              return "FruityLoops";
            case ALOEPluginHost:           return "Aloe AudioPluginHost";
            case MagixSamplitude:          return "Magix Samplitude";
            case MagixSequoia:             return "Magix Sequoia";
            case pluginval:                return "pluginval";
            case MergingPyramix:           return "Pyramix";
            case MuseReceptorGeneric:      return "Muse Receptor";
            case Reaper:                   return "Reaper";
            case Reason:                   return "Reason";
            case Renoise:                  return "Renoise";
            case SADiE:                    return "SADiE";
            case SteinbergCubase4:         return "Steinberg Cubase 4";
            case SteinbergCubase5:         return "Steinberg Cubase 5";
            case SteinbergCubase5Bridged:  return "Steinberg Cubase 5 Bridged";
            case SteinbergCubase6:         return "Steinberg Cubase 6";
            case SteinbergCubase7:         return "Steinberg Cubase 7";
            case SteinbergCubase8:         return "Steinberg Cubase 8";
            case SteinbergCubase8_5:       return "Steinberg Cubase 8.5";
            case SteinbergCubase9:         return "Steinberg Cubase 9";
            case SteinbergCubase9_5:       return "Steinberg Cubase 9.5";
            case SteinbergCubase10:        return "Steinberg Cubase 10";
            case SteinbergCubase10_5:      return "Steinberg Cubase 10.5";
            case SteinbergCubaseGeneric:   return "Steinberg Cubase";
            case SteinbergNuendo3:         return "Steinberg Nuendo 3";
            case SteinbergNuendo4:         return "Steinberg Nuendo 4";
            case SteinbergNuendo5:         return "Steinberg Nuendo 5";
            case SteinbergNuendoGeneric:   return "Steinberg Nuendo";
            case SteinbergWavelab5:        return "Steinberg Wavelab 5";
            case SteinbergWavelab6:        return "Steinberg Wavelab 6";
            case SteinbergWavelab7:        return "Steinberg Wavelab 7";
            case SteinbergWavelab8:        return "Steinberg Wavelab 8";
            case SteinbergWavelabGeneric:  return "Steinberg Wavelab";
            case SteinbergTestHost:        return "Steinberg TestHost";
            case StudioOne:                return "Studio One";
            case Tracktion3:               return "Tracktion 3";
            case TracktionGeneric:         return "Tracktion";
            case TracktionWaveform:        return "Tracktion Waveform";
            case VBVstScanner:             return "VBVstScanner";
            case ViennaEnsemblePro:        return "Vienna Ensemble Pro";
            case WaveBurner:               return "WaveBurner";
            case UnknownHost:
            default:                       break;
        }

        return "Unknown";
        */
    }
    
    pub fn get_host_type(&mut self) -> PluginHostKind {
        
        todo!();
        /*
            auto hostPath = getHostPath();
        auto hostFilename = File (hostPath).getFileName();

       #if ALOE_MAC
        if (hostPath.containsIgnoreCase       ("Final Cut Pro.app"))        return FinalCut;
        if (hostPath.containsIgnoreCase       ("Final Cut Pro Trial.app"))  return FinalCut;
        if (hostPath.containsIgnoreCase       ("Live 6"))                   return AbletonLive6;
        if (hostPath.containsIgnoreCase       ("Live 7"))                   return AbletonLive7;
        if (hostPath.containsIgnoreCase       ("Live 8"))                   return AbletonLive8;
        if (hostPath.containsIgnoreCase       ("Live 9"))                   return AbletonLive9;
        if (hostPath.containsIgnoreCase       ("Live 10"))                  return AbletonLive10;
        if (hostFilename.containsIgnoreCase   ("Live"))                     return AbletonLiveGeneric;
        if (hostFilename.containsIgnoreCase   ("Audition"))                 return AdobeAudition;
        if (hostFilename.containsIgnoreCase   ("Adobe Premiere"))           return AdobePremierePro;
        if (hostFilename.containsIgnoreCase   ("GarageBand"))               return AppleGarageBand;
        if (hostFilename.containsIgnoreCase   ("Logic"))                    return AppleLogic;
        if (hostFilename.containsIgnoreCase   ("MainStage"))                return AppleMainStage;
        if (hostFilename.containsIgnoreCase   ("AU Lab"))                   return AULab;
        if (hostFilename.containsIgnoreCase   ("Pro Tools"))                return AvidProTools;
        if (hostFilename.containsIgnoreCase   ("Nuendo 3"))                 return SteinbergNuendo3;
        if (hostFilename.containsIgnoreCase   ("Nuendo 4"))                 return SteinbergNuendo4;
        if (hostFilename.containsIgnoreCase   ("Nuendo 5"))                 return SteinbergNuendo5;
        if (hostFilename.containsIgnoreCase   ("Nuendo"))                   return SteinbergNuendoGeneric;
        if (hostFilename.containsIgnoreCase   ("Cubase 4"))                 return SteinbergCubase4;
        if (hostFilename.containsIgnoreCase   ("Cubase 5"))                 return SteinbergCubase5;
        if (hostFilename.containsIgnoreCase   ("Cubase 6"))                 return SteinbergCubase6;
        if (hostFilename.containsIgnoreCase   ("Cubase 7"))                 return SteinbergCubase7;
        if (hostPath.containsIgnoreCase       ("Cubase 8.app"))             return SteinbergCubase8;
        if (hostPath.containsIgnoreCase       ("Cubase 8.5.app"))           return SteinbergCubase8_5;
        if (hostPath.containsIgnoreCase       ("Cubase 9.app"))             return SteinbergCubase9;
        if (hostPath.containsIgnoreCase       ("Cubase 9.5.app"))           return SteinbergCubase9_5;
        if (hostPath.containsIgnoreCase       ("Cubase 10.app"))            return SteinbergCubase10;
        if (hostPath.containsIgnoreCase       ("Cubase 10.5.app"))          return SteinbergCubase10_5;
        if (hostFilename.containsIgnoreCase   ("Cubase"))                   return SteinbergCubaseGeneric;
        if (hostPath.containsIgnoreCase       ("Wavelab 7"))                return SteinbergWavelab7;
        if (hostPath.containsIgnoreCase       ("Wavelab 8"))                return SteinbergWavelab8;
        if (hostFilename.containsIgnoreCase   ("Wavelab"))                  return SteinbergWavelabGeneric;
        if (hostFilename.containsIgnoreCase   ("WaveBurner"))               return WaveBurner;
        if (hostPath.containsIgnoreCase       ("Digital Performer"))        return DigitalPerformer;
        if (hostFilename.containsIgnoreCase   ("reaper"))                   return Reaper;
        if (hostFilename.containsIgnoreCase   ("Reason"))                   return Reason;
        if (hostPath.containsIgnoreCase       ("Studio One"))               return StudioOne;
        if (hostFilename.startsWithIgnoreCase ("Waveform"))                 return TracktionWaveform;
        if (hostPath.containsIgnoreCase       ("Tracktion 3"))              return Tracktion3;
        if (hostFilename.containsIgnoreCase   ("Tracktion"))                return TracktionGeneric;
        if (hostFilename.containsIgnoreCase   ("Renoise"))                  return Renoise;
        if (hostFilename.containsIgnoreCase   ("Resolve"))                  return DaVinciResolve;
        if (hostFilename.startsWith           ("Bitwig"))                   return BitwigStudio;
        if (hostFilename.containsIgnoreCase   ("OsxFL"))                    return FruityLoops;
        if (hostFilename.containsIgnoreCase   ("pluginval"))                return pluginval;
        if (hostFilename.containsIgnoreCase   ("AudioPluginHost"))          return ALOEPluginHost;
        if (hostFilename.containsIgnoreCase   ("Vienna Ensemble Pro"))      return ViennaEnsemblePro;

        if (hostIdReportedByWrapper == "com.apple.logic.pro")               return AppleLogic;
        if (hostIdReportedByWrapper == "com.apple.garageband")              return AppleGarageBand;
        if (hostIdReportedByWrapper == "com.apple.mainstage")               return AppleMainStage;

        const auto procName = nsStringToAloe ([[NSRunningApplication currentApplication] localizedName]);

        const auto matchesInOrder = [&] (const Vec<String>& strings)
        {
            return procName.matchesWildcard ("AUHostingService*(" + strings.joinIntoString ("*") + ")", false);
        };

        // Depending on localization settings, spaces are not always plain ascii spaces
        if (matchesInOrder ({ "Logic", "Pro" }))                            return AppleLogic;
        if (matchesInOrder ({ "GarageBand" }))                              return AppleGarageBand;
        if (matchesInOrder ({ "MainStage" }))                               return AppleMainStage;
        if (matchesInOrder ({ "Final", "Cut", "Pro" }))                     return FinalCut;

       #elif ALOE_WINDOWS
        if (hostFilename.containsIgnoreCase   ("Live 6"))                return AbletonLive6;
        if (hostFilename.containsIgnoreCase   ("Live 7"))                return AbletonLive7;
        if (hostFilename.containsIgnoreCase   ("Live 8"))                return AbletonLive8;
        if (hostFilename.containsIgnoreCase   ("Live 9"))                return AbletonLive9;
        if (hostFilename.containsIgnoreCase   ("Live 10"))               return AbletonLive10;
        if (hostFilename.containsIgnoreCase   ("Live "))                 return AbletonLiveGeneric;
        if (hostFilename.containsIgnoreCase   ("Audition"))              return AdobeAudition;
        if (hostFilename.containsIgnoreCase   ("Adobe Premiere"))        return AdobePremierePro;
        if (hostFilename.containsIgnoreCase   ("ProTools"))              return AvidProTools;
        if (hostPath.containsIgnoreCase       ("SONAR 8"))               return CakewalkSonar8;
        if (hostFilename.containsIgnoreCase   ("SONAR"))                 return CakewalkSonarGeneric;
        if (hostFilename.containsIgnoreCase   ("Cakewalk.exe"))          return CakewalkByBandlab;
        if (hostFilename.containsIgnoreCase   ("GarageBand"))            return AppleGarageBand;
        if (hostFilename.containsIgnoreCase   ("Logic"))                 return AppleLogic;
        if (hostFilename.containsIgnoreCase   ("MainStage"))             return AppleMainStage;
        if (hostFilename.startsWithIgnoreCase ("Waveform"))              return TracktionWaveform;
        if (hostPath.containsIgnoreCase       ("Tracktion 3"))           return Tracktion3;
        if (hostFilename.containsIgnoreCase   ("Tracktion"))             return TracktionGeneric;
        if (hostFilename.containsIgnoreCase   ("reaper"))                return Reaper;
        if (hostFilename.containsIgnoreCase   ("Cubase4"))               return SteinbergCubase4;
        if (hostFilename.containsIgnoreCase   ("Cubase5"))               return SteinbergCubase5;
        if (hostFilename.containsIgnoreCase   ("Cubase6"))               return SteinbergCubase6;
        if (hostFilename.containsIgnoreCase   ("Cubase7"))               return SteinbergCubase7;
        if (hostFilename.containsIgnoreCase   ("Cubase8.exe"))           return SteinbergCubase8;
        if (hostFilename.containsIgnoreCase   ("Cubase8.5.exe"))         return SteinbergCubase8_5;
        // Later version of Cubase scan plug-ins with a separate executable "vst2xscanner"
        if (hostFilename.containsIgnoreCase   ("Cubase9.5.exe")
            || hostPath.containsIgnoreCase    ("Cubase 9.5"))            return SteinbergCubase9_5;
        if (hostFilename.containsIgnoreCase   ("Cubase9.exe")
            || hostPath.containsIgnoreCase    ("Cubase 9"))              return SteinbergCubase9;
        if (hostFilename.containsIgnoreCase   ("Cubase10.5.exe")
            || hostPath.containsIgnoreCase    ("Cubase 10.5"))           return SteinbergCubase10_5;
        if (hostFilename.containsIgnoreCase   ("Cubase10.exe")
            || hostPath.containsIgnoreCase    ("Cubase 10"))             return SteinbergCubase10;
        if (hostFilename.containsIgnoreCase   ("Cubase"))                return SteinbergCubaseGeneric;
        if (hostFilename.containsIgnoreCase   ("VstBridgeApp"))          return SteinbergCubase5Bridged;
        if (hostPath.containsIgnoreCase       ("Wavelab 5"))             return SteinbergWavelab5;
        if (hostPath.containsIgnoreCase       ("Wavelab 6"))             return SteinbergWavelab6;
        if (hostPath.containsIgnoreCase       ("Wavelab 7"))             return SteinbergWavelab7;
        if (hostPath.containsIgnoreCase       ("Wavelab 8"))             return SteinbergWavelab8;
        if (hostPath.containsIgnoreCase       ("Nuendo"))                return SteinbergNuendoGeneric;
        if (hostFilename.containsIgnoreCase   ("Wavelab"))               return SteinbergWavelabGeneric;
        if (hostFilename.containsIgnoreCase   ("TestHost"))              return SteinbergTestHost;
        if (hostFilename.containsIgnoreCase   ("rm-host"))               return MuseReceptorGeneric;
        if (hostFilename.startsWith           ("FL"))                    return FruityLoops;
        if (hostFilename.contains             ("ilbridge."))             return FruityLoops;
        if (hostPath.containsIgnoreCase       ("Studio One"))            return StudioOne;
        if (hostPath.containsIgnoreCase       ("Digital Performer"))     return DigitalPerformer;
        if (hostFilename.containsIgnoreCase   ("Vst_Scanner"))           return VBVstScanner;
        if (hostPath.containsIgnoreCase       ("Merging Technologies"))  return MergingPyramix;
        if (hostFilename.startsWithIgnoreCase ("Sam"))                   return MagixSamplitude;
        if (hostFilename.startsWithIgnoreCase ("Sequoia"))               return MagixSequoia;
        if (hostFilename.containsIgnoreCase   ("Reason"))                return Reason;
        if (hostFilename.containsIgnoreCase   ("Renoise"))               return Renoise;
        if (hostFilename.containsIgnoreCase   ("Resolve"))               return DaVinciResolve;
        if (hostPath.containsIgnoreCase       ("Bitwig Studio"))         return BitwigStudio;
        if (hostFilename.containsIgnoreCase   ("Sadie"))                 return SADiE;
        if (hostFilename.containsIgnoreCase   ("pluginval"))             return pluginval;
        if (hostFilename.containsIgnoreCase   ("AudioPluginHost"))       return ALOEPluginHost;
        if (hostFilename.containsIgnoreCase   ("Vienna Ensemble Pro"))   return ViennaEnsemblePro;

       #elif ALOE_LINUX || ALOE_BSD
        if (hostFilename.containsIgnoreCase   ("Ardour"))            return Ardour;
        if (hostFilename.startsWithIgnoreCase ("Waveform"))          return TracktionWaveform;
        if (hostFilename.containsIgnoreCase   ("Tracktion"))         return TracktionGeneric;
        if (hostFilename.startsWith           ("Bitwig"))            return BitwigStudio;
        if (hostFilename.containsIgnoreCase   ("pluginval"))         return pluginval;
        if (hostFilename.containsIgnoreCase   ("AudioPluginHost"))   return ALOEPluginHost;

       #elif ALOE_IOS
       #elif ALOE_ANDROID
       #else
        #error
       #endif
        return UnknownHost;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_PluginHostType.cpp]

#[cfg(all(all(AloePlugin_Enable_IAA,AloePlugin_Build_Standalone),all(target_os="ios",not(ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP))))]
pub fn aloe_is_inter_app_audio_connected() -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(all(all(AloePlugin_Enable_IAA,AloePlugin_Build_Standalone),all(target_os="ios",not(ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP))))]
pub fn aloe_switch_to_host_application()  {
    
    todo!();
        /*
        
        */
}

#[cfg(all(all(AloePlugin_Enable_IAA,AloePlugin_Build_Standalone),all(target_os="ios",not(ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP))))]
pub fn aloe_get_iaa_host_icon(_0: i32) -> Image {
    
    todo!();
        /*
        
        */
}

lazy_static!{
    /*
    AudioProcessor::WrapperType PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_Undefined;
    std::function<bool (AudioProcessor&)> PluginHostType::aloePlugInIsRunningInAudioSuiteFn = nullptr;
    String PluginHostType::hostIdReportedByWrapper;
    */
}
