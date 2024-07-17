#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub struct Mp3DecoderVBRTagData {
    toc:         [u8; 100],
    sample_rate: i32,
    vbr_scale:   i32,
    headersize:  i32,
    flags:       u32,
    frames:      u32,
    bytes:       u32,
}

impl Mp3DecoderVBRTagData {

    pub fn read(&mut self, data: *const u8) -> bool {
        
        todo!();
        /*
            flags = 0;

            const int layer = (data[1] >> 1) & 3;
            if (layer != 1)
                return false;

            const int type = (data[1] >> 3) & 1;
            const int sampleRateIndex = (data[2] >> 2) & 3;
            const int mode = (data[3] >> 6) & 3;

            static constexpr short bitRates[3][16] =
            {
                { 0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, -1 }, // MPEG2
                { 0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, -1 }, // MPEG1
                { 0, 8, 16, 24, 32, 40, 48, 56, 64, -1, -1, -1, -1, -1, -1, -1 }, // MPEG 2.5
            };

            const int bitrate = bitRates[type][((data[2] >> 4) & 15)];

            const int sampleRates[3][4] =
            {
                { 22050, 24000, 16000, -1 }, // MPEG2
                { 44100, 48000, 32000, -1 }, // MPEG1
                { 11025, 12000, 8000,  -1 }, // MPEG2.5
            };

            if ((data[1] >> 4) == 0xe)
                sampleRate = sampleRates[2][sampleRateIndex];
            else
                sampleRate = sampleRates[type][sampleRateIndex];

            data += type != 0 ? (mode != 3 ? (32 + 4) : (17 + 4))
                              : (mode != 3 ? (17 + 4) : (9 + 4));

            if (! isVbrTag (data))
                return false;

            data += 4;
            flags = ByteOrder::bigEndianInt (data);
            data += 4;

            if (flags & 1)
            {
                frames = ByteOrder::bigEndianInt (data);
                data += 4;
            }

            if (flags & 2)
            {
                bytes = ByteOrder::bigEndianInt (data);
                data += 4;
            }

            if (flags & 4)
            {
                for (int i = 0; i < 100; ++i)
                    toc[i] = data[i];

                data += 100;
            }

            vbrScale = -1;

            if (flags & 8)
                vbrScale = (int) ByteOrder::bigEndianInt (data);

            headersize = ((type + 1) * 72000 * bitrate) / sampleRate;
            return true;
        */
    }
    
    pub fn is_vbr_tag(d: *const u8) -> bool {
        
        todo!();
        /*
            return (d[0] == 'X' && d[1] == 'i' && d[2] == 'n' && d[3] == 'g')
                || (d[0] == 'I' && d[1] == 'n' && d[2] == 'f' && d[3] == 'o');
        */
    }
}
