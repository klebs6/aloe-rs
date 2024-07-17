crate::ix!();

/**
  | A struct containing information about
  | the DAW track inside which your
  | 
  | AudioProcessor is loaded.
  |
  */
pub struct AudioProcessorTrackProperties
{
    /**
      | The name of the track - this will be empty
      | if the track name is not known
      |
      */
    name:   String,

    /**
      | The colour of the track - this will be
      | transparentBlack if the colour is not
      | known
      |
      */
    colour: Colour,

    /*
       other properties may be added in the future
       */
}
