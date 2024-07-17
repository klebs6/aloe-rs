crate::ix!();

/**
  | The set of available button-types that
  | can be put on the title bar.
  | 
  | @see setTitleBarButtonsRequired
  |
  */
pub enum DocumentWindowTitleBarButtons
{
    minimiseButton = 1,
    maximiseButton = 2,
    closeButton = 4,

    /**
      | A combination of all the buttons above.
      |
      */
    allButtons = 7
}
