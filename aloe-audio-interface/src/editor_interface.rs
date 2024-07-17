crate::ix!();

pub trait AudioProcessorEditorInterface 
: GetAudioProcessor
+ IsResizable
+ GetConstrainer
+ GetHostContext
+ SetHostContext
+ AudioProcessorEditorFactory
+ SetControlHighlight
+ GetControlParameterIndex
+ SupportsHostMidiControllerPresence
+ HostMidiControllerIsAvailable
+ Initialise
+ SetResizable
+ SetResizeLimits
+ SetConstrainer
+ AttachConstrainer
+ AttachResizableCornerComponent
+ SetBoundsConstrained
+ EditorResized
+ UpdatePeer
+ SetScaleFactor
+ CreateNewPeer
{ }

pub trait AudioProcessorEditorFactory {

    /**
      | Creates an editor for the specified
      | processor.
      |
      */
    fn new_from_audio_processor_ref(p: &mut dyn AudioProcessorInterface) 
        -> Self where Self: Sized 
    {
        todo!();
        /*
        : processor(p),

        initialise();
        */
    }

    /**
      | Creates an editor for the specified
      | processor.
      |
      */
    fn new_from_audio_processor_ptr(p: *mut dyn AudioProcessorInterface) 
        -> Self where Self: Sized
    {
        todo!();
        /*
        : processor (*p)

        // the filter must be valid..
        jassert (p != nullptr);
        initialise();
        */
    }
}
