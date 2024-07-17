crate::ix!();

#[no_copy]
#[leak_detector]
pub struct OSCMonitorDemo<'a> {
    base:                    Component<'a>,
    port_number_label:       Label<'a>,          // default = { "UDP Port Number: "  }
    port_number_field:       Label<'a>,          // default = { "9002"  }
    connect_button:          TextButton<'a>,     // default = { "Connect"  }
    clear_button:            TextButton<'a>,     // default = { "Clear"  }
    connection_status_label: Label<'a>,
    osc_log_list_box:        OSCLogListBox<'a>,
    osc_receiver:            OSCReceiver,
    current_port_number:     i32,            // default = -1
}

impl<'a> OSCReceiverListener<OSCReceiverMessageLoopCallback> for OSCMonitorDemo<'a> {

    fn osc_message_received(&mut self, message: &OSCMessage)  {
        
        todo!();
        /*
            oscLogListBox.addOSCMessage (message);
        */
    }
    
    fn osc_bundle_received(&mut self, bundle: &OSCBundle)  {
        
        todo!();
        /*
            oscLogListBox.addOSCBundle (bundle);
        */
    }
}

impl<'a> Default for OSCMonitorDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            portNumberLabel.setBounds (10, 18, 130, 25);
            addAndMakeVisible (portNumberLabel);

            portNumberField.setEditable (true, true, true);
            portNumberField.setBounds (140, 18, 50, 25);
            addAndMakeVisible (portNumberField);

            connectButton.setBounds (210, 18, 100, 25);
            addAndMakeVisible (connectButton);
            connectButton.onClick = [this] { connectButtonClicked(); };

            clearButton.setBounds (320, 18, 60, 25);
            addAndMakeVisible (clearButton);
            clearButton.onClick = [this] { clearButtonClicked(); };

            connectionStatusLabel.setBounds (450, 18, 240, 25);
            updateConnectionStatusLabel();
            addAndMakeVisible (connectionStatusLabel);

            oscLogListBox.setBounds (0, 60, 700, 340);
            addAndMakeVisible (oscLogListBox);

            oscReceiver.addListener (this);
            oscReceiver.registerFormatErrorHandler ([this] (const char* data, int dataSize)
                                                    {
                                                        oscLogListBox.addInvalidOSCPacket (data, dataSize);
                                                    })
        */
    }
}

impl<'a> OSCMonitorDemo<'a> {

    pub fn connect_button_clicked(&mut self)  {
        
        todo!();
        /*
            if (! isConnected())
                connect();
            else
                disconnect();

            updateConnectionStatusLabel();
        */
    }
    
    pub fn clear_button_clicked(&mut self)  {
        
        todo!();
        /*
            oscLogListBox.clear();
        */
    }
    
    pub fn connect(&mut self)  {
        
        todo!();
        /*
            auto portToConnect = portNumberField.getText().getIntValue();

            if (! isValidOscPort (portToConnect))
            {
                handleInvalidPortNumberEntered();
                return;
            }

            if (oscReceiver.connect (portToConnect))
            {
                currentPortNumber = portToConnect;
                connectButton.setButtonText ("Disconnect");
            }
            else
            {
                handleConnectError (portToConnect);
            }
        */
    }
    
    pub fn disconnect(&mut self)  {
        
        todo!();
        /*
            if (oscReceiver.disconnect())
            {
                currentPortNumber = -1;
                connectButton.setButtonText ("Connect");
            }
            else
            {
                handleDisconnectError();
            }
        */
    }
    
    pub fn handle_connect_error(&mut self, failed_port: i32)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                              "OSC Connection error",
                                              "Error: could not connect to port " + String (failedPort),
                                              "OK");
        */
    }
    
    pub fn handle_disconnect_error(&mut self)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                              "Unknown error",
                                              "An unknown error occurred while trying to disconnect from UDP port.",
                                              "OK");
        */
    }
    
    pub fn handle_invalid_port_number_entered(&mut self)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                              "Invalid port number",
                                              "Error: you have entered an invalid UDP port number.",
                                              "OK");
        */
    }
    
    pub fn is_connected(&self) -> bool {
        
        todo!();
        /*
            return currentPortNumber != -1;
        */
    }
    
    pub fn is_valid_osc_port(&self, port: i32) -> bool {
        
        todo!();
        /*
            return port > 0 && port < 65536;
        */
    }
    
    pub fn update_connection_status_label(&mut self)  {
        
        todo!();
        /*
            String text = "Status: ";

            if (isConnected())
                text += "Connected to UDP port " + String (currentPortNumber);
            else
                text += "Disconnected";

            auto textColour = isConnected() ? Colours::green : Colours::red;

            connectionStatusLabel.setText (text, dontSendNotification);
            connectionStatusLabel.setFont (Font (15.00f, Font::bold));
            connectionStatusLabel.setColour (Label::textColourId, textColour);
            connectionStatusLabel.setJustificationType (Justification::centredRight);
        */
    }
}
