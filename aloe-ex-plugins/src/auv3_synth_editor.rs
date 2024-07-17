crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AUv3SynthEditor<'a> {
    base:                   AudioProcessorEditor<'a>,
    base2:                  Timer,
    material_look_and_feel: MaterialLookAndFeel<'a>,
    record_button:          TextButton<'a>, // default = { "Record"  }
    room_size_slider:       Slider<'a>,
    pro_audio_icon:         DrawablePath<'a>,
}

impl<'a> Drop for AUv3SynthEditor<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            setLookAndFeel (nullptr);
         */
    }
}

pub trait StartRecording {

    fn start_recording(&mut self);
}

impl<'a> StartRecording for AUv3SynthEditor<'a> {

    fn start_recording(&mut self)  {
        
        todo!();
        /*
            recordButton.setEnabled (false);
            setParameterValue ("isRecording", 1.0f);
        */
    }
}

pub trait TimerCallback {

    fn timer_callback(&mut self);
}

impl<'a> TimerCallback for AUv3SynthEditor<'a> {

    fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto isRecordingNow = (getParameterValue ("isRecording") >= 0.5f);

            recordButton.setEnabled (! isRecordingNow);
            roomSizeSlider.setValue (getParameterValue ("roomSize"), NotificationType::dontSendNotification);
        */
    }
}

pub trait GetParameterValue {

    fn get_parameter_value(&mut self, param_id: &String) -> f32;
}

impl<'a> GetParameterValue for AUv3SynthEditor<'a> {
    fn get_parameter_value(&mut self, param_id: &String) -> f32 {
        
        todo!();
        /*
            if (auto* param = getParameter (paramId))
                return param->getValue();

            return 0.0f;
        */
    }
}

pub trait Paint {

    fn paint(&mut self, g: &mut Graphics);
}

impl<'a> Paint for AUv3SynthEditor<'a> {
    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (ResizableWindow::backgroundColourId));
        */
    }
}

impl<'a> Resized for AUv3SynthEditor<'a> {
    fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

            auto guiElementAreaHeight = r.getHeight() / 3;

            proAudioIcon.setTransformToFit (r.removeFromLeft (proportionOfWidth (0.25))
                                             .withSizeKeepingCentre (guiElementAreaHeight, guiElementAreaHeight)
                                             .toFloat(),
                                            RectanglePlacement::fillDestination);

            auto margin = guiElementAreaHeight / 4;
            r.reduce (margin, margin);

            auto buttonHeight = guiElementAreaHeight - margin;

            recordButton  .setBounds (r.removeFromTop (guiElementAreaHeight).withSizeKeepingCentre (r.getWidth(), buttonHeight));
            roomSizeSlider.setBounds (r.removeFromTop (guiElementAreaHeight).withSizeKeepingCentre (r.getWidth(), buttonHeight));
        */
    }
}

pub trait GetParameter {

    fn get_parameter(&mut self, param_id: &String) -> *mut AudioProcessorParameter;
}

impl<'a> GetParameter for AUv3SynthEditor<'a> {

    fn get_parameter(&mut self, param_id: &String) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            if (auto* audioProcessor = getAudioProcessor())
            {
                auto& params = audioProcessor->getParameters();

                for (auto p : params)
                {
                    if (auto* param = dynamic_cast<AudioProcessorParameterWithID*> (p))
                    {
                        if (param->paramID == paramId)
                            return param;
                    }
                }
            }

            return nullptr;
        */
    }
}

pub trait SetParameterValue {

    fn set_parameter_value(
        &mut self, 
        param_id: &String,
        value:    f32
    );
}

impl<'a> SetParameterValue for AUv3SynthEditor<'a> {
    fn set_parameter_value(&mut self, 
        param_id: &String,
        value:    f32)  {
        
        todo!();
        /*
            if (auto* param = getParameter (paramId))
                param->setValueNotifyingHost (value);
        */
    }
}

impl<'a> AUv3SynthEditor<'a> {

    pub fn new(processor_in: &mut AudioProcessor) -> Self {
    
        todo!();
        /*
        : audio_processor_editor(processorIn),
        : room_size_slider(Slider::LinearHorizontal, Slider::NoTextBox),

            setLookAndFeel (&materialLookAndFeel);

            roomSizeSlider.setValue (getParameterValue ("roomSize"), NotificationType::dontSendNotification);

            recordButton.onClick = [this] { startRecording(); };
            addAndMakeVisible (recordButton);

            roomSizeSlider.onValueChange = [this] { setParameterValue ("roomSize", (float) roomSizeSlider.getValue()); };
            roomSizeSlider.setRange (0.0, 1.0);
            addAndMakeVisible (roomSizeSlider);

            if (auto fileStream = createAssetInputStream ("proaudio.path"))
            {
                Path proAudioPath;
                proAudioPath.loadPathFromStream (*fileStream);
                proAudioIcon.setPath (proAudioPath);
                addAndMakeVisible (proAudioIcon);

                auto proAudioIconColour = findColour (TextButton::buttonOnColourId);
                proAudioIcon.setFill (FillType (proAudioIconColour));
            }

            setSize (600, 400);
            startTimer (100);
        */
    }
}
