#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub const MP3_FRAME_DOWNSAMPLE_LIMIT: usize = 32;

pub enum Mp3FrameParseSuccessful { 
    no, 
    yes 
}

pub struct MP3Frame {
    layer:                 i32,
    frame_size:            i32,
    num_channels:          i32,
    single:                i32,

    /**
      | 0 = mpeg-1, 1 = mpeg-2/LSF
      |
      */
    lsf:                   i32,

    /**
      | true = mpeg-2.5, false = mpeg-1/2
      |
      */
    mpeg25:                bool,

    crc_16follows_header:  bool,
    bitrate_index:         i32,
    sample_rate_index:     i32,
    padding:               i32,
    mode:                  i32,
    mode_ext:              i32,
    layer_2sub_band_limit: i32,
    allocation_table:      *const Mp3DecoderAllocationTable,
}

impl Default for MP3Frame {
    
    fn default() -> Self {
        todo!();
        /*
            zeromem (this, sizeof (MP3Frame));
            single = -1;
        */
    }
}

impl MP3Frame {

    pub fn select_layer_2table(&mut self)  {
        
        todo!();
        /*
            static constexpr int translate[3][2][16] =
            {
                { { 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 1, 1, 1, 1, 1, 0 }, { 0, 2, 2, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0 } },
                { { 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, { 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 } },
                { { 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 1, 1, 1, 1, 1, 0 }, { 0, 3, 3, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0 } }
            };

            static const Mp3DecoderAllocationTable* const tables[] = { allocTable0, allocTable1, allocTable2, allocTable3, allocTable4 };
            static constexpr int8 limits[] = { 27, 30, 8, 12, 30 };

            const int index = lsf ? 4 : translate[sampleRateIndex][2 - numChannels][bitrateIndex];
            layer2SubBandLimit = limits[index];
            allocationTable = tables[index];
        */
    }
    
    pub fn get_frequency(&self) -> i32 {
        
        todo!();
        /*
            const int frequencies[] = { 44100, 48000, 32000, 22050, 24000, 16000, 11025, 12000, 8000 };
            return frequencies[sampleRateIndex];
        */
    }
    
    pub fn decode_header(&mut self, header: u32) -> Mp3FrameParseSuccessful {
        
        todo!();
        /*
            jassert (((header >> 10) & 3) != 3);

            mpeg25              = (header & (1 << 20)) == 0;
            lsf                 = mpeg25 ? 1 : ((header & (1 << 19)) ? 0 : 1);
            layer               = (int) (4 - ((header >> 17) & 3));
            sampleRateIndex     = (int) ((header >> 10) & 3) + (mpeg25 ? 6 : (lsf * 3));
            crc16FollowsHeader  = ((header >> 16) & 1) == 0;
            bitrateIndex        = (header >> 12) & 15;
            padding             = (header >> 9) & 1;
            mode                = (header >> 6) & 3;
            modeExt             = (header >> 4) & 3;
            //extension         = (header >> 8) & 1;
            //copyright         = (header >> 3) & 1;
            //original          = (header >> 2) & 1;
            //emphasis          = header & 3;
            numChannels         = (mode == 3) ? 1 : 2;

            static constexpr int frameSizes[2][3][16] =
            {
                { { 0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448 },
                  { 0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384 },
                  { 0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320 } },

                { { 0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256 },
                  { 0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160 },
                  { 0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160 } }
            };

            if (bitrateIndex == 0)
            {
                jassertfalse; // This means the file is using "free format". Apparently very few decoders
                              // support this mode, and this one certainly doesn't handle it correctly!
                frameSize = 0;
                return Mp3FrameParseSuccessful::no;
            }

            switch (layer)
            {
                case 1: frameSize = (((frameSizes[lsf][0][bitrateIndex] * 12000) / getFrequency() + padding) * 4) - 4; break;
                case 2: frameSize = (frameSizes[lsf][1][bitrateIndex] * 144000)  / getFrequency() + (padding - 4); break;
                case 3: frameSize = (bitrateIndex == 0) ? 0 : ((frameSizes[lsf][2][bitrateIndex] * 144000) / (getFrequency() << lsf) + (padding - 4)); break;
                default: break;
            }

            return Mp3FrameParseSuccessful::yes;
        */
    }
}
