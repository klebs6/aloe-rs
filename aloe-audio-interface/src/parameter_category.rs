crate::ix!();

pub enum AudioProcessorParameterCategory
{
    /**
      | If your parameter is not a meter then
      | you should use this category
      |
      */
    genericParameter = (0 << 16) | 0,        

    /**
      | Currently not used
      |
      */
    inputGain        = (1 << 16) | 0,        
    outputGain       = (1 << 16) | 1,

    /**
      | The following categories tell the host
      | that this parameter is a meter level
      | value and therefore read-only. Most
      | hosts will display these type of parameters
      | as a meter in the generic view of your
      | plug-in. Pro-Tools will also show the
      | meter in the mixer view.
      |
      */
    inputMeter                          = (2 << 16) | 0,
    outputMeter                         = (2 << 16) | 1,
    compressorLimiterGainReductionMeter = (2 << 16) | 2,
    expanderGateGainReductionMeter      = (2 << 16) | 3,
    analysisMeter                       = (2 << 16) | 4,
    otherMeter                          = (2 << 16) | 5
}
