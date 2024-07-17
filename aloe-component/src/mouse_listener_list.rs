crate::ix!();

#[no_copy]
pub struct MouseListenerListBailOutChecker2<'a> {
    checker:      &'a mut ComponentBailOutChecker<'a>,
    safe_pointer: WeakReference<Component<'a>>,
}


impl<'a> MouseListenerListBailOutChecker2<'a> {

    pub fn new(
        boc:  &mut ComponentBailOutChecker<'a>,
        comp: *mut Component<'a>) -> Self {
    
        todo!();
        /*
        : checker(boc),
        : safe_pointer(comp),

        
        */
    }
    
    pub fn should_bail_out(&self) -> bool {
        
        todo!();
        /*
            return checker.shouldBailOut() || safePointer == nullptr;
        */
    }
}

///-----------------------------
#[no_copy]
#[derive(Default)]
pub struct MouseListenerList {
    listeners:                Vec<*mut dyn MouseListener>,
    num_deep_mouse_listeners: i32, // default = 0
}

impl MouseListenerList {
    
    pub fn add_listener(
        &mut self, 
        new_listener:                                 *mut dyn MouseListener,
        wants_events_for_all_nested_child_components: bool

    ) {
        
        todo!();

        /*
            if (! listeners.contains (newListener))
                {
                    if (wantsEventsForAllNestedChildComponents)
                    {
                        listeners.insert (0, newListener);
                        ++numDeepMouseListeners;
                    }
                    else
                    {
                        listeners.add (newListener);
                    }
                }
        */
    }
    
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn MouseListener)  {
        
        todo!();
        /*
            auto index = listeners.indexOf (listenerToRemove);

                if (index >= 0)
                {
                    if (index < numDeepMouseListeners)
                        --numDeepMouseListeners;

                    listeners.remove (index);
                }
        */
    }
    
    pub fn send_mouse_event<'a,Params>(
        comp:         &mut Component<'a>,
        checker:      &mut ComponentBailOutChecker,
        event_method: fn(_0: Params) -> c_void,
        params:       Params)  {
    
        todo!();
        /*
            if (checker.shouldBailOut())
                return;

            if (auto* list = comp.mouseListeners.get())
            {
                for (int i = list->listeners.size(); --i >= 0;)
                {
                    (list->listeners.getUnchecked(i)->*eventMethod) (params...);

                    if (checker.shouldBailOut())
                        return;

                    i = jmin (i, list->listeners.size());
                }
            }

            for (Component* p = comp.parentComponent; p != nullptr; p = p->parentComponent)
            {
                if (auto* list = p->mouseListeners.get())
                {
                    if (list->numDeepMouseListeners > 0)
                    {
                        MouseListenerListBailOutChecker2 checker2 (checker, p);

                        for (int i = list->numDeepMouseListeners; --i >= 0;)
                        {
                            (list->listeners.getUnchecked(i)->*eventMethod) (params...);

                            if (checker2.shouldBailOut())
                                return;

                            i = jmin (i, list->numDeepMouseListeners);
                        }
                    }
                }
            }
        */
    }
}
