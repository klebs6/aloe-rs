crate::ix!();

/**
  | Represents the host type and also its
  | version for some hosts.
  |
  */
pub enum PluginHostKind
{
    /**
      | Represents an unknown host.
      |
      */
    UnknownHost,                

    /**
      | Represents Ableton Live 6.
      |
      */
    AbletonLive6,               

    /**
      | Represents Ableton Live 7.
      |
      */
    AbletonLive7,               

    /**
      | Represents Ableton Live 8.
      |
      */
    AbletonLive8,               

    /**
      | Represents Ableton Live 9.
      |
      */
    AbletonLive9,               

    /**
      | Represents Ableton Live 10.
      |
      */
    AbletonLive10,              

    /**
      | Represents Ableton Live.
      |
      */
    AbletonLiveGeneric,         

    /**
      | Represents Adobe Audition.
      |
      */
    AdobeAudition,              

    /**
      | Represents Adobe Premiere Pro.
      |
      */
    AdobePremierePro,           

    /**
      | Represents Apple GarageBand.
      |
      */
    AppleGarageBand,            

    /**
      | Represents Apple Logic Pro.
      |
      */
    AppleLogic,                 

    /**
      | Represents Apple Main Stage.
      |
      */
    AppleMainStage,             

    /**
      | Represents Ardour.
      |
      */
    Ardour,                     

    /**
      | Represents AU Lab.
      |
      */
    AULab,                      

    /**
      | Represents Avid Pro Tools.
      |
      */
    AvidProTools,               

    /**
      | Represents Bitwig Studio.
      |
      */
    BitwigStudio,               

    /**
      | Represents Cakewalk Sonar 8.
      |
      */
    CakewalkSonar8,             

    /**
      | Represents Cakewalk Sonar.
      |
      */
    CakewalkSonarGeneric,       

    /**
      | Represents Cakewalk by Bandlab.
      |
      */
    CakewalkByBandlab,          

    /**
      | Represents DaVinci Resolve.
      |
      */
    DaVinciResolve,             

    /**
      | Represents Digital Performer.
      |
      */
    DigitalPerformer,           

    /**
      | Represents Apple Final Cut Pro.
      |
      */
    FinalCut,                   

    /**
      | Represents Fruity Loops.
      |
      */
    FruityLoops,                

    /**
  | Represents the Aloe AudioPluginHost
  |
  */
    ALOEPluginHost,             

    /**
      | Represents Magix Samplitude.
      |
      */
    MagixSamplitude,            

    /**
      | Represents Magix Sequoia.
      |
      */
    MagixSequoia,               

    /**
      | Represents Merging Pyramix.
      |
      */
    MergingPyramix,             

    /**
      | Represents Muse Receptor.
      |
      */
    MuseReceptorGeneric,        

    /**
      | Represents pluginval.
      |
      */
    pluginval,                  

    /**
      | Represents Cockos Reaper.
      |
      */
    Reaper,                     

    /**
      | Represents Reason.
      |
      */
    Reason,                     

    /**
      | Represents Renoise.
      |
      */
    Renoise,                    

    /**
      | Represents SADiE.
      |
      */
    SADiE,                      

    /**
      | Represents Steinberg Cubase 4.
      |
      */
    SteinbergCubase4,           

    /**
      | Represents Steinberg Cubase 5.
      |
      */
    SteinbergCubase5,           

    /**
      | Represents Steinberg Cubase 5 Bridged.
      |
      */
    SteinbergCubase5Bridged,    

    /**
      | Represents Steinberg Cubase 6.
      |
      */
    SteinbergCubase6,           

    /**
      | Represents Steinberg Cubase 7.
      |
      */
    SteinbergCubase7,           

    /**
      | Represents Steinberg Cubase 8.
      |
      */
    SteinbergCubase8,           

    /**
      | Represents Steinberg Cubase 8.5.
      |
      */
    SteinbergCubase8_5,         

    /**
      | Represents Steinberg Cubase 9.
      |
      */
    SteinbergCubase9,           

    /**
      | Represents Steinberg Cubase 9.5.
      |
      */
    SteinbergCubase9_5,         

    /**
      | Represents Steinberg Cubase 10.
      |
      */
    SteinbergCubase10,          

    /**
      | Represents Steinberg Cubase 10.5.
      |
      */
    SteinbergCubase10_5,        

    /**
      | Represents Steinberg Cubase.
      |
      */
    SteinbergCubaseGeneric,     

    /**
      | Represents Steinberg Nuendo 3.
      |
      */
    SteinbergNuendo3,           

    /**
      | Represents Steinberg Nuendo 4.
      |
      */
    SteinbergNuendo4,           

    /**
      | Represents Steinberg Nuendo 5.
      |
      */
    SteinbergNuendo5,           

    /**
      | Represents Steinberg Nuendo.
      |
      */
    SteinbergNuendoGeneric,     

    /**
      | Represents Steinberg Wavelab 5.
      |
      */
    SteinbergWavelab5,          

    /**
      | Represents Steinberg Wavelab 6.
      |
      */
    SteinbergWavelab6,          

    /**
      | Represents Steinberg Wavelab 7.
      |
      */
    SteinbergWavelab7,          

    /**
      | Represents Steinberg Wavelab 8.
      |
      */
    SteinbergWavelab8,          

    /**
      | Represents Steinberg Wavelab.
      |
      */
    SteinbergWavelabGeneric,    

    /**
      | Represents Steinberg's Vst3 Test Host.
      |
      */
    SteinbergTestHost,          

    /**
      | Represents PreSonus Studio One.
      |
      */
    StudioOne,                  

    /**
      | Represents Tracktion 3.
      |
      */
    Tracktion3,                 

    /**
      | Represents Tracktion.
      |
      */
    TracktionGeneric,           

    /**
      | Represents Tracktion Waveform.
      |
      */
    TracktionWaveform,          

    /**
      | Represents VB Audio Vst Scanner.
      |
      */
    VBVstScanner,               

    /**
      | Represents Vienna Ensemble Pro.
      |
      */
    ViennaEnsemblePro,          

    /**
      | Represents Apple WaveBurner.
      |
      */
    WaveBurner,                  
}
