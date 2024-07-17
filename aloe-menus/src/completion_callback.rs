crate::ix!();

/**
  | This invokes any command manager commands
  | and deletes the menu window when it is
  | dismissed
  |
  */
#[no_copy]
pub struct PopupMenuCompletionCallback<'a> {
    manager_of_chosen_command: *mut dyn CommandManagerInterface, // default = nullptr
    component:                 Box<Component<'a>>,
    prev_focused:              WeakReference<Component<'a>>, //{ Component::getCurrentlyFocusedComponent() };
}

impl<'a> ModalComponentManagerCallback for PopupMenuCompletionCallback<'a> {

    fn modal_state_finished(&mut self, result: i32)  {
        
        todo!();
        /*
            if (managerOfChosenCommand != nullptr && result != 0)
            {
                typename ApplicationCommandTarget::InvocationInfo info (result);
                info.invocationMethod = typename ApplicationCommandTarget::InvocationInfo::fromMenu;

                managerOfChosenCommand->invoke (info, true);
            }

            // (this would be the place to fade out the component, if that's what's required)
            component.reset();

            if (PopupMenuSettings::menuWasHiddenBecauseOfAppChange)
                return;

            auto* focusComponent = getComponentToPassFocusTo();

            const auto focusedIsNotMinimised = [focusComponent]
            {
                if (focusComponent != nullptr)
                    if (auto* peer = focusComponent->getPeer())
                        return ! peer->isMinimised();

                return false;
            }();

            if (focusedIsNotMinimised)
            {
                if (auto* topLevel = focusComponent->getTopLevelComponent())
                    topLevel->toFront (true);

                if (focusComponent->isShowing() && ! focusComponent->hasKeyboardFocus (true))
                    focusComponent->grabKeyboardFocus();
            }
        */
    }
}

impl<'a> PopupMenuCompletionCallback<'a> {
    
    pub fn get_component_to_pass_focus_to(&self) -> *mut Component {
        
        todo!();
        /*
            if (auto* current = Component::getCurrentlyFocusedComponent())
                return current;

            return prevFocused.get();
        */
    }
}
