crate::ix!();

impl AUBaseInterface for AUBase {

}

impl AUPostConstructor for AUBase {

    fn post_constructor(&mut self) {}
}

impl AUPostDestructor for AUBase {

    fn post_destructor(&mut self) {}
}

impl AUCreateExtendedElements for AUBase {

    fn create_extended_elements(&mut self) {}
}

impl AUInitialize                 for AUBase {}
impl AUCleanup                    for AUBase {}
impl AUReset                      for AUBase {}
impl AUGetPropertyInfo            for AUBase {}
impl AUGetProperty                for AUBase {}
impl AUSetProperty                for AUBase {}
impl AURemovePropertyValue        for AUBase {}
impl AUAddPropertyListener        for AUBase {}
impl AURemovePropertyListener     for AUBase {}
impl AUSetRenderNotification      for AUBase {}
impl AURemoveRenderNotification   for AUBase {}
impl AUGetParameter               for AUBase {}
impl AUSetParameter               for AUBase {}
impl AUScheduleParameter          for AUBase {}
impl AUProcessBufferLists         for AUBase {}
impl AUProcessMultipleBufferLists for AUBase {}
impl AUComplexRender              for AUBase {}
impl AURenderBus                  for AUBase {}
impl AURender                     for AUBase {}
impl AUBusCountWritable           for AUBase {}
impl AUSetBusCount                for AUBase {}
impl AUSetConnection              for AUBase {}
impl AUSetInputCallback           for AUBase {}
impl AUGetParameterList           for AUBase {}
impl AUGetParameterInfo           for AUBase {}
impl GetParameterHistoryInfo      for AUBase {}
impl SaveState                    for AUBase {}
impl SaveExtendedScopes           for AUBase {}
impl RestoreState                 for AUBase {}
impl GetParameterValueStrings     for AUBase {}
impl CopyClumpName                for AUBase {}
impl GetPresets                   for AUBase {}
impl NewFactoryPresetSet          for AUBase {}
impl NewCustomPresetSet           for AUBase {}
impl GetNumCustomUiComponents     for AUBase {}
impl GetUiComponentDescs          for AUBase {}
impl CopyIconLocation             for AUBase {}
impl GetLatency                   for AUBase {}
impl GetTailTime                  for AUBase {}
impl SupportsTail                 for AUBase {}
impl SupportedNumChannels         for AUBase {}
impl ValidFormat                  for AUBase {}
impl GetStreamFormat              for AUBase {}
impl ChangeStreamFormat           for AUBase {}
impl GetScopeExtended             for AUBase {}
impl PropertyChanged              for AUBase {}
impl CreateElement                for AUBase {}
impl Start                        for AUBase {}
impl Stop                         for AUBase {}
impl PrepareInstrument            for AUBase {}
impl ReleaseInstrument            for AUBase {}
impl MidiEvent                    for AUBase {}
impl SysEx                        for AUBase {}
impl StartNote                    for AUBase {}
impl StopNote                     for AUBase {}
impl ReallocateBuffers            for AUBase {}
impl DeallocateIoBuffers          for AUBase {}
impl SetMaxFramesPerSlice         for AUBase {}
impl CanSetMaxFrames              for AUBase {}
impl GetChannelLayoutTags         for AUBase {}
impl GetAudioChannelLayout        for AUBase {}
impl SetAudioChannelLayout        for AUBase {}
impl RemoveAudioChannelLayout     for AUBase {}
impl ProcessForScheduledParams    for AUBase {}
impl ProcessScheduledSlice        for AUBase {}
