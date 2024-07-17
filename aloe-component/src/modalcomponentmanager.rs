crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_ModalComponentManager.h]

/**
  | Receives callbacks when a modal component
  | is dismissed.
  | 
  | You can register a callback using Component::enterModalState()
  | or
  | 
  | ModalComponentManager::attachCallback().
  | 
  | For some quick ways of creating callback
  | objects, see the ModalCallbackFunction
  | class. @see ModalCallbackFunction
  |
  */
pub trait ModalComponentManagerCallback {

    /**
      | Called to indicate that a modal component
      | has been dismissed.
      | 
      | You can register a callback using Component::enterModalState()
      | or
      | 
      | ModalComponentManager::attachCallback().
      | 
      | The returnValue parameter is the value
      | that was passed to Component::exitModalState()
      | when the component was dismissed.
      | 
      | The callback object will be deleted
      | shortly after this method is called.
      |
      */
    fn modal_state_finished(&mut self, return_value: i32);
}

/**
  | Manages the system's stack of modal
  | components.
  | 
  | Normally you'll just use the Component
  | methods to invoke modal states in components,
  | and won't have to deal with this class
  | directly, but this is the singleton
  | object that's used internally to manage
  | the stack.
  | 
  | @see Component::enterModalState,
  | Component::exitModalState, Component::isCurrentlyModal,
  | 
  | Component::getCurrentlyModalComponent,
  | Component::isCurrentlyBlockedByAnotherModalComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
pub struct ModalComponentManager<'a> {
    base:  AsyncUpdater<'a>,
    base2: DeletedAtShutdown,
    stack: Vec<ModalItem<'a>>,
}

aloe_declare_singleton_singlethreaded_minimal!{
    ModalComponentManager
}

impl<'a> Default for ModalComponentManager<'a> {
    
    /**
      | Creates a ModalComponentManager.
      | 
      | You shouldn't ever call the constructor
      | - it's a singleton, so use ModalComponentManager::getInstance()
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

impl<'a> Drop for ModalComponentManager<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
        stack.clear();
        clearSingletonInstance();
         */
    }
}

aloe_implement_singleton!{
    ModalComponentManager
}

impl<'a> ModalComponentManager<'a> {

    pub fn start_modal(&mut self, 
        component:   *mut Component<'a>,
        auto_delete: bool)  {
        
        todo!();
        /*
            if (component != nullptr)
            stack.add (new ModalItem (component, autoDelete));
        */
    }
    
    /**
      | Adds a new callback that will be called
      | when the specified modal component
      | is dismissed.
      | 
      | If the component is modal, then when
      | it is dismissed, either by being hidden,
      | or by calling
      | 
      | Component::exitModalState(), then
      | the ModalComponentManagerCallback::modalStateFinished()
      | method will be called.
      | 
      | Each component can have any number of
      | callbacks associated with it, and this
      | one is added to that list.
      | 
      | The object that is passed in will be deleted
      | by the manager when it's no longer needed.
      | If the given component is not currently
      | modal, the callback object is deleted
      | immediately and no action is taken.
      |
      */
    pub fn attach_callback(
        &mut self, 
        component: *mut Component<'a>,
        callback:  *mut dyn ModalComponentManagerCallback

    ) {
        
        todo!();
        /*
            if (callback != nullptr)
        {
            std::unique_ptr<ModalComponentManagerCallback> callbackDeleter (callback);

            for (int i = stack.size(); --i >= 0;)
            {
                auto* item = stack.getUnchecked (i);

                if (item->component == component)
                {
                    item->callbacks.add (callback);
                    callbackDeleter.release();
                    break;
                }
            }
        }
        */
    }
    
    pub fn end_modal(&mut self, component: *mut Component<'a>)  {
        
        todo!();
        /*
            for (int i = stack.size(); --i >= 0;)
        {
            auto* item = stack.getUnchecked (i);

            if (item->component == component)
                item->cancel();
        }
        */
    }
    
    pub fn end_modal_with_return_value(
        &mut self, 
        component:    *mut Component<'a>,
        return_value: i32

    ) {
        
        todo!();
        /*
            for (int i = stack.size(); --i >= 0;)
        {
            auto* item = stack.getUnchecked (i);

            if (item->component == component)
            {
                item->returnValue = returnValue;
                item->cancel();
            }
        }
        */
    }
    
    /**
      | Returns the number of components currently
      | being shown modally. @see getModalComponent
      |
      */
    pub fn get_num_modal_components(&self) -> i32 {
        
        todo!();
        /*
            int n = 0;

        for (auto* item : stack)
            if (item->isActive)
                ++n;

        return n;
        */
    }
    
    /**
      | Returns one of the components being
      | shown modally.
      | 
      | An index of 0 is the most recently-shown,
      | topmost component.
      |
      */
    pub fn get_modal_component(&self, index: i32) -> *mut Component {
        
        todo!();
        /*
            int n = 0;

        for (int i = stack.size(); --i >= 0;)
        {
            auto* item = stack.getUnchecked (i);

            if (item->isActive)
                if (n++ == index)
                    return item->component;
        }

        return nullptr;
        */
    }
    
    /**
      | Returns true if the specified component
      | is in a modal state.
      |
      */
    pub fn is_modal(&self, comp: *const Component<'a>) -> bool {
        
        todo!();
        /*
            for (auto* item : stack)
            if (item->isActive && item->component == comp)
                return true;

        return false;
        */
    }
    
    /**
      | Returns true if the specified component
      | is currently the topmost modal component.
      |
      */
    pub fn is_front_modal_component(&self, comp: *const Component<'a>) -> bool {
        
        todo!();
        /*
            return comp == getModalComponent (0);
        */
    }
    
    /**
      | @internal
      |
      */
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            for (int i = stack.size(); --i >= 0;)
        {
            auto* item = stack.getUnchecked (i);

            if (! item->isActive)
            {
                std::unique_ptr<ModalItem> deleter (stack.removeAndReturn (i));
                Component::SafePointer<Component> compToDelete (item->autoDelete ? item->component : nullptr);

                for (int j = item->callbacks.size(); --j >= 0;)
                    item->callbacks.getUnchecked (j)->modalStateFinished (item->returnValue);

                compToDelete.deleteAndZero();
            }
        }
        */
    }
    
    /**
      | Brings any modal components to the front.
      |
      */
    pub fn bring_modal_components_to_front(
        &mut self, 
        top_one_should_grab_focus: Option<bool>

    ) {

        let top_one_should_grab_focus: bool = top_one_should_grab_focus.unwrap_or(true);
        
        todo!();
        /*
            ComponentPeer* lastOne = nullptr;

        for (int i = 0; i < getNumModalComponents(); ++i)
        {
            auto* c = getModalComponent (i);

            if (c == nullptr)
                break;

            if (auto* peer = c->getPeer())
            {
                if (peer != lastOne)
                {
                    if (lastOne == nullptr)
                    {
                        peer->toFront (topOneShouldGrabFocus);

                        if (topOneShouldGrabFocus)
                            peer->grabFocus();
                    }
                    else
                    {
                        peer->toBehind (lastOne);
                    }

                    lastOne = peer;
                }
            }
        }
        */
    }
    
    /**
      | Calls exitModalState (0) on any components
      | that are currently modal.
      | 
      | 
      | -----------
      | @return
      | 
      | true if any components were modal; false
      | if nothing needed cancelling
      |
      */
    pub fn cancel_all_modal_components(&mut self) -> bool {
        
        todo!();
        /*
            auto numModal = getNumModalComponents();

        for (int i = numModal; --i >= 0;)
            if (auto* c = getModalComponent (i))
                c->exitModalState (0);

        return numModal > 0;
        */
    }

    /**
      | Runs the event loop until the currently
      | topmost modal component is dismissed,
      | and returns the exit code for that component.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn run_event_loop_for_current_component(&mut self) -> i32 {
        
        todo!();
        /*
            // This can only be run from the message thread!
        ALOE_ASSERT_MESSAGE_THREAD

        int returnValue = 0;

        if (auto* currentlyModal = getModalComponent (0))
        {
            FocusRestorer focusRestorer;
            bool finished = false;

            attachCallback (currentlyModal, ModalCallbackFunction::create ([&] (int r) { returnValue = r; finished = true; }));

            ALOE_TRY
            {
                while (! finished)
                {
                    if  (! MessageManager::getInstance()->runDispatchLoopUntil (20))
                        break;
                }
            }
            ALOE_CATCH_EXCEPTION
        }

        return returnValue;
        */
    }
}
