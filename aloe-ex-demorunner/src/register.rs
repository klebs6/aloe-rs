crate::ix!();

macro_rules! file_ext { () => { /* .h */ } }

macro_rules! expand {
    ($x:ident) => {
        /*
                x
        */
    }
}

macro_rules! create_filepath {
    ($DemoName:ident, $category:ident) => {
        /*
                ALOE_STRINGIFY(EXPAND(category)/EXPAND(DemoName)EXPAND(FILE_EXT))
        */
    }
}

macro_rules! register_demo {
    ($DemoName:ident, 
     $category:ident, 
     $heavyweight:ident) => {
        /*
                ALOEDemos::registerDemo ([] { return new DemoName(); }, CREATE_FILEPATH(DemoName, category), ALOE_STRINGIFY (category), heavyweight);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/Demos/DemoPIPs2.cpp]

pub fn register_demos_two()  {
    
    todo!();
    /*
        #if ALOE_MAC || ALOE_WINDOWS
        REGISTER_DEMO (AccessibilityDemo,         GUI, false)
       #endif
        REGISTER_DEMO (AnimationAppDemo,          GUI, false)
        REGISTER_DEMO (AnimationDemo,             GUI, false)
        REGISTER_DEMO (BouncingBallWavetableDemo, GUI, false)
       #if ALOE_USE_CAMERA && ! (ALOE_LINUX || ALOE_BSD)
        REGISTER_DEMO (CameraDemo,                GUI, true)
       #endif
       #if ! ALOE_ANDROID
        REGISTER_DEMO (CodeEditorDemo,            GUI, false)
       #endif
        REGISTER_DEMO (ComponentDemo,             GUI, false)
        REGISTER_DEMO (ComponentTransformsDemo,   GUI, false)
        REGISTER_DEMO (DialogsDemo,               GUI, false)
        REGISTER_DEMO (FlexBoxDemo,               GUI, false)
        REGISTER_DEMO (FontsDemo,                 GUI, false)
        REGISTER_DEMO (GraphicsDemo,              GUI, false)
        REGISTER_DEMO (GridDemo,                  GUI, false)
        REGISTER_DEMO (ImagesDemo,                GUI, false)
        REGISTER_DEMO (KeyMappingsDemo,           GUI, false)
        REGISTER_DEMO (LookAndFeelDemo,           GUI, false)
        REGISTER_DEMO (MDIDemo,                   GUI, false)
        REGISTER_DEMO (MenusDemo,                 GUI, false)
        REGISTER_DEMO (MultiTouchDemo,            GUI, false)
       #if ALOE_OPENGL
        REGISTER_DEMO (OpenGLAppDemo,             GUI, true)
        REGISTER_DEMO (OpenGLDemo2D,              GUI, true)
        REGISTER_DEMO (OpenGLDemo,                GUI, true)
       #endif
        REGISTER_DEMO (PropertiesDemo,            GUI, false)
       #if ! (ALOE_LINUX || ALOE_BSD)
        REGISTER_DEMO (VideoDemo,                 GUI, true)
       #endif
        REGISTER_DEMO (WebBrowserDemo,            GUI, true)
        REGISTER_DEMO (WidgetsDemo,               GUI, false)
        REGISTER_DEMO (WindowsDemo,               GUI, false)
    */
}

pub fn get_dark_colour_scheme() -> CodeEditorComponentColourScheme {
    
    todo!();
    /*
        return getDarkCodeEditorColourScheme();
    */
}

pub fn get_light_colour_scheme() -> CodeEditorComponentColourScheme {
    
    todo!();
    /*
        return getLightCodeEditorColourScheme();
    */
}
//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/Demos/DemoPIPs1.cpp]

pub fn register_demos_one()  {
    
    todo!();
    /*
        REGISTER_DEMO (AudioAppDemo,            Audio,     false)
        REGISTER_DEMO (AudioLatencyDemo,        Audio,     false)
        REGISTER_DEMO (AudioPlaybackDemo,       Audio,     false)
        REGISTER_DEMO (AudioRecordingDemo,      Audio,     false)
        REGISTER_DEMO (AudioSettingsDemo,       Audio,     false)
        REGISTER_DEMO (AudioSynthesizerDemo,    Audio,     false)
        REGISTER_DEMO (MidiDemo,                Audio,     false)
        REGISTER_DEMO (MPEDemo,                 Audio,     false)
        REGISTER_DEMO (PluckedStringsDemo,      Audio,     false)
        REGISTER_DEMO (SimpleFFTDemo,           Audio,     false)

        REGISTER_DEMO (ConvolutionDemo,         DSP,       false)
        REGISTER_DEMO (FIRFilterDemo,           DSP,       false)
        REGISTER_DEMO (GainDemo,                DSP,       false)
        REGISTER_DEMO (IIRFilterDemo,           DSP,       false)
        REGISTER_DEMO (OscillatorDemo,          DSP,       false)
        REGISTER_DEMO (OverdriveDemo,           DSP,       false)
        #if ALOE_USE_SIMD
         REGISTER_DEMO (SIMDRegisterDemo,       DSP,       false)
        #endif
        REGISTER_DEMO (StateVariableFilterDemo, DSP,       false)
        REGISTER_DEMO (WaveShaperTanhDemo,      DSP,       false)

        REGISTER_DEMO (Box2DDemo,               Utilities, false)
       #if ALOE_MAC || ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
        REGISTER_DEMO (ChildProcessDemo,        Utilities, false)
       #endif
        REGISTER_DEMO (CryptographyDemo,        Utilities, false)
        REGISTER_DEMO (JavaScriptDemo,          Utilities, false)
        REGISTER_DEMO (LiveConstantDemo,        Utilities, false)
        REGISTER_DEMO (MultithreadingDemo,      Utilities, false)
        REGISTER_DEMO (NetworkingDemo,          Utilities, false)
        REGISTER_DEMO (OSCDemo,                 Utilities, false)
        REGISTER_DEMO (SystemInfoDemo,          Utilities, false)
        REGISTER_DEMO (TimersAndEventsDemo,     Utilities, false)
        REGISTER_DEMO (UnitTestsDemo,           Utilities, false)
        REGISTER_DEMO (ValueTreesDemo,          Utilities, false)
        REGISTER_DEMO (XMLandJSONDemo,          Utilities, false)
    */
}

pub fn create_intro_demo<'a>() -> *mut Component<'a> {
    
    todo!();
    /*
        return new IntroScreen();
    */
}

pub fn is_component_intro_demo<'a>(comp: *mut Component<'a>) -> bool {
    
    todo!();
    /*
        return (dynamic_cast<IntroScreen*> (comp) != nullptr);
    */
}
pub fn register_all_demos()  {
    
    todo!();
    /*
        registerDemos_One();
        registerDemos_Two();
    */
}

/**
  | used by child-process demo
  |
  */
pub fn invoke_child_process_demo(command_line: &String) -> bool {
    
    todo!();
    /*
    
    */
}

lazy_static!{
    /*
    extern std::unique_ptr<AudioDeviceManager> sharedAudioDeviceManager;
    */
}

pub fn get_global_command_manager<'a>() -> &'a mut ApplicationCommandManager<'a> {
    
    todo!();
    /*
    
    */
}
