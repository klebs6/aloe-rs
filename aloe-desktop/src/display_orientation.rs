crate::ix!();

/**
  | In a tablet/mobile device which can
  | be turned around, this is used to indicate
  | the orientation.
  |
  */
pub enum DesktopDisplayOrientation
{
    /**
      | Indicates that the device is the normal
      | way up.
      |
      */
    upright                 = 1,  

    /**
      | Indicates that the device is upside-down.
      |
      */
    upsideDown              = 2,  

    /**
      | Indicates that the device is turned
      | 90 degrees clockwise from its upright
      | position.
      |
      */
    rotatedClockwise        = 4,  

    /**
      | Indicates that the device is turned
      | 90 degrees anti-clockwise from its
      | upright position.
      |
      */
    rotatedAntiClockwise    = 8,  

    /**
      | A combination of all the orientation
      | values
      |
      */
    allOrientations         = 1 + 2 + 4 + 8,   
}
