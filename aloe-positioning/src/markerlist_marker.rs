crate::ix!();

/**
  | Represents a marker in a MarkerList.
  |
  */
pub struct MarkerListMarker {

    /**
      | The marker's name.
      |
      */
    name:     String,

    /**
      | The marker's position.
      | 
      | The expression used to define the coordinate
      | may use the names of other markers, so
      | that markers can be linked in arbitrary
      | ways, but be careful not to create recursive
      | loops of markers whose positions are
      | based on each other! It can also refer
      | to "parent.right" and "parent.bottom"
      | so that you can set markers which are
      | relative to the size of the component
      | that contains them.
      | 
      | To resolve the coordinate, you can use
      | the MarkerList::getMarkerPosition()
      | method.
      |
      */
    position: RelativeCoordinate,
}

impl PartialEq<MarkerListMarker> for MarkerListMarker {
    
    /**
      | Returns true if both the names and positions
      | of these two markers match.
      |
      */
    #[inline] fn eq(&self, other: &MarkerListMarker) -> bool {
        todo!();
        /*
            return name == other.name && position == other.position;
        */

    }
}

impl Eq for MarkerListMarker {}

impl MarkerListMarker {
    
    /**
      | Creates a copy of another MarkerListMarker.
      |
      */
    pub fn new_marker_list_marker(other: &MarkerListMarker) -> Self {
    
        todo!();
        /*
        : name(other.name),
        : position(other.position),

        
        */

    }
    
    /**
      | Creates a MarkerListMarker with a given name and
      | position.
      |
      */
    pub fn new(
        name:     &String,
        position: &RelativeCoordinate) -> Self {
    
        todo!();
        /*
        : name(name_),
        : position(position_),

        
        */

    }
}
