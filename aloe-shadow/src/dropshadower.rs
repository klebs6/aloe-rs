crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/misc/aloe_DropShadower.h]

/**
  | Adds a drop-shadow to a component.
  |
  | This object creates and manages a set of
  | components which sit around a component,
  | creating a gaussian shadow around it. The
  | components will track the position of the
  | component and if it's brought to the front
  | they'll also follow this.
  |
  | For desktop windows you don't need to use this
  | class directly - just set the
  | Component::windowHasDropShadow flag when
  | calling Component::addToDesktop(), and the
  | system will create one of these if it's needed
  | (which it obviously isn't on the Mac, for
  | example).
  |
  | @tags{GUI}
  */
#[no_copy]
#[leak_detector]
pub struct DropShadower<'a> {
    owner:            WeakReference<Component<'a>>,
    shadow_windows:   Vec<Box<Component<'a>>>,
    shadow:           DropShadow,
    reentrant:        bool, // default = false
    last_parent_comp: WeakReference<Component<'a>>,
}

impl<'a> ComponentListener for DropShadower<'a> {

}

impl<'a> ComponentMovedOrResized for DropShadower<'a> {

    fn component_moved_or_resized(
        &mut self, 
        c:           &mut Component<'_>,
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            if (owner == &c)
            updateShadows();
        */
    }
}

impl<'a> ComponentBroughtToFront for DropShadower<'a> {

    fn component_brought_to_front(&mut self, c: &mut Component<'_>)  {
        
        todo!();
        /*
            if (owner == &c)
            updateShadows();
        */
    }
}

impl<'a> ComponentVisibilityChanged for DropShadower<'a> {

    fn component_visibility_changed(&mut self, c: &mut Component<'_>)  {
        
        todo!();
        /*
            if (owner == &c)
            updateShadows();
        */
    }
}

impl<'a> ComponentChildrenChanged for DropShadower<'a> {

    fn component_children_changed(&mut self, _0: &mut Component<'_>)  {
        
        todo!();
        /*
            updateShadows();
        */
    }
}

impl<'a> ComponentParentHierarchyChanged for DropShadower<'a> {

    fn component_parent_hierarchy_changed(&mut self, c: &mut Component<'_>)  {
        
        todo!();
        /*
            if (owner == &c)
        {
            updateParent();
            updateShadows();
        }
        */
    }
}

impl<'a> ComponentNameChanged for DropShadower<'a> { }
impl<'a> ComponentBeingDeleted for DropShadower<'a> { }
impl<'a> ComponentEnablementChanged for DropShadower<'a> { }

impl<'a> Drop for DropShadower<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        if (owner != nullptr)
        {
            owner->removeComponentListener (this);
            owner = nullptr;
        }

        updateParent();

        const ScopedValueSetter<bool> setter (reentrant, true);
        shadowWindows.clear();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/misc/aloe_DropShadower.cpp]
impl<'a> DropShadower<'a> {

    /** Creates a DropShadower. */
    pub fn new(ds: &DropShadow) -> Self {
    
        todo!();
        /*
        : shadow(ds),

        
        */
    }
    
    /**
      | Attaches the DropShadower to the component
      | you want to shadow.
      |
      */
    pub fn set_owner(&mut self, component_to_follow: *mut Component<'a>)  {
        
        todo!();
        /*
            if (componentToFollow != owner)
        {
            if (owner != nullptr)
                owner->removeComponentListener (this);

            // (the component can't be null)
            jassert (componentToFollow != nullptr);

            owner = componentToFollow;
            jassert (owner != nullptr);

            updateParent();
            owner->addComponentListener (this);

            updateShadows();
        }
        */
    }
    
    pub fn update_parent(&mut self)  {
        
        todo!();
        /*
            if (Component* p = lastParentComp)
            p->removeComponentListener (this);

        lastParentComp = owner != nullptr ? owner->getParentComponent() : nullptr;

        if (Component* p = lastParentComp)
            p->addComponentListener (this);
        */
    }
    
    pub fn update_shadows(&mut self)  {
        
        todo!();
        /*
            if (reentrant)
            return;

        const ScopedValueSetter<bool> setter (reentrant, true);

        if (owner == nullptr)
        {
            shadowWindows.clear();
            return;
        }

        if (owner->isShowing()
             && owner->getWidth() > 0 && owner->getHeight() > 0
             && (Desktop::canUseSemiTransparentWindows() || owner->getParentComponent() != nullptr))
        {
            while (shadowWindows.size() < 4)
                shadowWindows.add (new ShadowWindow (owner, shadow));

            const int shadowEdge = jmax (shadow.offset.x, shadow.offset.y) + shadow.radius;
            const int x = owner->getX();
            const int y = owner->getY() - shadowEdge;
            const int w = owner->getWidth();
            const int h = owner->getHeight() + shadowEdge + shadowEdge;

            for (int i = 4; --i >= 0;)
            {
                // there seem to be rare situations where the dropshadower may be deleted by
                // callbacks during this loop, so use a weak ref to watch out for this..
                WeakReference<Component> sw (shadowWindows[i]);

                if (sw != nullptr)
                {
                    sw->setAlwaysOnTop (owner->isAlwaysOnTop());

                    if (sw == nullptr)
                        return;

                    switch (i)
                    {
                        case 0: sw->setBounds (x - shadowEdge, y, shadowEdge, h); break;
                        case 1: sw->setBounds (x + w, y, shadowEdge, h); break;
                        case 2: sw->setBounds (x, y, w, shadowEdge); break;
                        case 3: sw->setBounds (x, owner->getBottom(), w, shadowEdge); break;
                        default: break;
                    }

                    if (sw == nullptr)
                        return;

                    sw->toBehind (i == 3 ? owner.get() : shadowWindows.getUnchecked (i + 1));
                }
            }
        }
        else
        {
            shadowWindows.clear();
        }
        */
    }
}
