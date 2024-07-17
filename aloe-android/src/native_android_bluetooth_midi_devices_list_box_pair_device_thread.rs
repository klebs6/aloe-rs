crate::ix!();

pub type AndroidBluetoothMidiDevicesListBoxDeviceStatus = AndroidBluetoothMidiDeviceConnectionStatus;
    
pub struct AndroidBluetoothMidiDevicesListBoxPairDeviceThread<'a> {
    base:              Thread,
    base2:             AsyncUpdater<'a>,
    bluetooth_address: String,
    owner:             ComponentSafePointer<'a, AndroidBluetoothMidiDevicesListBox<'a>>,
}

impl<'a> AndroidBluetoothMidiDevicesListBoxPairDeviceThread<'a> {

    pub fn new(
        bluetooth_address_of_device_to_pair: &String,
        owner_list_box:                      &mut AndroidBluetoothMidiDevicesListBox) -> Self {
    
        todo!();
        /*


            : Thread ("Aloe Bluetooth MIDI Device Pairing Thread"),
                  bluetoothAddress (bluetoothAddressOfDeviceToPair),
                  owner (&ownerListBox)

                startThread();
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            AndroidBluetoothMidiInterface::pairBluetoothMidiDevice (bluetoothAddress);
                triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            if (owner != nullptr)
                    owner->pairDeviceThreadFinished();

                delete this;
        */
    }
}

