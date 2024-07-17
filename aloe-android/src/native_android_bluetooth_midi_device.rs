crate::ix!();

pub enum AndroidBluetoothMidiDeviceConnectionStatus
{
    offline,
    connected,
    disconnected,
    connecting,
    disconnecting
}

///-------------------------------------
pub struct AndroidBluetoothMidiDevice {
    name:              String,
    bluetooth_address: String,
    connection_status: AndroidBluetoothMidiDeviceConnectionStatus,
}

impl PartialEq<AndroidBluetoothMidiDevice> for AndroidBluetoothMidiDevice {
    
    #[inline] fn eq(&self, other: &AndroidBluetoothMidiDevice) -> bool {
        todo!();
        /*
            return bluetoothAddress == other.bluetoothAddress;
        */
    }
}

impl Eq for AndroidBluetoothMidiDevice {}

impl AndroidBluetoothMidiDevice {

    pub fn new(
        device_name: String,
        address:     String,
        status:      AndroidBluetoothMidiDeviceConnectionStatus) -> Self {
    
        todo!();
        /*
        : name(deviceName),
        : bluetooth_address(address),
        : connection_status(status),

            // can't create a device without a valid name and bluetooth address!
            jassert (! name.isEmpty());
            jassert (! bluetoothAddress.isEmpty());
        */
    }
}

