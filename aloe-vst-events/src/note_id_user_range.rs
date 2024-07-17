crate::ix!();

/**
  | Reserved note identifier (noteId)
  | range for a plug-in. Guaranteed not
  | used by the host.
  |
  */
pub enum NoteIDUserRange
{
    kNoteIDUserRangeLowerBound = -10000, 
    kNoteIDUserRangeUpperBound = -1000,
}
