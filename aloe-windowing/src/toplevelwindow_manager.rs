crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_TopLevelWindow.cpp]

/**
  | Keeps track of the active top level window.
  |
  */
#[no_copy]
pub struct TopLevelWindowManager<'a> {
    base:           Timer,
    base2:          DeletedAtShutdown,
    windows:        Vec<*mut TopLevelWindow<'a>>,
    current_active: *mut TopLevelWindow<'a>, // default = nullptr
}

impl<'a> Default for TopLevelWindowManager<'a> {

    fn default() -> Self {
        todo!();
    }
}

impl<'a> Drop for TopLevelWindowManager<'a> {
    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

aloe_declare_singleton_singlethreaded_minimal!{
    TopLevelWindowManager
}

impl<'a> TopLevelWindowManager<'a> {

    
    pub fn check_focus_async(&mut self)  {
        
        todo!();
        /*
            startTimer (10);
        */
    }
    
    pub fn check_focus(&mut self)  {
        
        todo!();
        /*
            startTimer (jmin (1731, getTimerInterval() * 2));

            auto* newActive = findCurrentlyActiveWindow();

            if (newActive != currentActive)
            {
                currentActive = newActive;

                for (int i = windows.size(); --i >= 0;)
                    if (auto* tlw = windows[i])
                        tlw->setWindowActive (isWindowActive (tlw));

                Desktop::getInstance().triggerFocusCallback();
            }
        */
    }
    
    pub fn add_window(&mut self, w: *mut TopLevelWindow) -> bool {
        
        todo!();
        /*
            windows.add (w);
            checkFocusAsync();

            return isWindowActive (w);
        */
    }
    
    pub fn remove_window(&mut self, w: *mut TopLevelWindow)  {
        
        todo!();
        /*
            checkFocusAsync();

            if (currentActive == w)
                currentActive = nullptr;

            windows.removeFirstMatchingValue (w);

            if (windows.isEmpty())
                deleteInstance();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            checkFocus();
        */
    }
    
    pub fn is_window_active(&self, tlw: *mut TopLevelWindow) -> bool {
        
        todo!();
        /*
            return (tlw == currentActive
                     || tlw->isParentOf (currentActive)
                     || tlw->hasKeyboardFocus (true))
                    && tlw->isShowing();
        */
    }
    
    pub fn find_currently_active_window(&self) -> *mut TopLevelWindow {
        
        todo!();
        /*
            if (Process::isForegroundProcess())
            {
                auto* focusedComp = Component::getCurrentlyFocusedComponent();
                auto* w = dynamic_cast<TopLevelWindow*> (focusedComp);

                if (w == nullptr && focusedComp != nullptr)
                    w = focusedComp->findParentComponentOfClass<TopLevelWindow>();

                if (w == nullptr)
                    w = currentActive;

                if (w != nullptr && w->isShowing())
                    return w;
            }

            return nullptr;
        */
    }
}

aloe_implement_singleton!{
    TopLevelWindowManager
}

pub fn aloe_check_currently_focused_top_level_window()  {
    
    todo!();
        /*
            if (auto* wm = TopLevelWindowManager::getInstanceWithoutCreating())
            wm->checkFocusAsync();
        */
}
