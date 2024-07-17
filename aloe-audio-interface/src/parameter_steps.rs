crate::ix!();

pub trait GetDefaultNumParameterSteps {

    /**
      | Returns the default number of steps
      | for a parameter.
      | 
      | NOTE! This method is deprecated! It's
      | recommended that you use
      | 
      | AudioProcessorParameter::getNumSteps()
      | instead.
      | 
      | @see getParameterNumSteps
      |
      */
    fn get_default_num_parameter_steps() -> i32 where Self: Sized;
}
