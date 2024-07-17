crate::ix!();

///------------------
pub struct AndroidBluetoothMidiDevicesListBox<'a> {
    base:               ListBox<'a>,
    base2:              ListBoxModel,
    base3:              Timer,
    devices:            Vec<AndroidBluetoothMidiDevice>,
    timer_period_in_ms: i32,
}

impl<'a> Default for AndroidBluetoothMidiDevicesListBox<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : timer_period_in_ms(1000),

            setRowHeight (40);
            setModel (this);
            setOutlineThickness (1);
            startTimer (timerPeriodInMs)
        */
    }
}

impl<'a> AndroidBluetoothMidiDevicesListBox<'a> {

    /**
      | callback from AndroidBluetoothMidiDevicesListBoxPairDeviceThread
      |
      */
    pub fn pair_device_thread_finished(&mut self)  {
        
        todo!();
        /*
            updateDeviceList();
            startTimer (timerPeriodInMs);
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return devices.size();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row_number: i32,
        g:          &mut Graphics,
        width:      i32,
        height:     i32,
        _4:         bool)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (rowNumber, devices.size()))
            {
                const AndroidBluetoothMidiDevice& device = devices.getReference (rowNumber);
                const String statusString (getDeviceStatusString (device.connectionStatus));

                g.fillAll (Colours::white);

                const float xmargin = 3.0f;
                const float ymargin = 3.0f;
                const float fontHeight = 0.4f * (float) height;
                const float deviceNameWidth = 0.6f * (float) width;

                g.setFont (fontHeight);

                g.setColour (getDeviceNameFontColour (device.connectionStatus));
                g.drawText (device.name,
                            Rectangle<float> (xmargin, ymargin, deviceNameWidth - (2.0f * xmargin), (float) height - (2.0f * ymargin)),
                            Justification::topLeft, true);

                g.setColour (getDeviceStatusFontColour (device.connectionStatus));
                g.drawText (statusString,
                            Rectangle<float> (deviceNameWidth + xmargin, ymargin,
                                              (float) width - deviceNameWidth - (2.0f * xmargin), (float) height - (2.0f * ymargin)),
                            Justification::topRight, true);

                g.setColour (Colours::grey);
                g.drawHorizontalLine (height - 1, xmargin, (float) width);
            }
        */
    }
    
    pub fn get_device_name_font_colour(device_status: AndroidBluetoothMidiDevicesListBoxDeviceStatus) -> Colour {
        
        todo!();
        /*
            if (deviceStatus == AndroidBluetoothMidiDevice::offline)
                return Colours::grey;

            return Colours::black;
        */
    }
    
    pub fn get_device_status_font_colour(device_status: AndroidBluetoothMidiDevicesListBoxDeviceStatus) -> Colour {
        
        todo!();
        /*
            if (deviceStatus == AndroidBluetoothMidiDevice::offline
                || deviceStatus == AndroidBluetoothMidiDevice::connecting
                || deviceStatus == AndroidBluetoothMidiDevice::disconnecting)
                return Colours::grey;

            if (deviceStatus == AndroidBluetoothMidiDevice::connected)
                return Colours::green;

            return Colours::black;
        */
    }
    
    pub fn get_device_status_string(device_status: AndroidBluetoothMidiDevicesListBoxDeviceStatus) -> String {
        
        todo!();
        /*
            if (deviceStatus == AndroidBluetoothMidiDevice::offline)        return "Offline";
            if (deviceStatus == AndroidBluetoothMidiDevice::connected)      return "Connected";
            if (deviceStatus == AndroidBluetoothMidiDevice::disconnected)   return "Not connected";
            if (deviceStatus == AndroidBluetoothMidiDevice::connecting)     return "Connecting...";
            if (deviceStatus == AndroidBluetoothMidiDevice::disconnecting)  return "Disconnecting...";

            // unknown device state!
            jassertfalse;
            return "Status unknown";
        */
    }
    
    pub fn list_box_item_clicked(&mut self, 
        row: i32,
        _1:  &MouseEvent)  {
        
        todo!();
        /*
            const AndroidBluetoothMidiDevice& device = devices.getReference (row);

            if (device.connectionStatus == AndroidBluetoothMidiDevice::disconnected)
                disconnectedDeviceClicked (row);

            else if (device.connectionStatus == AndroidBluetoothMidiDevice::connected)
                connectedDeviceClicked (row);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            updateDeviceList();
        */
    }
    
    pub fn disconnected_device_clicked(&mut self, row: i32)  {
        
        todo!();
        /*
            stopTimer();

            AndroidBluetoothMidiDevice& device = devices.getReference (row);
            device.connectionStatus = AndroidBluetoothMidiDevice::connecting;
            updateContent();
            repaint();

            new AndroidBluetoothMidiDevicesListBoxPairDeviceThread (device.bluetoothAddress, *this);
        */
    }
    
    pub fn connected_device_clicked(&mut self, row: i32)  {
        
        todo!();
        /*
            AndroidBluetoothMidiDevice& device = devices.getReference (row);
            device.connectionStatus = AndroidBluetoothMidiDevice::disconnecting;
            updateContent();
            repaint();
            AndroidBluetoothMidiInterface::unpairBluetoothMidiDevice (device.bluetoothAddress);
        */
    }
    
    pub fn update_device_list(&mut self)  {
        
        todo!();
        /*
            StringArray bluetoothAddresses = AndroidBluetoothMidiInterface::getBluetoothMidiDevicesNearby();

            Vec<AndroidBluetoothMidiDevice> newDevices;

            for (String* address = bluetoothAddresses.begin();
                 address != bluetoothAddresses.end(); ++address)
            {
                String name = AndroidBluetoothMidiInterface::getHumanReadableStringForBluetoothAddress (*address);

                AndroidBluetoothMidiDevicesListBoxDeviceStatus status;
                switch (AndroidBluetoothMidiInterface::isBluetoothDevicePaired (*address))
                {
                    case AndroidBluetoothMidiInterface::pairing:
                        status = AndroidBluetoothMidiDevice::connecting;
                        break;
                    case AndroidBluetoothMidiInterface::paired:
                        status = AndroidBluetoothMidiDevice::connected;
                        break;
                    case AndroidBluetoothMidiInterface::unpaired:
                    default:
                        status = AndroidBluetoothMidiDevice::disconnected;
                }

                newDevices.add (AndroidBluetoothMidiDevice (name, *address, status));
            }

            devices.swapWith (newDevices);
            updateContent();
            repaint();
        */
    }
}
