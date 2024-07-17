crate::ix!();

#[cfg(target_os="android")]
pub struct PushNotificationsDemoTabbedComponent<'a> {
    base:                       TabbedComponent<'a>,
    showed_remote_instructions: bool, // default = false
}

#[cfg(target_os="android")]
impl<'a> PushNotificationsDemoTabbedComponent<'a> {

    pub fn new(orientation: TabbedButtonBar::Orientation) -> Self {
    
        todo!();
        /*
        : tabbed_component(orientation),

        
        */
    }
    
    pub fn current_tab_changed(&mut self, 
        _0:                   i32,
        new_current_tab_name: &String)  {
        
        todo!();
        /*
            if (! showedRemoteInstructions && newCurrentTabName == "Remote")
                {
                    PushNotificationsDemo::showRemoteInstructions();
                    showedRemoteInstructions = true;
                }
        */
    }
}
