crate::ix!();

pub fn is_layout_supported<const numLayouts: usize>(
    processor:           &AudioProcessor,
    is_input:            bool,
    bus_idx:             i32,
    num_channels:        i32,
    channel_layout_list: &[[i16; numLayouts]; 2],
    has_layout_map:      Option<bool>

) -> bool {

    let has_layout_map: bool = has_layout_map.unwrap_or(true);

    todo!();
    /*
        if (const AudioProcessor::Bus* bus = processor.getBus (isInput, busIdx))
        {
            if (! bus->isNumberOfChannelsSupported (numChannels))
                return false;

            if (! hasLayoutMap)
                return true;

            const int numConfigs = sizeof (channelLayoutList) / sizeof (short[2]);

            for (int i = 0; i < numConfigs; ++i)
            {
                if (channelLayoutList[i][isInput ? 0 : 1] == numChannels)
                    return true;
            }
        }

        return false;
    */
}

pub fn get_au_channel_info(processor: &AudioProcessor) -> Vec<AUChannelInfo> {
    
    todo!();
    /*
        Vec<AUChannelInfo> channelInfo;

        auto hasMainInputBus  = (AudioUnitHelpers::getBusCountForWrapper (processor, true)  > 0);
        auto hasMainOutputBus = (AudioUnitHelpers::getBusCountForWrapper (processor, false) > 0);

        if ((! hasMainInputBus) && (! hasMainOutputBus))
        {
            // midi effect plug-in: no audio
            AUChannelInfo info;
            info.inChannels = 0;
            info.outChannels = 0;

            return { &info, 1 };
        }

        auto layout = processor.getBusesLayout();
        auto maxNumChanToCheckFor = 9;

        auto defaultInputs  = processor.getChannelCountOfBus (true,  0);
        auto defaultOutputs = processor.getChannelCountOfBus (false, 0);

        struct Channels
        {
            SInt16 ins, outs;

            std::pair<SInt16, SInt16> makePair() const  { return std::make_pair (ins, outs); }

            bool operator<  (const Channels& other) const  { return makePair() <  other.makePair(); }
            bool operator== (const Channels& other) const  { return makePair() == other.makePair(); }
        };

        SortedSet<Channels> supportedChannels;

        // add the current configuration
        if (defaultInputs != 0 || defaultOutputs != 0)
            supportedChannels.add ({ static_cast<SInt16> (defaultInputs),
                                     static_cast<SInt16> (defaultOutputs) });

        for (auto inChanNum = hasMainInputBus ? 1 : 0; inChanNum <= (hasMainInputBus ? maxNumChanToCheckFor : 0); ++inChanNum)
        {
            auto inLayout = layout;

            if (auto* inBus = processor.getBus (true, 0))
                if (! isNumberOfChannelsSupported (inBus, inChanNum, inLayout))
                    continue;

            for (auto outChanNum = hasMainOutputBus ? 1 : 0; outChanNum <= (hasMainOutputBus ? maxNumChanToCheckFor : 0); ++outChanNum)
            {
                auto outLayout = inLayout;

                if (auto* outBus = processor.getBus (false, 0))
                    if (! isNumberOfChannelsSupported (outBus, outChanNum, outLayout))
                        continue;

                supportedChannels.add ({ static_cast<SInt16> (hasMainInputBus  ? outLayout.getMainInputChannels()  : 0),
                                         static_cast<SInt16> (hasMainOutputBus ? outLayout.getMainOutputChannels() : 0) });
            }
        }

        auto hasInOutMismatch = false;

        for (const auto& supported : supportedChannels)
        {
            if (supported.ins != supported.outs)
            {
                hasInOutMismatch = true;
                break;
            }
        }

        auto hasUnsupportedInput = ! hasMainInputBus, hasUnsupportedOutput = ! hasMainOutputBus;

        for (auto inChanNum = hasMainInputBus ? 1 : 0; inChanNum <= (hasMainInputBus ? maxNumChanToCheckFor : 0); ++inChanNum)
        {
            Channels channelConfiguration { static_cast<SInt16> (inChanNum),
                                            static_cast<SInt16> (hasInOutMismatch ? defaultOutputs : inChanNum) };

            if (! supportedChannels.contains (channelConfiguration))
            {
                hasUnsupportedInput = true;
                break;
            }
        }

        for (auto outChanNum = hasMainOutputBus ? 1 : 0; outChanNum <= (hasMainOutputBus ? maxNumChanToCheckFor : 0); ++outChanNum)
        {
            Channels channelConfiguration { static_cast<SInt16> (hasInOutMismatch ? defaultInputs : outChanNum),
                                            static_cast<SInt16> (outChanNum) };

            if (! supportedChannels.contains (channelConfiguration))
            {
                hasUnsupportedOutput = true;
                break;
            }
        }

        for (const auto& supported : supportedChannels)
        {
            AUChannelInfo info;

            // see here: https://developer.apple.com/library/mac/documentation/MusicAudio/Conceptual/AudioUnitProgrammingGuide/TheAudioUnit/TheAudioUnit.html
            info.inChannels  = static_cast<SInt16> (hasMainInputBus  ? (hasUnsupportedInput  ? supported.ins  : (hasInOutMismatch && (! hasUnsupportedOutput) ? -2 : -1)) : 0);
            info.outChannels = static_cast<SInt16> (hasMainOutputBus ? (hasUnsupportedOutput ? supported.outs : (hasInOutMismatch && (! hasUnsupportedInput)  ? -2 : -1)) : 0);

            if (info.inChannels == -2 && info.outChannels == -2)
                info.inChannels = -1;

            int j;
            for (j = 0; j < channelInfo.size(); ++j)
                if (info.inChannels == channelInfo.getReference (j).inChannels
                      && info.outChannels == channelInfo.getReference (j).outChannels)
                    break;

            if (j >= channelInfo.size())
                channelInfo.add (info);
        }

        return channelInfo;
    */
}

pub fn is_number_of_channels_supported(
    b:                     *const AudioProcessorBus,
    num_channels:          i32,
    in_out_current_layout: &mut AudioProcessorBusesLayout

) -> bool {
    
    todo!();
    /*
        auto potentialSets = AudioChannelSet::channelSetsWithNumberOfChannels (static_cast<int> (numChannels));

        for (auto set : potentialSets)
        {
            auto copy = inOutCurrentLayout;

            if (b->isLayoutSupported (set, &copy))
            {
                inOutCurrentLayout = copy;
                return true;
            }
        }

        return false;
    */
}

pub fn get_bus_count(
    aloe_filter: &AudioProcessor,
    is_input:    bool) -> i32 {
    
    todo!();
    /*
        int busCount = aloeFilter.getBusCount (isInput);

       #ifdef AloePlugin_PreferredChannelConfigurations
        short configs[][2] = {AloePlugin_PreferredChannelConfigurations};
        const int numConfigs = sizeof (configs) / sizeof (short[2]);

        bool hasOnlyZeroChannels = true;

        for (int i = 0; i < numConfigs && hasOnlyZeroChannels == true; ++i)
            if (configs[i][isInput ? 0 : 1] != 0)
                hasOnlyZeroChannels = false;

        busCount = jmin (busCount, hasOnlyZeroChannels ? 0 : 1);
       #endif

        return busCount;
    */
}

pub fn get_bus_count_for_wrapper(
    aloe_filter: &AudioProcessor,
    is_input:    bool) -> i32 {
    
    todo!();
    /*
        #if AloePlugin_IsMidiEffect
        const auto numRequiredBuses = isInput ? 0 : 1;
       #else
        const auto numRequiredBuses = 0;
       #endif

        return jmax (numRequiredBuses, getBusCount (aloeFilter, isInput));
    */
}

pub fn set_buses_layout(
    aloe_filter:       *mut AudioProcessor,
    requested_layouts: &AudioProcessorBusesLayout
) -> bool {
    
    todo!();
    /*
        #ifdef AloePlugin_PreferredChannelConfigurations
        AudioProcessor::BusesLayout copy (requestedLayouts);

        for (int dir = 0; dir < 2; ++dir)
        {
            const bool isInput = (dir == 0);

            const int actualBuses = aloeFilter->getBusCount (isInput);
            const int auNumBuses  = getBusCount (*aloeFilter, isInput);
            Vec<AudioChannelSet>& buses = (isInput ? copy.inputBuses : copy.outputBuses);

            for (int i = auNumBuses; i < actualBuses; ++i)
                buses.add (AudioChannelSet::disabled());
        }

        return aloeFilter->setBusesLayout (copy);
       #else
        return aloeFilter->setBusesLayout (requestedLayouts);
       #endif
    */
}

pub fn get_buses_layout(aloe_filter: *const AudioProcessor) -> AudioProcessorBusesLayout {
    
    todo!();
    /*
        #ifdef AloePlugin_PreferredChannelConfigurations
        AudioProcessor::BusesLayout layout = aloeFilter->getBusesLayout();

        for (int dir = 0; dir < 2; ++dir)
        {
            const bool isInput = (dir == 0);

            const int actualBuses = aloeFilter->getBusCount (isInput);
            const int auNumBuses  = getBusCount (*aloeFilter, isInput);
            auto& buses = (isInput ? layout.inputBuses : layout.outputBuses);

            for (int i = auNumBuses; i < actualBuses; ++i)
                buses.removeLast();
        }

        return layout;
       #else
        return aloeFilter->getBusesLayout();
       #endif
    */
}
