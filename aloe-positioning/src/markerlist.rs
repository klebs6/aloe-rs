crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_MarkerList.h]

/**
  | Holds a set of named marker points along
  | a one-dimensional axis.
  | 
  | This class is used to store sets of X and
  | Y marker points in components. @see
  | Component::getMarkers().
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct MarkerList {
    markers:   Vec<Box<MarkerListMarker>>,
    listeners: ListenerList<Box<dyn MarkerListListener>>,
}

impl Default for MarkerList {

    fn default() -> Self {
        todo!();
    }
}

impl Drop for MarkerList {

    fn drop(&mut self) {
        todo!();
        /*
            listeners.call ([this] (MarkerListListener& l) { l.markerListBeingDeleted (this); });
        */

    }
}

impl PartialEq<MarkerList> for MarkerList {
    
    /**
      | Returns true if all the markers in these
      | two lists match exactly.
      |
      */
    #[inline] fn eq(&self, other: &MarkerList) -> bool {
        todo!();
        /*
            if (other.markers.size() != markers.size())
            return false;

        for (int i = markers.size(); --i >= 0;)
        {
            const MarkerListMarker* const m1 = markers.getUnchecked(i);
            jassert (m1 != nullptr);

            const MarkerListMarker* const m2 = other.getMarker (m1->name);

            if (m2 == nullptr || *m1 != *m2)
                return false;
        }

        return true;
        */

    }
}

impl Eq for MarkerList {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_MarkerList.cpp]
impl MarkerList {
    
    pub fn new(other: &MarkerList) -> Self {
    
        todo!();
        /*


            operator= (other);
        */

    }
    
    pub fn assign_from(&mut self, other: &MarkerList) -> &mut MarkerList {
        
        todo!();
        /*
            if (other != *this)
        {
            markers.clear();
            markers.addCopiesOf (other.markers);
            markersHaveChanged();
        }

        return *this;
        */

    }
    
    /**
      | Returns the number of markers in the
      | list.
      |
      */
    pub fn get_num_markers(&self) -> i32 {
        
        todo!();
        /*
            return markers.size();
        */

    }
    
    /**
      | Returns one of the markers in the list,
      | by its index.
      |
      */
    pub fn get_marker(&self, index: i32) -> *const MarkerListMarker {
        
        todo!();
        /*
            return markers [index];
        */

    }
    
    /**
      | Returns a named marker, or nullptr if
      | no such name is found.
      | 
      | -----------
      | @note
      | 
      | name comparisons are case-sensitive.
      |
      */
    pub fn get_marker_with_name(&self, name: &String) -> *const MarkerListMarker {
        
        todo!();
        /*
            return getMarkerByName (name);
        */

    }
    
    pub fn get_marker_by_name(&self, name: &String) -> *mut MarkerListMarker {
        
        todo!();
        /*
            for (int i = 0; i < markers.size(); ++i)
        {
            MarkerListMarker* const m = markers.getUnchecked(i);

            if (m->name == name)
                return m;
        }

        return nullptr;
        */

    }
    
    /**
      | Sets the position of a marker.
      | 
      | If the name already exists, then the
      | existing marker is moved; if it doesn't
      | exist, then a new marker is added.
      |
      */
    pub fn set_marker(&mut self, 
        name:     &String,
        position: &RelativeCoordinate)  {
        
        todo!();
        /*
            if (MarkerListMarker* const m = getMarkerByName (name))
        {
            if (m->position != position)
            {
                m->position = position;
                markersHaveChanged();
            }

            return;
        }

        markers.add (new MarkerListMarker (name, position));
        markersHaveChanged();
        */

    }
    
    /**
      | Deletes the marker at the given list
      | index.
      |
      */
    pub fn remove_marker(&mut self, index: i32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, markers.size()))
        {
            markers.remove (index);
            markersHaveChanged();
        }
        */

    }
    
    /**
      | Deletes the marker with the given name.
      |
      */
    pub fn remove_marker_with_name(&mut self, name: &String)  {
        
        todo!();
        /*
            for (int i = 0; i < markers.size(); ++i)
        {
            const MarkerListMarker* const m = markers.getUnchecked(i);

            if (m->name == name)
            {
                markers.remove (i);
                markersHaveChanged();
            }
        }
        */

    }
    
    /**
      | Synchronously calls markersChanged()
      | on all the registered listeners.
      |
      */
    pub fn markers_have_changed(&mut self)  {
        
        todo!();
        /*
            listeners.call ([this] (MarkerListListener& l) { l.markersChanged (this); });
        */

    }
    
    /**
      | Registers a listener that will be called
      | when the markers are changed.
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn MarkerListListener)  {
        
        todo!();
        /*
            listeners.add (listener);
        */

    }
    
    /**
      | Deregisters a previously-registered
      | listener.
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn MarkerListListener)  {
        
        todo!();
        /*
            listeners.remove (listener);
        */

    }
    
    /**
      | Evaluates the given marker and returns
      | its absolute position.
      | 
      | The parent component must be supplied
      | in case the marker's expression refers
      | to the size of its parent component.
      |
      */
    pub fn get_marker_position(&self, 
        marker:           &MarkerListMarker,
        parent_component: *mut Component) -> f64 {
        
        todo!();
        /*
            if (parentComponent == nullptr)
            return marker.position.resolve (nullptr);

        RelativeCoordinatePositionerBase::ComponentScope scope (*parentComponent);
        return marker.position.resolve (&scope);
        */

    }
}
