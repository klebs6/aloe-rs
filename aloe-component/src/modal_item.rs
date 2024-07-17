crate::ix!();

#[no_copy]
pub struct ModalItem<'a> {
    base:         ComponentMovementWatcher<'a>,
    component:    *mut Component<'a>,
    callbacks:    Vec<Box<dyn ModalComponentManagerCallback>>,
    return_value: i32, // default = 0
    is_active:    bool, // default = true
    auto_delete:  bool,
}

impl<'a> Drop for ModalItem<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
            if (autoDelete)
                std::unique_ptr<Component> componentDeleter (component);
         */
    }
}

impl<'a> ModalItem<'a> {

    pub fn new(
        comp:               *mut Component<'a>,
        should_auto_delete: bool) -> Self {
    
        todo!();
        /*
        : component_movement_watcher(comp),
        : component(comp),
        : auto_delete(shouldAutoDelete),

            jassert (comp != nullptr);
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        _0: bool,
        _1: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
            componentVisibilityChanged();
        */
    }
    
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
            if (! component->isShowing())
                    cancel();
        */
    }
    
    pub fn component_being_deleted(&mut self, comp: &mut Component<'a>)  {
        
        todo!();
        /*
            ComponentMovementWatcher::componentBeingDeleted (comp);

                if (component == &comp || comp.isParentOf (component))
                {
                    autoDelete = false;
                    cancel();
                }
        */
    }
    
    pub fn cancel(&mut self)  {
        
        todo!();
        /*
            if (isActive)
                {
                    isActive = false;

                    if (auto* mcm = ModalComponentManager::getInstanceWithoutCreating())
                        mcm->triggerAsyncUpdate();
                }
        */
    }
}
