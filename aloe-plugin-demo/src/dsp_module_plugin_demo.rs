crate::ix!();

impl<'a> Default for DspModulePluginDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : DspModulePluginDemo (AudioProcessorValueTreeState::ParameterLayout{}
        */
    }
}

impl<'a> DspModulePluginDemo<'a> {

    pub fn prepare_to_play(&mut self, 
        sample_rate:       f64,
        samples_per_block: i32)  {
        
        todo!();
        /*
            const auto channels = jmax (getTotalNumInputChannels(), getTotalNumOutputChannels());

            if (channels == 0)
                return;

            chain.prepare ({ sampleRate, (uint32) samplesPerBlock, (uint32) channels });

            reset();
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "DSPModulePluginDemo";
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn is_midi_effect(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn change_program_name(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_buses_layout_supported(&self, layout: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            return layout == BusesLayout { { AudioChannelSet::stereo() },
                                           { AudioChannelSet::stereo() } };
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            copyXmlToBinary (*apvts.copyState().createXml(), destData);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            apvts.replaceState (ValueTree::fromXml (*getXmlFromBinary (data, sizeInBytes)));
        */
    }
    
    pub fn get_current_ir_size(&self) -> i32 {
        
        todo!();
        /*
            return irSize;
        */
    }
    
    pub fn get_parameter_values(&self) -> &DspModulePluginDemoParameterReferences {
        
        todo!();
        /*
            return parameters;
        */
    }
    
    pub fn new(layout: AudioProcessorValueTreeStateParameterLayout) -> Self {
    
        todo!();
        /*


            : AudioProcessor (BusesProperties().withInput ("In",   AudioChannelSet::stereo())
                                               .withOutput ("Out", AudioChannelSet::stereo())),
              parameters { layout },
              apvts { *this, nullptr, "state", std::move (layout) }

            apvts.state.addListener (this);

            forEach ([] (Gain<float>& gain) { gain.setRampDurationSeconds (0.05); },
                     get<inputGainIndex>  (chain),
                     get<outputGainIndex> (chain));

            get<pannerIndex> (chain).setRule (PannerRule::linear);
        */
    }
    
    pub fn value_tree_property_changed(&mut self, 
        _0: &mut ValueTree,
        _1: &Identifier)  {
        
        todo!();
        /*
            requiresUpdate.store (true);
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            {
                DistortionProcessor& distortion = get<distortionIndex> (chain);

                if (distortion.currentIndexOversampling != parameters.distortionOversampler.getIndex())
                {
                    distortion.currentIndexOversampling = parameters.distortionOversampler.getIndex();
                    prepareToPlay (getSampleRate(), getBlockSize());
                    return;
                }

                distortion.currentIndexWaveshaper = parameters.distortionType.getIndex();
                distortion.lowpass .setCutoffFrequency (parameters.distortionLowpass.get());
                distortion.highpass.setCutoffFrequency (parameters.distortionHighpass.get());
                distortion.distGain.setGainDecibels (parameters.distortionInGain.get());
                distortion.compGain.setGainDecibels (parameters.distortionCompGain.get());
                distortion.mixer.setWetMixProportion (parameters.distortionMix.get() / 100.0f);
                setBypassed<distortionIndex> (chain, ! parameters.distortionEnabled);
            }

            {
                ConvolutionProcessor& convolution = get<convolutionIndex> (chain);
                convolution.cabEnabled    = parameters.convolutionCabEnabled;
                convolution.reverbEnabled = parameters.convolutionReverbEnabled;
                convolution.mixer.setWetMixProportion (parameters.convolutionReverbMix.get() / 100.0f);
            }

            get<inputGainIndex>  (chain).setGainDecibels (parameters.inputGain.get());
            get<outputGainIndex> (chain).setGainDecibels (parameters.outputGain.get());
            get<pannerIndex> (chain).setPan (parameters.pan.get() / 100.0f);

            {
                MultiBandProcessor& multiband = get<multiBandIndex> (chain);
                const auto multibandFreq = parameters.multiBandFreq.get();
                multiband.lowpass .setCutoffFrequency (multibandFreq);
                multiband.highpass.setCutoffFrequency (multibandFreq);
                const bool enabled = parameters.multiBandEnabled;
                multiband.lowVolume .setGainDecibels (enabled ? parameters.multiBandLowVolume .get() : 0.0f);
                multiband.highVolume.setGainDecibels (enabled ? parameters.multiBandHighVolume.get() : 0.0f);
                setBypassed<multiBandIndex> (chain, ! enabled);
            }

            {
                Compressor<float>& compressor = get<compressorIndex> (chain);
                compressor.setThreshold (parameters.compressorThreshold.get());
                compressor.setRatio     (parameters.compressorRatio.get());
                compressor.setAttack    (parameters.compressorAttack.get());
                compressor.setRelease   (parameters.compressorRelease.get());
                setBypassed<compressorIndex> (chain, ! parameters.compressorEnabled);
            }

            {
                NoiseGate<float>& noiseGate = get<noiseGateIndex> (chain);
                noiseGate.setThreshold (parameters.noiseGateThreshold.get());
                noiseGate.setRatio     (parameters.noiseGateRatio.get());
                noiseGate.setAttack    (parameters.noiseGateAttack.get());
                noiseGate.setRelease   (parameters.noiseGateRelease.get());
                setBypassed<noiseGateIndex> (chain, ! parameters.noiseGateEnabled);
            }

            {
                Limiter<float>& limiter = get<limiterIndex> (chain);
                limiter.setThreshold (parameters.limiterThreshold.get());
                limiter.setRelease   (parameters.limiterRelease.get());
                setBypassed<limiterIndex> (chain, ! parameters.limiterEnabled);
            }

            {
                DirectDelayProcessor& delay = get<directDelayIndex> (chain);
                delay.delayLineDirectType = parameters.directDelayType.getIndex();

                std::fill (delay.delayDirectValue.begin(),
                           delay.delayDirectValue.end(),
                           (double) parameters.directDelayValue.get());

                delay.smoothFilter.setCutoffFrequency (1000.0 / parameters.directDelaySmoothing.get());
                delay.mixer.setWetMixProportion (parameters.directDelayMix.get() / 100.0f);
                setBypassed<directDelayIndex> (chain, ! parameters.directDelayEnabled);
            }

            {
                DelayEffectProcessor& delay = get<delayEffectIndex> (chain);
                delay.delayEffectType = parameters.delayEffectType.getIndex();

                std::fill (delay.delayEffectValue.begin(),
                           delay.delayEffectValue.end(),
                           (double) parameters.delayEffectValue.get() / 1000.0 * getSampleRate());

                const auto feedbackGain = Decibels::decibelsToGain (parameters.delayEffectFeedback.get(), -100.0f);

                for (auto& volume : delay.delayFeedbackVolume)
                    volume.setTargetValue (feedbackGain);

                delay.smoothFilter.setCutoffFrequency (1000.0 / parameters.delayEffectSmoothing.get());
                delay.lowpass.setCutoffFrequency (parameters.delayEffectLowpass.get());
                delay.mixer.setWetMixProportion (parameters.delayEffectMix.get() / 100.0f);
                setBypassed<delayEffectIndex> (chain, ! parameters.delayEffectEnabled);
            }

            {
                Phaser<float>& phaser = get<phaserIndex> (chain);
                phaser.setRate            (parameters.phaserRate.get());
                phaser.setDepth           (parameters.phaserDepth.get() / 100.0f);
                phaser.setCentreFrequency (parameters.phaserCentreFrequency.get());
                phaser.setFeedback        (parameters.phaserFeedback.get() / 100.0f * 0.95f);
                phaser.setMix             (parameters.phaserMix.get() / 100.0f);
                setBypassed<phaserIndex> (chain, ! parameters.phaserEnabled);
            }

            {
                Chorus<float>& chorus = get<chorusIndex> (chain);
                chorus.setRate        (parameters.chorusRate.get());
                chorus.setDepth       (parameters.chorusDepth.get() / 100.0f);
                chorus.setCentreDelay (parameters.chorusCentreDelay.get());
                chorus.setFeedback    (parameters.chorusFeedback.get() / 100.0f * 0.95f);
                chorus.setMix         (parameters.chorusMix.get() / 100.0f);
                setBypassed<chorusIndex> (chain, ! parameters.chorusEnabled);
            }

            {
                LadderFilter<float>& ladder = get<ladderIndex> (chain);

                ladder.setCutoffFrequencyHz (parameters.ladderCutoff.get());
                ladder.setResonance         (parameters.ladderResonance.get() / 100.0f);
                ladder.setDrive (Decibels::decibelsToGain (parameters.ladderDrive.get()));

                ladder.setMode ([&]
                {
                    switch (parameters.ladderMode.getIndex())
                    {
                        case 0: return LadderFilterMode::LPF12;
                        case 1: return LadderFilterMode::LPF24;
                        case 2: return LadderFilterMode::HPF12;
                        case 3: return LadderFilterMode::HPF24;
                        case 4: return LadderFilterMode::BPF12;

                        default: break;
                    }

                    return LadderFilterMode::BPF24;
                }());

                setBypassed<ladderIndex> (chain, ! parameters.ladderEnabled);
            }

            requiresUpdate.store (false);
        */
    }
    
    pub fn get_panning_text_for_value(value: f32) -> String {
        
        todo!();
        /*
            if (value == 0.5f)
                return "center";

            if (value < 0.5f)
                return String (roundToInt ((0.5f - value) * 200.0f)) + "%L";

            return String (roundToInt ((value - 0.5f) * 200.0f)) + "%R";
        */
    }
    
    pub fn get_panning_value_for_text(str_text: String) -> f32 {
        
        todo!();
        /*
            if (strText.compareIgnoreCase ("center") == 0 || strText.compareIgnoreCase ("c") == 0)
                return 0.5f;

            strText = strText.trim();

            if (strText.indexOfIgnoreCase ("%L") != -1)
            {
                auto percentage = (float) strText.substring (0, strText.indexOf ("%")).getDoubleValue();
                return (100.0f - percentage) / 100.0f * 0.5f;
            }

            if (strText.indexOfIgnoreCase ("%R") != -1)
            {
                auto percentage = (float) strText.substring (0, strText.indexOf ("%")).getDoubleValue();
                return percentage / 100.0f * 0.5f + 0.5f;
            }

            return 0.5f;
        */
    }
}
