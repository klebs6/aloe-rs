crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/enums/aloe_AccessibilityEvent.h]

/**
  | A list of events that can be notified
  | to any subscribed accessibility clients.
  | 
  | To post a notification, call `AccessibilityHandler::notifyAccessibilityEvent`
  | on the associated handler with the appropriate
  | `AccessibilityEvent` type and listening
  | clients will be notified.
  | 
  | @tags{Accessibility}
  |
  */
pub enum AccessibilityEvent
{
    /**
      | Indicates that the UI element's value
      | has changed.
      | 
      | This should be called on the handler
      | that implements `AccessibilityValueInterface`
      | for the UI element that has changed.
      |
      */
    valueChanged,

    /**
      | Indicates that the title of the UI element
      | has changed.
      | 
      | This should be called on the handler
      | whose title has changed.
      |
      */
    titleChanged,

    /**
      | Indicates that the structure of the
      | UI elements has changed in a significant
      | way.
      | 
      | This should be called on the top-level
      | handler whose structure has changed.
      |
      */
    structureChanged,

    /**
      | Indicates that the selection of a text
      | element has changed.
      | 
      | This should be called on the handler
      | that implements `AccessibilityTextInterface`
      | for the text element that has changed.
      |
      */
    textSelectionChanged,

    /**
      | Indicates that the visible text of a
      | text element has changed.
      | 
      | This should be called on the handler
      | that implements `AccessibilityTextInterface`
      | for the text element that has changed.
      |
      */
    textChanged,

    /**
      | Indicates that the selection of rows
      | in a list or table has changed.
      | 
      | This should be called on the handler
      | that implements `AccessibilityTableInterface`
      | for the UI element that has changed.
      |
      */
    rowSelectionChanged
}
