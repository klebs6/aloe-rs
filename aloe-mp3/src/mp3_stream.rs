#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub const MP3_STREAM_STORED_START_POS_INTERVAL: usize = 4;

pub struct Mp3StreamSideInfoLayer1
{
    allocation:   [[u8; 32]; 2],
    scale_factor: [[u8; 32]; 2],
}

pub struct Mp3StreamSideInfoLayer2
{
    allocation:   [[u8; 32]; 2],
    scale_factor: [[[u8; 32]; 2]; 3],
}

//-----------------------------
#[no_copy]
#[leak_detector]
pub struct MP3Stream {
    frame:                      MP3Frame,
    vbr_tag_data:               Mp3DecoderVBRTagData,
    stream:                     BufferedInputStream<Box<dyn Read>>,
    num_frames:                 i32, // default = 0
    current_frame_index:        i32, // default = 0
    vbr_header_found:           bool, // default = false
    header_parsed:              bool,
    side_parsed:                bool,
    data_parsed:                bool,
    need_to_sync_bit_stream:    bool,
    is_free_format:             bool,
    was_free_format:            bool,
    side_info_size:             i32,
    data_size:                  i32,
    frame_size:                 i32,
    last_frame_size:            i32,
    last_frame_size_no_padding: i32,
    buffer_space_index:         i32,
    sideinfo:                   Mp3DecoderLayer3SideInfo,
    buffer_space:               [[u8; 2]; 2880 + 1024],
    buffer_pointer:             *mut u8,
    bit_index:                  i32,
    synth_bo:                   i32,
    hybrid_block:               [[[f32; 2]; 2]; 32 * 18],
    hybrid_block_index:         [i32; 2],
    synth_buffers:              [[[f32; 2]; 2]; 0x110],
    hybrid_in:                  [[[f32; 2]; 32]; 18],
    hybrid_out:                 [[[f32; 2]; 18]; 32],
    frame_stream_positions:     Vec<i64>,
}

impl MP3Stream {

    pub fn new(source: &mut dyn Read) -> Self {
    
        todo!();
        /*
        : stream(source, 8192),

            reset();
        */
    }
    
    pub fn decode_next_block(&mut self, 
        out0: *mut f32,
        out1: *mut f32,
        done: &mut i32) -> i32 {
        
        todo!();
        /*
            if (! headerParsed)
            {
                auto nextFrameOffset = scanForNextFrameHeader (false);

                if (lastFrameSize == -1 || needToSyncBitStream)
                {
                    needToSyncBitStream = false;
                    readVBRHeader();

                    if (vbrHeaderFound)
                        return 1;
                }

                if (nextFrameOffset < 0)
                    return -1;

                if (nextFrameOffset > 0)
                {
                    wasFreeFormat = false;
                    needToSyncBitStream = true;
                    auto size = (int) (bufferPointer - (bufferSpace[bufferSpaceIndex] + 512));

                    if (size > 2880)
                    {
                        size = 0;
                        bufferPointer = bufferSpace[bufferSpaceIndex] + 512;
                    }

                    auto toSkip = (size + nextFrameOffset) - 2880;

                    if (toSkip > 0)
                    {
                        stream.skipNextBytes (toSkip);
                        nextFrameOffset -= toSkip;
                    }

                    stream.read (bufferPointer, nextFrameOffset);
                    lastFrameSize += nextFrameOffset;
                }

                const auto successful = frame.decodeHeader ((uint32) stream.readIntBigEndian());

                if (successful == MP3Frame::ParseSuccessful::no)
                    return -1;

                headerParsed = true;
                frameSize = frame.frameSize;
                isFreeFormat = (frameSize == 0);
                sideInfoSize = frame.lsf != 0 ? ((frame.numChannels == 1) ? 9 : 17)
                                              : ((frame.numChannels == 1) ? 17 : 32);

                if (frame.crc16FollowsHeader)
                    sideInfoSize += 2;

                bufferSpaceIndex = 1 - bufferSpaceIndex;
                bufferPointer = bufferSpace[bufferSpaceIndex] + 512;
                bitIndex = 0;

                if (lastFrameSize < 0)
                    return 1;
            }

            if (! sideParsed)
            {
                if (frame.layer == 3)
                {
                    stream.read (bufferPointer, sideInfoSize);

                    if (frame.crc16FollowsHeader)
                        getBits (16);

                    auto bits = jmax (0, decodeLayer3SideInfo());
                    dataSize = (bits + 7) / 8;

                    if (! isFreeFormat)
                        dataSize = jmin (dataSize, frame.frameSize - sideInfoSize);
                }
                else
                {
                    dataSize = frame.frameSize;
                    sideInfoSize = 0;
                }

                sideParsed = true;
            }

            int result = 1;

            if (! dataParsed)
            {
                stream.read (bufferPointer, dataSize);

                if (out0 != nullptr)
                {
                    if (frame.layer < 3 && frame.crc16FollowsHeader)
                        getBits (16);

                    switch (frame.layer)
                    {
                        case 1:  decodeLayer1Frame (out0, out1, done); break;
                        case 2:  decodeLayer2Frame (out0, out1, done); break;
                        case 3:  decodeLayer3Frame (out0, out1, done); break;
                        default: break;
                    }
                }

                bufferPointer = bufferSpace[bufferSpaceIndex] + 512 + sideInfoSize + dataSize;
                dataParsed = true;
                result = 0;
            }

            if (isFreeFormat)
            {
                if (wasFreeFormat)
                {
                    frameSize = lastFrameSizeNoPadding + frame.padding;
                }
                else
                {
                    auto nextFrameOffset = scanForNextFrameHeader (true);

                    wasFreeFormat = isFreeFormat;

                    if (nextFrameOffset < 0)
                    {
                        lastFrameSize = frameSize;
                        return result;
                    }

                    frameSize = nextFrameOffset + sideInfoSize + dataSize;
                    lastFrameSizeNoPadding = frameSize - frame.padding;
                }
            }

            if (result == 0)
                return result;

            int bytes = frameSize - (sideInfoSize + dataSize);

            if (bytes > 0)
            {
                auto toSkip = bytes - 512;

                if (toSkip > 0)
                {
                    stream.skipNextBytes (toSkip);
                    bytes -= toSkip;
                    frameSize -= toSkip;
                }

                stream.read (bufferPointer, bytes);
                bufferPointer += bytes;
            }

            lastFrameSize = frameSize;
            wasFreeFormat = isFreeFormat;
            frameSize = 0;
            headerParsed = sideParsed = dataParsed = false;
            return result;
        */
    }
    
    pub fn seek(&mut self, frame_index: i32) -> bool {
        
        todo!();
        /*
            frameIndex = jmax (0, frameIndex);

            while (frameIndex >= frameStreamPositions.size() * storedStartPosInterval)
            {
                int dummy = 0;
                auto result = decodeNextBlock (nullptr, nullptr, dummy);

                if (result < 0)
                    return false;

                if (result > 0)
                    break;
            }

            frameIndex = jmin (frameIndex & ~(storedStartPosInterval - 1),
                               (frameStreamPositions.size() - 1) * storedStartPosInterval);

            stream.setPosition (frameStreamPositions.getUnchecked (frameIndex / storedStartPosInterval));
            currentFrameIndex = frameIndex;
            reset();
            return true;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            headerParsed = sideParsed = dataParsed = isFreeFormat = wasFreeFormat = false;
            lastFrameSize = -1;
            needToSyncBitStream = true;
            frameSize = sideInfoSize = dataSize = bitIndex = 0;
            lastFrameSizeNoPadding = bufferSpaceIndex = 0;
            bufferPointer = bufferSpace[bufferSpaceIndex] + 512;
            synthBo = 1;

            zerostruct (sideinfo);
            zeromem (bufferSpace, sizeof (bufferSpace));
            zeromem (hybridBlock, sizeof (hybridBlock));
            zeromem (hybridBlockIndex, sizeof (hybridBlockIndex));
            zeromem (synthBuffers, sizeof (synthBuffers));
        */
    }
    
    pub fn is_valid_header(
        header:    u32,
        old_layer: i32) -> bool {
        
        todo!();
        /*
            auto newLayer = (int) (4 - ((header >> 17) & 3));

            return (header & 0xffe00000) == 0xffe00000
                    && newLayer != 4
                    && (oldLayer <= 0 || newLayer == oldLayer)
                    && ((header >> 12) & 15) != 15
                    && ((header >> 10) & 3) != 3
                    && (header & 3) != 2;
        */
    }
    
    pub fn roll_back_buffer_pointer(&mut self, backstep: i32) -> bool {
        
        todo!();
        /*
            if (lastFrameSize < 0 && backstep > 0)
                return false;

            auto* oldBuffer = bufferSpace[1 - bufferSpaceIndex] + 512;
            bufferPointer -= backstep;

            if (backstep != 0)
                memcpy (bufferPointer, oldBuffer + lastFrameSize - backstep, (size_t) backstep);

            bitIndex = 0;
            return true;
        */
    }
    
    pub fn get_bits(&mut self, num_bits: i32) -> u32 {
        
        todo!();
        /*
            if (numBits <= 0 || bufferPointer == nullptr)
                return 0;

            const auto result = (uint32) (((((((bufferPointer[0] << 8) | bufferPointer[1]) << 8)
                                           | bufferPointer[2]) << bitIndex) & 0xffffff) >> (24 - numBits));
            bitIndex += numBits;
            bufferPointer += (bitIndex >> 3);
            bitIndex &= 7;
            return result;
        */
    }
    
    pub fn get_one_bit(&mut self) -> u32 {
        
        todo!();
        /*
            auto result = (uint8) (*bufferPointer << bitIndex);
            ++bitIndex;
            bufferPointer += (bitIndex >> 3);
            bitIndex &= 7;
            return (uint32) (result >> 7);
        */
    }
    
    pub fn get_bits_unchecked(&mut self, num_bits: i32) -> u32 {
        
        todo!();
        /*
            const auto result = (uint32) (((((bufferPointer[0] << 8) | bufferPointer[1]) << bitIndex) & 0xffff) >> (16 - numBits));
            bitIndex += numBits;
            bufferPointer += (bitIndex >> 3);
            bitIndex &= 7;
            return result;
        */
    }
    
    #[inline] pub fn get_bits_uint8(&mut self, num_bits: i32) -> u8 {
        
        todo!();
        /*
            return (uint8)  getBitsUnchecked (numBits);
        */
    }
    
    #[inline] pub fn get_bits_uint16(&mut self, num_bits: i32) -> u16 {
        
        todo!();
        /*
            return (uint16) getBitsUnchecked (numBits);
        */
    }
    
    pub fn scan_for_next_frame_header(&mut self, check_type_against_last_frame: bool) -> i32 {
        
        todo!();
        /*
            auto oldPos = stream.getPosition();
            int offset = -3;
            uint32 header = 0;

            for (;;)
            {
                if (stream.isExhausted() || stream.getPosition() > oldPos + 32768)
                {
                    offset = -1;
                    break;
                }

                header = (header << 8) | (uint8) stream.readByte();

                if (offset >= 0 && isValidHeader (header, frame.layer))
                {
                    if (! checkTypeAgainstLastFrame)
                        break;

                    const bool mpeg25            = (header & (1 << 20)) == 0;
                    const uint32 lsf             = mpeg25 ? 1 : ((header & (1 << 19)) ? 0 : 1);
                    const uint32 sampleRateIndex = mpeg25 ? (6 + ((header >> 10) & 3)) : (((header >> 10) & 3) + (lsf * 3));
                    const uint32 mode            = (header >> 6) & 3;
                    const uint32 numChannels     = (mode == 3) ? 1 : 2;

                    if (numChannels == (uint32) frame.numChannels && lsf == (uint32) frame.lsf
                          && mpeg25 == frame.mpeg25 && sampleRateIndex == (uint32) frame.sampleRateIndex)
                        break;
                }

                ++offset;
            }

            if (offset >= 0)
            {
                if ((currentFrameIndex & (storedStartPosInterval - 1)) == 0)
                    frameStreamPositions.set (currentFrameIndex / storedStartPosInterval, oldPos + offset);

                ++currentFrameIndex;
            }

            stream.setPosition (oldPos);
            return offset;
        */
    }
    
    pub fn read_vbr_header(&mut self)  {
        
        todo!();
        /*
            auto oldPos = stream.getPosition();
            uint8 xing[194];
            stream.read (xing, sizeof (xing));

            vbrHeaderFound = vbrTagData.read (xing);

            if (vbrHeaderFound)
            {
                numFrames = (int) vbrTagData.frames;
                oldPos += jmax (vbrTagData.headersize, 1);
            }

            stream.setPosition (oldPos);
        */
    }
    
    pub fn decode_layer_1frame(&mut self, 
        pcm0:         *mut f32,
        pcm1:         *mut f32,
        samples_done: &mut i32)  {
        
        todo!();
        /*
            float fraction[2][32];
            Mp3StreamSideInfoLayer1 si;
            layer1Step1 (si);
            auto single = (frame.numChannels == 1 || frame.single == 3) ? 0 : frame.single;

            if (single >= 0)
            {
                for (int i = 0; i < 12; ++i)
                {
                    layer1Step2 (si, fraction);
                    synthesise (fraction[single], 0, pcm0, samplesDone);
                }
            }
            else
            {
                for (int i = 0; i < 12; ++i)
                {
                    layer1Step2 (si, fraction);
                    synthesiseStereo (fraction[0], fraction[1], pcm0, pcm1, samplesDone);
                }
            }
        */
    }
    
    pub fn decode_layer_2frame(&mut self, 
        pcm0:         *mut f32,
        pcm1:         *mut f32,
        samples_done: &mut i32)  {
        
        todo!();
        /*
            float fraction[2][4][32];
            frame.selectLayer2Table();
            Mp3StreamSideInfoLayer2 si;
            layer2Step1 (si);
            auto single = (frame.numChannels == 1 || frame.single == 3) ? 0 : frame.single;

            if (single >= 0)
            {
                for (int i = 0; i < 12; ++i)
                {
                    layer2Step2 (si, i >> 2, fraction);

                    for (int j = 0; j < 3; ++j)
                        synthesise (fraction[single][j], 0, pcm0, samplesDone);
                }
            }
            else
            {
                for (int i = 0; i < 12; ++i)
                {
                    layer2Step2 (si, i >> 2, fraction);

                    for (int j = 0; j < 3; ++j)
                        synthesiseStereo (fraction[0][j], fraction[1][j], pcm0, pcm1, samplesDone);
                }
            }
        */
    }
    
    pub fn decode_layer_3frame(&mut self, 
        pcm0:         *mut f32,
        pcm1:         *mut f32,
        samples_done: &mut i32)  {
        
        todo!();
        /*
            if (! rollBackBufferPointer ((int) sideinfo.mainDataStart))
                return;

            const int single = frame.numChannels == 1 ? 0 : frame.single;
            const int numChans = (frame.numChannels == 1 || single >= 0) ? 1 : 2;
            const bool msStereo = (frame.mode == 1) && (frame.modeExt & 2) != 0;
            const bool iStereo  = (frame.mode == 1) && (frame.modeExt & 1) != 0;
            const int granules = frame.lsf ? 1 : 2;
            int scaleFactors[2][39];

            for (int gr = 0; gr < granules; ++gr)
            {
                {
                    auto& granule = sideinfo.ch[0].gr[gr];
                    auto part2bits = frame.lsf ? getLayer3ScaleFactors2 (scaleFactors[0], granule, 0)
                                               : getLayer3ScaleFactors1 (scaleFactors[0], granule);

                    if (layer3DequantizeSample (hybridIn[0], scaleFactors[0], granule, frame.sampleRateIndex, part2bits))
                        return;
                }

                if (frame.numChannels == 2)
                {
                    auto& granule = sideinfo.ch[1].gr[gr];
                    auto part2bits = frame.lsf ? getLayer3ScaleFactors2 (scaleFactors[1], granule, iStereo)
                                               : getLayer3ScaleFactors1 (scaleFactors[1], granule);

                    if (layer3DequantizeSample (hybridIn[1], scaleFactors[1], granule, frame.sampleRateIndex, part2bits))
                        return;

                    if (msStereo)
                    {
                        for (int i = 0; i < 32 * 18; ++i)
                        {
                            auto tmp0 = ((const float*) hybridIn[0])[i];
                            auto tmp1 = ((const float*) hybridIn[1])[i];
                            ((float*) hybridIn[1])[i] = tmp0 - tmp1;
                            ((float*) hybridIn[0])[i] = tmp0 + tmp1;
                        }
                    }

                    if (iStereo)
                        granule.doIStereo (hybridIn, scaleFactors[1], frame.sampleRateIndex, msStereo, frame.lsf);

                    if (msStereo || iStereo || single == 3)
                    {
                        if (granule.maxb > sideinfo.ch[0].gr[gr].maxb)
                            sideinfo.ch[0].gr[gr].maxb = granule.maxb;
                        else
                            granule.maxb = sideinfo.ch[0].gr[gr].maxb;
                    }

                    switch (single)
                    {
                        case 3:
                        {
                            auto* in0 = (float*) hybridIn[0];
                            auto* in1 = (const float*) hybridIn[1];

                            for (int i = 0; i < (int) (18 * granule.maxb); ++i, ++in0)
                                *in0 = (*in0 + *in1++);
                        }
                        break;

                        case 1:
                        {
                            auto* in0 = (float*) hybridIn[0];
                            auto* in1 = (const float*) hybridIn[1];

                            for (int i = 0; i < (int) (18 * granule.maxb); ++i)
                                *in0++ = *in1++;
                        }
                        break;

                        default:
                            break;
                    }
                }

                for (int ch = 0; ch < numChans; ++ch)
                {
                    auto& granule = sideinfo.ch[ch].gr[gr];
                    granule.doAntialias (hybridIn[ch]);
                    layer3Hybrid (hybridIn[ch], hybridOut[ch], ch, granule);
                }

                for (int ss = 0; ss < 18; ++ss)
                {
                    if (single >= 0)
                        synthesise (hybridOut[0][ss], 0, pcm0, samplesDone);
                    else
                        synthesiseStereo (hybridOut[0][ss], hybridOut[1][ss], pcm0, pcm1, samplesDone);
                }
            }
        */
    }
    
    pub fn decode_layer_3side_info(&mut self) -> i32 {
        
        todo!();
        /*
            const int numChannels = frame.numChannels;
            const int sampleRate = frame.sampleRateIndex;
            const int single = (numChannels == 1) ? 0 : frame.single;
            const bool msStereo = (frame.mode == 1) && (frame.modeExt & 2) != 0;
            const int granules = frame.lsf ? 1 : 2;

            if (frame.lsf == 0)
                getLayer3SideInfo1 (numChannels, msStereo, sampleRate, single);
            else
                getLayer3SideInfo2 (numChannels, msStereo, sampleRate, single);

            int databits = 0;

            for (int gr = 0; gr < granules; ++gr)
                for (int ch = 0; ch < numChannels; ++ch)
                    databits += (int) sideinfo.ch[ch].gr[gr].part2_3Length;

            return databits - 8 * (int) sideinfo.mainDataStart;
        */
    }
    
    pub fn layer_1step1(&mut self, si: &mut Mp3StreamSideInfoLayer1)  {
        
        todo!();
        /*
            zerostruct (si);
            int i, jsbound = (frame.mode == 1) ? (frame.modeExt << 2) + 4 : 32;

            if (frame.numChannels == 2)
            {
                for (i = 0; i < jsbound; ++i)
                {
                    si.allocation[i][0] = getBitsUint8 (4);
                    si.allocation[i][1] = getBitsUint8 (4);
                }

                for (i = jsbound; i < 32; ++i)
                    si.allocation[i][0] = si.allocation[i][1] = getBitsUint8 (4);

                for (i = 0; i < 32; ++i)
                {
                    si.scaleFactor[i][0] = si.allocation[i][0] ? getBitsUint8 (6) : 0;
                    si.scaleFactor[i][1] = si.allocation[i][1] ? getBitsUint8 (6) : 0;
                }
            }
            else
            {
                for (i = 0; i < 32; ++i)
                    si.allocation[i][0] = getBitsUint8 (4);

                for (i = 0; i < 32; ++i)
                    si.scaleFactor[i][0] = si.allocation[i][0] ? getBitsUint8 (6) : 0;
            }
        */
    }
    
    pub fn layer_1step2(&mut self, 
        si:       &mut Mp3StreamSideInfoLayer1,
        fraction: [[f32; 2]; 32])  {
        
        todo!();
        /*
            if (frame.numChannels == 2)
            {
                int i, jsbound = (frame.mode == 1) ? (frame.modeExt << 2) + 4 : 32;

                for (i = 0; i < jsbound; ++i)
                {
                    const uint8 n0 = si.allocation[i][0];
                    const uint8 n1 = si.allocation[i][1];
                    fraction[0][i] = n0 > 0 ? ((float) (-(1 << n0) + getBitsUint16 (n0 + 1) + 1) * constants.muls[n0 + 1][si.scaleFactor[i][0]]) : 0.0f;
                    fraction[1][i] = n1 > 0 ? ((float) (-(1 << n1) + getBitsUint16 (n1 + 1) + 1) * constants.muls[n1 + 1][si.scaleFactor[i][1]]) : 0.0f;
                }

                for (i = jsbound; i < 32; ++i)
                {
                    const uint8 n = si.allocation[i][0];

                    if (n > 0)
                    {
                        const uint32 w = ((uint32) -(1 << n) + getBitsUint16 (n + 1) + 1);
                        fraction[0][i] = ((float) w * constants.muls[n + 1][si.scaleFactor[i][0]]);
                        fraction[1][i] = ((float) w * constants.muls[n + 1][si.scaleFactor[i][1]]);
                    }
                    else
                        fraction[0][i] = fraction[1][i] = 0;
                }
            }
            else
            {
                for (int i = 0; i < 32; ++i)
                {
                    const uint8 n = si.allocation[i][0];
                    const uint8 j = si.scaleFactor[i][0];

                    if (n > 0)
                        fraction[0][i] = ((float) (-(1 << n) + getBitsUint16 (n + 1) + 1) * constants.muls[n + 1][j]);
                    else
                        fraction[0][i] = 0;
                }
            }
        */
    }
    
    pub fn layer_2step1(&mut self, si: &mut Mp3StreamSideInfoLayer2)  {
        
        todo!();
        /*
            zerostruct (si);
            const auto sblimit = frame.layer2SubBandLimit;
            const auto jsbound = (frame.mode == 1 ? jmin ((frame.modeExt << 2) + 4, sblimit) : sblimit);
            auto* allocTable = frame.allocationTable;
            uint8 scfsi[32][2];

            if (frame.numChannels == 2)
            {
                for (int i = 0; i < jsbound; ++i)
                {
                    auto step = allocTable->bits;
                    allocTable += (static_cast<intptr_t> (1) << step);
                    si.allocation[i][0] = getBitsUint8 (step);
                    si.allocation[i][1] = getBitsUint8 (step);
                }

                for (int i = jsbound; i < sblimit; ++i)
                {
                    auto step = allocTable->bits;
                    auto b0 = getBitsUint8 (step);
                    allocTable += (static_cast<intptr_t> (1) << step);
                    si.allocation[i][0] = b0;
                    si.allocation[i][1] = b0;
                }

                for (int i = 0; i < sblimit; ++i)
                {
                    scfsi[i][0] = si.allocation[i][0] ? getBitsUint8 (2) : 0;
                    scfsi[i][1] = si.allocation[i][1] ? getBitsUint8 (2) : 0;
                }
            }
            else
            {
                for (int i = 0; i < sblimit; ++i)
                {
                    const int16 step = allocTable->bits;
                    allocTable += (static_cast<intptr_t> (1) << step);
                    si.allocation[i][0] = getBitsUint8 (step);
                }

                for (int i = 0; i < sblimit; ++i)
                    scfsi[i][0] = si.allocation[i][0] ? getBitsUint8 (2) : 0;
            }

            for (int i = 0; i < sblimit; ++i)
            {
                for (int ch = 0; ch < frame.numChannels; ++ch)
                {
                    uint8 s0 = 0, s1 = 0, s2 = 0;

                    if (si.allocation[i][ch])
                    {
                        switch (scfsi[i][ch])
                        {
                            case 0:
                                s0 = getBitsUint8 (6);
                                s1 = getBitsUint8 (6);
                                s2 = getBitsUint8 (6);
                                break;
                            case 1:
                                s1 = s0 = getBitsUint8 (6);
                                s2 = getBitsUint8 (6);
                                break;
                            case 2:
                                s2 = s1 = s0 = getBitsUint8 (6);
                                break;
                            case 3:
                                s0 = getBitsUint8 (6);
                                s2 = s1 = getBitsUint8 (6);
                                break;
                            default:
                                break;
                        }
                    }

                    si.scaleFactor[i][ch][0] = s0;
                    si.scaleFactor[i][ch][1] = s1;
                    si.scaleFactor[i][ch][2] = s2;
                }
            }
        */
    }
    
    pub fn layer_2step2(&mut self, 
        si:       &mut Mp3StreamSideInfoLayer2,
        gr:       i32,
        fraction: [[[f32; 2]; 4]; 32])  {
        
        todo!();
        /*
            auto* allocTable = frame.allocationTable;
            auto sblimit = frame.layer2SubBandLimit;
            const auto jsbound = (frame.mode == 1 ? jmin ((frame.modeExt << 2) + 4, sblimit) : sblimit);

            for (int i = 0; i < jsbound; ++i)
            {
                auto step = allocTable->bits;

                for (int ch = 0; ch < frame.numChannels; ++ch)
                {
                    if (auto ba = si.allocation[i][ch])
                    {
                        auto x1 = jmin ((uint8) 63, si.scaleFactor[i][ch][gr]);
                        auto* alloc2 = allocTable + ba;
                        auto k = jmin ((int16) 16, alloc2->bits);
                        auto d1 = alloc2->d;

                        if (d1 < 0)
                        {
                            const double cm = constants.muls[k][x1];
                            fraction[ch][0][i] = (float) (((int) getBits (k) + d1) * cm);
                            fraction[ch][1][i] = (float) (((int) getBits (k) + d1) * cm);
                            fraction[ch][2][i] = (float) (((int) getBits (k) + d1) * cm);
                        }
                        else
                        {
                            auto* tab = constants.getGroupTable (d1, getBits (k));
                            fraction[ch][0][i] = (float) constants.muls[tab[0]][x1];
                            fraction[ch][1][i] = (float) constants.muls[tab[1]][x1];
                            fraction[ch][2][i] = (float) constants.muls[tab[2]][x1];
                        }
                    }
                    else
                    {
                        fraction[ch][0][i] = fraction[ch][1][i] = fraction[ch][2][i] = 0;
                    }
                }

                allocTable += (static_cast<intptr_t> (1) << step);
            }

            for (int i = jsbound; i < frame.layer2SubBandLimit; ++i)
            {
                auto step = allocTable->bits;
                auto ba = si.allocation[i][0];

                if (ba != 0)
                {
                    auto* alloc2 = allocTable + ba;
                    int16 k = alloc2->bits;
                    int16 d1 = alloc2->d;
                    k = (k <= 16) ? k : 16;

                    if (d1 < 0)
                    {
                        auto v0 = (int) getBits (k);
                        auto v1 = (int) getBits (k);
                        auto v2 = (int) getBits (k);

                        for (int ch = 0; ch < frame.numChannels; ++ch)
                        {
                            auto x1 = jmin ((uint8) 63, si.scaleFactor[i][ch][gr]);
                            const double cm = constants.muls[k][x1];
                            fraction[ch][0][i] = (float) ((v0 + d1) * cm);
                            fraction[ch][1][i] = (float) ((v1 + d1) * cm);
                            fraction[ch][2][i] = (float) ((v2 + d1) * cm);
                        }
                    }
                    else
                    {
                        auto* tab = constants.getGroupTable (d1, getBits (k));
                        auto k0 = tab[0];
                        auto k1 = tab[1];
                        auto k2 = tab[2];

                        for (int ch = 0; ch < frame.numChannels; ++ch)
                        {
                            auto x1 = jmin ((uint8) 63, si.scaleFactor[i][ch][gr]);
                            fraction[ch][0][i] = (float) constants.muls[k0][x1];
                            fraction[ch][1][i] = (float) constants.muls[k1][x1];
                            fraction[ch][2][i] = (float) constants.muls[k2][x1];
                        }
                    }
                }
                else
                {
                    fraction[0][0][i] = fraction[0][1][i] = fraction[0][2][i] = 0;
                    fraction[1][0][i] = fraction[1][1][i] = fraction[1][2][i] = 0;
                }

                allocTable += (static_cast<intptr_t> (1) << step);
            }

            for (int ch = 0; ch < frame.numChannels; ++ch)
                for (int i = frame.layer2SubBandLimit; i < 32; ++i)
                    fraction[ch][0][i] = fraction[ch][1][i] = fraction[ch][2][i] = 0;
        */
    }
    
    pub fn get_layer_3side_info1(&mut self, 
        stereo:      i32,
        ms_stereo:   bool,
        sample_rate: i32,
        single:      i32)  {
        
        todo!();
        /*
            const int powdiff = (single == 3) ? 4 : 0;
            sideinfo.mainDataStart = getBits (9);
            sideinfo.privateBits = getBitsUnchecked (stereo == 1 ? 5 : 3);

            for (int ch = 0; ch < stereo; ++ch)
            {
                sideinfo.ch[ch].gr[0].scfsi = -1;
                sideinfo.ch[ch].gr[1].scfsi = (int) getBitsUnchecked (4);
            }

            for (int gr = 0; gr < 2; ++gr)
            {
                for (int ch = 0; ch < stereo; ++ch)
                {
                    auto& granule = sideinfo.ch[ch].gr[gr];

                    granule.part2_3Length = getBits (12);
                    granule.bigValues = jmin (288u, getBitsUnchecked (9));

                    const int qss = (int) getBitsUnchecked (8);
                    granule.pow2gain = constants.powToGains + 256 - qss + powdiff;

                    if (msStereo)
                        granule.pow2gain += 2;

                    granule.scaleFactorCompression = getBitsUnchecked (4);

                    if (getOneBit())
                    {
                        granule.blockType = getBitsUnchecked (2);
                        granule.mixedBlockFlag = getOneBit();
                        granule.tableSelect[0] = getBitsUnchecked (5);
                        granule.tableSelect[1] = getBitsUnchecked (5);
                        granule.tableSelect[2] = 0;

                        for (int i = 0; i < 3; ++i)
                        {
                            const uint32 sbg = (getBitsUnchecked (3) << 3);
                            granule.fullGain[i] = granule.pow2gain + sbg;
                        }

                        granule.region1Start = 36 >> 1;
                        granule.region2Start = 576 >> 1;
                    }
                    else
                    {
                        for (int i = 0; i < 3; ++i)
                            granule.tableSelect[i] = getBitsUnchecked (5);

                        const int r0c = (int) getBitsUnchecked (4);
                        const int r1c = (int) getBitsUnchecked (3);
                        const int region0index = jmin (22, r0c + 1);
                        const int region1index = jmin (22, r0c + 1 + r1c + 1);

                        granule.region1Start = (uint32) (bandInfo[sampleRate].longIndex[region0index] >> 1);
                        granule.region2Start = (uint32) (bandInfo[sampleRate].longIndex[region1index] >> 1);
                        granule.blockType = 0;
                        granule.mixedBlockFlag = 0;
                    }

                    granule.preflag = getOneBit();
                    granule.scaleFactorScale = getOneBit();
                    granule.count1TableSelect = getOneBit();
                }
            }
        */
    }
    
    pub fn get_layer_3side_info2(&mut self, 
        stereo:      i32,
        ms_stereo:   bool,
        sample_rate: i32,
        single:      i32)  {
        
        todo!();
        /*
            const int powdiff = (single == 3) ? 4 : 0;
            sideinfo.mainDataStart = getBits (8);
            sideinfo.privateBits = stereo == 1 ? getOneBit() : getBitsUnchecked (2);

            for (int ch = 0; ch < stereo; ++ch)
            {
                auto& granule = sideinfo.ch[ch].gr[0];

                granule.part2_3Length = getBits (12);
                granule.bigValues = jmin (288u, getBitsUnchecked (9));

                const uint32 qss = getBitsUnchecked (8);
                granule.pow2gain = constants.powToGains + 256 - qss + powdiff;

                if (msStereo)
                    granule.pow2gain += 2;

                granule.scaleFactorCompression = getBits (9);

                if (getOneBit())
                {
                    granule.blockType = getBitsUnchecked (2);
                    granule.mixedBlockFlag = getOneBit();
                    granule.tableSelect[0] = getBitsUnchecked (5);
                    granule.tableSelect[1] = getBitsUnchecked (5);
                    granule.tableSelect[2] = 0;

                    for (int i = 0; i < 3; ++i)
                    {
                        const uint32 sbg = (getBitsUnchecked (3) << 3);
                        granule.fullGain[i] = granule.pow2gain + sbg;
                    }

                    if (granule.blockType == 0)
                    {}

                    if (granule.blockType == 2)
                        granule.region1Start = sampleRate == 8 ? 36 : (36 >> 1);
                    else
                        granule.region1Start = sampleRate == 8 ? (108 >> 1) : (54 >> 1);

                    granule.region2Start = 576 >> 1;
                }
                else
                {
                    for (int i = 0; i < 3; ++i)
                        granule.tableSelect[i] = getBitsUnchecked (5);

                    const int r0c = (int) getBitsUnchecked (4);
                    const int r1c = (int) getBitsUnchecked (3);
                    const int region0index = jmin (22, r0c + 1);
                    const int region1index = jmin (22, r0c + 1 + r1c + 1);

                    granule.region1Start = (uint32) (bandInfo[sampleRate].longIndex[region0index] >> 1);
                    granule.region2Start = (uint32) (bandInfo[sampleRate].longIndex[region1index] >> 1);
                    granule.blockType = 0;
                    granule.mixedBlockFlag = 0;
                }
                granule.scaleFactorScale = getOneBit();
                granule.count1TableSelect = getOneBit();
            }
        */
    }
    
    pub fn get_layer_3scale_factors1(&mut self, 
        scf:     *mut i32,
        granule: &Mp3DecoderLayer3SideInfo) -> i32 {
        
        todo!();
        /*
            static const uint8 lengths[2][16] =
            {
                { 0, 0, 0, 0, 3, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4 },
                { 0, 1, 2, 3, 0, 1, 2, 3, 1, 2, 3, 1, 2, 3, 2, 3 }
            };

            int numBits;
            const int num0 = lengths[0][granule.scaleFactorCompression];
            const int num1 = lengths[1][granule.scaleFactorCompression];

            if (granule.blockType == 2)
            {
                int i = 18;
                numBits = (num0 + num1) * 18;

                if (granule.mixedBlockFlag)
                {
                    for (int j = 8; --j >= 0;)  *scf++ = (int) getBitsUnchecked (num0);
                    numBits -= num0;
                    i = 9;
                }

                for (; --i >= 0;)       *scf++ = (int) getBitsUnchecked (num0);
                for (i = 18; --i >= 0;) *scf++ = (int) getBitsUnchecked (num1);

                *scf++ = 0;
                *scf++ = 0;
                *scf++ = 0;
            }
            else
            {
                const int scfsi = granule.scfsi;

                if (scfsi < 0)
                {
                    for (int i = 11; --i >= 0;)   *scf++ = (int) getBitsUnchecked (num0);
                    for (int j = 10; --j >= 0;)   *scf++ = (int) getBitsUnchecked (num1);
                    numBits = (num0 + num1) * 10 + num0;
                }
                else
                {
                    numBits = 0;
                    if ((scfsi & 8) == 0)
                    {
                        for (int i = 6; --i >= 0;)  *scf++ = (int) getBitsUnchecked (num0);
                        numBits += num0 * 6;
                    }
                    else
                        scf += 6;

                    if ((scfsi & 4) == 0)
                    {
                        for (int i = 5; --i >= 0;)  *scf++ = (int) getBitsUnchecked (num0);
                        numBits += num0 * 5;
                    }
                    else
                        scf += 5;

                    if ((scfsi & 2) == 0)
                    {
                        for (int i = 5; --i >= 0;)  *scf++ = (int) getBitsUnchecked (num1);
                        numBits += num1 * 5;
                    }
                    else
                        scf += 5;

                    if ((scfsi & 1) == 0)
                    {
                        for (int i = 5; --i >= 0;)  *scf++ = (int) getBitsUnchecked (num1);
                        numBits += num1 * 5;
                    }
                    else
                        scf += 5;
                }

                *scf = 0;
            }

            return numBits;
        */
    }
    
    pub fn get_layer_3scale_factors2(&mut self, 
        scf:     *mut i32,
        granule: &mut Mp3DecoderLayer3SideInfo,
        stereo:  bool) -> i32 {
        
        todo!();
        /*
            static const uint8 scaleTable[3][6][4] =
            {
                { { 6, 5, 5, 5 }, { 6, 5, 7, 3 },  { 11, 10, 0, 0 }, { 7, 7, 7, 0 },    { 6, 6, 6, 3 },  { 8, 8, 5, 0 } },
                { { 9, 9, 9, 9 }, { 9, 9, 12, 6 }, { 18, 18, 0, 0 }, { 12, 12, 12, 0 }, { 12, 9, 9, 6 }, { 15, 12, 9, 0 } },
                { { 6, 9, 9, 9 }, { 6, 9, 12, 6 }, { 15, 18, 0, 0 }, { 6, 15, 12, 0 },  { 6, 12, 9, 6 }, { 6, 18, 9, 0 } }
            };

            uint32 len = iStereo ? constants.iLength2[granule.scaleFactorCompression >> 1]
                                 : constants.nLength2[granule.scaleFactorCompression];

            granule.preflag = (len >> 15) & 1;

            int n = 0;
            if (granule.blockType == 2)
            {
                ++n;
                if (granule.mixedBlockFlag)
                    ++n;
            }

            const uint8* const data = scaleTable[n][(len >> 12) & 7];
            int numBits = 0;

            for (int i = 0; i < 4; ++i)
            {
                int num = len & 7;
                len >>= 3;

                if (num)
                {
                    for (int j = 0; j < (int) (data[i]); ++j)
                        *scf++ = (int) getBitsUnchecked (num);

                    numBits += data[i] * num;
                }
                else
                {
                    for (int j = 0; j < (int) (data[i]); ++j)
                        *scf++ = 0;
                }
            }

            n = (n << 1) + 1;

            for (int i = 0; i < n; ++i)
                *scf++ = 0;

            return numBits;
        */
    }
    
    pub fn layer_3dequantize_sample(&mut self, 
        xr:          [[f32; 32]; 18],
        scf:         *mut i32,
        granule:     &mut Mp3DecoderLayer3SideInfo,
        sample_rate: i32,
        part2bits:   i32) -> bool {
        
        todo!();
        /*
            const uint32 shift = 1 + granule.scaleFactorScale;
            auto* xrpnt = (float*) xr;
            auto part2remain = (int) granule.part2_3Length - part2bits;

            zeromem (xrpnt, (size_t) (&xr[32][0] - xrpnt) * sizeof (float));

            auto bv = (int) granule.bigValues;
            auto region1 = (int) granule.region1Start;
            auto region2 = (int) granule.region2Start;
            int l3 = ((576 >> 1) - bv) >> 1;
            int l[3];

            if (bv <= region1)
            {
                l[0] = bv;
                l[1] = 0;
                l[2] = 0;
            }
            else
            {
                l[0] = region1;
                if (bv <= region2)
                {
                    l[1] = bv - l[0];
                    l[2] = 0;
                }
                else
                {
                    l[1] = region2 - l[0];
                    l[2] = bv - region2;
                }
            }

            for (int i = 0; i < 3; ++i)
                if (l[i] < 0)
                    l[i] = 0;

            if (granule.blockType == 2)
            {
                int max[4];
                int step = 0, lwin = 0, cb = 0, mc = 0;
                float v = 0;
                int* map;
                int* mapEnd;

                if (granule.mixedBlockFlag)
                {
                    max[3] = -1;
                    max[0] = max[1] = max[2] = 2;
                    map    = constants.map   [sampleRate][0];
                    mapEnd = constants.mapEnd[sampleRate][0];
                }
                else
                {
                    max[0] = max[1] = max[2] = max[3] = -1;
                    map    = constants.map   [sampleRate][1];
                    mapEnd = constants.mapEnd[sampleRate][1];
                }

                for (int i = 0; i < 2; ++i)
                {
                    auto* h = huffmanTables1 + granule.tableSelect[i];

                    for (int lp = l[i]; lp != 0; --lp, --mc)
                    {
                        int x, y;
                        if (mc == 0)
                        {
                            mc = *map++;
                            xrpnt = ((float*) xr) + (*map++);
                            lwin = *map++;
                            cb = *map++;

                            if (lwin == 3)
                            {
                                v = granule.pow2gain[ (*scf++) << shift];
                                step = 1;
                            }
                            else
                            {
                                v = granule.fullGain[lwin][ (*scf++) << shift];
                                step = 3;
                            }
                        }

                        auto* val = h->table;

                        while ((y = *val++) < 0)
                        {
                            if (getOneBit())
                                val -= y;

                            --part2remain;
                        }

                        x = y >> 4;
                        y &= 15;

                        if (x == 15)
                        {
                            max[lwin] = cb;
                            part2remain -= (int) (h->bits + 1);
                            x += (int) getBits ((int) h->bits);
                            *xrpnt = constants.nToThe4Over3[x] * (getOneBit() ? -v : v);
                        }
                        else if (x)
                        {
                            max[lwin] = cb;
                            *xrpnt = constants.nToThe4Over3[x] * (getOneBit() ? -v : v);
                            --part2remain;
                        }
                        else
                            *xrpnt = 0;

                        xrpnt += step;

                        if (y == 15)
                        {
                            max[lwin] = cb;
                            part2remain -= (int) (h->bits + 1);
                            y += (int) getBits ((int) h->bits);
                            *xrpnt = constants.nToThe4Over3[y] * (getOneBit() ? -v : v);
                        }
                        else if (y)
                        {
                            max[lwin] = cb;
                            *xrpnt = constants.nToThe4Over3[y] * (getOneBit() ? -v : v);
                            --part2remain;
                        }
                        else
                            *xrpnt = 0;

                        xrpnt += step;
                    }
                }

                for (; l3 && (part2remain > 0); --l3)
                {
                    auto* h = huffmanTables2 + granule.count1TableSelect;
                    auto* val = h->table;
                    int16 a;

                    while ((a = *val++) < 0)
                    {
                        if (part2remain <= 0)
                        {
                            a = 0;
                            break;
                        }

                        --part2remain;

                        if (getOneBit())
                            val -= a;
                    }

                    for (int i = 0; i < 4; ++i)
                    {
                        if ((i & 1) == 0)
                        {
                            if (mc == 0)
                            {
                                mc = *map++;
                                xrpnt = ((float*) xr) + (*map++);
                                lwin = *map++;
                                cb = *map++;

                                if (lwin == 3)
                                {
                                    v = granule.pow2gain[ (*scf++) << shift];
                                    step = 1;
                                }
                                else
                                {
                                    v = granule.fullGain[lwin][ (*scf++) << shift];
                                    step = 3;
                                }
                            }

                            --mc;
                        }

                        if ((a & (8 >> i)))
                        {
                            max[lwin] = cb;

                            if (part2remain == 0)
                                break;

                            --part2remain;
                            *xrpnt = getOneBit() ? -v : v;
                        }
                        else
                            *xrpnt = 0;

                        xrpnt += step;
                    }
                }

                while (map < mapEnd)
                {
                    if (mc == 0)
                    {
                        mc = *map++;
                        xrpnt = ((float*) xr) + *map++;
                        step = (*map++ == 3) ? 1 : 3;
                        ++map;
                    }

                    --mc;
                    *xrpnt = 0;  xrpnt += step;
                    *xrpnt = 0;  xrpnt += step;
                }

                granule.maxBand[0] = (uint32) (max[0] + 1);
                granule.maxBand[1] = (uint32) (max[1] + 1);
                granule.maxBand[2] = (uint32) (max[2] + 1);
                granule.maxBandl   = (uint32) (max[3] + 1);

                const int rmax = jmax (max[0], max[1], max[3]) + 1;
                granule.maxb = rmax ? (uint32) constants.shortLimit[sampleRate][rmax]
                                    : (uint32) constants.longLimit[sampleRate][max[3] + 1];
            }
            else
            {
                static const int pretab1[] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 3, 2, 0 };
                static const int pretab2[] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };

                auto* pretab = (const int*) (granule.preflag ? pretab1 : pretab2);
                int max = -1, cb = 0, mc = 0;
                auto* map = constants.map[sampleRate][2];
                float v = 0;

                for (int i = 0; i < 3; ++i)
                {
                    auto* h = huffmanTables1 + granule.tableSelect[i];

                    for (int lp = l[i]; lp != 0; --lp, --mc)
                    {
                        if (mc == 0)
                        {
                            mc = *map++;
                            v = granule.pow2gain[((*scf++) + (*pretab++)) << shift];
                            cb = *map++;
                        }

                        auto* val = h->table;
                        int y;

                        while ((y = *val++) < 0)
                        {
                            if (getOneBit()) val -= y;
                            --part2remain;
                        }

                        int x = y >> 4;
                        y &= 15;

                        if (x == 15)
                        {
                            max = cb;
                            part2remain -= (int) (h->bits + 1);
                            x += (int) getBits ((int) h->bits);
                            *xrpnt++ = constants.nToThe4Over3[x] * (getOneBit() ? -v : v);
                        }
                        else if (x)
                        {
                            max = cb;
                            *xrpnt++ = constants.nToThe4Over3[x] * (getOneBit() ? -v : v);
                            --part2remain;
                        }
                        else
                            *xrpnt++ = 0;

                        if (y == 15)
                        {
                            max = cb;
                            part2remain -= (int) (h->bits + 1);
                            y += (int) getBits ((int) h->bits);
                            *xrpnt++ = constants.nToThe4Over3[y] * (getOneBit() ? -v : v);
                        }
                        else if (y)
                        {
                            max = cb;
                            *xrpnt++ = constants.nToThe4Over3[y] * (getOneBit() ? -v : v);
                            --part2remain;
                        }
                        else
                            *xrpnt++ = 0;
                    }
                }

                for (; l3 && part2remain > 0; --l3)
                {
                    auto* h = huffmanTables2 + granule.count1TableSelect;
                    auto* values = h->table;
                    int16 a;

                    while ((a = *values++) < 0)
                    {
                        if (part2remain <= 0)
                        {
                            a = 0;
                            break;
                        }

                        --part2remain;

                        if (getOneBit())
                            values -= a;
                    }

                    for (int i = 0; i < 4; ++i)
                    {
                        if ((i & 1) == 0)
                        {
                            if (mc == 0)
                            {
                                mc = *map++;
                                cb = *map++;
                                v = granule.pow2gain[((*scf++) + (*pretab++)) << shift];
                            }
                            --mc;
                        }

                        if ((a & (0x8 >> i)))
                        {
                            max = cb;

                            if (part2remain <= 0)
                                break;

                            --part2remain;
                            *xrpnt++ = getOneBit() ? -v : v;
                        }
                        else
                            *xrpnt++ = 0;
                    }
                }

                zeromem (xrpnt, (size_t) (&xr[32][0] - xrpnt) * sizeof (float));

                granule.maxBandl = (uint32) (max + 1);
                granule.maxb = (uint32) constants.longLimit[sampleRate][granule.maxBandl];
            }

            while (part2remain > 16)
            {
                getBits (16);
                part2remain -= 16;
            }

            if (part2remain > 0)
                getBits (part2remain);
            else if (part2remain < 0)
                return true;

            return false;
        */
    }
    
    pub fn layer_3hybrid(&mut self, 
        fs_in:   [[f32; 32]; 18],
        ts_out:  [[f32; 18]; 32],
        ch:      i32,
        granule: &Mp3DecoderLayer3SideInfo)  {
        
        todo!();
        /*
            auto* ts = (float*) tsOut;
            float* rawout1, *rawout2;
            int sb = 0;

            {
                int b = hybridBlockIndex[ch];
                rawout1 = hybridBlock[b][ch];
                b = 1 - b;
                rawout2 = hybridBlock[b][ch];
                hybridBlockIndex[ch] = b;
            }

            if (granule.mixedBlockFlag)
            {
                sb = 2;
                DCT::dct36 (fsIn[0], rawout1, rawout2, constants.win[0], ts);
                DCT::dct36 (fsIn[1], rawout1 + 18, rawout2 + 18, constants.win1[0], ts + 1);
                rawout1 += 36;
                rawout2 += 36;
                ts += 2;
            }

            auto bt = granule.blockType;

            if (bt == 2)
            {
                for (; sb < (int) granule.maxb; sb += 2, ts += 2, rawout1 += 36, rawout2 += 36)
                {
                    DCT::dct12 (fsIn[sb], rawout1, rawout2, constants.win[2], ts);
                    DCT::dct12 (fsIn[sb + 1], rawout1 + 18, rawout2 + 18, constants.win1[2], ts + 1);
                }
            }
            else
            {
                for (; sb < (int) granule.maxb; sb += 2, ts += 2, rawout1 += 36, rawout2 += 36)
                {
                    DCT::dct36 (fsIn[sb], rawout1, rawout2, constants.win[bt], ts);
                    DCT::dct36 (fsIn[sb + 1], rawout1 + 18, rawout2 + 18, constants.win1[bt], ts + 1);
                }
            }

            for (; sb < 32; ++sb, ++ts)
            {
                for (int i = 0; i < 18; ++i)
                {
                    ts[i * 32] = *rawout1++;
                    *rawout2++ = 0;
                }
            }
        */
    }
    
    pub fn synthesise_stereo(&mut self, 
        band_ptr0:    *const f32,
        band_ptr1:    *const f32,
        out0:         *mut f32,
        out1:         *mut f32,
        samples_done: &mut i32)  {
        
        todo!();
        /*
            auto dummy = samplesDone;
            synthesise (bandPtr0, 0, out0, dummy);
            synthesise (bandPtr1, 1, out1, samplesDone);
        */
    }
    
    pub fn synthesise(&mut self, 
        band_ptr:     *const f32,
        channel:      i32,
        out:          *mut f32,
        samples_done: &mut i32)  {
        
        todo!();
        /*
            out += samplesDone;
            const int bo = channel == 0 ? ((synthBo - 1) & 15) : synthBo;
            float (*buf)[0x110] = synthBuffers[channel];
            float* b0;
            auto bo1 = bo;

            if (bo & 1)
            {
                b0 = buf[0];
                DCT::dct64 (buf[1] + ((bo + 1) & 15), buf[0] + bo, bandPtr);
            }
            else
            {
                ++bo1;
                b0 = buf[1];
                DCT::dct64 (buf[0] + bo, buf[1] + bo1, bandPtr);
            }

            synthBo = bo;
            const float* window = constants.decodeWin + 16 - bo1;

            for (int j = 16; j != 0; --j, b0 += 16, window += 32)
            {
                auto sum = window[0] * b0[0];  sum -= window[1] * b0[1];
                sum += window[2]  * b0[2];   sum -= window[3]  * b0[3];
                sum += window[4]  * b0[4];   sum -= window[5]  * b0[5];
                sum += window[6]  * b0[6];   sum -= window[7]  * b0[7];
                sum += window[8]  * b0[8];   sum -= window[9]  * b0[9];
                sum += window[10] * b0[10];  sum -= window[11] * b0[11];
                sum += window[12] * b0[12];  sum -= window[13] * b0[13];
                sum += window[14] * b0[14];  sum -= window[15] * b0[15];
                *out++ = sum;
            }

            {
                auto sum = window[0] * b0[0];   sum += window[2] * b0[2];
                sum += window[4]  * b0[4];   sum += window[6]  * b0[6];
                sum += window[8]  * b0[8];   sum += window[10] * b0[10];
                sum += window[12] * b0[12];  sum += window[14] * b0[14];
                *out++ = sum;
                b0 -= 16; window -= 32;
                window += (ptrdiff_t) bo1 << 1;
            }

            for (int j = 15; j != 0; --j, b0 -= 16, window -= 32)
            {
                auto sum = -window[-1] * b0[0];  sum -= window[-2] * b0[1];
                sum -= window[-3]  * b0[2];   sum -= window[-4]  * b0[3];
                sum -= window[-5]  * b0[4];   sum -= window[-6]  * b0[5];
                sum -= window[-7]  * b0[6];   sum -= window[-8]  * b0[7];
                sum -= window[-9]  * b0[8];   sum -= window[-10] * b0[9];
                sum -= window[-11] * b0[10];  sum -= window[-12] * b0[11];
                sum -= window[-13] * b0[12];  sum -= window[-14] * b0[13];
                sum -= window[-15] * b0[14];  sum -= window[0]   * b0[15];
                *out++ = sum;
            }

            samplesDone += 32;
        */
    }
}
