crate::ix!();

#[no_copy]
#[leak_detector]
pub struct BluetoothMidiSelectorOverlay<'a> {
    base:                   Component<'a>,
    bounds:                 Rectangle<i32>,
    bluetooth_devices_list: AndroidBluetoothMidiDevicesListBox<'a>,
}

impl<'a> Drop for BluetoothMidiSelectorOverlay<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            AndroidBluetoothMidiInterface::startStopScan (false);
         */
    }
}

impl<'a> BluetoothMidiSelectorOverlay<'a> {

    pub fn new(
        exit_callback_to_use: *mut dyn ModalComponentManagerCallback,
        bounds_to_use:        &Rectangle<i32>

    ) -> Self {
    
        todo!();
        /*
        : bounds(boundsToUse),

            std::unique_ptr<ModalComponentManager::Callback> exitCallback (exitCallbackToUse);

            AndroidBluetoothMidiInterface::startStopScan (true);

            setAlwaysOnTop (true);
            setVisible (true);
            addToDesktop (ComponentPeer::windowHasDropShadow);

            if (bounds.isEmpty())
                setBounds (0, 0, getParentWidth(), getParentHeight());
            else
                setBounds (bounds);

            toFront (true);
            setOpaque (! bounds.isEmpty());

            addAndMakeVisible (bluetoothDevicesList);
            enterModalState (true, exitCallback.release(), true);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (bounds.isEmpty() ? Colours::black.withAlpha (0.6f) : Colours::black);

            g.setColour (Colour (0xffdfdfdf));
            Rectangle<int> overlayBounds = getOverlayBounds();
            g.fillRect (overlayBounds);

            g.setColour (Colours::black);
            g.setFont (16);
            g.drawText ("Bluetooth MIDI Devices",
                        overlayBounds.removeFromTop (20).reduced (3, 3),
                        Justification::topLeft, true);

            overlayBounds.removeFromTop (2);

            g.setFont (12);
            g.drawText ("tap to connect/disconnect",
                        overlayBounds.removeFromTop (18).reduced (3, 3),
                        Justification::topLeft, true);
        */
    }
    
    pub fn input_attempt_when_modal(&mut self)  {
        
        todo!();
        /*
            exitModalState (0);
        */
    }
    
    pub fn mouse_drag(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            exitModalState (0);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            update();
        */
    }
    
    pub fn parent_size_changed(&mut self)  {
        
        todo!();
        /*
            update();
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            if (bounds.isEmpty())
                setBounds (0, 0, getParentWidth(), getParentHeight());
            else
                setBounds (bounds);

            bluetoothDevicesList.setBounds (getOverlayBounds().withTrimmedTop (40));
        */
    }
    
    pub fn get_overlay_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (bounds.isEmpty())
            {
                const int pw = getParentWidth();
                const int ph = getParentHeight();

                return Rectangle<int> (pw, ph).withSizeKeepingCentre (jmin (400, pw - 14),
                                                                      jmin (300, ph - 40));
            }

            return bounds.withZeroOrigin();
        */
    }
}
