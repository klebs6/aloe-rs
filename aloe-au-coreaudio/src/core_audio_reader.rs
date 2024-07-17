crate::ix!();

#[cfg(any(target_os="macos",target_os="ios"))]
#[no_copy]
#[leak_detector]
pub struct CoreAudioReader<'a> {
    base:                     AudioFormatReader<'a>,
    ok:                       bool, // default = false
    audio_fileid:             AudioFileID,
    audio_file_ref:           ExtAudioFileRef,
    channel_set:              AudioChannelSet,
    destination_audio_format: AudioStreamBasicDescription,
    audio_data_block:         MemoryBlock,
    buffer_list:              HeapBlock<AudioBufferList>,
    last_read_position:       i64, // default = 0
    channel_map:              HeapBlock<i32>,
}

#[cfg(any(target_os="macos",target_os="ios"))]
impl<'a> Drop for CoreAudioReader<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            ExtAudioFileDispose (audioFileRef);
            AudioFileClose (audioFileID);
        */
    }
}

#[cfg(any(target_os="macos",target_os="ios"))]
impl<'a> CoreAudioReader<'a> {

    pub fn new<InputStreamType: InputStream>(inp: *mut InputStreamType) -> Self {
    
        todo!();
        /*
        : audio_format_reader(inp, coreAudioFormatName),

            usesFloatingPointData = true;
            bitsPerSample = 32;

            if (input != nullptr)
                CoreAudioFormatMetatdata::read (*input, metadataValues);

            auto status = AudioFileOpenWithCallbacks (this,
                                                      &readCallback,
                                                      nullptr,  // write needs to be null to avoid permissions errors
                                                      &getSizeCallback,
                                                      nullptr,  // setSize needs to be null to avoid permissions errors
                                                      0,        // AudioFileTypeID inFileTypeHint
                                                      &audioFileID);
            if (status == noErr)
            {
                status = ExtAudioFileWrapAudioFileID (audioFileID, false, &audioFileRef);

                if (status == noErr)
                {
                    AudioStreamBasicDescription sourceAudioFormat;
                    UInt32 audioStreamBasicDescriptionSize = sizeof (AudioStreamBasicDescription);
                    ExtAudioFileGetProperty (audioFileRef,
                                             kExtAudioFileProperty_FileDataFormat,
                                             &audioStreamBasicDescriptionSize,
                                             &sourceAudioFormat);

                    numChannels = sourceAudioFormat.mChannelsPerFrame;
                    sampleRate  = sourceAudioFormat.mSampleRate;

                    UInt32 sizeOfLengthProperty = sizeof (int64);
                    ExtAudioFileGetProperty (audioFileRef,
                                             kExtAudioFileProperty_FileLengthFrames,
                                             &sizeOfLengthProperty,
                                             &lengthInSamples);

                    HeapBlock<AudioChannelLayout> caLayout;
                    bool hasLayout = false;
                    UInt32 sizeOfLayout = 0, isWritable = 0;

                    status = AudioFileGetPropertyInfo (audioFileID, kAudioFilePropertyChannelLayout, &sizeOfLayout, &isWritable);

                    if (status == noErr && sizeOfLayout >= (sizeof (AudioChannelLayout) - sizeof (AudioChannelDescription)))
                    {
                        caLayout.malloc (1, static_cast<size_t> (sizeOfLayout));

                        status = AudioFileGetProperty (audioFileID, kAudioFilePropertyChannelLayout,
                                                       &sizeOfLayout, caLayout.get());

                        if (status == noErr)
                        {
                            auto fileLayout = CoreAudioLayouts::fromCoreAudio (*caLayout.get());

                            if (fileLayout.size() == static_cast<int> (numChannels))
                            {
                                hasLayout = true;
                                channelSet = fileLayout;
                            }
                        }
                    }

                    destinationAudioFormat.mSampleRate       = sampleRate;
                    destinationAudioFormat.mFormatID         = kAudioFormatLinearPCM;
                    destinationAudioFormat.mFormatFlags      = kLinearPCMFormatFlagIsFloat | kLinearPCMFormatFlagIsNonInterleaved | kAudioFormatFlagsNativeEndian;
                    destinationAudioFormat.mBitsPerChannel   = sizeof (float) * 8;
                    destinationAudioFormat.mChannelsPerFrame = numChannels;
                    destinationAudioFormat.mBytesPerFrame    = sizeof (float);
                    destinationAudioFormat.mFramesPerPacket  = 1;
                    destinationAudioFormat.mBytesPerPacket   = destinationAudioFormat.mFramesPerPacket * destinationAudioFormat.mBytesPerFrame;

                    status = ExtAudioFileSetProperty (audioFileRef,
                                                      kExtAudioFileProperty_ClientDataFormat,
                                                      sizeof (AudioStreamBasicDescription),
                                                      &destinationAudioFormat);
                    if (status == noErr)
                    {
                        bufferList.malloc (1, sizeof (AudioBufferList) + numChannels * sizeof (::AudioBuffer));
                        bufferList->mNumberBuffers = numChannels;
                        channelMap.malloc (numChannels);

                        if (hasLayout && caLayout != nullptr)
                        {
                            auto caOrder = CoreAudioLayouts::getCoreAudioLayoutChannels (*caLayout);

                            for (int i = 0; i < static_cast<int> (numChannels); ++i)
                            {
                                auto idx = channelSet.getChannelIndexForType (caOrder.getReference (i));
                                jassert (isPositiveAndBelow (idx, static_cast<int> (numChannels)));

                                channelMap[i] = idx;
                            }
                        }
                        else
                        {
                            for (int i = 0; i < static_cast<int> (numChannels); ++i)
                                channelMap[i] = i;
                        }

                        ok = true;
                    }
                }
            }
        */
    }
    
    pub fn read_samples(&mut self, 
        dest_samples:                *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 i32) -> bool {
        
        todo!();
        /*
            clearSamplesBeyondAvailableLength (destSamples, numDestChannels, startOffsetInDestBuffer,
                                               startSampleInFile, numSamples, lengthInSamples);

            if (numSamples <= 0)
                return true;

            if (lastReadPosition != startSampleInFile)
            {
                OSStatus status = ExtAudioFileSeek (audioFileRef, startSampleInFile);
                if (status != noErr)
                    return false;

                lastReadPosition = startSampleInFile;
            }

            while (numSamples > 0)
            {
                auto numThisTime = jmin (8192, numSamples);
                auto numBytes = (size_t) numThisTime * sizeof (float);

                audioDataBlock.ensureSize (numBytes * numChannels, false);
                auto* data = static_cast<float*> (audioDataBlock.getData());

                for (int j = (int) numChannels; --j >= 0;)
                {
                    bufferList->mBuffers[j].mNumberChannels = 1;
                    bufferList->mBuffers[j].mDataByteSize = (UInt32) numBytes;
                    bufferList->mBuffers[j].mData = data;
                    data += numThisTime;
                }

                auto numFramesToRead = (UInt32) numThisTime;
                auto status = ExtAudioFileRead (audioFileRef, &numFramesToRead, bufferList);

                if (status != noErr)
                    return false;

                if (numFramesToRead == 0)
                    break;

                if ((int) numFramesToRead < numThisTime)
                {
                    numThisTime = (int) numFramesToRead;
                    numBytes    = (size_t) numThisTime * sizeof (float);
                }

                for (int i = numDestChannels; --i >= 0;)
                {
                    auto* dest = destSamples[(i < (int) numChannels ? channelMap[i] : i)];

                    if (dest != nullptr)
                    {
                        if (i < (int) numChannels)
                            memcpy (dest + startOffsetInDestBuffer, bufferList->mBuffers[i].mData, numBytes);
                        else
                            zeromem (dest + startOffsetInDestBuffer, numBytes);
                    }
                }

                startOffsetInDestBuffer += numThisTime;
                numSamples -= numThisTime;
                lastReadPosition += numThisTime;
            }

            return true;
        */
    }
    
    pub fn get_channel_layout(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            if (channelSet.size() == static_cast<int> (numChannels))
                return channelSet;

            return AudioFormatReader::getChannelLayout();
        */
    }
    
    pub fn get_size_callback(in_client_data: *mut c_void) -> SInt64 {
        
        todo!();
        /*
            return static_cast<CoreAudioReader*> (inClientData)->input->getTotalLength();
        */
    }
    
    pub fn read_callback(
        in_client_data: *mut c_void,
        in_position:    SInt64,
        request_count:  u32,
        buffer:         *mut c_void,
        actual_count:   *mut u32) -> OSStatus {
        
        todo!();
        /*
            auto* reader = static_cast<CoreAudioReader*> (inClientData);
            reader->input->setPosition (inPosition);
            *actualCount = (UInt32) reader->input->read (buffer, (int) requestCount);
            return noErr;
        */
    }
}
