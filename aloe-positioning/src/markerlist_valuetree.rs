crate::ix!();

/**
  | Forms a wrapper around a ValueTree that
  | can be used for storing a MarkerList.
  |
  */
pub struct MarkerListValueTreeWrapper {
    state: ValueTree,
}

pub mod value_tree_wrapper {
    use super::*;
    lazy_static!{
        /*
        static const Identifier markerTag, nameProperty, posProperty;
        const Identifier MarkerListValueTreeWrapper::markerTag ("MarkerListMarker");
        const Identifier MarkerListValueTreeWrapper::nameProperty ("name");
        const Identifier MarkerListValueTreeWrapper::posProperty ("position");
        */
    }
}

impl MarkerListValueTreeWrapper {
    
    pub fn get_state(&mut self) -> &mut ValueTree {
        
        todo!();
        /*
            return state;
        */

    }
    
    pub fn new(state: &ValueTree) -> Self {
    
        todo!();
        /*
        : state(state_),

        
        */

    }
    
    pub fn get_num_markers(&self) -> i32 {
        
        todo!();
        /*
            return state.getNumChildren();
        */
    }
    
    pub fn get_marker_state_by_index(&self, index: i32) -> ValueTree {
        
        todo!();
        /*
            return state.getChild (index);
        */

    }
    
    pub fn get_marker_state_by_name(&self, name: &String) -> ValueTree {
        
        todo!();
        /*
            return state.getChildWithProperty (nameProperty, name);
        */

    }
    
    pub fn contains_marker(&self, marker: &ValueTree) -> bool {
        
        todo!();
        /*
            return marker.isAChildOf (state);
        */

    }
    
    pub fn get_marker(&self, marker: &ValueTree) -> MarkerListMarker {
        
        todo!();
        /*
            jassert (containsMarker (marker));

        return MarkerList::MarkerListMarker (marker [nameProperty], RelativeCoordinate (marker [posProperty].toString()));
        */

    }
    
    pub fn set_marker(
        &mut self, 
        m:            &MarkerListMarker,
        undo_manager: *mut UndoManager

    )  {
        
        todo!();
        /*
            ValueTree marker (state.getChildWithProperty (nameProperty, m.name));

        if (marker.isValid())
        {
            marker.setProperty (posProperty, m.position.toString(), undoManager);
        }
        else
        {
            marker = ValueTree (markerTag);
            marker.setProperty (nameProperty, m.name, nullptr);
            marker.setProperty (posProperty, m.position.toString(), nullptr);
            state.appendChild (marker, undoManager);
        }
        */

    }
    
    pub fn remove_marker(
        &mut self, 
        marker:       &ValueTree,
        undo_manager: *mut UndoManager

    )  {
        
        todo!();
        /*
            state.removeChild (marker, undoManager);
        */

    }
    
    pub fn apply_to(&mut self, marker_list: &mut MarkerList)  {
        
        todo!();
        /*
            const int numMarkers = getNumMarkers();

        StringArray updatedMarkers;

        for (int i = 0; i < numMarkers; ++i)
        {
            const ValueTree marker (state.getChild (i));
            const String name (marker [nameProperty].toString());
            markerList.setMarker (name, RelativeCoordinate (marker [posProperty].toString()));
            updatedMarkers.add (name);
        }

        for (int i = markerList.getNumMarkers(); --i >= 0;)
            if (! updatedMarkers.contains (markerList.getMarker (i)->name))
                markerList.removeMarker (i);
        */

    }
    
    pub fn read_from(
        &mut self, 
        marker_list:  &MarkerList,
        undo_manager: *mut UndoManager

    )  {
        
        todo!();
        /*
            state.removeAllChildren (undoManager);

        for (int i = 0; i < markerList.getNumMarkers(); ++i)
            setMarker (*markerList.getMarker(i), undoManager);
        */

    }
}
