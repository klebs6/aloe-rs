crate::ix!();

/**
  | Options for the way items should be displayed.
  | @see setStyle, getStyle
  |
  */
pub enum ToolbarItemStyle
{
    /**
      | Means that the toolbar should just contain
      | icons.
      |
      */
    iconsOnly,       

    /**
      | Means that the toolbar should have text
      | labels under each icon.
      |
      */
    iconsWithText,   

    /**
      | Means that the toolbar only display
      | text labels for each item.
      |
      */
    textOnly,         
}

/**
  | Flags used by the showCustomisationDialog()
  | method.
  |
  */
pub enum ToolbarCustomisationFlags {

    /**
      | If this flag is specified, the customisation
      | dialog can show the "icons only" option
      | on its choice of toolbar styles.
      |
      */
    AllowIconsOnlyChoice,

    /**
      | If this flag is specified, the customisation
      | dialog can show the "icons with text"
      | option on its choice of toolbar styles.
      |
      */
    AllowIconsWithTextChoice,

    /**
      | If this flag is specified, the customisation
      | dialog can show the "text only" option
      | on its choice of toolbar styles.
      |
      */
    AllowTextOnlyChoice,

    /**
      | If this flag is specified, the customisation
      | dialog can show a button to reset the
      | toolbar to its default set of items.
      |
      */
    ShowResetToDefaultsButton,

    AllCustomisationOptionsEnabled,
}

impl ToolbarCustomisationFlags {
    pub fn value(&self) -> u32 {
        match self {
            Self::AllowIconsOnlyChoice      => 1,
            Self::AllowIconsWithTextChoice  => 2,
            Self::AllowTextOnlyChoice       => 4,
            Self::ShowResetToDefaultsButton => 8,
            Self::AllCustomisationOptionsEnabled => {
                Self::AllowIconsOnlyChoice.value() 
                | Self::AllowIconsWithTextChoice.value() 
                | Self::AllowTextOnlyChoice.value() 
                | Self::ShowResetToDefaultsButton.value()
            },
        }
    }
}
