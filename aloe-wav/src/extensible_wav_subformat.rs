crate::ix!();

//#[bitfield]
pub struct ExtensibleWavSubFormat
{
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl PartialEq<ExtensibleWavSubFormat> for ExtensibleWavSubFormat {
    
    #[inline] fn eq(&self, other: &ExtensibleWavSubFormat) -> bool {
        todo!();
        /*
            return memcmp (this, &other, sizeof (*this)) == 0;
        */
    }
}

impl Eq for ExtensibleWavSubFormat {}

lazy_static!{
    /*
    static const ExtensibleWavSubFormat pcmFormat       = { 0x00000001, 0x0000, 0x0010, { 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71 } };
        static const ExtensibleWavSubFormat IEEEFloatFormat = { 0x00000003, 0x0000, 0x0010, { 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71 } };
        static const ExtensibleWavSubFormat ambisonicFormat = { 0x00000001, 0x0721, 0x11d3, { 0x86, 0x44, 0xC8, 0xC1, 0xCA, 0x00, 0x00, 0x00 } };
    */
}
