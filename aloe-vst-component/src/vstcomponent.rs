/**
   Description : Basic Vst Plug-in Implementation
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstcomponent.h]

/**
  | Default implementation for a Vst 3 Component.
  | \ingroup vstClasses
  | 
  | Can be used as base class for a Vst 3 component
  | implementation.
  |
  */
pub struct Component {
    base:             ComponentBase,
    controller_class: FUID,
    audio_inputs:     BusList,
    audio_outputs:    BusList,
    event_inputs:     BusList,
    event_outputs:    BusList,
}

impl FUnknown for Component {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut c_void) -> i32 { 
        todo!() 
    }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl IPluginBase for Component {

    #[PLUGIN_API]
    fn initialize(&mut self, context: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            return ComponentBase::initialize (context);
        */
    }
    
    #[PLUGIN_API]
    fn terminate(&mut self) -> tresult {
        
        todo!();
        /*
            // remove all busses
        removeAllBusses ();

        return ComponentBase::terminate ();
        */
    }
}

impl VstIComponent for Component {

    #[PLUGIN_API]
    fn get_controller_class_id(&mut self, classid: TUID) -> tresult {
        
        todo!();
        /*
            if (controllerClass.isValid ())
        {
            controllerClass.toTUID (classID);
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn set_io_mode(&mut self, mode: IoMode) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    fn get_bus_count(&mut self, 
        ty:  MediaType,
        dir: BusDirection) -> i32 {
        
        todo!();
        /*
            BusList* busList = getBusList (type, dir);
        return busList ? static_cast<int32> (busList->size ()) : 0;
        */
    }
    
    #[PLUGIN_API]
    fn get_bus_info(&mut self, 
        ty:    MediaType,
        dir:   BusDirection,
        index: i32,
        info:  &mut VstBusInfo) -> tresult {
        
        todo!();
        /*
            if (index < 0)
            return kInvalidArgument;
        BusList* busList = getBusList (type, dir);
        if (busList == nullptr)
            return kInvalidArgument;
        if (index >= static_cast<int32> (busList->size ()))
            return kInvalidArgument;

        Bus* bus = busList->at (index);
        info.mediaType = type;
        info.direction = dir;
        if (bus->getInfo (info))
            return kResultTrue;
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn get_routing_info(&mut self, 
        in_info:  &mut VstRoutingInfo,
        out_info: &mut VstRoutingInfo) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    fn activate_bus(&mut self, 
        ty:    MediaType,
        dir:   BusDirection,
        index: i32,
        state: TBool) -> tresult {
        
        todo!();
        /*
            if (index < 0)
            return kInvalidArgument;
        BusList* busList = getBusList (type, dir);
        if (busList == nullptr)
            return kInvalidArgument;
        if (index >= static_cast<int32> (busList->size ()))
            return kInvalidArgument;

        Bus* bus = busList->at (index);
        bus->setActive (state);
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn set_active(&mut self, state: TBool) -> tresult {
        
        todo!();
        /*
            return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    fn set_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    fn get_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
}

impl Default for Component {

    fn default() -> Self {
    
        todo!();
        /*
        : audio_inputs(kAudio, kInput),
        : audio_outputs(kAudio, kOutput),
        : event_inputs(kEvent, kInput),
        : event_outputs(kEvent, kOutput),
        */
    }
}

impl Component {

    /**
      | Sets the controller Class ID associated
      | to its component.
      |
      */
    pub fn set_controller_class(&mut self, cid: &FUID)  {
        
        todo!();
        /*
            controllerClass = cid;
        */
    }
    
    //---from ComponentBase------
    
    lazy_static!{
        /*
        OBJ_METHODS (Component, ComponentBase)
            DEFINE_INTERFACES
                DEF_INTERFACE (IComponent)
            END_DEFINE_INTERFACES (ComponentBase)
            REFCOUNT_METHODS (ComponentBase)
        */
    }
    
    pub fn get_bus_list(&mut self, 
        ty:  MediaType,
        dir: BusDirection) -> *mut BusList {
        
        todo!();
        /*
            if (type == kAudio)
            return dir == kInput ? &audioInputs : &audioOutputs;
        else if (type == kEvent)
            return dir == kInput ? &eventInputs : &eventOutputs;
        return nullptr;
        */
    }
    
    /**
      | Removes all Audio Busses.
      |
      */
    pub fn remove_audio_busses(&mut self) -> tresult {
        
        todo!();
        /*
            audioInputs.clear ();
        audioOutputs.clear ();

        return kResultOk;
        */
    }
    
    /**
      | Removes all Event Busses.
      |
      */
    pub fn remove_event_busses(&mut self) -> tresult {
        
        todo!();
        /*
            eventInputs.clear ();
        eventOutputs.clear ();

        return kResultOk;
        */
    }
    
    pub fn remove_all_busses(&mut self) -> tresult {
        
        todo!();
        /*
            removeAudioBusses ();
        removeEventBusses ();

        return kResultOk;
        */
    }
    
    /**
      | Renames a specific bus. Do not forget
      | to inform the host about this (see \ref
      | 
      | IComponentHandler::restartComponent
      | (kIoTitlesChanged)).
      |
      */
    pub fn rename_bus(&mut self, 
        ty:       MediaType,
        dir:      BusDirection,
        index:    i32,
        new_name: String128) -> tresult {
        
        todo!();
        /*
            if (index < 0)
            return kInvalidArgument;
        BusList* busList = getBusList (type, dir);
        if (busList == nullptr)
            return kInvalidArgument;
        if (index >= static_cast<int32> (busList->size ()))
            return kInvalidArgument;

        Bus* bus = busList->at (index);
        bus->setName (newName);
        return kResultTrue;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstcomponent.cpp]

/**
  | Gets the channel index of a given speaker
  | in a arrangement, returns kResultFalse
  | if speaker not part of the arrangement
  | else returns kResultTrue.
  |
  */
pub fn get_speaker_channel_index(
        arrangement: SpeakerArrangement,
        speaker:     u64,
        channel:     &mut i32) -> tresult {
    
    todo!();
        /*
            channel = SpeakerArr::getSpeakerIndex (speaker, arrangement);
        return (channel < 0) == true ? kResultFalse : kResultTrue;
        */
}
