crate::ix!();

pub struct MidiDeviceListEntry {
    base:        ReferenceCountedObject,
    device_info: MidiDeviceInfo,
    in_device:   Box<MidiInput>,
    out_device:  Box<MidiOutput>,
}

impl HasPtr for MidiDeviceListEntry {
    type Ptr = ReferenceCountedObjectPtr<MidiDeviceListEntry>;
}

impl MidiDeviceListEntry {

    pub fn new(info: MidiDeviceInfo) -> Self {
    
        todo!();
        /*
        : device_info(info),

        
        */
    }
}
