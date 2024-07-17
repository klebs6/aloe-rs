crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DspModulePluginDemoEditor<'a> {
    base:                     AudioProcessorEditor<'a>,
    combo_effect:             ComboBox<'a>,
    label_effect:             Label<'a>,                     // default = { {}, "Audio effect: "  }
    proc:                     &'a mut DspModulePluginDemo<'a>,
    basic_controls:           BasicControls<'a>,             //{ *this, proc.getParameterValues() };
    distortion_controls:      DistortionControls<'a>,        //{ *this, proc.getParameterValues() };
    convolution_controls:     ConvolutionControls<'a>,       //{ *this, proc.getParameterValues() };
    multiband_controls:       MultiBandControls<'a>,         //{ *this, proc.getParameterValues() };
    compressor_controls:      CompressorControls<'a>,        //{ *this, proc.getParameterValues() };
    noise_gate_controls:      NoiseGateControls<'a>,         //{ *this, proc.getParameterValues() };
    limiter_controls:         LimiterControls<'a>,           //{ *this, proc.getParameterValues() };
    direct_delay_controls:    DirectDelayControls<'a>,       //{ *this, proc.getParameterValues() };
    delay_effect_controls:    DelayEffectControls<'a>,       //{ *this, proc.getParameterValues() };
    phaser_controls:          PhaserControls<'a>,            //{ *this, proc.getParameterValues() };
    chorus_controls:          ChorusControls<'a>,            //{ *this, proc.getParameterValues() };
    ladder_controls:          LadderControls<'a>,            //{ *this, proc.getParameterValues() };
}

pub const DSP_MODULE_PLUGIN_DEMO_EDITOR_TOP_SIZE:    usize = 40;
pub const DSP_MODULE_PLUGIN_DEMO_EDITOR_BOTTOM_SIZE: usize = 40;
pub const DSP_MODULE_PLUGIN_DEMO_EDITOR_MID_SIZE:    usize = 40;
pub const DSP_MODULE_PLUGIN_DEMO_EDITOR_TAB_SIZE:    usize = 155;

impl<'a> DspModulePluginDemoEditor<'a> {

    pub fn new(p: &mut DspModulePluginDemo) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (&p),
              proc (p)

            comboEffect.addSectionHeading ("Main");
            comboEffect.addItem ("Distortion", TabDistortion);
            comboEffect.addItem ("Convolution", TabConvolution);
            comboEffect.addItem ("Multi-band", TabMultiBand);

            comboEffect.addSectionHeading ("Dynamics");
            comboEffect.addItem ("Compressor", TabCompressor);
            comboEffect.addItem ("Noise gate", TabNoiseGate);
            comboEffect.addItem ("Limiter", TabLimiter);

            comboEffect.addSectionHeading ("Delay");
            comboEffect.addItem ("Delay line direct", TabDelayLineDirect);
            comboEffect.addItem ("Delay line effect", TabDelayLineEffect);

            comboEffect.addSectionHeading ("Others");
            comboEffect.addItem ("Phaser", TabPhaser);
            comboEffect.addItem ("Chorus", TabChorus);
            comboEffect.addItem ("Ladder filter", TabLadder);

            comboEffect.setSelectedId (proc.indexTab + 1, dontSendNotification);
            comboEffect.onChange = [this]
            {
                proc.indexTab = comboEffect.getSelectedId() - 1;
                updateVisibility();
            };

            addAllAndMakeVisible (*this,
                                  comboEffect,
                                  labelEffect,
                                  basicControls,
                                  distortionControls,
                                  convolutionControls,
                                  multibandControls,
                                  compressorControls,
                                  noiseGateControls,
                                  limiterControls,
                                  directDelayControls,
                                  delayEffectControls,
                                  phaserControls,
                                  chorusControls,
                                  ladderControls);
            labelEffect.setJustificationType (Justification::centredRight);
            labelEffect.attachToComponent (&comboEffect, true);

            updateVisibility();

            setSize (800, 430);
            setResizable (false, false);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto rect = getLocalBounds();

            auto rectTop    = rect.removeFromTop (DSP_MODULE_PLUGIN_DEMO_EDITOR_TOP_SIZE);
            auto rectBottom = rect.removeFromBottom (DSP_MODULE_PLUGIN_DEMO_EDITOR_BOTTOM_SIZE);

            auto rectEffects = rect.removeFromBottom (DSP_MODULE_PLUGIN_DEMO_EDITOR_TAB_SIZE);
            auto rectChoice  = rect.removeFromBottom (DSP_MODULE_PLUGIN_DEMO_EDITOR_MID_SIZE);

            g.setColour (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
            g.fillRect (rect);

            g.setColour (getLookAndFeel().findColour (ResizableWindow::backgroundColourId).brighter (0.2f));
            g.fillRect (rectEffects);

            g.setColour (getLookAndFeel().findColour (ResizableWindow::backgroundColourId).darker (0.2f));
            g.fillRect (rectTop);
            g.fillRect (rectBottom);
            g.fillRect (rectChoice);

            g.setColour (Colours::white);
            g.setFont (Font (20.0f).italicised().withExtraKerningFactor (0.1f));
            g.drawFittedText ("DSP MODULE DEMO", rectTop.reduced (10, 0), Justification::centredLeft, 1);

            g.setFont (Font (14.0f));
            String strText = "IR length (reverb): " + String (proc.getCurrentIRSize()) + " samples";
            g.drawFittedText (strText, rectBottom.reduced (10, 0), Justification::centredRight, 1);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto rect = getLocalBounds();
            rect.removeFromTop (DSP_MODULE_PLUGIN_DEMO_EDITOR_TOP_SIZE);
            rect.removeFromBottom (DSP_MODULE_PLUGIN_DEMO_EDITOR_BOTTOM_SIZE);

            auto rectEffects = rect.removeFromBottom (DSP_MODULE_PLUGIN_DEMO_EDITOR_TAB_SIZE);
            auto rectChoice  = rect.removeFromBottom (DSP_MODULE_PLUGIN_DEMO_EDITOR_MID_SIZE);

            comboEffect.setBounds (rectChoice.withSizeKeepingCentre (200, 24));

            rect.reduce (80, 0);
            rectEffects.reduce (20, 0);

            basicControls.setBounds (rect);

            forEach ([&] (Component& comp) { comp.setBounds (rectEffects); },
                     distortionControls,
                     convolutionControls,
                     multibandControls,
                     compressorControls,
                     noiseGateControls,
                     limiterControls,
                     directDelayControls,
                     delayEffectControls,
                     phaserControls,
                     chorusControls,
                     ladderControls);
        */
    }
    
    pub fn update_visibility(&mut self)  {
        
        todo!();
        /*
            const auto indexEffect = comboEffect.getSelectedId();

            const auto op = [&] (const std::tuple<Component&, int>& tup)
            {
                Component& comp    = std::get<0> (tup);
                const int tabIndex = std::get<1> (tup);
                comp.setVisible (tabIndex == indexEffect);
            };

            forEach (op,
                     std::forward_as_tuple (distortionControls,  TabDistortion),
                     std::forward_as_tuple (convolutionControls, TabConvolution),
                     std::forward_as_tuple (multibandControls,   TabMultiBand),
                     std::forward_as_tuple (compressorControls,  TabCompressor),
                     std::forward_as_tuple (noiseGateControls,   TabNoiseGate),
                     std::forward_as_tuple (limiterControls,     TabLimiter),
                     std::forward_as_tuple (directDelayControls, TabDelayLineDirect),
                     std::forward_as_tuple (delayEffectControls, TabDelayLineEffect),
                     std::forward_as_tuple (phaserControls,      TabPhaser),
                     std::forward_as_tuple (chorusControls,      TabChorus),
                     std::forward_as_tuple (ladderControls,      TabLadder));
        */
    }
    
    pub fn perform_layout<Components>(
        bounds:     &Rectangle<i32>,
        components: &mut Components)  {
    
        todo!();
        /*
            Grid grid;
            using Track = Grid::TrackInfo;

            grid.autoColumns     = Track (1_fr);
            grid.autoRows        = Track (1_fr);
            grid.columnGap       = Grid::Px (10);
            grid.rowGap          = Grid::Px (0);
            grid.autoFlow        = Grid::AutoFlow::column;

            grid.templateColumns = { GetTrackInfo{} (components)... };
            grid.items           = { GridItem (components)... };

            grid.performLayout (bounds);
        */
    }
}
