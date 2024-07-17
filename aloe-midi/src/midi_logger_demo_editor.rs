crate::ix!();

pub struct MidiLoggerPluginDemoProcessorEditor<'a> {
    owner:          &'a mut MidiLoggerPluginDemoProcessor<'a>,
    table:          MidiTable<'a>,
    clear_button:   TextButton<'a>, // default = { "Clear"  }
    last_ui_width:  Value<'a>,
    last_ui_height: Value<'a>,
}

impl<'a> AudioProcessorEditorInterface for MidiLoggerPluginDemoProcessorEditor<'a> {

}

impl<'a> CreateNewPeer                      for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SetScaleFactor                     for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> UpdatePeer                         for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> EditorResized                      for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SetBoundsConstrained               for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> AttachResizableCornerComponent     for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> AttachConstrainer                  for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SetConstrainer                     for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SetResizeLimits                    for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SetResizable                       for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> Initialise                         for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> HostMidiControllerIsAvailable      for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SupportsHostMidiControllerPresence for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> GetControlParameterIndex           for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SetControlHighlight                for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> AudioProcessorEditorFactory        for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> SetHostContext                     for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> GetHostContext                     for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> GetConstrainer                     for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> IsResizable                        for MidiLoggerPluginDemoProcessorEditor<'a> { }
impl<'a> GetAudioProcessor                  for MidiLoggerPluginDemoProcessorEditor<'a> { }

impl<'a> ValueListener for MidiLoggerPluginDemoProcessorEditor<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            setSize (lastUIWidth.getValue(), lastUIHeight.getValue());
        */
    }
}

impl<'a> MidiLoggerPluginDemoProcessorEditor<'a> {

    pub fn new(owner_in: &mut MidiLoggerPluginDemoProcessor) -> Self {
    
        todo!();
        /*
        : audio_processor_editor(ownerIn),
        : owner(ownerIn),
        : table(owner.model),

            addAndMakeVisible (table);
                addAndMakeVisible (clearButton);

                setResizable (true, true);
                lastUIWidth .referTo (owner.state.getChildWithName ("uiState").getPropertyAsValue ("width",  nullptr));
                lastUIHeight.referTo (owner.state.getChildWithName ("uiState").getPropertyAsValue ("height", nullptr));
                setSize (lastUIWidth.getValue(), lastUIHeight.getValue());

                lastUIWidth. addListener (this);
                lastUIHeight.addListener (this);

                clearButton.onClick = [&] { owner.model.clear(); };
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

                clearButton.setBounds (bounds.removeFromBottom (30).withSizeKeepingCentre (50, 24));
                table.setBounds (bounds);

                lastUIWidth  = getWidth();
                lastUIHeight = getHeight();
        */
    }
}
