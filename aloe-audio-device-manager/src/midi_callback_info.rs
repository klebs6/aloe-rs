crate::ix!();

pub struct MidiCallbackInfo
{
    device_identifier: String,
    callback:          *mut dyn MidiInputCallback,
}
