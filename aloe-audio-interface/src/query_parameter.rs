crate::ix!();

pub trait GetParameterID {

    fn get_parameterid(&mut self, parameter_index: i32) -> String;
}

pub trait GetParameter {

    fn get_parameter(&mut self, parameter_index: i32) -> f32;
}

pub trait SetParameter {

    fn set_parameter(
        &mut self, 
        parameter_index: i32,
        new_value:       f32
    );
}

pub trait GetParameterName {

    fn get_parameter_name(&mut self, parameter_index: i32) -> String;

    fn get_parameter_name_with_maxlen(
        &mut self, 
        parameter_index:       i32,
        maximum_string_length: i32
    ) -> String;
}

pub trait GetParameterText {

    fn get_parameter_text(&mut self, parameter_index: i32) -> String;

    fn get_parameter_text_with_maxlen(
        &mut self, 
        parameter_index:       i32,
        maximum_string_length: i32
    ) -> String;
}

pub trait GetParameterDefaultValue {

    fn get_parameter_default_value(&mut self, parameter_index: i32) -> f32;
}

pub trait GetParameterNumSteps {

    fn get_parameter_num_steps(&mut self, parameter_index: i32) -> i32;
}

pub trait IsParameterDiscrete {

    fn is_parameter_discrete(&self, parameter_index: i32) -> bool;
}

pub trait IsParameterAutomatable {

    fn is_parameter_automatable(&self, parameter_index: i32) -> bool;
}

pub trait GetParameterLabel {

    fn get_parameter_label(&self, parameter_index: i32) -> String;
}

pub trait IsParameterOrientationInverted {

    fn is_parameter_orientation_inverted(&self, parameter_index: i32) -> bool;
}

pub trait IsMetaParameter {

    fn is_meta_parameter(&self, parameter_index: i32) -> bool;
}

pub trait GetParameterCategory {

    fn get_parameter_category(&self, parameter_index: i32) -> AudioProcessorParameterCategory;
}
