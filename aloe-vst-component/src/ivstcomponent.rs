/*!
  | All Vst specific interfaces are located
  | in Vst namespace
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstcomponent.h]

/**
  | Standard value for PFactoryInfo::flags
  |
  */
pub const Vst_DEFAULT_FACTORY_FLAGS: PFactoryInfoFactoryFlags = PFactoryInfoFactoryFlags::kUnicode;

macro_rules! vst_begin_factory_def {
    ($vendor:ident, 
     $url:ident, 
     $email:ident) => {
        /*
                using namespace Steinberg; 
            SMTG_EXPORT_SYMBOL IPluginFactory* PLUGIN_API GetPluginFactory () { 
            if (!gPluginFactory) 
            { 
                static PFactoryInfo factoryInfo (vendor, url, email, Vst::kDefaultFactoryFlags); 
                gPluginFactory = new CPluginFactory (factoryInfo);
        */
    }
}

/*
  | \defgroup vstBus Vst busses
  | 
  | Bus Description
  | 
  | A bus can be understood as a "collection
  | of data channels" belonging together.
  | 
  | It describes a data input or a data output
  | of the plug-in.
  | 
  | A Vst component can define any desired
  | number of busses.
  | 
  | Dynamic usage of busses is handled in
  | the host by activating and deactivating
  | busses.
  | 
  | All busses are initially inactive.
  | 
  | The component has to define the maximum
  | number of supported busses and it has
  | to define which of them have to be activated
  | by default after instantiation of the
  | plug-in (This is only a wish, the host
  | is allow to not follow it, and only activate
  | the first bus for example).
  | 
  | A host that can handle multiple busses,
  | allows the user to activate busses which
  | are initially all inactive.
  | 
  | The kMain busses have to place before
  | any others kAux busses.
  | 
  | See also: VstIComponent::getBusInfo,
  | VstIComponent::activateBus
  |
  */

/**
  | Bus media types
  |
  */
pub enum VstMediaTypes
{
    kAudio = 0,     // audio
    kEvent,         // events
    kNumMediaTypes
}

/**
  | Bus directions
  |
  */
pub enum VstBusDirections
{
    kInput = 0,     // input bus
    kOutput         // output bus
}

/**
  | Bus types
  |
  */
pub enum VstBusTypes
{
    kMain = 0,      // main bus
    kAux            // auxiliary bus (sidechain)
}

pub enum VstBusInfoBusFlags
{
    /**
      | The bus should be activated by the host
      | per default on instantiation (activateBus
      | call is requested).
      | 
      | By default a bus is inactive.
      |
      */
    kDefaultActive = 1 << 0,

    /**
      | The bus does not contain ordinary audio
      | data, but data used for control changes
      | at sample rate.
      | 
      | The data is in the same format as the audio
      | data [-1..1].
      | 
      | A host has to prevent unintended routing
      | to speakers to prevent damage.
      | 
      | Only valid for audio media type busses.
      | [released: 3.7.0]
      |
      */
    kIsControlVoltage = 1 << 1
}

pub enum VstIoModes
{
    kSimple = 0,        // 1:1 Input / Output. Only used for Instruments. See \ref vst3IoMode
    kAdvanced,          // n:m Input / Output. Only used for Instruments.
    kOfflineProcessing  // plug-in used in an offline processing context
}

/**
  | Routing Information:
  | 
  | When the plug-in supports multiple
  | I/O busses, a host may want to know how
  | the busses are related. The relation
  | of an event-input-channel to an audio-output-bus
  | in particular is of interest to the host
  | (in order to relate MIDI-tracks to audio-channels)
  | \n See also: VstIComponent::getRoutingInfo,
  | \ref vst3Routing
  |
  */
pub struct VstRoutingInfo
{
    /**
      | media type see \ref VstMediaTypes
      |
      */
    media_type: MediaType,

    /**
      | bus index
      |
      */
    bus_index:  i32,

    /**
      | channel (-1 for all channels)
      |
      */
    channel:    i32,
}

/**
  | Component base interface: Vst::VstIComponent
  | \ingroup vstIPlug vst300
  | 
  | - [plug imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | This is the basic interface for a Vst
  | component and must always be supported.
  | 
  | It contains the common parts of any kind
  | of processing class. The parts that
  | are specific to a media type are defined
  | in a separate interface. An implementation
  | component must provide both the specific
  | interface and VstIComponent. \see IPluginBase
  |
  */
pub trait VstIComponent: IPluginBase {

    /**
      | Called before initializing the component
      | to get information about the controller
      | class.
      |
      */
    #[PLUGIN_API]
    fn get_controller_class_id(&mut self, class_id: TUID) -> tresult;

    /**
      | Called before 'initialize' to set the
      | component usage (optional). See \ref
      | VstIoModes
      |
      */
    #[PLUGIN_API]
    fn set_io_mode(&mut self, mode: IoMode) -> tresult;

    /**
      | Called after the plug-in is initialized.
      | See \ref VstMediaTypes, VstBusDirections
      |
      */
    #[PLUGIN_API]
    fn get_bus_count(&mut self, 
            ty:  MediaType,
            dir: BusDirection) -> i32;

    /**
      | Called after the plug-in is initialized.
      | See \ref VstMediaTypes, VstBusDirections
      |
      */
    #[PLUGIN_API]
    fn get_bus_info(&mut self, 
            ty:    MediaType,
            dir:   BusDirection,
            index: i32,
            bus:   &mut VstBusInfo) -> tresult;

    /**
      | Retrieves routing information (to
      | be implemented when more than one regular
      | input or output bus exists).
      | 
      | The inInfo always refers to an input
      | bus while the returned outInfo must
      | refer to an output bus!
      |
      */
    #[PLUGIN_API]
    fn get_routing_info(&mut self, 
            in_info:  &mut VstRoutingInfo,
            out_info: &mut VstRoutingInfo) -> tresult;

    /**
      | Called upon (de-)activating a bus in
      | the host application. The plug-in should
      | only processed an activated bus, the
      | host could provide less see \ref AudioBusBuffers
      | in the process call (see \ref IAudioProcessor::process)
      | if last busses are not activated. An
      | already activated bus does not need
      | to be reactivated after a IAudioProcessor::setBusArrangements
      | call.
      |
      */
    #[PLUGIN_API]
    fn activate_bus(&mut self, 
            ty:    MediaType,
            dir:   BusDirection,
            index: i32,
            state: TBool) -> tresult;

    /**
      | Activates / deactivates the component.
      |
      */
    #[PLUGIN_API]
    fn set_active(&mut self, state: TBool) -> tresult;

    /**
      | Sets complete state of component.
      |
      */
    #[PLUGIN_API]
    fn set_state(&mut self, state: *mut dyn IBStream) -> tresult;

    /**
      | Retrieves complete state of component.
      |
      */
    #[PLUGIN_API]
    fn get_state(&mut self, state: *mut dyn IBStream) -> tresult;
}

lazy_static!{
    /*
    static const FUID vst_icomponent_iid;
    */
}

declare_class_iid!{
    VstIComponent, 
    0xE831FF31, 
    0xF2D54301, 
    0x928EBBEE, 
    0x25697802
}
