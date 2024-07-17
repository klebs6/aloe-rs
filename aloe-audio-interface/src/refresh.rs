crate::ix!();

pub trait RefreshParameterList {

    /**
      | A processor should implement this method
      | so that the host can ask it to rebuild
      | its parameter tree.
      | 
      | For most plug-ins it's enough to simply
      | add your parameters in the constructor
      | and leave this unimplemented.
      |
      */
    fn refresh_parameter_list(&mut self);
}
