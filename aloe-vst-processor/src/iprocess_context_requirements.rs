crate::ix!();

pub enum IProcessContextRequirementsFlags
{
    kNeedSystemTime             = 1 <<  0, // kSystemTimeValid
    kNeedContinousTimeSamples   = 1 <<  1, // kContTimeValid
    kNeedProjectTimeMusic       = 1 <<  2, // kProjectTimeMusicValid
    kNeedBarPositionMusic       = 1 <<  3, // kBarPositionValid
    kNeedCycleMusic             = 1 <<  4, // kCycleValid
    kNeedSamplesToNextClock     = 1 <<  5, // kClockValid
    kNeedTempo                  = 1 <<  6, // kTempoValid
    kNeedTimeSignature          = 1 <<  7, // kTimeSigValid
    kNeedChord                  = 1 <<  8, // kChordValid
    kNeedFrameRate              = 1 <<  9, // kSmpteValid
    kNeedTransportState         = 1 << 10, // kPlaying, kCycleActive, kRecording
}

/**
  | Extended IAudioProcessor interface
  | for a component: Vst::IProcessContextRequirements
  | \ingroup vstIPlug vst370
  | 
  | - [plug imp]
  | 
  | - [extends IAudioProcessor]
  | 
  | - [released: 3.7.0]
  | 
  | - [mandatory]
  | 
  | To get accurate process context information
  | (Vst::ProcessContext), it is now required
  | to implement this interface and return
  | the desired bit mask of flags which your
  | audio effect needs. If you do not implement
  | this interface, you may not get any information
  | at all of the process function.
  | 
  | The host asks for this information once
  | between initialize and setActive.
  | It cannot be changed afterwards.
  | 
  | This gives the host the opportunity
  | to better optimize the audio process
  | graph when it knows which plug-ins need
  | which information.
  | 
  | Plug-Ins built with an earlier SDK version
  | (< 3.7) will still get the old information,
  | but the information may not be as accurate
  | as when using this interface.
  |
  */
pub trait IProcessContextRequirements: FUnknown {

    #[PLUGIN_API]
    fn get_process_context_requirements(&mut self) -> u32;
}

lazy_static!{
    /*
    static const FUID iprocess_context_requirements_iid;
    */
}

declare_class_iid!{
    IProcessContextRequirements, 
    0x2A654303, 
    0xEF764E3D, 
    0x95B5FE83, 
    0x730EF6D0
}
