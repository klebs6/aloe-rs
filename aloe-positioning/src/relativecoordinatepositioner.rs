crate::ix!();

pub trait RegisterCoordinates {

    fn register_coordinates(&mut self) -> bool;
}

pub trait ApplyToComponentBounds {

    fn apply_to_component_bounds(&mut self);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeCoordinatePositioner.h]

pub trait RelativeCoordinatePositionerBaseInterface: 
RegisterCoordinates 
+ ApplyToComponentBounds { }

/**
  | Base class for Component::Positioners
  | that are based upon relative coordinates.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct RelativeCoordinatePositionerBase<'a> {
    base:                ComponentPositioner<'a>,
    source_components:   Vec<*mut Component<'a>>,
    source_marker_lists: Vec<*mut MarkerList>,
    registered_ok:       bool,
}

impl<'a> MarkerListListener for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> MarkerListBeingDeleted for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> MarkersChanged for RelativeCoordinatePositionerBase<'a> {

    fn markers_changed(&mut self, _0: *mut MarkerList)  {
        
        todo!();
        /*
            apply();
        */

    }
}

impl<'a> ComponentListener for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentMovedOrResized for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentBroughtToFront for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentVisibilityChanged for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentChildrenChanged for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentParentHierarchyChanged for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentNameChanged for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentBeingDeleted for RelativeCoordinatePositionerBase<'a> {

}

impl<'a> ComponentEnablementChanged for RelativeCoordinatePositionerBase<'a> {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeCoordinatePositioner.cpp]
impl<'a> Drop for RelativeCoordinatePositionerBase<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            unregisterListeners();
        */

    }
}

impl<'a> RelativeCoordinatePositionerBase<'a> {
    
    pub fn new(comp: &mut Component) -> Self {
    
        todo!();
        /*
            : Component::Positioner (comp), registeredOk (false)
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        _0:          &mut Component,
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            apply();
        */

    }
    
    pub fn component_parent_hierarchy_changed(&mut self, _0: &mut Component)  {
        
        todo!();
        /*
            apply();
        */

    }
    
    pub fn component_children_changed(&mut self, changed: &mut Component)  {
        
        todo!();
        /*
            if (getComponent().getParentComponent() == &changed && ! registeredOk)
            apply();
        */

    }
    
    pub fn component_being_deleted(&mut self, comp: &mut Component)  {
        
        todo!();
        /*
            jassert (sourceComponents.contains (&comp));
        sourceComponents.removeFirstMatchingValue (&comp);
        registeredOk = false;
        */

    }
    
    pub fn marker_list_being_deleted(&mut self, marker_list: *mut MarkerList)  {
        
        todo!();
        /*
            jassert (sourceMarkerLists.contains (markerList));
        sourceMarkerLists.removeFirstMatchingValue (markerList);
        */

    }
    
    pub fn apply(&mut self)  {
        
        todo!();
        /*
            if (! registeredOk)
        {
            unregisterListeners();
            registeredOk = registerCoordinates();
        }

        applyToComponentBounds();
        */

    }
    
    pub fn add_coordinate(&mut self, coord: &RelativeCoordinate) -> bool {
        
        todo!();
        /*
            bool ok = true;
        RelativeCoordinatePositionerBaseDependencyFinderScope finderScope (getComponent(), *this, ok);
        coord.getExpression().evaluate (finderScope);
        return ok;
        */

    }
    
    pub fn add_point(&mut self, point: &RelativePoint) -> bool {
        
        todo!();
        /*
            const bool ok = addCoordinate (point.x);
        return addCoordinate (point.y) && ok;
        */

    }
    
    pub fn register_component_listener(&mut self, comp: &mut Component)  {
        
        todo!();
        /*
            if (! sourceComponents.contains (&comp))
        {
            comp.addComponentListener (this);
            sourceComponents.add (&comp);
        }
        */

    }
    
    pub fn register_marker_list_listener(&mut self, list: *mut MarkerList)  {
        
        todo!();
        /*
            if (list != nullptr && ! sourceMarkerLists.contains (list))
        {
            list->addListener (this);
            sourceMarkerLists.add (list);
        }
        */

    }
    
    pub fn unregister_listeners(&mut self)  {
        
        todo!();
        /*
            for (int i = sourceComponents.size(); --i >= 0;)
            sourceComponents.getUnchecked(i)->removeComponentListener (this);

        for (int i = sourceMarkerLists.size(); --i >= 0;)
            sourceMarkerLists.getUnchecked(i)->removeListener (this);

        sourceComponents.clear();
        sourceMarkerLists.clear();
        */

    }
}
