crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/Unity/aloe_UnityPluginInterface.h]

pub const UNITY_AUDIO_PLUGIN_API_VERSION: usize = 0x010401;

lazy_static!{
    /*
    #if ALOE_MSVC
     #define UNITY_INTERFACE_API __stdcall
     #define UNITY_INTERFACE_EXPORT __declspec(dllexport)
    #else
     #define UNITY_INTERFACE_API
     #define UNITY_INTERFACE_EXPORT __attribute__ ((visibility("default")))
    #endif
    */
}

pub type CreateCallback = fn(state: *mut UnityAudioEffectState) -> i32;

pub type ReleaseCallback = fn(state: *mut UnityAudioEffectState) -> i32;

pub type ResetCallback = fn(state: *mut UnityAudioEffectState) -> i32;

pub type ProcessCallback = fn(
        state:            *mut UnityAudioEffectState,
        in_buffer:        *mut f32,
        out_buffer:       *mut f32,
        buffer_size:      u32,
        num_in_channels:  i32,
        num_out_channels: i32
) -> i32;

pub type SetPositionCallback = fn(state: *mut UnityAudioEffectState, pos: u32) -> i32;

pub type SetFloatParameterCallback = fn(
        state: *mut UnityAudioEffectState,
        index: i32,
        value: f32
) -> i32;

pub type GetFloatParameterCallback = fn(
        state:    *mut UnityAudioEffectState,
        index:    i32,
        value:    *mut f32,
        valuestr: *mut u8
) -> i32;

pub type GetFloatBufferCallback = fn(
        state:      *mut UnityAudioEffectState,
        name:       *const u8,
        buffer:     *mut f32,
        numsamples: i32
) -> i32;

pub type DistanceAttenuationCallback = fn(
        state:           *mut UnityAudioEffectState,
        distance_in:     f32,
        attenuation_in:  f32,
        attenuation_out: *mut f32
) -> i32;

pub type RenderCallback = fn(event_id: i32) -> c_void;

pub enum UnityAudioEffectDefinitionFlags
{
    isSideChainTarget          = 1,
    isSpatializer              = 2,
    isAmbisonicDecoder         = 4,
    appliesDistanceAttenuation = 8
}

pub enum UnityAudioEffectStateFlags
{
    stateIsPlaying        = 1,
    stateIsPaused         = 2,
    stateIsMuted          = 8,
    statIsSideChainTarget = 16
}

pub enum UnityEventModifiers
{
    shift       = 1,
    control     = 2,
    alt         = 4,
    command     = 8,
    numeric     = 16,
    capsLock    = 32,
    functionKey = 64
}

pub struct UnityAudioSpatializerData
{
    listener_matrix:      [f32; 16],
    source_matrix:        [f32; 16],
    spatial_blend:        f32,
    reverb_zone_mix:      f32,
    spread:               f32,
    stereo_pan:           f32,
    attenuation_callback: DistanceAttenuationCallback,
    min_distance:         f32,
    max_distance:         f32,
}

pub struct UnityAudioAmbisonicData
{
    listener_matrix:        [f32; 16],
    source_matrix:          [f32; 16],
    spatial_blend:          f32,
    reverb_zone_mix:        f32,
    spread:                 f32,
    stereo_pan:             f32,
    attenuation_callback:   DistanceAttenuationCallback,
    ambisonic_out_channels: i32,
    volume:                 f32,
}

pub struct UnityAudioEffectState
{
    struct_size:       u32,
    sample_rate:       u32,
    dsp_current_tick:  u64,
    dsp_previous_tick: u64,
    sidechain_buffer:  *mut f32,
    effect_data:       *mut c_void,
    flags:             u32,
    internal:          *mut c_void,
    spatializer_data:  *mut UnityAudioSpatializerData,
    dsp_buffer_size:   u32,
    host_api_version:  u32,
    ambisonic_data:    *mut UnityAudioAmbisonicData,
}

impl UnityAudioEffectState {
    
    #[inline] pub fn get_effect_data<T>(&self) -> *mut T {
    
        todo!();
        /*
            jassert (effectData != nullptr);
            jassert (internal != nullptr);

            return (T*) effectData;
        */
    }
}

pub struct UnityAudioParameterDefinition
{
    name:             [u8; 16],
    unit:             [u8; 16],
    description:      *const u8,
    min:              f32,
    max:              f32,
    default_val:      f32,
    display_scale:    f32,
    display_exponent: f32,
}

pub struct UnityAudioEffectDefinition
{
    struct_size:           u32,
    parameter_struct_size: u32,
    api_version:           u32,
    plugin_version:        u32,
    channels:              u32,
    num_parameters:        u32,
    flags:                 u64,
    name:                  [u8; 32],
    create:                CreateCallback,
    release:               ReleaseCallback,
    reset:                 ResetCallback,
    process:               ProcessCallback,
    set_position:          SetPositionCallback,
    parameter_defintions:  *mut UnityAudioParameterDefinition,
    set_float_parameter:   SetFloatParameterCallback,
    get_float_parameter:   GetFloatParameterCallback,
    get_float_buffer:      GetFloatBufferCallback,
}

/**
   Unity callback
  */
extern "C"  {
    lazy_static!{
        /*
        UNITY_INTERFACE_EXPORT int  UNITY_INTERFACE_API UnityGetAudioEffectDefinitions (UnityAudioEffectDefinition*** definitionsPtr);

            // GUI script callbacks
            UNITY_INTERFACE_EXPORT renderCallback UNITY_INTERFACE_API getRenderCallback();

            UNITY_INTERFACE_EXPORT void UNITY_INTERFACE_API unityInitialiseTexture (int id, void* textureHandle, int w, int h);

            UNITY_INTERFACE_EXPORT void UNITY_INTERFACE_API unityMouseDown (int id, float x, float y, UnityEventModifiers mods, int button);
            UNITY_INTERFACE_EXPORT void UNITY_INTERFACE_API unityMouseDrag (int id, float x, float y, UnityEventModifiers mods, int button);
            UNITY_INTERFACE_EXPORT void UNITY_INTERFACE_API unityMouseUp   (int id, float x, float y, UnityEventModifiers mods);

            UNITY_INTERFACE_EXPORT void UNITY_INTERFACE_API unityKeyEvent (int id, int code, UnityEventModifiers mods, const char* name);

            UNITY_INTERFACE_EXPORT void UNITY_INTERFACE_API unitySetScreenBounds (int id, float x, float y, float w, float h);
        */
    }
}
