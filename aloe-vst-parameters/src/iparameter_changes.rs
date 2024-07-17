crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstparameterchanges.h]

pub struct ParameterInfo
{
    /**
      | unique identifier of this parameter (named
      | tag too)
      */
    id:                       ParamID,

    /**
      | parameter title (e.g. "Volume")
      |
      */
    title:                    String128,

    /**
      | parameter shortTitle (e.g. "Vol")
      |
      */
    short_title:              String128,

    /**
      | parameter unit (e.g. "dB")
      |
      */
    units:                    String128,

    /**
      | number of discrete steps (0: continuous, 1:
      | toggle, discrete value otherwise
      | (corresponding to max - min, for example: 127
      | for a min = 0 and a max = 127) - see \ref
      | vst3ParameterIntro)
      */
    step_count:               i32,

    /**
      | default normalized value [0,1] (in case of
      | discrete value: defaultNormalizedValue
      | = defDiscreteValue / stepCount)
      */
    default_normalized_value: ParamValue,

    /**
      | id of unit this parameter belongs to
      | (see \ref vst3Units)
      |
      */
    unit_id:                  UnitID,

    /**
      | ParameterInfoParameterFlags (see
      | below)
      |
      */
    flags:                    i32,
}

/**
  | All parameter changes of a processing
  | block: Vst::IParameterChanges \ingroup
  | vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | This interface is used to transmit any
  | changes to be applied to parameters
  | in the current processing block. A change
  | can be caused by GUI interaction as well
  | as automation. They are transmitted
  | as a list of queues (\ref IParamValueQueue)
  | containing only queues for parameters
  | that actually did change.
  | 
  | See \ref IParamValueQueue, \ref ProcessData
  |
  */
pub trait IParameterChanges: FUnknown {

    /**
      | Returns count of Parameter changes
      | in the list.
      |
      */
    #[PLUGIN_API]
    fn get_parameter_count(&mut self) -> i32;

    /**
      | Returns the queue at a given index.
      |
      */
    #[PLUGIN_API]
    fn get_parameter_data(&mut self, index: i32) -> *mut dyn IParamValueQueue;

    /**
      | Adds a new parameter queue with a given
      | ID at the end of the list, returns it and
      | its index in the parameter changes list.
      |
      */
    #[PLUGIN_API]
    fn add_parameter_data(
        &mut self, 
        id:    &ParamID,
        index: &mut i32

    ) -> *mut dyn IParamValueQueue;
}

lazy_static!{
    /*
    static const FUID iparameter_changes_iid;
    */
}

declare_class_iid!{
    IParameterChanges, 
    0xA4779663, 
    0x0BB64A56, 
    0xB44384A8, 
    0x466FEB9D
}
