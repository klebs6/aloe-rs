crate::ix!();

/**
  | Controller Parameter Info.
  | 
  | A parameter info describes a parameter
  | of the controller.
  | 
  | The id must always be the same for a parameter
  | as this uniquely identifies the parameter.
  |
  */
bitflags!{
    pub struct ParameterInfoParameterFlags: u32
    {
        /**
          | no flags wanted
          |
          */
        const NoFlags         = 0;       

        /**
          | parameter can be automated
          |
          */
        const CanAutomate     = 1 << 0;  

        /**
          | parameter cannot be changed from outside
          | the plug-in (implies that kCanAutomate
          | is NOT set)
          */
        const IsReadOnly      = 1 << 1;  

        /**
          | attempts to set the parameter value out
          | of the limits will result in a wrap
          | around [SDK 3.0.2]
          */
        const IsWrapAround    = 1 << 2;  

        /**
          | parameter should be displayed as list in
          | generic editor or automation editing
          | [SDK 3.1.0]
          */
        const IsList          = 1 << 3;  

        /**
          | parameter should be NOT displayed and
          | cannot be changed from outside the
          | plug-in < (implies that kCanAutomate is
          | NOT set and kIsReadOnly is set) [SDK
          | 3.7.0]
          */
        const IsHidden        = 1 << 4;  

        /**
          | parameter is a program change (unitId
          | gives info about associated unit < - see
          | \ref vst3ProgramLists)
          */
        const IsProgramChange = 1 << 15; 

        /**
          | special bypass parameter (only one
          | allowed): plug-in can handle bypass
          | < (highly recommended to export a bypass
          | parameter for effect plug-in)
          */
        const IsBypass        = 1 << 16;  
    }
}
