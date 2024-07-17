crate::ix!();

pub trait HandleNewParameterValue {
    fn handle_new_parameter_value(&mut self);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_GenericAudioProcessorEditor.h]

/**
  | A type of UI component that displays
  | the parameters of an AudioProcessor
  | as a simple list of sliders, combo boxes
  | and switches.
  | 
  | This can be used for showing an editor
  | for a processor that doesn't supply
  | its own custom editor.
  | 
  | @see AudioProcessor
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct GenericAudioProcessorEditor<'a> {
    base:  AudioProcessorEditor<'a>,
    pimpl: Box<GenericAudioProcessorEditorPimpl<'a>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_GenericAudioProcessorEditor.cpp]
impl<'a> GenericAudioProcessorEditor<'a> {

    pub fn new(p: &mut AudioProcessor) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (p), pimpl (new GenericAudioProcessorEditorPimpl (*this))

        setSize (pimpl->view.getViewedComponent()->getWidth() + pimpl->view.getVerticalScrollBar().getWidth(),
                 jmin (pimpl->view.getViewedComponent()->getHeight(), 400));
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
            pimpl->resize (getLocalBounds());
        */
    }
}
