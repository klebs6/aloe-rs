crate::ix!();

lazy_static!{
    /*
    createUnityPeerFunctionType aloe_createUnityPeerFn = nullptr;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorEditor.h]

/**
  | Base class for the component that acts
  | as the GUI for an AudioProcessor.
  | 
  | Derive your editor component from this
  | class, and create an instance of it by
  | overriding the AudioProcessor::createEditor()
  | method.
  | 
  | @see AudioProcessor, GenericAudioProcessorEditor
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
pub struct AudioProcessorEditor<'a> {

    base:      Component<'a>,

    /**
      | The AudioProcessor that this editor
      | represents.
      |
      */
    processor: &'a mut dyn AudioProcessorInterface,

    /**
      | The ResizableCornerComponent which
      | is currently being used by this editor,
      | or nullptr if it does not have one.
      |
      */
    resizable_corner:     Box<ResizableCornerComponent<'a>>,

    resize_listener:      Box<AudioProcessorEditorListener<'a>>,
    resizable_by_host:    bool, // default = false
    default_constrainer:  ComponentBoundsConstrainer,
    constrainer:          *mut ComponentBoundsConstrainer, // default = nullptr
    host_context:         *mut dyn AudioProcessorEditorHostContext, // default = nullptr
    splash_screen:        ComponentSafePointer<'a, Component<'a>>,
    host_scale_transform: AffineTransform,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorEditor.cpp]
impl<'a> Drop for AudioProcessorEditor<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            splashScreen.deleteAndZero();

        // if this fails, then the wrapper hasn't called editorBeingDeleted() on the
        // filter for some reason..
        jassert (processor.getActiveEditor() != this);
        removeComponentListener (resizeListener.get());
        */
    }
}

impl<'a> GetAudioProcessor                  for AudioProcessorEditor<'a> { }
impl<'a> IsResizable                        for AudioProcessorEditor<'a> { }
impl<'a> GetConstrainer                     for AudioProcessorEditor<'a> { }
impl<'a> GetHostContext                     for AudioProcessorEditor<'a> { }
impl<'a> SetHostContext                     for AudioProcessorEditor<'a> { }
impl<'a> AudioProcessorEditorFactory        for AudioProcessorEditor<'a> { }
impl<'a> SetControlHighlight                for AudioProcessorEditor<'a> { }
impl<'a> GetControlParameterIndex           for AudioProcessorEditor<'a> { }
impl<'a> SupportsHostMidiControllerPresence for AudioProcessorEditor<'a> { }
impl<'a> HostMidiControllerIsAvailable      for AudioProcessorEditor<'a> { }
impl<'a> Initialise                         for AudioProcessorEditor<'a> { }
impl<'a> SetResizable                       for AudioProcessorEditor<'a> { }
impl<'a> SetResizeLimits                    for AudioProcessorEditor<'a> { }
impl<'a> SetConstrainer                     for AudioProcessorEditor<'a> { }
impl<'a> AttachConstrainer                  for AudioProcessorEditor<'a> { }
impl<'a> AttachResizableCornerComponent     for AudioProcessorEditor<'a> { }
impl<'a> SetBoundsConstrained               for AudioProcessorEditor<'a> { }
impl<'a> EditorResized                      for AudioProcessorEditor<'a> { }
impl<'a> UpdatePeer                         for AudioProcessorEditor<'a> { }
impl<'a> CreateNewPeer                      for AudioProcessorEditor<'a> { }
impl<'a> SetScaleFactor                     for AudioProcessorEditor<'a> { }
