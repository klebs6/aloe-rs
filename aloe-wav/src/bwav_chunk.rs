crate::ix!();

//#[bitfield]
pub struct BWAVChunk {
    description:      [B8; 256],
    originator:       [B8; 32],
    originator_ref:   [B8; 32],
    origination_date: [B8; 10],
    origination_time: [B8; 8],
    time_ref_low:     B32,
    time_ref_high:    B32,
    version:          B16,
    umid:             [B8; 64],
    reserved:         [B8; 190],
    coding_history:   [B8; 1],
}

impl BWAVChunk {

    pub fn copy_to(
        &self, 
        values:     &mut Vec<(String,String)>,
        total_size: i32

    ) {
        
        todo!();
        /*
            values.set (WavAudioFormat::bwavDescription,     String::fromUTF8 (description,     sizeof (description)));
                values.set (WavAudioFormat::bwavOriginator,      String::fromUTF8 (originator,      sizeof (originator)));
                values.set (WavAudioFormat::bwavOriginatorRef,   String::fromUTF8 (originatorRef,   sizeof (originatorRef)));
                values.set (WavAudioFormat::bwavOriginationDate, String::fromUTF8 (originationDate, sizeof (originationDate)));
                values.set (WavAudioFormat::bwavOriginationTime, String::fromUTF8 (originationTime, sizeof (originationTime)));

                auto timeLow  = ByteOrder::swapIfBigEndian (timeRefLow);
                auto timeHigh = ByteOrder::swapIfBigEndian (timeRefHigh);
                auto time = (((int64) timeHigh) << 32) + timeLow;

                values.set (WavAudioFormat::bwavTimeReference, String (time));
                values.set (WavAudioFormat::bwavCodingHistory,
                            String::fromUTF8 (codingHistory, totalSize - (int) offsetof (BWAVChunk, codingHistory)));
        */
    }
    
    pub fn create_from(
        values: &Vec<(String,String)>

    ) -> MemoryBlock {

        todo!();
        /*
            MemoryBlock data (roundUpSize (sizeof (BWAVChunk) + values[WavAudioFormat::bwavCodingHistory].getNumBytesAsUTF8()));
                data.fillWith (0);

                auto* b = (BWAVChunk*) data.getData();

                // Allow these calls to overwrite an extra byte at the end, which is fine as long
                // as they get called in the right order..
                values[WavAudioFormat::bwavDescription]    .copyToUTF8 (b->description, 257);
                values[WavAudioFormat::bwavOriginator]     .copyToUTF8 (b->originator, 33);
                values[WavAudioFormat::bwavOriginatorRef]  .copyToUTF8 (b->originatorRef, 33);
                values[WavAudioFormat::bwavOriginationDate].copyToUTF8 (b->originationDate, 11);
                values[WavAudioFormat::bwavOriginationTime].copyToUTF8 (b->originationTime, 9);

                auto time = values[WavAudioFormat::bwavTimeReference].getLargeIntValue();
                b->timeRefLow = ByteOrder::swapIfBigEndian ((uint32) (time & 0xffffffff));
                b->timeRefHigh = ByteOrder::swapIfBigEndian ((uint32) (time >> 32));

                values[WavAudioFormat::bwavCodingHistory].copyToUTF8 (b->codingHistory, 0x7fffffff);

                if (b->description[0] != 0
                    || b->originator[0] != 0
                    || b->originationDate[0] != 0
                    || b->originationTime[0] != 0
                    || b->codingHistory[0] != 0
                    || time != 0)
                {
                    return data;
                }

                return {};
        */
    }
}

