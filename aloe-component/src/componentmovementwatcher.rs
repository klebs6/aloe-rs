crate::ix!();

pub trait ComponentPeerChanged {

    /**
      | This callback happens when the component's
      | top-level peer is changed.
      |
      */
    fn component_peer_changed(&mut self);
}

pub trait ComponentVisabilityChanged {

    /**
      | This callback happens when the component's
      | visibility state changes, possibly
      | due to one of its parents being made visible
      | or invisible.
      |
      */
    fn component_visibility_changed(&mut self);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentMovementWatcher.h]

/**
  | An object that watches for any movement
  | of a component or any of its parent components.
  | 
  | This makes it easy to check when a component
  | is moved relative to its top-level peer
  | window. The normal Component::moved()
  | method is only called when a component
  | moves relative to its immediate parent,
  | and sometimes you want to know if any
  | of components higher up the tree have
  | moved (which of course will affect the
  | overall position of all their sub-components).
  | 
  | It also includes a callback that lets
  | you know when the top-level peer is changed.
  | 
  | This class is used by specialised components
  | like WebBrowserComponent because
  | they need to keep their custom windows
  | in the right place and respond to changes
  | in the peer.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentMovementWatcher<'a> {
    component:               WeakReference<Component<'a>>,
    last_peerid:             u32, // default = 0
    registered_parent_comps: Vec<*mut Component<'a>>,
    reentrant:               bool, // default = false
    was_showing:             bool,
    last_bounds:             Rectangle<i32>,
}

impl<'a> ComponentListener for ComponentMovementWatcher<'a> {

}

impl<'a> ComponentBroughtToFront    for ComponentMovementWatcher<'a> {}
impl<'a> ComponentChildrenChanged   for ComponentMovementWatcher<'a> {}
impl<'a> ComponentNameChanged       for ComponentMovementWatcher<'a> {}
impl<'a> ComponentEnablementChanged for ComponentMovementWatcher<'a> {}

impl<'a> ComponentParentHierarchyChanged for ComponentMovementWatcher<'a> {

    fn component_parent_hierarchy_changed(&mut self, _0: &mut Component<'_>)  {
        
        todo!();
        /*
            if (component != nullptr && ! reentrant)
        {
            const ScopedValueSetter<bool> setter (reentrant, true);

            auto* peer = component->getPeer();
            auto peerID = peer != nullptr ? peer->getUniqueID() : 0;

            if (peerID != lastPeerID)
            {
                componentPeerChanged();

                if (component == nullptr)
                    return;

                lastPeerID = peerID;
            }

            unregister();
            registerWithParentComps();

            componentMovedOrResized (*component, true, true);

            if (component != nullptr)
                componentVisibilityChanged (*component);
        }
        */
    }
}

impl<'a> ComponentMovedOrResized for ComponentMovementWatcher<'a> {

    fn component_moved_or_resized(
        &mut self, 
        _0:          &mut Component<'_>,
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            if (component != nullptr)
        {
            if (wasMoved)
            {
                Point<int> newPos;
                auto* top = component->getTopLevelComponent();

                if (top != component)
                    newPos = top->getLocalPoint (component, Point<int>());
                else
                    newPos = top->getPosition();

                wasMoved = lastBounds.getPosition() != newPos;
                lastBounds.setPosition (newPos);
            }

            wasResized = (lastBounds.getWidth() != component->getWidth() || lastBounds.getHeight() != component->getHeight());
            lastBounds.setSize (component->getWidth(), component->getHeight());

            if (wasMoved || wasResized)
                componentMovedOrResized (wasMoved, wasResized);
        }
        */
    }
}

impl<'a> ComponentVisibilityChanged for ComponentMovementWatcher<'a> {

    fn component_visibility_changed(&mut self, _0: &mut Component<'_>)  {
        
        todo!();
        /*
            if (component != nullptr)
        {
            const bool isShowingNow = component->isShowing();

            if (wasShowing != isShowingNow)
            {
                wasShowing = isShowingNow;
                componentVisibilityChanged();
            }
        }
        */
    }
}

impl<'a> ComponentBeingDeleted for ComponentMovementWatcher<'a> {

    fn component_being_deleted(&mut self, comp: &mut Component<'_>)  {
        
        todo!();
        /*
            registeredParentComps.removeFirstMatchingValue (&comp);

        if (component == &comp)
            unregister();
        */
    }
}
    

impl<'a> Drop for ComponentMovementWatcher<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
        if (component != nullptr)
            component->removeComponentListener (this);

        unregister();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentMovementWatcher.cpp]
impl<'a> ComponentMovementWatcher<'a> {

    /**
      | Returns the component that's being
      | watched.
      |
      */
    pub fn get_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return component.get();
        */
    }
    
    /**
      | Creates a ComponentMovementWatcher
      | to watch a given target component.
      |
      */
    pub fn new(comp: *mut Component<'a>) -> Self {
    
        todo!();
        /*


            : component (comp),
          wasShowing (comp->isShowing())

        jassert (component != nullptr); // can't use this with a null pointer..

        component->addComponentListener (this);
        registerWithParentComps();
        */
    }
    
    
    
    
    pub fn register_with_parent_comps(&mut self)  {
        
        todo!();
        /*
            for (auto* p = component->getParentComponent(); p != nullptr; p = p->getParentComponent())
        {
            p->addComponentListener (this);
            registeredParentComps.add (p);
        }
        */
    }
    
    pub fn unregister(&mut self)  {
        
        todo!();
        /*
            for (auto* c : registeredParentComps)
            c->removeComponentListener (this);

        registeredParentComps.clear();
        */
    }
}
