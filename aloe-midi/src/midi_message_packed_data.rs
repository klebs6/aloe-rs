crate::ix!();

pub union MidiMessagePackedData
{
    allocated_data: *mut u8,
    as_bytes:       [u8; size_of::<*mut u8>()],
}
