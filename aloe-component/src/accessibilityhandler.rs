crate::ix!();

pub fn is_component_visible_within_window(comp: &Component) -> bool {
    
    todo!();
    /*
        if (auto* peer = comp.getPeer())
            return ! peer->getAreaCoveredBy (comp).getIntersection (peer->getComponent().getLocalBounds()).isEmpty();

        return false;
    */
}

pub fn is_component_visible_within_parent(comp: *mut Component) -> bool {
    
    todo!();
    /*
        if (auto* parent = comp->getParentComponent())
        {
            if (comp->getBoundsInParent().getIntersection (parent->getLocalBounds()).isEmpty())
                return false;

            return isComponentVisibleWithinParent (parent);
        }

        return true;
    */
}

pub fn find_enclosing_handler<'a>(comp: *mut Component) -> *mut AccessibilityHandler<'a> {
    
    todo!();
    /*
        if (comp != nullptr)
        {
            if (auto* handler = comp->getAccessibilityHandler())
                return handler;

            return findEnclosingHandler (comp->getParentComponent());
        }

        return nullptr;
    */
}

pub fn get_unignored_ancestor<'a>(handler: *mut AccessibilityHandler<'a>) -> *mut AccessibilityHandler<'a> {
    
    todo!();
    /*
        while (handler != nullptr
               && (handler->isIgnored() || ! handler->isVisibleWithinParent())
               && handler->getParent() != nullptr)
        {
            handler = handler->getParent();
        }

        return handler;
    */
}

pub fn find_first_unignored_child<'a>(handlers: &Vec<*mut AccessibilityHandler<'a>>) 
-> *mut AccessibilityHandler<'a> 
{
    todo!();
    /*
        if (! handlers.empty())
        {
            const auto iter = std::find_if (handlers.cbegin(), handlers.cend(),
                                            [] (const AccessibilityHandler* handler) { return ! handler->isIgnored() && handler->isVisibleWithinParent(); });

            if (iter != handlers.cend())
                return *iter;

            for (auto* handler : handlers)
                if (auto* unignored = findFirstUnignoredChild (handler->getChildren()))
                    return unignored;
        }

        return nullptr;
    */
}

pub fn get_first_unignored_descendant<'a>(handler: *mut AccessibilityHandler<'a>) -> *mut AccessibilityHandler<'a> {
    
    todo!();
    /*
        if (handler != nullptr && (handler->isIgnored() || ! handler->isVisibleWithinParent()))
            return findFirstUnignoredChild (handler->getChildren());

        return handler;
    */
}

pub enum InternalAccessibilityEvent
{
    elementCreated,
    elementDestroyed,
    elementMovedOrResized,
    focusChanged,
    windowOpened,
    windowClosed
}


#[cfg(not(ALOE_NATIVE_ACCESSIBILITY_INCLUDED))]
pub fn notify_accessibility_event_internal<'a>(
    _0: &AccessibilityHandler<'a>,
    _1: InternalAccessibilityEvent)  {

    todo!();
        /*
        
        */
}

#[cfg(ALOE_NATIVE_ACCESSIBILITY_INCLUDED)]
pub fn notify_accessibility_event_internal<'a>(
    _0: &AccessibilityHandler<'a>,
    _1: InternalAccessibilityEvent)  {
    
    todo!();
    /*
    
    */
}

#[inline] pub fn get_accessible_application_or_plugin_name() -> String {
    
    todo!();
    /*
        #if defined (AloePlugin_Name)
        return AloePlugin_Name;
       #else
        if (auto* app = ALOEApplicationBase::getInstance())
            return app->getApplicationName();

        return "Aloe Application";
       #endif
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/aloe_AccessibilityHandler.h]

/**
  | Base class for accessible Components.
  | 
  | This class wraps a Component and provides
  | methods that allow an accessibility
  | client, such as VoiceOver on macOS,
  | or Narrator on Windows, to control it.
  | 
  | It handles hierarchical navigation,
  | properties, state, and various interfaces.
  | 
  | @tags{Accessibility}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AccessibilityHandler<'a> {
    component:   &'a mut Component<'a>,
    type_index:  TypeId,
    role:        AccessibilityRole,
    actions:     AccessibilityActions,
    interfaces:  AccessibilityHandlerInterfaces,

    //todo!
    //native_impl: Box<dyn AccessibilityNativeImpl>,
}

lazy_static!{
    /*
    static AccessibilityHandler* currentlyFocusedHandler;
    AccessibilityHandler* AccessibilityHandler::currentlyFocusedHandler = nullptr;
    */
}

impl<'a> Drop for AccessibilityHandler<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
        giveAwayFocus();
        notifyAccessibilityEventInternal (*this, InternalAccessibilityEvent::elementDestroyed);
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/aloe_AccessibilityHandler.cpp]
impl<'a> AccessibilityHandler<'a> {

    /**
      | Returns the Component that this handler
      | represents.
      |
      */
    pub fn get_component(&self) -> &Component {
        
        todo!();
        /*
            return component;
        */
    }

    /**
      | Returns the Component that this handler
      | represents.
      |
      */
    pub fn get_component_mut(&mut self) -> &mut Component {
        
        todo!();
        /*
            return component;
        */
    }

    /**
      | The type of UI element that this accessibility
      | handler represents.
      | 
      | @see AccessibilityRole
      |
      */
    pub fn get_role(&self) -> AccessibilityRole {
        
        todo!();
        /*
            return role;
        */
    }

    /**
      | @internal
      |
      */
    pub fn get_type_index(&self) -> TypeId {
        
        todo!();
        /*
            return typeIndex;
        */
    }
    
    /*
    pub fn create_native_impl(_0: &mut AccessibilityHandler) -> Box<AccessibilityNativeImpl> {
        
        todo!();
        /*
        
        */
    }
    */

    /**
      | Constructor.
      | 
      | This will create a AccessibilityHandler
      | which wraps the provided Component
      | and makes it visible to accessibility
      | clients. You must also specify a role
      | for the UI element from the `AccessibilityRole`
      | list which best describes it.
      | 
      | To enable users to interact with the
      | UI element you should provide the set
      | of supported actions and their associated
      | callbacks via the `accessibilityActions`
      | parameter.
      | 
      | For UI elements that support more complex
      | interaction the value, text, table,
      | and cell interfaces should be implemented
      | as required and passed as the final argument
      | of this constructor. See the documentation
      | of these classes for more information
      | about the types of control they represent
      | and which methods need to be implemented.
      |
      */
    pub fn new(
        comp:                  &mut Component,
        accessibility_role:    AccessibilityRole,
        accessibility_actions: AccessibilityActions,
        interfaces_in:         AccessibilityHandlerInterfaces) -> Self {
    
        todo!();
        /*


            : component (comp),
          typeIndex (typeid (component)),
          role (accessibilityRole),
          actions (std::move (accessibilityActions)),
          interfaces (std::move (interfacesIn)),
          nativeImpl (createNativeImpl (*this))

        notifyAccessibilityEventInternal (*this, InternalAccessibilityEvent::elementCreated);
        */
    }
    
    pub fn get_current_state(&self) -> AccessibleState {
        
        todo!();
        /*
            if (component.isCurrentlyBlockedByAnotherModalComponent()
            && Component::getCurrentlyModalComponent()->isVisible())
            return {};

        auto state = AccessibleState().withFocusable();

        return hasFocus (false) ? state.withFocused() : state;
        */
    }
    
    /**
      | Returns true if this UI element should
      | be ignored by accessibility clients.
      |
      */
    pub fn is_ignored(&self) -> bool {
        
        todo!();
        /*
            return role == AccessibilityRole::ignored || getCurrentState().isIgnored();
        */
    }
    
    /**
      | Returns true if this UI element is visible
      | within its parent.
      | 
      | This will always return true for UI elements
      | with the AccessibleState::accessibleOffscreen
      | flag set.
      |
      */
    pub fn is_visible_within_parent(&self) -> bool {
        
        todo!();
        /*
            return getCurrentState().isAccessibleOffscreen()
              || (isComponentVisibleWithinParent (&component) && isComponentVisibleWithinWindow (component));
        */
    }
    
    /**
      | Returns the set of actions that the UI
      | element supports and the associated
      | callbacks.
      |
      */
    pub fn get_actions(&self) -> &AccessibilityActions {
        
        todo!();
        /*
            return actions;
        */
    }
    
    /**
      | Returns the value interface for this
      | UI element, or nullptr if it is not supported.
      | 
      | @see AccessibilityValueInterface
      |
      */
    pub fn get_value_interface(&self) -> *mut dyn AccessibilityValueInterface {
        
        todo!();
        /*
            return interfaces.value.get();
        */
    }
    
    /**
      | Returns the table interface for this
      | UI element, or nullptr if it is not supported.
      | 
      | @see AccessibilityTableInterface
      |
      */
    pub fn get_table_interface(&self) -> *mut dyn AccessibilityTableInterface {
        
        todo!();
        /*
            return interfaces.table.get();
        */
    }
    
    /**
      | Returns the cell interface for this
      | UI element, or nullptr if it is not supported.
      | 
      | @see AccessibilityCellInterface
      |
      */
    pub fn get_cell_interface(&self) -> *mut dyn AccessibilityCellInterface {
        
        todo!();
        /*
            return interfaces.cell.get();
        */
    }
    
    /**
      | Returns the text interface for this
      | UI element, or nullptr if it is not supported.
      | 
      | @see AccessibilityTextInterface
      |
      */
    pub fn get_text_interface(&self) -> *mut dyn AccessibilityTextInterface {
        
        todo!();
        /*
            return interfaces.text.get();
        */
    }
    
    /**
      | Returns the first unignored parent
      | of this UI element in the accessibility
      | hierarchy, or nullptr if this is a root
      | element without a parent.
      |
      */
    pub fn get_parent(&self) -> *mut AccessibilityHandler {
        
        todo!();
        /*
            if (auto* focusContainer = component.findFocusContainer())
            return getUnignoredAncestor (findEnclosingHandler (focusContainer));

        return nullptr;
        */
    }
    
    /**
      | Returns the unignored children of this
      | UI element in the accessibility hierarchy.
      |
      */
    pub fn get_children(&self) -> Vec<*mut AccessibilityHandler> {
        
        todo!();
        /*
            if (! component.isFocusContainer() && component.getParentComponent() != nullptr)
            return {};

        const auto addChildComponentHandler = [this] (Component* focusableComponent,
                                                      std::vector<AccessibilityHandler*>& childHandlers)
        {
            if (focusableComponent == nullptr)
                return;

            if (auto* handler = findEnclosingHandler (focusableComponent))
            {
                if (! handler->getCurrentState().isFocusable() || ! isParentOf (handler))
                    return;

                if (auto* unignored = getFirstUnignoredDescendant (handler))
                    if (std::find (childHandlers.cbegin(), childHandlers.cend(), unignored) == childHandlers.cend())
                        childHandlers.push_back (unignored);
            }
        };

        std::vector<AccessibilityHandler*> children;

        if (auto traverser = component.createFocusTraverser())
        {
            addChildComponentHandler (traverser->getDefaultComponent (&component), children);

            for (auto* focusableChild : traverser->getAllComponents (&component))
                addChildComponentHandler (focusableChild, children);
        }

        return children;
        */
    }
    
    /**
      | Checks whether a given UI element is
      | a child of this one in the accessibility
      | hierarchy.
      |
      */
    pub fn is_parent_of(&self, possible_child: *const AccessibilityHandler) -> bool {
        
        todo!();
        /*
            while (possibleChild != nullptr)
        {
            possibleChild = possibleChild->getParent();

            if (possibleChild == this)
                return true;
        }

        return false;
        */
    }
    
    /**
      | Returns the deepest child of this UI
      | element in the accessibility hierarchy
      | that contains the given screen point,
      | or nullptr if there is no child at this
      | point.
      |
      */
    pub fn get_child_at(&mut self, screen_point: Point<i32>) -> *mut AccessibilityHandler {
        
        todo!();
        /*
            if (auto* comp = Desktop::getInstance().findComponentAt (screenPoint))
        {
            if (auto* handler = getUnignoredAncestor (findEnclosingHandler (comp)))
                if (isParentOf (handler))
                    return handler;
        }

        return nullptr;
        */
    }
    
    /**
      | Returns the deepest UI element which
      | currently has focus.
      | 
      | This can be a child of this UI element
      | or, if no child is focused, this element
      | itself.
      | 
      | -----------
      | @note
      | 
      | this can be different to the value of
      | the Component with keyboard focus returned
      | by Component::getCurrentlyFocusedComponent().
      | 
      | @see hasFocus
      |
      */
    pub fn get_child_focus(&mut self) -> *mut AccessibilityHandler {
        
        todo!();
        /*
            return hasFocus (true) ? getUnignoredAncestor (currentlyFocusedHandler)
                               : nullptr;
        */
    }
    
    /**
      | Returns true if this UI element has the
      | focus.
      | 
      | -----------
      | @param trueIfChildFocused
      | 
      | if this is true, this method will also
      | return true if any child of this UI element
      | in the accessibility hierarchy has
      | focus
      |
      */
    pub fn has_focus(&self, true_if_child_focused: bool) -> bool {
        
        todo!();
        /*
            return currentlyFocusedHandler != nullptr
                && (currentlyFocusedHandler == this
                    || (trueIfChildFocused && isParentOf (currentlyFocusedHandler)));
        */
    }
    
    /**
      | Tries to give focus to this UI element.
      | 
      | If the UI element is focusable and not
      | ignored this will update the currently
      | focused element, try to give keyboard
      | focus to the Component it represents,
      | and notify any listening accessibility
      | clients that the current focus has changed.
      | 
      | @see hasFocus, giveAwayFocus
      |
      */
    pub fn grab_focus(&mut self)  {
        
        todo!();
        /*
            if (! hasFocus (false))
            grabFocusInternal (true);
        */
    }
    
    /**
      | If this UI element or any of its children
      | in the accessibility hierarchy currently
      | have focus, this will defocus it.
      | 
      | This will also give away the keyboard
      | focus from the Component it represents,
      | and notify any listening accessibility
      | clients that the current focus has changed.
      | 
      | @see hasFocus, grabFocus
      |
      */
    pub fn give_away_focus(&self)  {
        
        todo!();
        /*
            if (hasFocus (true))
            giveAwayFocusInternal();
        */
    }
    
    pub fn grab_focus_internal(&mut self, can_try_parent: bool)  {
        
        todo!();
        /*
            if (getCurrentState().isFocusable() && ! isIgnored())
        {
            takeFocus();
            return;
        }

        if (isParentOf (currentlyFocusedHandler))
            return;

        if (auto traverser = component.createFocusTraverser())
        {
            if (auto* defaultComp = traverser->getDefaultComponent (&component))
            {
                if (auto* handler = getUnignoredAncestor (findEnclosingHandler (defaultComp)))
                {
                    if (isParentOf (handler))
                    {
                        handler->grabFocusInternal (false);
                        return;
                    }
                }
            }
        }

        if (canTryParent)
            if (auto* parent = getParent())
                parent->grabFocusInternal (true);
        */
    }
    
    pub fn give_away_focus_internal(&self)  {
        
        todo!();
        /*
            currentlyFocusedHandler = nullptr;
        notifyAccessibilityEventInternal (*this, InternalAccessibilityEvent::focusChanged);
        */
    }
    
    pub fn take_focus(&mut self)  {
        
        todo!();
        /*
            currentlyFocusedHandler = this;
        notifyAccessibilityEventInternal (*this, InternalAccessibilityEvent::focusChanged);

        if ((component.isShowing() || component.isOnDesktop())
            && component.getWantsKeyboardFocus()
            && ! component.hasKeyboardFocus (true))
        {
            component.grabKeyboardFocus();
        }
        */
    }
}
