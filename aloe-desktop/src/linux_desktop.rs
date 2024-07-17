crate::ix!();

impl<'a> Desktop<'a> {

    #[cfg(target_os="linux")]
    pub fn set_kiosk_component(&mut self, 
        comp:              *mut Component<'a>,
        enable_or_disable: bool,
        _2:                bool)  {
        
        todo!();
        /*
            if (enableOrDisable)
            comp->setBounds (getDisplays().getDisplayForRect (comp->getScreenBounds())->totalArea);
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn find_displays(&mut self, master_scale: f32)  {
        
        todo!();
        /*
            if (XWindowSystem::getInstance()->getDisplay() != nullptr)
        {
            displays = XWindowSystem::getInstance()->findDisplays (masterScale);

            if (! displays.isEmpty())
                updateToLogical();
        }
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn can_use_semi_transparent_windows(&mut self) -> bool {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->canUseSemiTransparentWindows();
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn set_screen_saver_enabled(&mut self, is_enabled: bool)  {
        
        todo!();
        /*
            if (screenSaverAllowed != isEnabled)
        {
            screenSaverAllowed = isEnabled;
            XWindowSystem::getInstance()->setScreenSaverEnabled (screenSaverAllowed);
        }
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn is_screen_saver_enabled(&mut self) -> bool {
        
        todo!();
        /*
            return screenSaverAllowed;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_default_master_scale(&mut self) -> f64 {
        
        todo!();
        /*
            return 1.0;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn get_current_orientation(&self) -> DesktopDisplayOrientation {
        
        todo!();
        /*
            return upright;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn allowed_orientations_changed(&mut self)  {
        
        
    }
}

