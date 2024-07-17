crate::ix!();

pub trait MarkersChanged {

    /**
      | Called when something in the given marker
      | list changes.
      |
      */
    fn markers_changed(&mut self, marker_list: *mut MarkerList);
}

pub trait MarkerListBeingDeleted {

    /**
      | Called when the given marker list is
      | being deleted.
      |
      */
    fn marker_list_being_deleted(&mut self, marker_list: *mut MarkerList) {}
}

pub trait GetMarkers {

    /**
      | Objects can implement this method to
      | provide a MarkerList.
      |
      */
    fn get_markers(&mut self, x_axis: bool) -> *mut MarkerList;
}

/**
  | A class for receiving events when changes
  | are made to a MarkerList.
  | 
  | You can register a MarkerList::MarkerListListener
  | with a MarkerList using the MarkerList::addListener()
  | method, and it will be called when markers
  | are moved, added, or deleted.
  | 
  | @see MarkerList::addListener, MarkerList::removeListener
  |
  */
pub trait MarkerListListener: 
    MarkersChanged 
    + MarkerListBeingDeleted { }

/**
  | A base class for objects that want to
  | provide a MarkerList.
  |
  */
pub trait MarkerListHolder: GetMarkers { }
