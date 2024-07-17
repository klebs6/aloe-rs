crate::ix!();

#[repr(i32)]
pub enum AudioProcessorCurveDataType
{
    /**
      | an EQ curve - input is in Hz, output is
      | in dB
      |
      */
    EQ,             

    /**
      | a dynamics curve - input and output is
      | in dB
      |
      */
    Dynamics,       

    /**
      | a gain reduction curve - input and output
      | is in dB
      |
      */
    GainReduction,  

    Unknown = -1
}

/**
  | Some plug-ins support sharing response
  | curve data with the host so that it can
  | display this curve on a console or in
  | the mixer panel. For example, ProTools
  | allows you to see the total EQ curve of
  | a track. It does this by interrogating
  | each plug-in for their internal EQ curve.
  |
  */
pub struct AudioProcessorCurveData {

    /**
      | a function which represents your curve
      | (such as an eq)
      |
      */
    curve:     fn(_0: f32) -> f32,

    /**
      | the data range of your curve
      |
      */
    x_range:   Range<f32>,
    y_range:   Range<f32>,

    /**
     | For some curve types, your plug-in may
     | already measure the current input and
     | output values.  An host can use to
     | indicate where on the curve the current
     | signal is (for example by putting a dot on
     | the curve). Simply leave these strings empty
     | if you do not want to support this.
     */
    x_meterid: String,

    /** 
     | For some curve types, your plug-in may
     | already measure the current input and
     | output values.  An host can use to
     | indicate where on the curve the current
     | signal is (for example by putting a dot on
     | the curve). Simply leave these strings
     | empty if you do not want to support this.
     */
    y_meterid: String,
}
