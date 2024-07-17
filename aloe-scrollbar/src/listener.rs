crate::ix!();

/**
  | A class for receiving events from a ScrollBar.
  | 
  | You can register a ScrollBar::ScrollBarListener
  | with a ScrollBar using the ScrollBar::addListener()
  | method, and it will be called when the
  | bar's position changes.
  | 
  | @see ScrollBar::addListener, ScrollBar::removeListener
  |
  */
pub trait ScrollBarListener {

    /**
      | Called when a ScrollBar is moved.
      | 
      | -----------
      | @param scrollBarThatHasMoved
      | 
      | the bar that has moved
      | ----------
      | @param newRangeStart
      | 
      | the new range start of this bar
      |
      */
    fn scroll_bar_moved(
        &mut self, 
        scroll_bar_that_has_moved: *mut ScrollBar,
        new_range_start:           f64
    );
}
