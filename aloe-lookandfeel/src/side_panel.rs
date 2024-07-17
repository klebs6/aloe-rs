crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide
  | 
  | SidePanel drawing functionality.
  |
  */
pub trait SidePanelLookAndFeelMethods {

    fn get_side_panel_title_font(&mut self, _0: &mut SidePanel) -> Font;

    fn get_side_panel_title_justification(&mut self, _0: &mut SidePanel) -> Justification;

    fn get_side_panel_dismiss_button_shape(&mut self, _0: &mut SidePanel) -> Path;
}

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the SidePanel.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum SidePanelColourIds
{
    backgroundColour          = 0x100f001,
    titleTextColour           = 0x100f002,
    shadowBaseColour          = 0x100f003,
    dismissButtonNormalColour = 0x100f004,
    dismissButtonOverColour   = 0x100f005,
    dismissButtonDownColour   = 0x100f006
}

