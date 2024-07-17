crate::ix!();
 
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel.h]

/**
  | LookAndFeel objects define the appearance
  | of all the Aloe widgets, and subclasses
  | can be used to apply different 'skins'
  | to the application.
  | 
  | This class is an abstract base-class
  | - for actual look-and-feels that you
  | can instantiate, see LookAndFeel_V1,
  | LookAndFeel_V2 and LookAndFeel_V3.
  | 
  | @see LookAndFeel_V1, LookAndFeel_V2,
  | LookAndFeel_V3
  | 
  | @tags{GUI}
  |
  */
pub trait LookAndFeel:
ScrollBarLookAndFeelMethods
+ ButtonLookAndFeelMethods
+ ImageButtonLookAndFeelMethods
+ TextEditorLookAndFeelMethods
+ FileBrowserComponentLookAndFeelMethods
+ TreeViewLookAndFeelMethods
+ BubbleComponentLookAndFeelMethods
+ AlertWindowLookAndFeelMethods
+ PopupMenuLookAndFeelMethods
+ ComboBoxLookAndFeelMethods
+ LabelLookAndFeelMethods
+ SliderLookAndFeelMethods
+ ResizableWindowLookAndFeelMethods
+ DocumentWindowLookAndFeelMethods
+ TooltipWindowLookAndFeelMethods
+ TabbedButtonBarLookAndFeelMethods
+ PropertyComponentLookAndFeelMethods
+ FilenameComponentLookAndFeelMethods
+ GroupComponentLookAndFeelMethods
+ TableHeaderComponentLookAndFeelMethods
+ CallOutBoxLookAndFeelMethods
+ ToolbarLookAndFeelMethods
+ ConcertinaPanelLookAndFeelMethods
+ ProgressBarLookAndFeelMethods
+ StretchableLayoutResizerBarLookAndFeelMethods
+ extra_look_and_feel_traits::KeyMappingEditorComponentMethods
+ extra_look_and_feel_traits::AudioDeviceSelectorComponentMethods
+ extra_look_and_feel_traits::LassoComponentMethods
+ SidePanelLookAndFeelMethods
+ HasLookAndFeelData<LookAndFeelData = LookAndFeelData>
{}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel.cpp]

pub fn get_typeface_for_font_from_look_and_feel(font: &Font) -> TypefacePtr {
    
    todo!();
    /*
        return LookAndFeel::getDefaultLookAndFeel().getTypefaceForFont (font);
    */
}

pub type get_typeface_for_font = fn(_0: &Font) -> TypefacePtr;

lazy_static!{
    /*
    extern GetTypefaceForFont aloe_getTypefaceForFont;
    */
}
