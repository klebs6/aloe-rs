crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AAX/aloe_AAX_Wrapper.cpp]

#[cfg(all(AloePlugin_Build_AAX,any(ALOE_INCLUDED_AAX_IN_MM,any(_WIN32,_WIN64))))]
pub mod aloe_plugin_build_aax {

    use super::*;

    static_assert!{
        AAX_SDK_CURRENT_REVISION >= AAX_SDK_2p3p0_REVISION, 
        "Aloe requires AAX SDK version 2.3.0 or higher"
    }

    #[cfg(target_os="windows")] #[cfg(ALOE_64BIT)]      pub const ALOE_AAX_LIB: &'static str = "AAXLibrary_x64";
    #[cfg(target_os="windows")] #[cfg(not(ALOE_64BIT))] pub const ALOE_AAX_LIB: &'static str = "AAXLibrary";

    #[cfg(target_os="windows")] #[cfg(ALOE_DEBUG)]      pub const ALOE_AAX_LIB_PATH:   &'static str = "\\Debug\\";
    #[cfg(target_os="windows")] #[cfg(ALOE_DEBUG)]      pub const ALOE_AAX_LIB_SUFFIX: &'static str = "_D";
    #[cfg(target_os="windows")] #[cfg(not(ALOE_DEBUG))] pub const ALOE_AAX_LIB_PATH:   &'static str = "\\Release\\";
    #[cfg(target_os="windows")] #[cfg(not(ALOE_DEBUG))] pub const ALOE_AAX_LIB_SUFFIX: &'static str = "";

    // #pragma comment(lib, AloePlugin_AAXLibs_path ALOE_AAX_LIB_PATH ALOE_AAX_LIB ALOE_AAX_LIB_SUFFIX ".lib")

    #[cfg(not(AloePlugin_AAX_Chunk_Identifier))]
    pub const AloePlugin_AAX_Chunk_Identifier: &'static str = "aloe";

    pub const aloe_chunk_type: i32 = AloePlugin_AAX_Chunk_Identifier;

    pub mod aax_classes
    {
        use super::*;

        pub fn get_aax_param_hash(paramid: AAX_CParamID) -> i32 {
            
            todo!();
                /*
                    int32 result = 0;

                    while (*paramID != 0)
                        result = (31 * result) + (*paramID++);

                    return result;
                */
        }

        pub fn check(result: AAX_Result)  {
            
            todo!();
                /*
                    jassertquiet (result == AAX_SUCCESS);
                */
        }

        /**
           maps a channel index of an AAX format to an
           index of a aloe format
          */
        pub struct AAXChannelStreamOrder
        {
            aax_stem_format: AAX_EStemFormat,
            speaker_order:   [AudioChannelSet::ChannelType; 10],
        }

        pub fn stem_format_for_ambisonic_order(order: i32) -> AAX_EStemFormat {
            
            todo!();
                /*
                    switch (order)
                    {
                        case 1:   return AAX_eStemFormat_Ambi_1_ACN;
                        case 2:   return AAX_eStemFormat_Ambi_2_ACN;
                        case 3:   return AAX_eStemFormat_Ambi_3_ACN;
                        default:  break;
                    }

                    return AAX_eStemFormat_INT32_MAX;
                */
        }

        lazy_static!{
            /*
            static AAXChannelStreamOrder aaxChannelOrder[] =
                {
                    { AAX_eStemFormat_Mono,     { AudioChannelSet::centre, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown,
                                                  AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_Stereo,   { AudioChannelSet::left, AudioChannelSet::right, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown,
                                                  AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_LCR,      { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::unknown, AudioChannelSet::unknown,
                                                  AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_LCRS,     { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::centreSurround, AudioChannelSet::unknown,
                                                  AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_Quad,     { AudioChannelSet::left, AudioChannelSet::right,  AudioChannelSet::leftSurround, AudioChannelSet::rightSurround, AudioChannelSet::unknown,
                                                  AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_5_0,      { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurround, AudioChannelSet::rightSurround,
                                                  AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_5_1,      { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurround, AudioChannelSet::rightSurround,
                                                  AudioChannelSet::LFE, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_6_0,      { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurround, AudioChannelSet::centreSurround,
                                                  AudioChannelSet::rightSurround, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_6_1,      { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurround, AudioChannelSet::centreSurround,
                                                  AudioChannelSet::rightSurround, AudioChannelSet::LFE, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_7_0_SDDS, { AudioChannelSet::left, AudioChannelSet::leftCentre, AudioChannelSet::centre, AudioChannelSet::rightCentre, AudioChannelSet::right,
                                                  AudioChannelSet::leftSurround, AudioChannelSet::rightSurround, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_7_0_DTS,  { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurroundSide, AudioChannelSet::rightSurroundSide,
                                                  AudioChannelSet::leftSurroundRear, AudioChannelSet::rightSurroundRear, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_7_1_SDDS, { AudioChannelSet::left, AudioChannelSet::leftCentre, AudioChannelSet::centre, AudioChannelSet::rightCentre, AudioChannelSet::right,
                                                  AudioChannelSet::leftSurround, AudioChannelSet::rightSurround, AudioChannelSet::LFE, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_7_1_DTS,  { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurroundSide, AudioChannelSet::rightSurroundSide,
                                                  AudioChannelSet::leftSurroundRear, AudioChannelSet::rightSurroundRear, AudioChannelSet::LFE, AudioChannelSet::unknown, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_7_0_2,    { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurroundSide, AudioChannelSet::rightSurroundSide,
                                                  AudioChannelSet::leftSurroundRear, AudioChannelSet::rightSurroundRear, AudioChannelSet::topSideLeft, AudioChannelSet::topSideRight, AudioChannelSet::unknown } },

                    { AAX_eStemFormat_7_1_2,    { AudioChannelSet::left, AudioChannelSet::centre, AudioChannelSet::right, AudioChannelSet::leftSurroundSide, AudioChannelSet::rightSurroundSide,
                                                  AudioChannelSet::leftSurroundRear, AudioChannelSet::rightSurroundRear, AudioChannelSet::LFE, AudioChannelSet::topSideLeft, AudioChannelSet::topSideRight } },

                    { AAX_eStemFormat_None,     { AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown,
                                                  AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown, AudioChannelSet::unknown } },
                };
            */
        }

        lazy_static!{
            /*
            static AAX_EStemFormat aaxFormats[] =
                {
                    AAX_eStemFormat_Mono,
                    AAX_eStemFormat_Stereo,
                    AAX_eStemFormat_LCR,
                    AAX_eStemFormat_LCRS,
                    AAX_eStemFormat_Quad,
                    AAX_eStemFormat_5_0,
                    AAX_eStemFormat_5_1,
                    AAX_eStemFormat_6_0,
                    AAX_eStemFormat_6_1,
                    AAX_eStemFormat_7_0_SDDS,
                    AAX_eStemFormat_7_1_SDDS,
                    AAX_eStemFormat_7_0_DTS,
                    AAX_eStemFormat_7_1_DTS,
                    AAX_eStemFormat_7_0_2,
                    AAX_eStemFormat_7_1_2,
                    AAX_eStemFormat_Ambi_1_ACN,
                    AAX_eStemFormat_Ambi_2_ACN,
                    AAX_eStemFormat_Ambi_3_ACN
                };
            */
        }

        pub fn get_format_for_audio_channel_set(
                set:           &AudioChannelSet,
                ignore_layout: bool) -> AAX_EStemFormat {
            
            todo!();
                /*
                    // if the plug-in ignores layout, it is ok to convert between formats only by their numchannnels
                    if (ignoreLayout)
                    {
                        auto numChannels = set.size();

                        switch (numChannels)
                        {
                            case 0:   return AAX_eStemFormat_None;
                            case 1:   return AAX_eStemFormat_Mono;
                            case 2:   return AAX_eStemFormat_Stereo;
                            case 3:   return AAX_eStemFormat_LCR;
                            case 4:   return AAX_eStemFormat_Quad;
                            case 5:   return AAX_eStemFormat_5_0;
                            case 6:   return AAX_eStemFormat_5_1;
                            case 7:   return AAX_eStemFormat_7_0_DTS;
                            case 8:   return AAX_eStemFormat_7_1_DTS;
                            case 9:   return AAX_eStemFormat_7_0_2;
                            case 10:  return AAX_eStemFormat_7_1_2;
                            default:  break;
                        }

                        // check for ambisonics support
                        auto sqrtMinusOne   = std::sqrt (static_cast<float> (numChannels)) - 1.0f;
                        auto ambisonicOrder = jmax (0, static_cast<int> (std::floor (sqrtMinusOne)));

                        if (static_cast<float> (ambisonicOrder) == sqrtMinusOne)
                            return stemFormatForAmbisonicOrder (ambisonicOrder);

                        return AAX_eStemFormat_INT32_MAX;
                    }

                    if (set == AudioChannelSet::disabled())             return AAX_eStemFormat_None;
                    if (set == AudioChannelSet::mono())                 return AAX_eStemFormat_Mono;
                    if (set == AudioChannelSet::stereo())               return AAX_eStemFormat_Stereo;
                    if (set == AudioChannelSet::createLCR())            return AAX_eStemFormat_LCR;
                    if (set == AudioChannelSet::createLCRS())           return AAX_eStemFormat_LCRS;
                    if (set == AudioChannelSet::quadraphonic())         return AAX_eStemFormat_Quad;
                    if (set == AudioChannelSet::create5point0())        return AAX_eStemFormat_5_0;
                    if (set == AudioChannelSet::create5point1())        return AAX_eStemFormat_5_1;
                    if (set == AudioChannelSet::create6point0())        return AAX_eStemFormat_6_0;
                    if (set == AudioChannelSet::create6point1())        return AAX_eStemFormat_6_1;
                    if (set == AudioChannelSet::create7point0())        return AAX_eStemFormat_7_0_DTS;
                    if (set == AudioChannelSet::create7point1())        return AAX_eStemFormat_7_1_DTS;
                    if (set == AudioChannelSet::create7point0SDDS())    return AAX_eStemFormat_7_0_SDDS;
                    if (set == AudioChannelSet::create7point1SDDS())    return AAX_eStemFormat_7_1_SDDS;
                    if (set == AudioChannelSet::create7point0point2())  return AAX_eStemFormat_7_0_2;
                    if (set == AudioChannelSet::create7point1point2())  return AAX_eStemFormat_7_1_2;

                    auto order = set.getAmbisonicOrder();
                    if (order >= 0)
                        return stemFormatForAmbisonicOrder (order);

                    return AAX_eStemFormat_INT32_MAX;
                */
        }

        pub fn channel_set_from_stem_format(
                format:        AAX_EStemFormat,
                ignore_layout: bool) -> AudioChannelSet {
            
            todo!();
                /*
                    if (! ignoreLayout)
                    {
                        switch (format)
                        {
                            case AAX_eStemFormat_None:       return AudioChannelSet::disabled();
                            case AAX_eStemFormat_Mono:       return AudioChannelSet::mono();
                            case AAX_eStemFormat_Stereo:     return AudioChannelSet::stereo();
                            case AAX_eStemFormat_LCR:        return AudioChannelSet::createLCR();
                            case AAX_eStemFormat_LCRS:       return AudioChannelSet::createLCRS();
                            case AAX_eStemFormat_Quad:       return AudioChannelSet::quadraphonic();
                            case AAX_eStemFormat_5_0:        return AudioChannelSet::create5point0();
                            case AAX_eStemFormat_5_1:        return AudioChannelSet::create5point1();
                            case AAX_eStemFormat_6_0:        return AudioChannelSet::create6point0();
                            case AAX_eStemFormat_6_1:        return AudioChannelSet::create6point1();
                            case AAX_eStemFormat_7_0_SDDS:   return AudioChannelSet::create7point0SDDS();
                            case AAX_eStemFormat_7_0_DTS:    return AudioChannelSet::create7point0();
                            case AAX_eStemFormat_7_1_SDDS:   return AudioChannelSet::create7point1SDDS();
                            case AAX_eStemFormat_7_1_DTS:    return AudioChannelSet::create7point1();
                            case AAX_eStemFormat_7_0_2:      return AudioChannelSet::create7point0point2();
                            case AAX_eStemFormat_7_1_2:      return AudioChannelSet::create7point1point2();
                            case AAX_eStemFormat_Ambi_1_ACN: return AudioChannelSet::ambisonic (1);
                            case AAX_eStemFormat_Ambi_2_ACN: return AudioChannelSet::ambisonic (2);
                            case AAX_eStemFormat_Ambi_3_ACN: return AudioChannelSet::ambisonic (3);
                            case AAX_eStemFormat_Reserved_1:
                            case AAX_eStemFormat_Reserved_2:
                            case AAX_eStemFormat_Reserved_3:
                            case AAX_eStemFormatNum:
                            case AAX_eStemFormat_Any:
                            case AAX_eStemFormat_INT32_MAX:
                            default:                         return AudioChannelSet::disabled();
                        }
                    }

                    return AudioChannelSet::discreteChannels (jmax (0, static_cast<int> (AAX_STEM_FORMAT_CHANNEL_COUNT (format))));
                */
        }

        pub fn get_meter_type_for_category(category: AudioProcessorParameter::Category) -> AAX_EMeterType {
            
            todo!();
                /*
                    switch (category)
                    {
                        case AudioProcessorParameter::inputMeter:                           return AAX_eMeterType_Input;
                        case AudioProcessorParameter::outputMeter:                          return AAX_eMeterType_Output;
                        case AudioProcessorParameter::compressorLimiterGainReductionMeter:  return AAX_eMeterType_CLGain;
                        case AudioProcessorParameter::expanderGateGainReductionMeter:       return AAX_eMeterType_EGGain;
                        case AudioProcessorParameter::analysisMeter:                        return AAX_eMeterType_Analysis;
                        case AudioProcessorParameter::genericParameter:
                        case AudioProcessorParameter::inputGain:
                        case AudioProcessorParameter::outputGain:
                        case AudioProcessorParameter::otherMeter:
                        default:                                                            return AAX_eMeterType_Other;
                    }
                */
        }

        pub fn get_colour_from_highlight_enum(colour: AAX_EHighlightColor) -> Colour {
            
            todo!();
                /*
                    switch (colour)
                    {
                        case AAX_eHighlightColor_Red:       return Colours::red;
                        case AAX_eHighlightColor_Blue:      return Colours::blue;
                        case AAX_eHighlightColor_Green:     return Colours::green;
                        case AAX_eHighlightColor_Yellow:    return Colours::yellow;
                        case AAX_eHighlightColor_Num:
                        default:                            jassertfalse; break;
                    }

                    return Colours::black;
                */
        }

        pub fn aloe_channel_index_to_aax(
                aloe_index:  i32,
                channel_set: &AudioChannelSet) -> i32 {
            
            todo!();
                /*
                    auto isAmbisonic = (channelSet.getAmbisonicOrder() >= 0);
                    auto currentLayout = getFormatForAudioChannelSet (channelSet, false);
                    int layoutIndex;

                    if (isAmbisonic && currentLayout != AAX_eStemFormat_INT32_MAX)
                        return aloeIndex;

                    for (layoutIndex = 0; aaxChannelOrder[layoutIndex].aaxStemFormat != currentLayout; ++layoutIndex)
                        if (aaxChannelOrder[layoutIndex].aaxStemFormat == 0) return aloeIndex;

                    auto& channelOrder = aaxChannelOrder[layoutIndex];
                    auto channelType = channelSet.getTypeOfChannel (static_cast<int> (aloeIndex));
                    auto numSpeakers = numElementsInArray (channelOrder.speakerOrder);

                    for (int i = 0; i < numSpeakers && channelOrder.speakerOrder[i] != 0; ++i)
                        if (channelOrder.speakerOrder[i] == channelType)
                            return i;

                    return aloeIndex;
                */
        }

        #[no_copy]
        pub struct PluginInstanceInfo
        {
            parameters: AloeAAX_Processor,
        }

        impl PluginInstanceInfo {
            
            pub fn new(p: &mut AloeAAX_Processor) -> Self {
            
                todo!();
                /*
                : parameters(p),

                
                */
            }
        }

        pub struct ALOEAlgorithmContext
        {
            input_channels:     *mut *mut f32,
            output_channels:    *mut *mut f32,
            buffer_size:        *mut i32,
            bypass:             *mut i32,

            #[cfg(any(AloePlugin_WantsMidiInput,AloePlugin_IsMidiEffect))]
            midi_node_in:       *mut AAX_IMIDINode,

            #[cfg(any(AloePlugin_ProducesMidiOutput,AloePlugin_IsSynth,AloePlugin_IsMidiEffect))]
            midi_node_out:      *mut AAX_IMIDINode,

            plugin_instance:    *mut PluginInstanceInfo,
            is_prepared:        *mut i32,
            meter_tap_buffers:  *mut *mut f32,
            side_chain_buffers: *mut i32,
        }

        pub mod aloe_algorithm_ids
        {
            use super::*;

            pub const inputChannels:       usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, inputChannels);
            pub const outputChannels:      usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, outputChannels);
            pub const bufferSize:          usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, bufferSize);
            pub const bypass:              usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, bypass);

            #[cfg(any(AloePlugin_WantsMidiInput,AloePlugin_IsMidiEffect))]
            pub const midiNodeIn:          usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, midiNodeIn);

            #[cfg(any(AloePlugin_ProducesMidiOutput,AloePlugin_IsSynth,AloePlugin_IsMidiEffect))]
            pub const midiNodeOut:         usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, midiNodeOut);

            pub const pluginInstance:      usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, pluginInstance);
            pub const preparedFlag:        usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, isPrepared);
            pub const meterTapBuffers:     usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, meterTapBuffers);
            pub const sideChainBuffers:    usize = AAX_FIELD_INDEX (ALOEAlgorithmContext, sideChainBuffers);
        }

        #[cfg(any(AloePlugin_WantsMidiInput,AloePlugin_IsMidiEffect))]
        pub fn get_midi_node_in(c: &ALOEAlgorithmContext) -> *mut AAX_IMIDINode {
            
            todo!();
                /*
                    return c.midiNodeIn;
                */
        }

        #[cfg(not(any(AloePlugin_WantsMidiInput,AloePlugin_IsMidiEffect)))]
        pub fn get_midi_node_in(_0: &ALOEAlgorithmContext) -> *mut AAX_IMIDINode {
            
            todo!();
                /*
                    return nullptr;
                */
        }

        #[cfg(any(AloePlugin_ProducesMidiOutput,AloePlugin_IsSynth,AloePlugin_IsMidiEffect))]
        lazy_static!{
            /*
            AAX_IMIDINode* midiNodeOut;
            */
        }

        #[cfg(any(AloePlugin_ProducesMidiOutput,AloePlugin_IsSynth,AloePlugin_IsMidiEffect))]
        pub fn get_midi_node_out(c: &ALOEAlgorithmContext) -> *mut AAX_IMIDINode {
            
            todo!();
                /*
                    return c.midiNodeOut;
                */
        }

        #[cfg(not(any(AloePlugin_ProducesMidiOutput,AloePlugin_IsSynth,AloePlugin_IsMidiEffect)))]
        pub fn get_midi_node_out(_0: &ALOEAlgorithmContext) -> *mut AAX_IMIDINode {
            
            todo!();
                /*
                    return nullptr;
                */
        }

        #[derive(Default)]
        #[no_copy]
        #[leak_detector]
        pub struct AloeAAX_GUI {
            base:                AAX_CEffectGUI,
            base2:               ModifierKeyProvider,
            component:           Box<ContentWrapperComponent>,
            library_initialiser: ScopedAloeInitialiser_GUI,
        }

        impl Drop for AloeAAX_GUI {
            fn drop(&mut self) {
                todo!();
                /*
                    DeleteViewContainer();
                */
            }
        }

        pub mod aloe_aax_gui {

            use super::*;
                
            #[no_copy]
            #[leak_detector]
            pub struct ContentWrapperComponent {
                base: Component,

                plugin_editor:        Box<AudioProcessorEditor>,
                owner:                AloeAAX_GUI,

                #[cfg(target_os="windows")]
                hooks:                WindowsHooks,

                fake_mouse_generator: FakeMouseMoveGenerator,
                last_valid_size:      Rectangle<i32>,
            }

            impl Drop for ContentWrapperComponent {
                fn drop(&mut self) {
                    todo!();
                    /*
                        if (pluginEditor != nullptr)
                                {
                                    typename PopupMenu::dismissAllActiveMenus();
                                    pluginEditor->removeMouseListener (this);
                                    pluginEditor->processor.editorBeingDeleted (pluginEditor.get());
                                }
                    */
                }
            }

            impl ContentWrapperComponent {

                pub fn new(
                    gui:    &mut AloeAAX_GUI,
                    plugin: &mut AudioProcessor) -> Self {
                
                    todo!();
                    /*
                    : owner(gui),

                        setOpaque (true);
                                setBroughtToFrontOnMouseClick (true);

                                pluginEditor.reset (plugin.createEditorIfNeeded());
                                addAndMakeVisible (pluginEditor.get());

                                if (pluginEditor != nullptr)
                                {
                                    lastValidSize = pluginEditor->getLocalBounds();
                                    setBounds (lastValidSize);
                                    pluginEditor->addMouseListener (this, true);
                                }

                                ignoreUnused (fakeMouseGenerator);
                    */
                }
                
                pub fn paint(&mut self, g: &mut Graphics)  {
                    
                    todo!();
                    /*
                        g.fillAll (Colours::black);
                    */
                }
                
                pub fn call_mouse_method<MethodType>(&mut self, 
                    e:      &MouseEvent,
                    method: MethodType)  {
                
                    todo!();
                    /*
                        if (auto* vc = owner.GetViewContainer())
                                {
                                    auto parameterIndex = pluginEditor->getControlParameterIndex (*e.eventComponent);

                                    if (auto aaxParamID = owner.getAAXParamIDFromAloeIndex (parameterIndex))
                                    {
                                        uint32_t mods = 0;
                                        vc->GetModifiers (&mods);

                                        (vc->*method) (aaxParamID, mods);
                                    }
                                }
                    */
                }
                
                pub fn mouse_down(&mut self, e: &MouseEvent)  {
                    
                    todo!();
                    /*
                        callMouseMethod (e, &AAX_IViewContainer::HandleParameterMouseDown);
                    */
                }
                
                pub fn mouse_up(&mut self, e: &MouseEvent)  {
                    
                    todo!();
                    /*
                        callMouseMethod (e, &AAX_IViewContainer::HandleParameterMouseUp);
                    */
                }
                
                pub fn mouse_drag(&mut self, e: &MouseEvent)  {
                    
                    todo!();
                    /*
                        callMouseMethod (e, &AAX_IViewContainer::HandleParameterMouseDrag);
                    */
                }
                
                pub fn parent_size_changed(&mut self)  {
                    
                    todo!();
                    /*
                        resizeHostWindow();

                                if (pluginEditor != nullptr)
                                    pluginEditor->repaint();
                    */
                }
                
                pub fn child_bounds_changed(&mut self, _0: *mut Component)  {
                    
                    todo!();
                    /*
                        if (resizeHostWindow())
                                {
                                    setSize (pluginEditor->getWidth(), pluginEditor->getHeight());
                                    lastValidSize = getBounds();
                                }
                                else
                                {
                                    pluginEditor->setBoundsConstrained (pluginEditor->getBounds().withSize (lastValidSize.getWidth(),
                                                                                                            lastValidSize.getHeight()));
                                }
                    */
                }
                
                pub fn resize_host_window(&mut self) -> bool {
                    
                    todo!();
                    /*
                        if (pluginEditor != nullptr)
                                {
                                    auto newSize = convertToHostBounds ({ (float) pluginEditor->getHeight(),
                                                                          (float) pluginEditor->getWidth() });

                                    return owner.GetViewContainer()->SetViewSize (newSize) == AAX_SUCCESS;
                                }

                                return false;
                    */
                }
            }
        }

        impl AloeAAX_GUI {

            pub fn create() -> *mut AAX_IEffectGUI {
                
                todo!();
                /*
                    return new AloeAAX_GUI();
                */
            }
            
            pub fn create_view_contents(&mut self)  {
                
                todo!();
                /*
                
                */
            }
            
            pub fn create_view_container(&mut self)  {
                
                todo!();
                /*
                    CreateViewContents();

                        if (void* nativeViewToAttachTo = GetViewContainerPtr())
                        {
                           #if ALOE_MAC
                            if (GetViewContainerType() == AAX_eViewContainer_Type_NSView)
                           #else
                            if (GetViewContainerType() == AAX_eViewContainer_Type_HWND)
                           #endif
                            {
                                component->setVisible (true);
                                component->addToDesktop (0, nativeViewToAttachTo);

                                if (ModifierKeyReceiver* modReceiver = dynamic_cast<ModifierKeyReceiver*> (component->getPeer()))
                                    modReceiver->setModifierKeyProvider (this);
                            }
                        }
                */
            }
            
            pub fn delete_view_container(&mut self)  {
                
                todo!();
                /*
                    if (component != nullptr)
                        {
                            ALOE_AUTORELEASEPOOL
                            {
                                if (auto* modReceiver = dynamic_cast<ModifierKeyReceiver*> (component->getPeer()))
                                    modReceiver->removeModifierKeyProvider();

                                component->removeFromDesktop();
                                component = nullptr;
                            }
                        }
                */
            }
            
            pub fn get_view_size(&self, view_size: *mut AAX_Point) -> AAX_Result {
                
                todo!();
                /*
                    if (component != nullptr)
                        {
                            *viewSize = convertToHostBounds ({ (float) component->getHeight(),
                                                               (float) component->getWidth() });

                            return AAX_SUCCESS;
                        }

                        return AAX_ERROR_NULL_OBJECT;
                */
            }
            
            pub fn parameter_updated(&mut self, _0: AAX_CParamID) -> AAX_Result {
                
                todo!();
                /*
                    return AAX_SUCCESS;
                */
            }
            
            pub fn set_control_highlight_info(&mut self, 
                paramid:        AAX_CParamID,
                is_highlighted: AAX_CBoolean,
                colour:         AAX_EHighlightColor) -> AAX_Result {
                
                todo!();
                /*
                    if (component != nullptr && component->pluginEditor != nullptr)
                        {
                            auto index = getParamIndexFromID (paramID);

                            if (index >= 0)
                            {
                                AudioProcessorEditorParameterControlHighlightInfo info;
                                info.parameterIndex  = index;
                                info.isHighlighted   = (isHighlighted != 0);
                                info.suggestedColour = getColourFromHighlightEnum (colour);

                                component->pluginEditor->setControlHighlight (info);
                            }

                            return AAX_SUCCESS;
                        }

                        return AAX_ERROR_NULL_OBJECT;
                */
            }
            
            pub fn get_win_32modifiers(&self) -> i32 {
                
                todo!();
                /*
                    int modifierFlags = 0;

                        if (auto* viewContainer = GetViewContainer())
                        {
                            uint32 aaxViewMods = 0;
                            const_cast<AAX_IViewContainer*> (viewContainer)->GetModifiers (&aaxViewMods);

                            if ((aaxViewMods & AAX_eModifiers_Shift) != 0) modifierFlags |= ModifierKeys::shiftModifier;
                            if ((aaxViewMods & AAX_eModifiers_Alt )  != 0) modifierFlags |= ModifierKeys::altModifier;
                        }

                        return modifierFlags;
                */
            }
            
            pub fn get_param_index_fromid(&self, paramid: AAX_CParamID) -> i32 {
                
                todo!();
                /*
                
                */
            }
            
            pub fn get_aax_param_id_from_aloe_index(&self, index: i32) -> AAX_CParamID {
                
                todo!();
                /*
                
                */
            }
            
            pub fn convert_to_host_bounds(plugin_size: AAX_Point) -> AAX_Point {
                
                todo!();
                /*
                    auto desktopScale = Desktop::getInstance().getGlobalScaleFactor();

                        if (approximatelyEqual (desktopScale, 1.0f))
                            return pluginSize;

                        return { pluginSize.vert * desktopScale,
                                 pluginSize.horz * desktopScale };
                */
            }
        }

        pub fn algorithm_process_callback(
                instances_begin: *mut &[ALOEAlgorithmContext],
                instances_end:   *const c_void)  {
            
            todo!();
                /*
                
                */
        }

        lazy_static!{
            /*
            static Vec<AloeAAX_Processor*> activeProcessors;
            */
        }

        #[no_copy]
        pub struct AloeAAX_Processor {
            base:                          AAX_CEffectParameters,
            base2:                         AudioPlayHead,
            base3:                         AudioProcessorListener,
            base4:                         AsyncUpdater,
            library_initialiser:           ScopedAloeInitialiser_GUI,
            plugin_instance:               Box<AudioProcessor>,
            is_prepared:                   bool, // default = false
            midi_buffer:                   MidiBuffer,
            channel_list:                  Vec<*mut f32>,
            aloe_chunk_index:              i32, // default = 0
            sample_rate:                   AAX_CSampleRate, // default = 0
            last_buffer_size:              i32, // default = 1024
            max_buffer_size:               i32, // default = 1024
            has_sidechain:                 bool, // default = false
            can_disable_sidechain:         bool, // default = false
            last_side_chain_state:         bool, // default = false
            processing_sidechain_change:   AtomicBool,
            sidechain_desired:             AtomicBool,
            side_chain_buffer:             HeapBlock<f32>,
            input_layout_map:              Vec<i32>,
            output_layout_map:             Vec<i32>,
            aax_param_ids:                 Vec<String>,
            param_map:                     HashMap<i32,*mut AudioProcessorParameter>,
            aloe_parameters:               LegacyAudioParametersWrapper,
            owned_bypass_parameter:        Box<AudioProcessorParameter>,
            aax_meters:                    Vec<*mut AudioProcessorParameter>,

            /**
              | temporary filter data is generated in GetChunkSize
              | and the size of the data returned. To avoid generating
              | it again in GetChunk, we need to store it somewhere.
              | However, as GetChunkSize and GetChunk can be called
              | on different threads, we store it in thread dependent storage
              | in a hash map with the thread id as a key.
              */
            per_thread_filter_data:        RefCell<ThreadLocalValue<ChunkMemoryBlock>>,

            per_thread_data_lock:          CriticalSection,
            in_parameter_changed_callback: ThreadLocalValue<bool>,
        }

        pub mod aloe_aax_processor {

            use super::*;

            pub struct ChunkMemoryBlock
            {
                data:     MemoryBlock,
                is_valid: bool,
            }
        }

        impl Default for AloeAAX_Processor {
            
            fn default() -> Self {
                todo!();
                /*

                    : pluginInstance (createPluginFilterOfType (AudioProcessor::wrapperType_AAX))

                        inParameterChangedCallback = false;

                        pluginInstance->setPlayHead (this);
                        pluginInstance->addListener (this);

                        rebuildChannelMapArrays();

                        AAX_CEffectParameters::GetNumberOfChunks (&aloeChunkIndex);
                        activeProcessors.add (this);
                */
            }
        }

        impl Drop for AloeAAX_Processor {
            fn drop(&mut self) {
                todo!();
                /*
                    activeProcessors.removeAllInstancesOf (this);
                */
            }
        }

        impl AloeAAX_Processor {

            pub fn create() -> *mut AAX_CEffectParameters {
                
                todo!();
                /*
                    PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_AAX;

                        if (PluginHostType::aloePlugInIsRunningInAudioSuiteFn == nullptr)
                        {
                            PluginHostType::aloePlugInIsRunningInAudioSuiteFn = [] (AudioProcessor& processor)
                            {
                                for (auto* p : activeProcessors)
                                    if (&p->getPluginInstance() == &processor)
                                        return p->isInAudioSuite();

                                return false;
                            };
                        }

                        return new AloeAAX_Processor();
                */
            }
            
            pub fn uninitialize(&mut self) -> AAX_Result {
                
                todo!();
                /*
                    cancelPendingUpdate();
                        aloeParameters.clear();

                        if (isPrepared && pluginInstance != nullptr)
                        {
                            isPrepared = false;
                            processingSidechainChange = false;

                            pluginInstance->releaseResources();
                        }

                        return AAX_CEffectParameters::Uninitialize();
                */
            }
            
            pub fn effect_init(&mut self) -> AAX_Result {
                
                todo!();
                /*
                    cancelPendingUpdate();
                        check (Controller()->GetSampleRate (&sampleRate));
                        processingSidechainChange = false;
                        auto err = preparePlugin();

                        if (err != AAX_SUCCESS)
                            return err;

                        addAudioProcessorParameters();

                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_number_of_chunks(&self, num_chunks: *mut i32) -> AAX_Result {
                
                todo!();
                /*
                    // The aloeChunk is the last chunk.
                        *numChunks = aloeChunkIndex + 1;
                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_chunk_id_from_index(&self, 
                index:   i32,
                chunkid: *mut AAX_CTypeID) -> AAX_Result {
                
                todo!();
                /*
                    if (index != aloeChunkIndex)
                            return AAX_CEffectParameters::GetChunkIDFromIndex (index, chunkID);

                        *chunkID = aloeChunkType;
                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_chunk_size(&self, 
                chunkid: AAX_CTypeID,
                o_size:  *mut u32) -> AAX_Result {
                
                todo!();
                /*
                    if (chunkID != aloeChunkType)
                            return AAX_CEffectParameters::GetChunkSize (chunkID, oSize);

                        auto& chunkMemoryBlock = perThreadFilterData.get();

                        chunkMemoryBlock.data.reset();
                        pluginInstance->getStateInformation (chunkMemoryBlock.data);
                        chunkMemoryBlock.isValid = true;

                        *oSize = (uint32_t) chunkMemoryBlock.data.getSize();
                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_chunk(&self, 
                chunkid: AAX_CTypeID,
                o_chunk: *mut AAX_SPlugInChunk) -> AAX_Result {
                
                todo!();
                /*
                    if (chunkID != aloeChunkType)
                            return AAX_CEffectParameters::GetChunk (chunkID, oChunk);

                        auto& chunkMemoryBlock = perThreadFilterData.get();

                        if (! chunkMemoryBlock.isValid)
                            return 20700; // AAX_ERROR_PLUGIN_API_INVALID_THREAD

                        oChunk->fSize = (int32_t) chunkMemoryBlock.data.getSize();
                        chunkMemoryBlock.data.copyTo (oChunk->fData, 0, chunkMemoryBlock.data.getSize());
                        chunkMemoryBlock.isValid = false;

                        return AAX_SUCCESS;
                */
            }
            
            pub fn set_chunk(&mut self, 
                chunkid: AAX_CTypeID,
                chunk:   *const AAX_SPlugInChunk) -> AAX_Result {
                
                todo!();
                /*
                    if (chunkID != aloeChunkType)
                            return AAX_CEffectParameters::SetChunk (chunkID, chunk);

                        pluginInstance->setStateInformation ((void*) chunk->fData, chunk->fSize);

                        // Notify Pro Tools that the parameters were updated.
                        // Without it a bug happens in these circumstances:
                        // * A preset is saved with the RTAS version of the plugin (".tfx" preset format).
                        // * The preset is loaded in PT 10 using the AAX version.
                        // * The session is then saved, and closed.
                        // * The saved session is loaded, but acting as if the preset was never loaded.
                        auto numParameters = aloeParameters.getNumParameters();

                        for (int i = 0; i < numParameters; ++i)
                            if (auto paramID = getAAXParamIDFromAloeIndex(i))
                                SetParameterNormalizedValue (paramID, aloeParameters.getParamForIndex (i)->getValue());

                        return AAX_SUCCESS;
                */
            }
            
            pub fn reset_field_data(&self, 
                field_index: AAX_CFieldIndex,
                data:        *mut c_void,
                data_size:   u32) -> AAX_Result {
                
                todo!();
                /*
                    switch (fieldIndex)
                        {
                            case ALOEAlgorithmIDs::pluginInstance:
                            {
                                auto numObjects = dataSize / sizeof (PluginInstanceInfo);
                                auto* objects = static_cast<PluginInstanceInfo*> (data);

                                jassert (numObjects == 1); // not sure how to handle more than one..

                                for (size_t i = 0; i < numObjects; ++i)
                                    new (objects + i) PluginInstanceInfo (const_cast<AloeAAX_Processor&> (*this));

                                break;
                            }

                            case ALOEAlgorithmIDs::preparedFlag:
                            {
                                const_cast<AloeAAX_Processor*>(this)->preparePlugin();

                                auto numObjects = dataSize / sizeof (uint32_t);
                                auto* objects = static_cast<uint32_t*> (data);

                                for (size_t i = 0; i < numObjects; ++i)
                                    objects[i] = 1;

                                break;
                            }

                            case ALOEAlgorithmIDs::meterTapBuffers:
                            {
                                // this is a dummy field only when there are no aaxMeters
                                jassert (aaxMeters.size() == 0);

                                {
                                    auto numObjects = dataSize / sizeof (float*);
                                    auto* objects = static_cast<float**> (data);

                                    for (size_t i = 0; i < numObjects; ++i)
                                        objects[i] = nullptr;
                                }
                                break;
                            }
                        }

                        return AAX_SUCCESS;
                */
            }
            
            pub fn set_audio_processor_parameter(&mut self, 
                paramid: AAX_CParamID,
                value:   f64)  {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                        {
                            auto newValue = static_cast<float> (value);

                            if (newValue != param->getValue())
                            {
                                param->setValue (newValue);

                                inParameterChangedCallback = true;
                                param->sendValueChangedMessageToListeners (newValue);
                            }
                        }
                */
            }
            
            pub fn update_parameter_normalized_value(&mut self, 
                paramid: AAX_CParamID,
                value:   f64,
                source:  AAX_EUpdateSource) -> AAX_Result {
                
                todo!();
                /*
                    auto result = AAX_CEffectParameters::UpdateParameterNormalizedValue (paramID, value, source);
                        setAudioProcessorParameter (paramID, value);

                        return result;
                */
            }
            
            pub fn get_parameter_value_from_string(&self, 
                paramid: AAX_CParamID,
                result:  *mut f64,
                text:    &AAX_IString) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                        {
                            if (! LegacyAudioParameter::isLegacy (param))
                            {
                                *result = param->getValueForText (text.Get());
                                return AAX_SUCCESS;
                            }
                        }

                        return AAX_CEffectParameters::GetParameterValueFromString (paramID, result, text);
                */
            }
            
            pub fn get_parameter_string_from_value(&self, 
                paramid: AAX_CParamID,
                value:   f64,
                result:  *mut AAX_IString,
                max_len: i32) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                            result->Set (param->getText ((float) value, maxLen).toRawUTF8());

                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_parameter_numberof_steps(&self, 
                paramid: AAX_CParamID,
                result:  *mut i32) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                            *result = getSafeNumberOfParameterSteps (*param);

                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_parameter_normalized_value(&self, 
                paramid: AAX_CParamID,
                result:  *mut f64) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                            *result = (double) param->getValue();
                        else
                            *result = 0.0;

                        return AAX_SUCCESS;
                */
            }
            
            pub fn set_parameter_normalized_value(&mut self, 
                paramid:   AAX_CParamID,
                new_value: f64) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* p = mParameterManager.GetParameterByID (paramID))
                            p->SetValueWithFloat ((float) newValue);

                        setAudioProcessorParameter (paramID, (float) newValue);

                        return AAX_SUCCESS;
                */
            }
            
            pub fn set_parameter_normalized_relative(&mut self, 
                paramid:         AAX_CParamID,
                new_delta_value: f64) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                        {
                            auto newValue = param->getValue() + (float) newDeltaValue;

                            setAudioProcessorParameter (paramID, jlimit (0.0f, 1.0f, newValue));

                            if (auto* p = mParameterManager.GetParameterByID (paramID))
                                p->SetValueWithFloat (newValue);
                        }

                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_parameter_name_of_length(&self, 
                paramid: AAX_CParamID,
                result:  *mut AAX_IString,
                max_len: i32) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                            result->Set (param->getName (maxLen).toRawUTF8());

                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_parameter_name(&self, 
                paramid: AAX_CParamID,
                result:  *mut AAX_IString) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                            result->Set (param->getName (31).toRawUTF8());

                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_parameter_default_normalized_value(&self, 
                paramid: AAX_CParamID,
                result:  *mut f64) -> AAX_Result {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                            *result = (double) param->getDefaultValue();
                        else
                            *result = 0.0;

                        jassert (*result >= 0 && *result <= 1.0f);

                        return AAX_SUCCESS;
                */
            }
            
            pub fn get_plugin_instance(&self) -> &mut AudioProcessor {
                
                todo!();
                /*
                    return *pluginInstance;
                */
            }
            
            pub fn get_current_position(&mut self, info: &mut AudioPlayHeadCurrentPositionInfo) -> bool {
                
                todo!();
                /*
                    const AAX_ITransport& transport = *Transport();

                        info.bpm = 0.0;
                        check (transport.GetCurrentTempo (&info.bpm));

                        int32_t num = 4, den = 4;
                        transport.GetCurrentMeter (&num, &den);
                        info.timeSigNumerator   = (int) num;
                        info.timeSigDenominator = (int) den;
                        info.timeInSamples = 0;

                        if (transport.IsTransportPlaying (&info.isPlaying) != AAX_SUCCESS)
                            info.isPlaying = false;

                        if (info.isPlaying
                             || transport.GetTimelineSelectionStartPosition (&info.timeInSamples) != AAX_SUCCESS)
                            check (transport.GetCurrentNativeSampleLocation (&info.timeInSamples));

                        info.timeInSeconds = (float) info.timeInSamples / sampleRate;

                        int64_t ticks = 0;

                        if (info.isPlaying)
                            check (transport.GetCustomTickPosition (&ticks, info.timeInSamples));
                        else
                            check (transport.GetCurrentTickPosition (&ticks));

                        info.ppqPosition = (double) ticks / 960000.0;

                        info.isLooping = false;
                        int64_t loopStartTick = 0, loopEndTick = 0;
                        check (transport.GetCurrentLoopPosition (&info.isLooping, &loopStartTick, &loopEndTick));
                        info.ppqLoopStart = (double) loopStartTick / 960000.0;
                        info.ppqLoopEnd   = (double) loopEndTick   / 960000.0;

                        info.editOriginTime = 0;
                        info.frameRate = AudioPlayHead::fpsUnknown;

                        AAX_EFrameRate frameRate;
                        int32_t offset;

                        if (transport.GetTimeCodeInfo (&frameRate, &offset) == AAX_SUCCESS)
                        {
                            double framesPerSec = 24.0;

                            switch (frameRate)
                            {
                                case AAX_eFrameRate_Undeclared:    break;
                                case AAX_eFrameRate_24Frame:       info.frameRate = AudioPlayHead::fps24;       break;
                                case AAX_eFrameRate_25Frame:       info.frameRate = AudioPlayHead::fps25;       framesPerSec = 25.0; break;
                                case AAX_eFrameRate_2997NonDrop:   info.frameRate = AudioPlayHead::fps2997;     framesPerSec = 30.0 * 1000.0 / 1001.0; break;
                                case AAX_eFrameRate_2997DropFrame: info.frameRate = AudioPlayHead::fps2997drop; framesPerSec = 30.0 * 1000.0 / 1001.0; break;
                                case AAX_eFrameRate_30NonDrop:     info.frameRate = AudioPlayHead::fps30;       framesPerSec = 30.0; break;
                                case AAX_eFrameRate_30DropFrame:   info.frameRate = AudioPlayHead::fps30drop;   framesPerSec = 30.0; break;
                                case AAX_eFrameRate_23976:         info.frameRate = AudioPlayHead::fps23976;    framesPerSec = 24.0 * 1000.0 / 1001.0; break;
                                default:                           break;
                            }

                            info.editOriginTime = offset / framesPerSec;
                        }

                        // No way to get these: (?)
                        info.isRecording = false;
                        info.ppqPositionOfLastBarStart = 0;

                        return true;
                */
            }
            
            pub fn audio_processor_parameter_changed(&mut self, 
                processor:       *mut AudioProcessor,
                parameter_index: i32,
                new_value:       f32)  {
                
                todo!();
                /*
                    if (inParameterChangedCallback.get())
                        {
                            inParameterChangedCallback = false;
                            return;
                        }

                        if (auto paramID = getAAXParamIDFromAloeIndex (parameterIndex))
                            SetParameterNormalizedValue (paramID, (double) newValue);
                */
            }
            
            pub fn audio_processor_changed(&mut self, 
                processor: *mut AudioProcessor,
                details:   &ChangeDetails)  {
                
                todo!();
                /*
                    ++mNumPlugInChanges;

                        if (details.parameterInfoChanged)
                        {
                            auto numParameters = aloeParameters.getNumParameters();

                            for (int i = 0; i < numParameters; ++i)
                            {
                                if (auto* p = mParameterManager.GetParameterByID (getAAXParamIDFromAloeIndex (i)))
                                {
                                    auto newName = aloeParameters.getParamForIndex (i)->getName (31);

                                    if (p->Name() != newName.toRawUTF8())
                                        p->SetName (AAX_CString (newName.toRawUTF8()));
                                }
                            }
                        }

                        if (details.latencyChanged)
                            check (Controller()->SetSignalLatency (processor->getLatencySamples()));
                */
            }
            
            pub fn audio_processor_parameter_change_gesture_begin(&mut self, 
                _0:              *mut AudioProcessor,
                parameter_index: i32)  {
                
                todo!();
                /*
                    if (auto paramID = getAAXParamIDFromAloeIndex (parameterIndex))
                            TouchParameter (paramID);
                */
            }
            
            pub fn audio_processor_parameter_change_gesture_end(&mut self, 
                _0:              *mut AudioProcessor,
                parameter_index: i32)  {
                
                todo!();
                /*
                    if (auto paramID = getAAXParamIDFromAloeIndex (parameterIndex))
                            ReleaseParameter (paramID);
                */
            }
            
            pub fn notification_received(&mut self, 
                ty:   AAX_CTypeID,
                data: *const c_void,
                size: u32) -> AAX_Result {
                
                todo!();
                /*
                    switch (type)
                        {
                            case AAX_eNotificationEvent_EnteringOfflineMode:  pluginInstance->setNonRealtime (true);  break;
                            case AAX_eNotificationEvent_ExitingOfflineMode:   pluginInstance->setNonRealtime (false); break;

                            case AAX_eNotificationEvent_ASProcessingState:
                            {
                                if (data != nullptr && size == sizeof (AAX_EProcessingState))
                                {
                                    const auto state = *static_cast<const AAX_EProcessingState*> (data);
                                    const auto nonRealtime = state == AAX_eProcessingState_Start
                                                          || state == AAX_eProcessingState_StartPass
                                                          || state == AAX_eProcessingState_BeginPassGroup;
                                    pluginInstance->setNonRealtime (nonRealtime);
                                }

                                break;
                            }

                            case AAX_eNotificationEvent_TrackNameChanged:
                                if (data != nullptr)
                                {
                                    AudioProcessor::TrackProperties props;
                                    props.name = String::fromUTF8 (static_cast<const AAX_IString*> (data)->Get());

                                    pluginInstance->updateTrackProperties (props);
                                }
                                break;

                            case AAX_eNotificationEvent_SideChainBeingConnected:
                            case AAX_eNotificationEvent_SideChainBeingDisconnected:
                            {
                                processingSidechainChange = true;
                                sidechainDesired = (type == AAX_eNotificationEvent_SideChainBeingConnected);
                                updateSidechainState();
                                break;
                            }
                        }

                        return AAX_CEffectParameters::NotificationReceived (type, data, size);
                */
            }
            
            pub fn get_audio_buffer_for_input(&self, 
                inputs:       *const *const f32,
                sidechain:    i32,
                main_num_ins: i32,
                idx:          i32) -> *const f32 {
                
                todo!();
                /*
                    jassert (idx < (mainNumIns + 1));

                        if (idx < mainNumIns)
                            return inputs[inputLayoutMap[idx]];

                        return (sidechain != -1 ? inputs[sidechain] : sideChainBuffer.getData());
                */
            }
            
            pub fn process(&mut self, 
                inputs:                *const *const f32,
                outputs:               *const *const f32,
                side_chain_buffer_idx: i32,
                buffer_size:           i32,
                bypass:                bool,
                midi_node_in:          *mut AAX_IMIDINode,
                midi_nodes_out:        *mut AAX_IMIDINode,
                meter_buffers:         *mut f32)  {
                
                todo!();
                /*
                    auto numIns    = pluginInstance->getTotalNumInputChannels();
                        auto numOuts   = pluginInstance->getTotalNumOutputChannels();
                        auto numMeters = aaxMeters.size();

                        const ScopedLock sl (pluginInstance->getCallbackLock());

                        bool isSuspended = [this, sideChainBufferIdx]
                        {
                            if (processingSidechainChange)
                                return true;

                            bool processWantsSidechain = (sideChainBufferIdx != -1);

                            if (hasSidechain && canDisableSidechain && (sidechainDesired != processWantsSidechain))
                            {
                                sidechainDesired = processWantsSidechain;
                                processingSidechainChange = true;
                                triggerAsyncUpdate();
                                return true;
                            }

                            return pluginInstance->isSuspended();
                        }();

                        if (isSuspended)
                        {
                            for (int i = 0; i < numOuts; ++i)
                                FloatVectorOperations::clear (outputs[i], bufferSize);

                            if (meterBuffers != nullptr)
                                FloatVectorOperations::clear (meterBuffers, numMeters);
                        }
                        else
                        {
                            auto mainNumIns = pluginInstance->getMainBusNumInputChannels();
                            auto sidechain = (pluginInstance->getChannelCountOfBus (true, 1) > 0 ? sideChainBufferIdx : -1);
                            auto numChans = jmax (numIns, numOuts);

                            if (numChans == 0)
                                return;

                            if (channelList.size() <= numChans)
                                channelList.insertMultiple (-1, nullptr, 1 + numChans - channelList.size());

                            float** channels = channelList.getRawDataPointer();

                            if (numOuts >= numIns)
                            {
                                for (int i = 0; i < numOuts; ++i)
                                    channels[i] = outputs[outputLayoutMap[i]];

                                for (int i = 0; i < numIns; ++i)
                                    memcpy (channels[i], getAudioBufferForInput (inputs, sidechain, mainNumIns, i), (size_t) bufferSize * sizeof (float));

                                for (int i = numIns; i < numOuts; ++i)
                                    zeromem (channels[i], (size_t) bufferSize * sizeof (float));

                                process (channels, numOuts, bufferSize, bypass, midiNodeIn, midiNodesOut);
                            }
                            else
                            {
                                for (int i = 0; i < numOuts; ++i)
                                    channels[i] = outputs[outputLayoutMap[i]];

                                for (int i = 0; i < numOuts; ++i)
                                    memcpy (channels[i], getAudioBufferForInput (inputs, sidechain, mainNumIns, i), (size_t) bufferSize * sizeof (float));

                                for (int i = numOuts; i < numIns; ++i)
                                    channels[i] = const_cast<float*> (getAudioBufferForInput (inputs, sidechain, mainNumIns, i));

                                process (channels, numIns, bufferSize, bypass, midiNodeIn, midiNodesOut);
                            }

                            if (meterBuffers != nullptr)
                                for (int i = 0; i < numMeters; ++i)
                                    meterBuffers[i] = aaxMeters[i]->getValue();
                        }
                */
            }
            
            /**
              | In aax, the format of the aux and
              | sidechain buses need to be fully
              | determined by the format on the main
              | buses. This function tried to provide
              | such a mapping.
              |
              | Returns false if the in/out main layout
              | is not supported
              */
            pub fn full_buses_layout_from_main_layout(
                p:           &AudioProcessor,
                main_input:  &AudioChannelSet,
                main_output: &AudioChannelSet,
                full_layout: &mut AudioProcessor::BusesLayout) -> bool {
                
                todo!();
                /*
                    auto currentLayout = getDefaultLayout (p, true);
                        bool success = p.checkBusesLayoutSupported (currentLayout);
                        jassertquiet (success);

                        auto numInputBuses  = p.getBusCount (true);
                        auto numOutputBuses = p.getBusCount (false);

                        if (auto* bus = p.getBus (true, 0))
                            if (! bus->isLayoutSupported (mainInput, &currentLayout))
                                return false;

                        if (auto* bus = p.getBus (false, 0))
                            if (! bus->isLayoutSupported (mainOutput, &currentLayout))
                                return false;

                        // did this change the input again
                        if (numInputBuses > 0 && currentLayout.inputBuses.getReference (0) != mainInput)
                            return false;

                       #ifdef AloePlugin_PreferredChannelConfigurations
                        short configs[][2] = { AloePlugin_PreferredChannelConfigurations };

                        if (! AudioProcessor::containsLayout (currentLayout, configs))
                            return false;
                       #endif

                        bool foundValid = false;
                        {
                            auto onlyMains = currentLayout;

                            for (int i = 1; i < numInputBuses; ++i)
                                onlyMains.inputBuses.getReference  (i) = AudioChannelSet::disabled();

                            for (int i = 1; i < numOutputBuses; ++i)
                                onlyMains.outputBuses.getReference (i) = AudioChannelSet::disabled();

                            if (p.checkBusesLayoutSupported (onlyMains))
                            {
                                foundValid = true;
                                fullLayout = onlyMains;
                            }
                        }

                        if (numInputBuses > 1)
                        {
                            // can the first bus be a sidechain or disabled, if not then we can't use this layout combination
                            if (auto* bus = p.getBus (true, 1))
                                if (! bus->isLayoutSupported (AudioChannelSet::mono(), &currentLayout) && ! bus->isLayoutSupported (AudioChannelSet::disabled(), &currentLayout))
                                    return foundValid;

                            // can all the other inputs be disabled, if not then we can't use this layout combination
                            for (int i = 2; i < numInputBuses; ++i)
                                if (auto* bus = p.getBus (true, i))
                                    if (! bus->isLayoutSupported (AudioChannelSet::disabled(), &currentLayout))
                                        return foundValid;

                            if (auto* bus = p.getBus (true, 0))
                                if (! bus->isLayoutSupported (mainInput, &currentLayout))
                                    return foundValid;

                            if (auto* bus = p.getBus (false, 0))
                                if (! bus->isLayoutSupported (mainOutput, &currentLayout))
                                    return foundValid;

                            // recheck if the format is correct
                            if ((numInputBuses  > 0 && currentLayout.inputBuses .getReference (0) != mainInput)
                             || (numOutputBuses > 0 && currentLayout.outputBuses.getReference (0) != mainOutput))
                                return foundValid;

                            auto& sidechainBus = currentLayout.inputBuses.getReference (1);

                            if (sidechainBus != AudioChannelSet::mono() && sidechainBus != AudioChannelSet::disabled())
                                return foundValid;

                            for (int i = 2; i < numInputBuses; ++i)
                                if (! currentLayout.inputBuses.getReference (i).isDisabled())
                                    return foundValid;
                        }

                        const bool hasSidechain = (numInputBuses > 1 && currentLayout.inputBuses.getReference (1) == AudioChannelSet::mono());

                        if (hasSidechain)
                        {
                            auto onlyMainsAndSidechain = currentLayout;

                            for (int i = 1; i < numOutputBuses; ++i)
                                onlyMainsAndSidechain.outputBuses.getReference (i) = AudioChannelSet::disabled();

                            if (p.checkBusesLayoutSupported (onlyMainsAndSidechain))
                            {
                                foundValid = true;
                                fullLayout = onlyMainsAndSidechain;
                            }
                        }

                        if (numOutputBuses > 1)
                        {
                            auto copy = currentLayout;
                            int maxAuxBuses = jmin (16, numOutputBuses);

                            for (int i = 1; i < maxAuxBuses; ++i)
                                copy.outputBuses.getReference (i) = mainOutput;

                            for (int i = maxAuxBuses; i < numOutputBuses; ++i)
                                copy.outputBuses.getReference (i) = AudioChannelSet::disabled();

                            if (p.checkBusesLayoutSupported (copy))
                            {
                                fullLayout = copy;
                                foundValid = true;
                            }
                            else
                            {
                                for (int i = 1; i < maxAuxBuses; ++i)
                                    if (currentLayout.outputBuses.getReference (i).isDisabled())
                                        return foundValid;

                                for (int i = maxAuxBuses; i < numOutputBuses; ++i)
                                    if (auto* bus = p.getBus (false, i))
                                        if (! bus->isLayoutSupported (AudioChannelSet::disabled(), &currentLayout))
                                            return foundValid;

                                if (auto* bus = p.getBus (true, 0))
                                    if (! bus->isLayoutSupported (mainInput, &currentLayout))
                                        return foundValid;

                                if (auto* bus = p.getBus (false, 0))
                                    if (! bus->isLayoutSupported (mainOutput, &currentLayout))
                                        return foundValid;

                                if ((numInputBuses  > 0 && currentLayout.inputBuses .getReference (0) != mainInput)
                                 || (numOutputBuses > 0 && currentLayout.outputBuses.getReference (0) != mainOutput))
                                    return foundValid;

                                if (numInputBuses > 1)
                                {
                                    auto& sidechainBus = currentLayout.inputBuses.getReference (1);

                                    if (sidechainBus != AudioChannelSet::mono() && sidechainBus != AudioChannelSet::disabled())
                                        return foundValid;
                                }

                                for (int i = maxAuxBuses; i < numOutputBuses; ++i)
                                    if (! currentLayout.outputBuses.getReference (i).isDisabled())
                                        return foundValid;

                                fullLayout = currentLayout;
                                foundValid = true;
                            }
                        }

                        return foundValid;
                */
            }
            
            pub fn is_in_audio_suite(&mut self) -> bool {
                
                todo!();
                /*
                    AAX_CBoolean res;
                        Controller()->GetIsAudioSuite (&res);

                        return res > 0;
                */
            }
            
            pub fn process(&mut self, 
                channels:       *const *const f32,
                num_chans:      i32,
                buffer_size:    i32,
                bypass:         bool,
                midi_node_in:   *mut AAX_IMIDINode,
                midi_nodes_out: *mut AAX_IMIDINode)  {
                
                todo!();
                /*
                    AudioBuffer<float> buffer (channels, numChans, bufferSize);
                        midiBuffer.clear();
                        ignoreUnused (midiNodeIn, midiNodesOut);

                       #if AloePlugin_WantsMidiInput || AloePlugin_IsMidiEffect
                        {
                            auto* midiStream = midiNodeIn->GetNodeBuffer();
                            auto numMidiEvents = midiStream->mBufferSize;

                            for (uint32_t i = 0; i < numMidiEvents; ++i)
                            {
                                auto& m = midiStream->mBuffer[i];
                                jassert ((int) m.mTimestamp < bufferSize);

                                midiBuffer.addEvent (m.mData, (int) m.mLength,
                                                     jlimit (0, (int) bufferSize - 1, (int) m.mTimestamp));
                            }
                        }
                       #endif

                        {
                            if (lastBufferSize != bufferSize)
                            {
                                lastBufferSize = bufferSize;
                                pluginInstance->setRateAndBufferSizeDetails (sampleRate, bufferSize);

                                if (bufferSize > maxBufferSize)
                                {
                                    // we only call prepareToPlay here if the new buffer size is larger than
                                    // the one used last time prepareToPlay was called.
                                    // currently, this should never actually happen, because as of Pro Tools 12,
                                    // the maximum possible value is 1024, and we call prepareToPlay with that
                                    // value during initialisation.
                                    pluginInstance->prepareToPlay (sampleRate, bufferSize);
                                    maxBufferSize = bufferSize;
                                    sideChainBuffer.calloc (static_cast<size_t> (maxBufferSize));
                                }
                            }

                            if (bypass)
                                pluginInstance->processBlockBypassed (buffer, midiBuffer);
                            else
                                pluginInstance->processBlock (buffer, midiBuffer);
                        }

                       #if AloePlugin_ProducesMidiOutput || AloePlugin_IsMidiEffect
                        {
                            AAX_CMidiPacket packet;
                            packet.mIsImmediate = false;

                            for (const auto metadata : midiBuffer)
                            {
                                jassert (isPositiveAndBelow (metadata.samplePosition, bufferSize));

                                if (metadata.numBytes <= 4)
                                {
                                    packet.mTimestamp   = (uint32_t) metadata.samplePosition;
                                    packet.mLength      = (uint32_t) metadata.numBytes;
                                    memcpy (packet.mData, metadata.data, (size_t) metadata.numBytes);

                                    check (midiNodesOut->PostMIDIPacket (&packet));
                                }
                            }
                        }
                       #endif
                */
            }
            
            pub fn is_bypass_part_of_regular_paremeters(&self) -> bool {
                
                todo!();
                /*
                    auto& audioProcessor = getPluginInstance();

                        int n = aloeParameters.getNumParameters();

                        if (auto* bypassParam = audioProcessor.getBypassParameter())
                            for (int i = 0; i < n; ++i)
                                if (aloeParameters.getParamForIndex (i) == bypassParam)
                                    return true;

                        return false;
                */
            }

            /**
              | Some older Pro Tools control surfaces
              | (EUCON [PT version 12.4] and Avid S6
              | before version 2.1) cannot cope with
              | a large number of parameter steps.
              */
            pub fn get_safe_number_of_parameter_steps(param: &AudioProcessorParameter) -> i32 {
                
                todo!();
                /*
                    return jmin (param.getNumSteps(), 2048);
                */
            }
            
            pub fn add_audio_processor_parameters(&mut self)  {
                
                todo!();
                /*
                    auto& audioProcessor = getPluginInstance();

                       #if ALOE_FORCE_USE_LEGACY_PARAM_IDS
                        const bool forceLegacyParamIDs = true;
                       #else
                        const bool forceLegacyParamIDs = false;
                       #endif

                        auto bypassPartOfRegularParams = isBypassPartOfRegularParemeters();

                        aloeParameters.update (audioProcessor, forceLegacyParamIDs);

                        auto* bypassParameter = pluginInstance->getBypassParameter();

                        if (bypassParameter == nullptr)
                        {
                            ownedBypassParameter.reset (new AudioParameterBool (cDefaultMasterBypassID, "Master Bypass", false, {}, {}, {}));
                            bypassParameter = ownedBypassParameter.get();
                        }

                        if (! bypassPartOfRegularParams)
                            aloeParameters.params.add (bypassParameter);

                        int parameterIndex = 0;

                        for (auto* aloeParam : aloeParameters.params)
                        {
                            auto isBypassParameter = (aloeParam == bypassParameter);

                            auto category = aloeParam->getCategory();
                            auto paramID  = isBypassParameter ? String (cDefaultMasterBypassID)
                                                              : aloeParameters.getParamID (audioProcessor, parameterIndex);

                            aaxParamIDs.add (paramID);
                            auto* aaxParamID = aaxParamIDs.getReference (parameterIndex++).toRawUTF8();

                            paramMap.set (AAXClasses::getAAXParamHash (aaxParamID), aloeParam);

                            // is this a meter?
                            if (((category & 0xffff0000) >> 16) == 2)
                            {
                                aaxMeters.add (aloeParam);
                                continue;
                            }

                            auto parameter = new AAX_CParameter<float> (aaxParamID,
                                                                        AAX_CString (aloeParam->getName (31).toRawUTF8()),
                                                                        aloeParam->getDefaultValue(),
                                                                        AAX_CLinearTaperDelegate<float, 0>(),
                                                                        AAX_CNumberDisplayDelegate<float, 3>(),
                                                                        aloeParam->isAutomatable());

                            parameter->AddShortenedName (aloeParam->getName (4).toRawUTF8());

                            auto parameterNumSteps = getSafeNumberOfParameterSteps (*aloeParam);
                            parameter->SetNumberOfSteps ((uint32_t) parameterNumSteps);

                           #if ALOE_FORCE_LEGACY_PARAMETER_AUTOMATION_TYPE
                            parameter->SetType (parameterNumSteps > 1000 ? AAX_eParameterType_Continuous
                                                                         : AAX_eParameterType_Discrete);
                           #else
                            parameter->SetType (aloeParam->isDiscrete() ? AAX_eParameterType_Discrete
                                                                        : AAX_eParameterType_Continuous);
                           #endif

                            parameter->SetOrientation (aloeParam->isOrientationInverted()
                                                        ? (AAX_eParameterOrientation_RightMinLeftMax | AAX_eParameterOrientation_TopMinBottomMax
                                                            | AAX_eParameterOrientation_RotarySingleDotMode | AAX_eParameterOrientation_RotaryRightMinLeftMax)
                                                        : (AAX_eParameterOrientation_LeftMinRightMax | AAX_eParameterOrientation_BottomMinTopMax
                                                            | AAX_eParameterOrientation_RotarySingleDotMode | AAX_eParameterOrientation_RotaryLeftMinRightMax));

                            mParameterManager.AddParameter (parameter);

                            if (isBypassParameter)
                                mPacketDispatcher.RegisterPacket (aaxParamID, ALOEAlgorithmIDs::bypass);
                        }
                */
            }
            
            pub fn get_main_bus_formats(&mut self, 
                input_set:  &mut AudioChannelSet,
                output_set: &mut AudioChannelSet) -> bool {
                
                todo!();
                /*
                    auto& audioProcessor = getPluginInstance();

                       #if AloePlugin_IsMidiEffect
                        // MIDI effect plug-ins do not support any audio channels
                        jassert (audioProcessor.getTotalNumInputChannels()  == 0
                              && audioProcessor.getTotalNumOutputChannels() == 0);

                        inputSet = outputSet = AudioChannelSet();
                        return true;
                       #else
                        auto inputBuses  = audioProcessor.getBusCount (true);
                        auto outputBuses = audioProcessor.getBusCount (false);

                        AAX_EStemFormat inputStemFormat = AAX_eStemFormat_None;
                        check (Controller()->GetInputStemFormat (&inputStemFormat));

                        AAX_EStemFormat outputStemFormat = AAX_eStemFormat_None;
                        check (Controller()->GetOutputStemFormat (&outputStemFormat));

                        #if AloePlugin_IsSynth
                         if (inputBuses == 0)
                             inputStemFormat = AAX_eStemFormat_None;
                        #endif

                        inputSet  = (inputBuses  > 0 ? channelSetFromStemFormat (inputStemFormat,  false) : AudioChannelSet());
                        outputSet = (outputBuses > 0 ? channelSetFromStemFormat (outputStemFormat, false) : AudioChannelSet());

                        if ((inputSet == AudioChannelSet::disabled() && inputStemFormat != AAX_eStemFormat_None) || (outputSet == AudioChannelSet::disabled() && outputStemFormat != AAX_eStemFormat_None)
                            || (inputSet != AudioChannelSet::disabled() && inputBuses == 0) || (outputSet != AudioChannelSet::disabled() && outputBuses == 0))
                            return false;

                        return true;
                       #endif
                */
            }
            
            pub fn prepare_plugin(&mut self) -> AAX_Result {
                
                todo!();
                /*
                    auto& audioProcessor = getPluginInstance();
                        auto oldLayout = audioProcessor.getBusesLayout();
                        AudioChannelSet inputSet, outputSet;

                        if (! getMainBusFormats (inputSet, outputSet))
                        {
                            if (isPrepared)
                            {
                                isPrepared = false;
                                audioProcessor.releaseResources();
                            }

                            return AAX_ERROR_UNIMPLEMENTED;
                        }

                        AudioProcessor::BusesLayout newLayout;

                        if (! fullBusesLayoutFromMainLayout (audioProcessor, inputSet, outputSet, newLayout))
                        {
                            if (isPrepared)
                            {
                                isPrepared = false;
                                audioProcessor.releaseResources();
                            }

                            return AAX_ERROR_UNIMPLEMENTED;
                        }

                        hasSidechain = (newLayout.getNumChannels (true, 1) == 1);

                        if (hasSidechain)
                        {
                            sidechainDesired = true;

                            auto disabledSidechainLayout = newLayout;
                            disabledSidechainLayout.inputBuses.getReference (1) = AudioChannelSet::disabled();

                            canDisableSidechain = audioProcessor.checkBusesLayoutSupported (disabledSidechainLayout);

                            if (canDisableSidechain && ! lastSideChainState)
                            {
                                sidechainDesired = false;
                                newLayout = disabledSidechainLayout;
                            }
                        }

                        if (isInAudioSuite())
                        {
                            // AudioSuite doesn't support multiple output buses
                            for (int i = 1; i < newLayout.outputBuses.size(); ++i)
                                newLayout.outputBuses.getReference (i) = AudioChannelSet::disabled();

                            if (! audioProcessor.checkBusesLayoutSupported (newLayout))
                            {
                                // your plug-in needs to support a single output bus if running in AudioSuite
                                jassertfalse;

                                if (isPrepared)
                                {
                                    isPrepared = false;
                                    audioProcessor.releaseResources();
                                }

                                return AAX_ERROR_UNIMPLEMENTED;
                            }
                        }

                        const bool layoutChanged = (oldLayout != newLayout);

                        if (layoutChanged)
                        {
                            if (! audioProcessor.setBusesLayout (newLayout))
                            {
                                if (isPrepared)
                                {
                                    isPrepared = false;
                                    audioProcessor.releaseResources();
                                }

                                return AAX_ERROR_UNIMPLEMENTED;
                            }

                            rebuildChannelMapArrays();
                        }

                        if (layoutChanged || (! isPrepared))
                        {
                            if (isPrepared)
                            {
                                isPrepared = false;
                                audioProcessor.releaseResources();
                            }

                            audioProcessor.setRateAndBufferSizeDetails (sampleRate, lastBufferSize);
                            audioProcessor.prepareToPlay (sampleRate, lastBufferSize);
                            maxBufferSize = lastBufferSize;

                            midiBuffer.ensureSize (2048);
                            midiBuffer.clear();

                            sideChainBuffer.calloc (static_cast<size_t> (maxBufferSize));
                        }

                        check (Controller()->SetSignalLatency (audioProcessor.getLatencySamples()));
                        isPrepared = true;

                        return AAX_SUCCESS;
                */
            }
            
            pub fn rebuild_channel_map_arrays(&mut self)  {
                
                todo!();
                /*
                    auto& audioProcessor = getPluginInstance();

                        for (int dir = 0; dir < 2; ++dir)
                        {
                            bool isInput = (dir == 0);
                            auto& layoutMap = isInput ? inputLayoutMap : outputLayoutMap;
                            layoutMap.clear();

                            auto numBuses = audioProcessor.getBusCount (isInput);
                            int chOffset = 0;

                            for (int busIdx = 0; busIdx < numBuses; ++busIdx)
                            {
                                auto channelFormat = audioProcessor.getChannelLayoutOfBus (isInput, busIdx);

                                if (channelFormat != AudioChannelSet::disabled())
                                {
                                    auto numChannels = channelFormat.size();

                                    for (int ch = 0; ch < numChannels; ++ch)
                                        layoutMap.add (aloeChannelIndexToAax (ch, channelFormat) + chOffset);

                                    chOffset += numChannels;
                                }
                            }
                        }
                */
            }
            
            pub fn algorithm_callback(
                instances_begin: *mut &[ALOEAlgorithmContext],
                instances_end:   *const c_void)  {
                
                todo!();
                /*
                    for (auto iter = instancesBegin; iter < instancesEnd; ++iter)
                        {
                            auto& i = **iter;

                            int sideChainBufferIdx = i.pluginInstance->parameters.hasSidechain && i.sideChainBuffers != nullptr
                                                         ? static_cast<int> (*i.sideChainBuffers) : -1;

                            // sidechain index of zero is an invalid index
                            if (sideChainBufferIdx <= 0)
                                sideChainBufferIdx = -1;

                            auto numMeters = i.pluginInstance->parameters.aaxMeters.size();
                            float* const meterTapBuffers = (i.meterTapBuffers != nullptr && numMeters > 0 ? *i.meterTapBuffers : nullptr);

                            i.pluginInstance->parameters.process (i.inputChannels, i.outputChannels, sideChainBufferIdx,
                                                                  *(i.bufferSize), *(i.bypass) != 0,
                                                                  getMidiNodeIn(i), getMidiNodeOut(i),
                                                                  meterTapBuffers);
                        }
                */
            }
            
            pub fn update_sidechain_state(&mut self)  {
                
                todo!();
                /*
                    if (! processingSidechainChange)
                            return;

                        auto& audioProcessor = getPluginInstance();
                        bool sidechainActual = audioProcessor.getChannelCountOfBus (true, 1) > 0;

                        if (hasSidechain && canDisableSidechain && sidechainDesired != sidechainActual)
                        {
                            lastSideChainState = sidechainDesired;

                            if (isPrepared)
                            {
                                isPrepared = false;
                                audioProcessor.releaseResources();
                            }

                            if (auto* bus = audioProcessor.getBus (true, 1))
                                bus->setCurrentLayout (lastSideChainState ? AudioChannelSet::mono()
                                                                          : AudioChannelSet::disabled());

                            audioProcessor.prepareToPlay (audioProcessor.getSampleRate(), audioProcessor.getBlockSize());
                            isPrepared = true;
                        }

                        processingSidechainChange = false;
                */
            }
            
            pub fn handle_async_update(&mut self)  {
                
                todo!();
                /*
                    updateSidechainState();
                */
            }
            
            pub fn aax_curve_type_toaloe(ty: AAX_CTypeID) -> AudioProcessor::CurveData::Type {
                
                todo!();
                /*
                    switch (type)
                        {
                        case AAX_eCurveType_EQ:              return AudioProcessor::CurveData::Type::EQ;
                        case AAX_eCurveType_Dynamics:        return AudioProcessor::CurveData::Type::Dynamics;
                        case AAX_eCurveType_Reduction:       return AudioProcessor::CurveData::Type::GainReduction;
                        default:  break;
                        }

                        return AudioProcessor::CurveData::Type::Unknown;
                */
            }
            
            pub fn get_aax_meter_id_for_param_id(&self, paramid: &String) -> u32 {
                
                todo!();
                /*
                    int idx;

                        for (idx = 0; idx < aaxMeters.size(); ++idx)
                            if (LegacyAudioParameter::getParamID (aaxMeters[idx], false) == paramID)
                                break;

                        // you specified a parameter id in your curve but the parameter does not have the meter
                        // category
                        jassert (idx < aaxMeters.size());
                        return 'Metr' + static_cast<AAX_CTypeID> (idx);
                */
            }
            
            pub fn get_curve_data(&self, 
                curve_type: AAX_CTypeID,
                values:     *const f32,
                num_values: u32,
                o_values:   *mut f32) -> AAX_Result {
                
                todo!();
                /*
                    auto curveType = aaxCurveTypeToALOE (iCurveType);

                        if (curveType != AudioProcessor::CurveData::Type::Unknown)
                        {
                            auto& audioProcessor = getPluginInstance();
                            auto curve = audioProcessor.getResponseCurve (curveType);

                            if (curve.curve)
                            {
                                if (oValues != nullptr && iValues != nullptr)
                                {
                                    for (uint32_t i = 0; i < iNumValues; ++i)
                                        oValues[i] = curve.curve (iValues[i]);
                                }

                                return AAX_SUCCESS;
                            }
                        }

                        return AAX_ERROR_UNIMPLEMENTED;
                */
            }
            
            pub fn get_curve_data_meter_ids(&self, 
                curve_type:  AAX_CTypeID,
                o_xmeter_id: *mut u32,
                o_ymeter_id: *mut u32) -> AAX_Result {
                
                todo!();
                /*
                    auto curveType = aaxCurveTypeToALOE (iCurveType);

                        if (curveType != AudioProcessor::CurveData::Type::Unknown)
                        {
                            auto& audioProcessor = getPluginInstance();
                            auto curve = audioProcessor.getResponseCurve (curveType);

                            if (curve.curve && curve.xMeterID.isNotEmpty() && curve.yMeterID.isNotEmpty())
                            {
                                if (oXMeterId != nullptr) *oXMeterId = getAAXMeterIdForParamId (curve.xMeterID);
                                if (oYMeterId != nullptr) *oYMeterId = getAAXMeterIdForParamId (curve.yMeterID);

                                return AAX_SUCCESS;
                            }
                        }

                        return AAX_ERROR_UNIMPLEMENTED;
                */
            }
            
            pub fn get_curve_data_display_range(&self, 
                curve_type: AAX_CTypeID,
                o_xmin:     *mut f32,
                o_xmax:     *mut f32,
                o_ymin:     *mut f32,
                o_ymax:     *mut f32) -> AAX_Result {
                
                todo!();
                /*
                    auto curveType = aaxCurveTypeToALOE (iCurveType);

                        if (curveType != AudioProcessor::CurveData::Type::Unknown)
                        {
                            auto& audioProcessor = getPluginInstance();
                            auto curve = audioProcessor.getResponseCurve (curveType);

                            if (curve.curve)
                            {
                                if (oXMin != nullptr) *oXMin = curve.xRange.getStart();
                                if (oXMax != nullptr) *oXMax = curve.xRange.getEnd();
                                if (oYMin != nullptr) *oYMin = curve.yRange.getStart();
                                if (oYMax != nullptr) *oYMax = curve.yRange.getEnd();

                                return AAX_SUCCESS;
                            }
                        }

                        return AAX_ERROR_UNIMPLEMENTED;
                */
            }
            
            #[inline] pub fn get_param_index_fromid(&self, paramid: AAX_CParamID) -> i32 {
                
                todo!();
                /*
                    if (auto* param = getParameterFromID (paramID))
                            return LegacyAudioParameter::getParamIndex (getPluginInstance(), param);

                        return -1;
                */
            }
            
            #[inline] pub fn get_aax_param_id_from_aloe_index(&self, index: i32) -> AAX_CParamID {
                
                todo!();
                /*
                    if (isPositiveAndBelow (index, aaxParamIDs.size()))
                            return aaxParamIDs.getReference (index).toRawUTF8();

                        return nullptr;
                */
            }
            
            pub fn get_parameter_fromid(&self, paramid: AAX_CParamID) -> *mut AudioProcessorParameter {
                
                todo!();
                /*
                    return paramMap [AAXClasses::getAAXParamHash (paramID)];
                */
            }
            
            pub fn get_default_layout(
                p:          &AudioProcessor,
                enable_all: bool) -> AudioProcessor::BusesLayout {
                
                todo!();
                /*
                    AudioProcessor::BusesLayout defaultLayout;

                        for (int dir = 0; dir < 2; ++dir)
                        {
                            bool isInput = (dir == 0);
                            auto numBuses = p.getBusCount (isInput);
                            auto& layouts = (isInput ? defaultLayout.inputBuses : defaultLayout.outputBuses);

                            for (int i = 0; i < numBuses; ++i)
                                if (auto* bus = p.getBus (isInput, i))
                                    layouts.add (enableAll || bus->isEnabledByDefault() ? bus->getDefaultLayout() : AudioChannelSet());
                        }

                        return defaultLayout;
                */
            }
            
            pub fn get_default_layout(p: &mut AudioProcessor) -> AudioProcessor::BusesLayout {
                
                todo!();
                /*
                    auto defaultLayout = getDefaultLayout (p, true);

                        if (! p.checkBusesLayoutSupported (defaultLayout))
                            defaultLayout = getDefaultLayout (p, false);

                        // Your processor must support the default layout
                        jassert (p.checkBusesLayoutSupported (defaultLayout));
                        return defaultLayout;
                */
            }
        }

        impl AloeAAX_GUI  {

            pub fn create_view_contents(&mut self)  {
                
                todo!();
                /*
                    if (component == nullptr)
                    {
                        if (auto* params = dynamic_cast<AloeAAX_Processor*> (GetEffectParameters()))
                            component.reset (new ContentWrapperComponent (*this, params->getPluginInstance()));
                        else
                            jassertfalse;
                    }
                */
            }
            
            pub fn get_param_index_fromid(&self, paramid: AAX_CParamID) -> i32 {
                
                todo!();
                /*
                    if (auto* params = dynamic_cast<const AloeAAX_Processor*> (GetEffectParameters()))
                        return params->getParamIndexFromID (paramID);

                    return -1;
                */
            }
            
            pub fn get_aax_param_id_from_aloe_index(&self, index: i32) -> AAX_CParamID {
                
                todo!();
                /*
                    if (auto* params = dynamic_cast<const AloeAAX_Processor*> (GetEffectParameters()))
                        return params->getAAXParamIDFromAloeIndex (index);

                    return nullptr;
                */
            }
        }
        
        #[derive(Default)]
        pub struct AAXFormatConfiguration {
            input_format:  AAX_EStemFormat, // default = AAX_eStemFormat_None
            output_format: AAX_EStemFormat, // default = AAX_eStemFormat_None
        }

        impl AAXFormatConfiguration {
            
            pub fn new(
                in_format:  AAX_EStemFormat,
                out_format: AAX_EStemFormat) -> Self {
            
                todo!();
                /*
                : input_format(inFormat),
                : output_format(outFormat),

                
                */
            }
        }

        impl PartialEq<AAXFormatConfiguration> for AAXFormatConfiguration {
            
            #[inline] fn eq(&self, other: &AAXFormatConfiguration) -> bool {
                todo!();
                /*
                    return inputFormat == other.inputFormat && outputFormat == other.outputFormat;
                */
            }
        }

        impl Eq for AAXFormatConfiguration {}

        impl Ord<AAXFormatConfiguration> for AAXFormatConfiguration {
            
            #[inline] fn cmp(&self, other: &AAXFormatConfiguration) -> Ordering {
                todo!();
                /*
                    return inputFormat == other.inputFormat ? (outputFormat < other.outputFormat)
                                                                : (inputFormat  < other.inputFormat);
                */
            }
        }

        impl PartialOrd<AAXFormatConfiguration> for AAXFormatConfiguration {
            #[inline] fn partial_cmp(&self, other: &AAXFormatConfiguration) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        pub fn add_aax_meters(
                p:          &mut AudioProcessor,
                descriptor: &mut AAX_IEffectDescriptor) -> i32 {
            
            todo!();
                /*
                    LegacyAudioParametersWrapper params;

                   #if ALOE_FORCE_USE_LEGACY_PARAM_IDS
                    const bool forceLegacyParamIDs = true;
                   #else
                    const bool forceLegacyParamIDs = false;
                   #endif

                    params.update (p, forceLegacyParamIDs);

                    int meterIdx = 0;

                    for (auto* param : params.params)
                    {
                        auto category = param->getCategory();

                        // is this a meter?
                        if (((category & 0xffff0000) >> 16) == 2)
                        {
                            if (auto* meterProperties = descriptor.NewPropertyMap())
                            {
                                meterProperties->AddProperty (AAX_eProperty_Meter_Type,        getMeterTypeForCategory (category));
                                meterProperties->AddProperty (AAX_eProperty_Meter_Orientation, AAX_eMeterOrientation_TopRight);

                                descriptor.AddMeterDescription ('Metr' + static_cast<AAX_CTypeID> (meterIdx++),
                                                                param->getName (1024).toRawUTF8(), meterProperties);
                            }
                        }
                    }

                    return meterIdx;
                */
        }

        pub fn create_descriptor(
                desc:        &mut AAX_IComponentDescriptor,
                full_layout: &AudioProcessor::BusesLayout,
                processor:   &mut AudioProcessor,
                plugin_ids:  &mut Vec<i32>,
                num_meters:  i32)  {
            
            todo!();
                /*
                    auto aaxInputFormat  = getFormatForAudioChannelSet (fullLayout.getMainInputChannelSet(),  false);
                    auto aaxOutputFormat = getFormatForAudioChannelSet (fullLayout.getMainOutputChannelSet(), false);

                   #if AloePlugin_IsSynth
                    if (aaxInputFormat == AAX_eStemFormat_None)
                        aaxInputFormat = aaxOutputFormat;
                   #endif

                   #if AloePlugin_IsMidiEffect
                    aaxInputFormat = aaxOutputFormat = AAX_eStemFormat_Mono;
                   #endif

                    check (desc.AddAudioIn  (ALOEAlgorithmIDs::inputChannels));
                    check (desc.AddAudioOut (ALOEAlgorithmIDs::outputChannels));

                    check (desc.AddAudioBufferLength (ALOEAlgorithmIDs::bufferSize));
                    check (desc.AddDataInPort (ALOEAlgorithmIDs::bypass, sizeof (int32_t)));

                   #if AloePlugin_WantsMidiInput || AloePlugin_IsMidiEffect
                    check (desc.AddMIDINode (ALOEAlgorithmIDs::midiNodeIn, AAX_eMIDINodeType_LocalInput,
                                             AloePlugin_Name, 0xffff));
                   #endif

                   #if AloePlugin_ProducesMidiOutput || AloePlugin_IsSynth || AloePlugin_IsMidiEffect
                    check (desc.AddMIDINode (ALOEAlgorithmIDs::midiNodeOut, AAX_eMIDINodeType_LocalOutput,
                                             AloePlugin_Name " Out", 0xffff));
                   #endif

                    check (desc.AddPrivateData (ALOEAlgorithmIDs::pluginInstance, sizeof (PluginInstanceInfo)));
                    check (desc.AddPrivateData (ALOEAlgorithmIDs::preparedFlag, sizeof (int32_t)));

                    if (numMeters > 0)
                    {
                        HeapBlock<AAX_CTypeID> meterIDs (static_cast<size_t> (numMeters));

                        for (int i = 0; i < numMeters; ++i)
                            meterIDs[i] = 'Metr' + static_cast<AAX_CTypeID> (i);

                        check (desc.AddMeters (ALOEAlgorithmIDs::meterTapBuffers, meterIDs.getData(), static_cast<uint32_t> (numMeters)));
                    }
                    else
                    {
                        // AAX does not allow there to be any gaps in the fields of the algorithm context structure
                        // so just add a dummy one here if there aren't any meters
                        check (desc.AddPrivateData (ALOEAlgorithmIDs::meterTapBuffers, sizeof (uintptr_t)));
                    }

                    // Create a property map
                    AAX_IPropertyMap* const properties = desc.NewPropertyMap();
                    jassert (properties != nullptr);

                    properties->AddProperty (AAX_eProperty_ManufacturerID,      AloePlugin_AAXManufacturerCode);
                    properties->AddProperty (AAX_eProperty_ProductID,           AloePlugin_AAXProductId);

                   #if AloePlugin_AAXDisableBypass
                    properties->AddProperty (AAX_eProperty_CanBypass,           false);
                   #else
                    properties->AddProperty (AAX_eProperty_CanBypass,           true);
                   #endif

                    properties->AddProperty (AAX_eProperty_InputStemFormat,     static_cast<AAX_CPropertyValue> (aaxInputFormat));
                    properties->AddProperty (AAX_eProperty_OutputStemFormat,    static_cast<AAX_CPropertyValue> (aaxOutputFormat));

                    // This value needs to match the RTAS wrapper's Type ID, so that
                    // the host knows that the RTAS/AAX plugins are equivalent.
                    const int32 pluginID = processor.getAAXPluginIDForMainBusConfig (fullLayout.getMainInputChannelSet(),
                                                                                     fullLayout.getMainOutputChannelSet(),
                                                                                     false);

                    // The plugin id generated from your AudioProcessor's getAAXPluginIDForMainBusConfig callback
                    // it not unique. Please fix your implementation!
                    jassert (! pluginIds.contains (pluginID));
                    pluginIds.add (pluginID);

                    properties->AddProperty (AAX_eProperty_PlugInID_Native, pluginID);

                   #if ! AloePlugin_AAXDisableAudioSuite
                    properties->AddProperty (AAX_eProperty_PlugInID_AudioSuite,
                                             processor.getAAXPluginIDForMainBusConfig (fullLayout.getMainInputChannelSet(),
                                                                                       fullLayout.getMainOutputChannelSet(),
                                                                                       true));
                   #endif

                   #if AloePlugin_AAXDisableMultiMono
                    properties->AddProperty (AAX_eProperty_Constraint_MultiMonoSupport, false);
                   #else
                    properties->AddProperty (AAX_eProperty_Constraint_MultiMonoSupport, true);
                   #endif

                   #if AloePlugin_AAXDisableDynamicProcessing
                    properties->AddProperty (AAX_eProperty_Constraint_AlwaysProcess, true);
                   #endif

                   #if AloePlugin_AAXDisableDefaultSettingsChunks
                    properties->AddProperty (AAX_eProperty_Constraint_DoNotApplyDefaultSettings, true);
                   #endif

                   #if AloePlugin_AAXDisableSaveRestore
                    properties->AddProperty (AAX_eProperty_SupportsSaveRestore, false);
                   #endif

                    if (fullLayout.getChannelSet (true, 1) == AudioChannelSet::mono())
                    {
                        check (desc.AddSideChainIn (ALOEAlgorithmIDs::sideChainBuffers));
                        properties->AddProperty (AAX_eProperty_SupportsSideChainInput, true);
                    }
                    else
                    {
                        // AAX does not allow there to be any gaps in the fields of the algorithm context structure
                        // so just add a dummy one here if there aren't any side chains
                        check (desc.AddPrivateData (ALOEAlgorithmIDs::sideChainBuffers, sizeof (uintptr_t)));
                    }

                    auto maxAuxBuses = jmax (0, jmin (15, fullLayout.outputBuses.size() - 1));

                    // add the output buses
                    // This is incredibly dumb: the output bus format must be well defined
                    // for every main bus in/out format pair. This means that there cannot
                    // be two configurations with different aux formats but
                    // identical main bus in/out formats.
                    for (int busIdx = 1; busIdx < maxAuxBuses + 1; ++busIdx)
                    {
                        auto set = fullLayout.getChannelSet (false, busIdx);

                        if (set.isDisabled())
                            break;

                        auto auxFormat = getFormatForAudioChannelSet (set, true);

                        if (auxFormat != AAX_eStemFormat_INT32_MAX && auxFormat != AAX_eStemFormat_None)
                        {
                            auto& name = processor.getBus (false, busIdx)->getName();
                            check (desc.AddAuxOutputStem (0, static_cast<int32_t> (auxFormat), name.toRawUTF8()));
                        }
                    }

                    check (desc.AddProcessProc_Native (algorithmProcessCallback, properties));
                */
        }

        pub fn host_supports_stem_format(
                stem_format:  AAX_EStemFormat,
                feature_info: *const AAX_IFeatureInfo) -> bool {
            
            todo!();
                /*
                    if (featureInfo != nullptr)
                    {
                        AAX_ESupportLevel supportLevel;

                        if (featureInfo->SupportLevel (supportLevel) == AAX_SUCCESS && supportLevel == AAX_eSupportLevel_ByProperty)
                        {
                            std::unique_ptr<const AAX_IPropertyMap> props (featureInfo->AcquireProperties());

                            // Due to a bug in ProTools 12.8, ProTools thinks that AAX_eStemFormat_Ambi_1_ACN is not supported
                            // To workaround this bug, check if ProTools supports AAX_eStemFormat_Ambi_2_ACN, and, if yes,
                            // we can safely assume that it will also support AAX_eStemFormat_Ambi_1_ACN
                            if (stemFormat == AAX_eStemFormat_Ambi_1_ACN)
                                stemFormat = AAX_eStemFormat_Ambi_2_ACN;

                            if (props != nullptr && props->GetProperty ((AAX_EProperty) stemFormat, (AAX_CPropertyValue*) &supportLevel) != 0)
                                return (supportLevel == AAX_eSupportLevel_Supported);
                        }
                    }

                    return (AAX_STEM_FORMAT_INDEX (stemFormat) <= 12);
                */
        }

        pub fn get_plug_in_description(
                descriptor:   &mut AAX_IEffectDescriptor,
                feature_info: *const AAX_IFeatureInfo)  {
            
            todo!();
                /*
                    PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_AAX;
                    std::unique_ptr<AudioProcessor> plugin (createPluginFilterOfType (AudioProcessor::wrapperType_AAX));
                    auto numInputBuses  = plugin->getBusCount (true);
                    auto numOutputBuses = plugin->getBusCount (false);

                    auto pluginNames = plugin->getAlternateDisplayNames();

                    pluginNames.insert (0, AloePlugin_Name);

                    pluginNames.removeDuplicates (false);

                    for (auto name : pluginNames)
                        descriptor.AddName (name.toRawUTF8());

                    descriptor.AddCategory (AloePlugin_AAXCategory);

                    const int numMeters = addAAXMeters (*plugin, descriptor);

                   #ifdef AloePlugin_AAXPageTableFile
                    // optional page table setting - define this macro in your project if you want
                    // to set this value - see Avid documentation for details about its format.
                    descriptor.AddResourceInfo (AAX_eResourceType_PageTable, AloePlugin_AAXPageTableFile);
                   #endif

                    check (descriptor.AddProcPtr ((void*) AloeAAX_GUI::Create,        kAAX_ProcPtrID_Create_EffectGUI));
                    check (descriptor.AddProcPtr ((void*) AloeAAX_Processor::Create,  kAAX_ProcPtrID_Create_EffectParameters));

                    Vec<int32> pluginIds;
                   #if AloePlugin_IsMidiEffect
                    // MIDI effect plug-ins do not support any audio channels
                    jassert (numInputBuses == 0 && numOutputBuses == 0);
                    ignoreUnused (featureInfo);

                    if (auto* desc = descriptor.NewComponentDescriptor())
                    {
                        createDescriptor (*desc, plugin->getBusesLayout(), *plugin, pluginIds, numMeters);
                        check (descriptor.AddComponent (desc));
                    }
                   #else
                    const int numIns  = numInputBuses  > 0 ? numElementsInArray (aaxFormats) : 0;
                    const int numOuts = numOutputBuses > 0 ? numElementsInArray (aaxFormats) : 0;

                    for (int inIdx = 0; inIdx < jmax (numIns, 1); ++inIdx)
                    {
                        auto aaxInFormat = numIns > 0 ? aaxFormats[inIdx] : AAX_eStemFormat_None;
                        auto inLayout = channelSetFromStemFormat (aaxInFormat, false);

                        for (int outIdx = 0; outIdx < jmax (numOuts, 1); ++outIdx)
                        {
                            auto aaxOutFormat = numOuts > 0 ? aaxFormats[outIdx] : AAX_eStemFormat_None;
                            auto outLayout = channelSetFromStemFormat (aaxOutFormat, false);

                            if (hostSupportsStemFormat (aaxInFormat, featureInfo)
                                 && hostSupportsStemFormat (aaxOutFormat, featureInfo))
                            {
                                AudioProcessor::BusesLayout fullLayout;

                                if (! AloeAAX_Processor::fullBusesLayoutFromMainLayout (*plugin, inLayout, outLayout, fullLayout))
                                    continue;

                                if (auto* desc = descriptor.NewComponentDescriptor())
                                {
                                    createDescriptor (*desc, fullLayout, *plugin, pluginIds, numMeters);
                                    check (descriptor.AddComponent (desc));
                                }
                            }
                        }
                    }

                    // You don't have any supported layouts
                    jassert (pluginIds.size() > 0);
                   #endif
                */
        }
    }

    impl AAXClasses {

        pub fn algorithm_process_callback(&mut self, 
            instances_begin: *mut &[ALOEAlgorithmContext],
            instances_end:   *const c_void)  {
            
            todo!();
            /*
                AAXClasses::AloeAAX_Processor::algorithmCallback (instancesBegin, instancesEnd);
            */
        }
    }

    extern "cdecl" {

        pub fn get_effect_descriptions(collection: *mut AAX_ICollection) -> AAX_Result {
            
            todo!();
                /*
                    ScopedAloeInitialiser_GUI libraryInitialiser;

                std::unique_ptr<const AAX_IFeatureInfo> stemFormatFeatureInfo;

                if (const auto* hostDescription = collection->DescriptionHost())
                    stemFormatFeatureInfo.reset (hostDescription->AcquireFeatureProperties (AAXATTR_ClientFeature_StemFormat));

                if (auto* descriptor = collection->NewDescriptor())
                {
                    AAXClasses::getPlugInDescription (*descriptor, stemFormatFeatureInfo.get());
                    collection->AddEffect (ALOE_STRINGIFY (AloePlugin_AAXIdentifier), descriptor);

                    collection->SetManufacturerName (AloePlugin_Manufacturer);
                    collection->AddPackageName (AloePlugin_Desc);
                    collection->AddPackageName (AloePlugin_Name);
                    collection->SetPackageVersion (AloePlugin_VersionCode);

                    return AAX_SUCCESS;
                }

                return AAX_ERROR_NULL_OBJECT;
                */
        }
    }

    lazy_static!{
        /*
        #if _MSC_VER || ALOE_MINGW
        extern "C" BOOL WINAPI DllMain (HINSTANCE instance, DWORD reason, LPVOID) { if (reason == DLL_PROCESS_ATTACH) Process::setCurrentModuleInstanceHandle (instance); return true; }
        #endif
        */
    }
}
