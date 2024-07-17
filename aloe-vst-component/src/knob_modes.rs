crate::ix!();

/**
  | Knob Mode
  |
  */
pub enum KnobModes
{
    /**
      | Circular with jump to clicked position
      |
      */
    CircularMode = 0,      

    /**
      | Circular without jump to clicked position
      |
      */
    RelativCircularMode,   

    /**
      | Linear: depending on vertical movement
      |
      */
    LinearMode             
}

/* ---- \defgroup vst3typedef VST 3 Data Types  ---- */

/**
  | Knob Mode Type
  |
  */
pub type KnobMode = i32;
