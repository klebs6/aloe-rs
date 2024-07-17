crate::ix!();

pub trait AccessibilityHandlerInterface {

    /**
      | The title of the UI element.
      | 
      | This will be read out by the system and
      | should be concise, preferably matching
      | the visible title of the UI element (if
      | any). For example, this might be the
      | text of a button or a simple label.
      | 
      | The default implementation will call
      | `Component::getTitle()`, but you
      | can override this to return a different
      | string if required.
      | 
      | If neither a name nor a description is
      | provided then the UI element may be ignored
      | by accessibility clients.
      | 
      | This must be a localised string.
      |
      */
    fn get_title(&self) -> String {

        todo!();
        /*
           return component.getTitle();
           */
    }

    /**
      | A short description of the UI element.
      | 
      | This may be read out by the system. It
      | should not include the type of the UI
      | element and should ideally be a single
      | word, for example "Open" for a button
      | that opens a window.
      | 
      | The default implementation will call
      | `Component::getDescription()`,
      | but you can override this to return a
      | different string if required.
      | 
      | If neither a name nor a description is
      | provided then the UI element may be ignored
      | by accessibility clients.
      | 
      | This must be a localised string.
      |
      */
    fn get_description(&self) -> String {

        todo!();
        /*
           return component.getDescription();
           */
    }

    /**
      | Some help text for the UI element (if
      | required).
      | 
      | This may be read out by the system. This
      | string functions in a similar way to
      | a tooltip, for example "Click to open
      | window." for a button which opens a window.
      | 
      | The default implementation will call
      | `Component::getHelpText()`, but
      | you can override this to return a different
      | string if required.
      | 
      | This must be a localised string.
      |
      */
    fn get_help(&self) -> String {

        todo!();
        /*
           return component.getHelpText();
           */
    }

    /**
      | Returns the current state of the UI element.
      | 
      | The default implementation of this
      | method will set the focusable flag and,
      | if this UI element is currently focused,
      | will also set the focused flag.
      |
      */
    fn get_current_state(&self) -> AccessibleState;
}

//------------------------------------------------
/**
  | Utility struct which holds one or more
  | accessibility interfaces.
  | 
  | The main purpose of this class is to provide
  | convenience constructors from each
  | of the four types of accessibility interface.
  |
  */
pub struct AccessibilityHandlerInterfaces {
    value: Box<dyn AccessibilityValueInterface>,
    text:  Box<dyn AccessibilityTextInterface>,
    table: Box<dyn AccessibilityTableInterface>,
    cell:  Box<dyn AccessibilityCellInterface>,
}

impl Default for AccessibilityHandlerInterfaces {

    fn default() -> Self {
        todo!();
    }
}

impl AccessibilityHandlerInterfaces {

    pub fn new_from_value_interface(ptr: Box<dyn AccessibilityValueInterface>) -> Self {
    
        todo!();
        /*
        : value(std::move (ptr)),

        
        */
    }

    pub fn new_from_text_interface(ptr: Box<dyn AccessibilityTextInterface>) -> Self {
    
        todo!();
        /*
        : text(std::move (ptr)),

        
        */
    }

    pub fn new_from_table_interface(ptr: Box<dyn AccessibilityTableInterface>) -> Self {
    
        todo!();
        /*
        : table(std::move (ptr)),

        
        */
    }

    pub fn new_from_cell_interface(ptr: Box<dyn AccessibilityCellInterface>) -> Self {
    
        todo!();
        /*
        : cell(std::move (ptr)),

        
        */
    }

    pub fn new_from_interfaces(
        value_in: Box<dyn AccessibilityValueInterface>,
        text_in:  Box<dyn AccessibilityTextInterface>,
        table_in: Box<dyn AccessibilityTableInterface>,
        cell_in:  Box<dyn AccessibilityCellInterface>) -> Self {
    
        todo!();
        /*
        : value(std::move (valueIn)),
        : text(std::move (textIn)),
        : table(std::move (tableIn)),
        : cell(std::move (cellIn)),

        
        */
    }
}
