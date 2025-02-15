crate::ix!();

#[no_copy]
#[leak_detector]
//#[cfg(target_os="android")]
pub struct AndroidViewComponentImpl<'a> {
    base:         ComponentMovementWatcher<'a>,
    view:         GlobalRef,
    owner:        &'a mut Component<'a>,
    current_peer: *mut ComponentPeer<'a>, // default = nullptr
}

//#[cfg(target_os="android")]
impl<'a> Drop for AndroidViewComponentImpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            removeFromParent();
        */
    }
}

//#[cfg(target_os="android")]
impl<'a> AndroidViewComponentImpl<'a> {

    pub fn new(
        v:    &LocalRef<jobject>,
        comp: &mut Component<'a>
    ) -> Self {
    
        todo!();
        /*
        : component_movement_watcher(&comp),
        : view(v),
        : owner(comp),

            if (owner.isShowing())
                componentPeerChanged();
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            auto* topComp = owner.getTopLevelComponent();

            if (topComp->getPeer() != nullptr)
            {
                auto pos = topComp->getLocalPoint (&owner, Point<int>());

                Rectangle<int> r (pos.x, pos.y, owner.getWidth(), owner.getHeight());
                r *= Desktop::getInstance().getDisplays().getPrimaryDisplay()->scale;

                getEnv()->CallVoidMethod (view, AndroidView.layout, r.getX(), r.getY(),
                                          r.getRight(), r.getBottom());
            }
        */
    }
    
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
            auto* peer = owner.getPeer();

            if (currentPeer != peer)
            {
                removeFromParent();
                currentPeer = peer;

                addToParent();
            }

            enum
            {
                VISIBLE   = 0,
                INVISIBLE = 4
            };

            getEnv()->CallVoidMethod (view, AndroidView.setVisibility, owner.isShowing() ? VISIBLE : INVISIBLE);
        */
    }
    
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
            componentPeerChanged();
        */
    }
    
    pub fn component_brought_to_front(&mut self, comp: &mut Component<'a>)  {
        
        todo!();
        /*
            ComponentMovementWatcher::componentBroughtToFront (comp);
        */
    }
    
    pub fn get_view_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            auto* env = getEnv();

            int width  = env->CallIntMethod (view, AndroidView.getWidth);
            int height = env->CallIntMethod (view, AndroidView.getHeight);

            return Rectangle<int> (width, height);
        */
    }
    
    pub fn add_to_parent(&mut self)  {
        
        todo!();
        /*
            if (currentPeer != nullptr)
            {
                jobject peerView = (jobject) currentPeer->getNativeHandle();

                // NB: Assuming a parent is always of ViewGroup type
                auto* env = getEnv();

                env->CallVoidMethod (peerView, AndroidViewGroup.addView, view.get());
                componentMovedOrResized (false, false);
            }
        */
    }
    
    pub fn remove_from_parent(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();
            auto parentView = env->CallObjectMethod (view, AndroidView.getParent);

            if (parentView != nullptr)
            {
                // Assuming a parent is always of ViewGroup type
                env->CallVoidMethod (parentView, AndroidViewGroup.removeView, view.get());
            }
        */
    }
}
