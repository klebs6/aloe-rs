crate::ix!();

/**
  | A priority level that can help an accessibility
  | client determine how to handle an announcement
  | request.
  | 
  | Exactly what this controls is platform-specific,
  | but generally a low priority announcement
  | will be read when the screen reader is
  | free, whereas a high priority announcement
  | will interrupt the current speech.
  |
  */
pub enum AccessibilityHandlerAnnouncementPriority
{
    low,
    medium,
    high
}
