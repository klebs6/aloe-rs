crate::ix!();

/**
  | MpeZoneLayoutListener class. Derive from this class
  | to allow your class to be notified about
  | changes to the zone layout.
  |
  */
pub trait MpeZoneLayoutListener
{

    /**
      | Implement this callback to be notified
      | about any changes to this MPEZoneLayout.
      | Will be called whenever a zone is added,
      | zones are removed, or any zone's master
      | or note pitchbend ranges change.
      |
      */
    fn zone_layout_changed(&mut self, layout: &MPEZoneLayout);
}
