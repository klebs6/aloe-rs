crate::ix!();

pub trait SetControlHighlight {

    fn set_control_highlight(&mut self, _0: AudioProcessorEditorParameterControlHighlightInfo) {}
}

pub trait GetControlParameterIndex {

    fn get_control_parameter_index(&mut self, _0: &mut Component) -> i32 { 
        -1 
    }
}
