crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_LiveConstantEditor.h]

/**
  | This macro wraps a primitive constant
  | value in some cunning boilerplate code
  | that allows its value to be interactively
  | tweaked in a popup window while your
  | application is running.
  | 
  | In a release build, this macro disappears
  | and is replaced by only the constant
  | that it wraps, but if ALOE_ENABLE_LIVE_CONSTANT_EDITOR
  | is enabled, it injects a class wrapper
  | that automatically pops-up a window
  | containing an editor that allows the
  | value to be tweaked at run-time. The
  | editor window will also force all visible
  | components to be resized and repainted
  | whenever a value is changed, so that
  | if you use this to wrap a colour or layout
  | parameter, you'll be able to immediately
  | see the effects of changing it.
  | 
  | The editor will also load the original
  | source-file that contains each ALOE_LIVE_CONSTANT
  | macro, and will display a preview of
  | the modified source code as you adjust
  | the values.
  | 
  | Things to note:
  | 
  | - Only one of these per line! The __FILE__
  | and __LINE__ macros are used to identify
  | the value, so things will get confused
  | if you have more than one per line
  | 
  | - Obviously because it needs to load
  | the source code based on the __FILE__
  | macro, it'll only work if the source
  | files are stored locally in the same
  | location as they were when you compiled
  | the program.
  | 
  | - It's only designed to cope with simple
  | types: primitives, string literals,
  | and the Colour class, so if you try using
  | it for other classes or complex expressions,
  | good luck!
  | 
  | - The editor window will get popped up
  | whenever a new value is used for the first
  | time. You can close the window, but there's
  | no way to get it back without restarting
  | the app!
  | 
  | e.g. in this example the colours, font
  | size, and text used in the paint method
  | can all be adjusted live:
  | 
  | -----------
  | @code
  | 
  | void MyComp::paint (Graphics& g) override
  | {
  |     g.fillAll (ALOE_LIVE_CONSTANT (Colour (0xffddddff)));
  | 
  |     Colour fontColour = ALOE_LIVE_CONSTANT (Colour (0xff005500));
  |     float fontSize = ALOE_LIVE_CONSTANT (16.0f);
  | 
  |     g.setColour (fontColour);
  |     g.setFont (fontSize);
  | 
  |     g.drawFittedText (ALOE_LIVE_CONSTANT ("Hello world!"),
  |                       getLocalBounds(), Justification::centred, 2);
  | }
  |
  */
#[cfg(ALOE_ENABLE_LIVE_CONSTANT_EDITOR)]
macro_rules! aloe_live_constant {
    ($initialValue:ident) => {
        /*
        
            (LiveConstantEditor::getValue (__FILE__, __LINE__ - 1, initialValue).get())
        */
    }
}

#[cfg(not(ALOE_ENABLE_LIVE_CONSTANT_EDITOR))]
macro_rules! aloe_live_constant {
    ($initialValue:ident) => {
        /*
        
            (initialValue)
        */
    }
}
