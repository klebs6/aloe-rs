crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstparameterfunctionname.h]

pub const FUNCTION_NAME_TYPE_COMP_GAIN_REDUCTION:           &'static str = "Comp:GainReduction";
pub const FUNCTION_NAME_TYPE_COMP_GAIN_REDUCTION_MAX:       &'static str = "Comp:GainReductionMax";
pub const FUNCTION_NAME_TYPE_COMP_GAIN_REDUCTION_PEAK_HOLD: &'static str = "Comp:GainReductionPeakHold";
pub const FUNCTION_NAME_TYPE_COMP_RESET_GAIN_REDUCTION_MAX: &'static str = "Comp:ResetGainReductionMax";

/**
  | Useful for live situation where low
  | latency is required:
  | 
  | 0 means LowLatency disable,
  | 
  | 1 means LowLatency enable
  |
  */
pub const FUNCTION_NAME_TYPE_LOW_LATENCY_MODE: &'static str = "LowLatencyMode";

/**
  | Allowing to mix the original (Dry) Signal
  | with the processed one (Wet):
  | 
  | 0.0 means Dry Signal only,
  | 
  | 0.5 means 50% Dry Signal + 50% Wet Signal,
  | 
  | 1.0 means Wet Signal only
  |
  */
pub const FUNCTION_NAME_TYPE_DRY_WET_MIX: &'static str = "DryWetMix";

/**
  | Allow to assign some randomized values
  | to some parameters in a controlled way
  |
  */
pub const FUNCTION_NAME_TYPE_RANDOMIZE: &'static str = "Randomize";
