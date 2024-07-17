crate::ix!();

#[cfg(target_os="android")]
pub struct PushNotificationsDemoParamsView<'a> {
    base:           Component<'a>,
    row_components: Vec<Box<PushNotificationsDemoRowComponent<'a>>>,
}

#[cfg(target_os="android")]
impl<'a> Default for PushNotificationsDemoParamsView<'a> {
    
    fn default() -> Self {

        todo!();
        /*


            // For now, to be able to dismiss mobile keyboard.
                setWantsKeyboardFocus (true)
        */
    }
}

#[cfg(target_os="android")]
impl<'a> PushNotificationsDemoParamsView<'a> {
    
    pub fn add_row_component(&mut self, rc: *mut PushNotificationsDemoRowComponent<'a>)  {
        
        todo!();
        /*
            rowComponents.add (rc);
                addAndMakeVisible (rc);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto totalRowUnits = 0;

                for (auto* rc : rowComponents)
                    totalRowUnits += rc->rowUnits;

                auto rowHeight = getHeight() / totalRowUnits;

                auto bounds = getLocalBounds();

                for (auto* rc : rowComponents)
                    rc->setBounds (bounds.removeFromTop (rc->rowUnits * rowHeight));

                auto* last = rowComponents[rowComponents.size() - 1];
                last->setBounds (last->getBounds().withHeight (getHeight() - last->getY()));
        */
    }
}

