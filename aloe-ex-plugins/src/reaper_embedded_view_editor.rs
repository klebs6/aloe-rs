crate::ix!();

pub struct ReaperEmbeddedViewDemo<'a> {
    base:             AudioProcessor<'a>,
    base2:            VstCallbackHandler,
    base3:            Vst3ClientExtensions,
    base5:            Timer,
    gain:             *mut AudioParameterFloat, // default = nullptr
    global_bypass_fn: fn(_0: i32) -> c_void,
    embedded_ui:      EmbeddedUI<'a>,               //{ *this };
    stored_level:     Atomic<f32>,              // default = { 0.0f  }
    level_to_draw:    f32,                      // default = 0.0f
}

impl<'a> EmbeddedViewListener for ReaperEmbeddedViewDemo<'a> {

    fn handled_embedded_ui_message(
        &mut self, 
        msg:   i32,
        parm2: TPtrInt,
        parm3: TPtrInt

    ) -> TPtrInt {
        
        todo!();
        /*
            switch (msg)
            {
                case REAPER_FXEMBED_WM_IS_SUPPORTED:
                    return 1;

                case REAPER_FXEMBED_WM_PAINT:
                    return doPaint (reinterpret_cast<reaper::REAPER_FXEMBED_IBitmap*> (parm2),
                                    reinterpret_cast<reaper::REAPER_FXEMBED_DrawInfo*> (parm3));

                case REAPER_FXEMBED_WM_GETMINMAXINFO:
                    return getSizeInfo (reinterpret_cast<reaper::REAPER_FXEMBED_SizeHints*> (parm3));

                // Implementing mouse behaviour is left as an exercise for the reaper, I mean reader
                case REAPER_FXEMBED_WM_CREATE:          break;
                case REAPER_FXEMBED_WM_DESTROY:         break;
                case REAPER_FXEMBED_WM_SETCURSOR:       break;
                case REAPER_FXEMBED_WM_MOUSEMOVE:       break;
                case REAPER_FXEMBED_WM_LBUTTONDOWN:     break;
                case REAPER_FXEMBED_WM_LBUTTONUP:       break;
                case REAPER_FXEMBED_WM_LBUTTONDBLCLK:   break;
                case REAPER_FXEMBED_WM_RBUTTONDOWN:     break;
                case REAPER_FXEMBED_WM_RBUTTONUP:       break;
                case REAPER_FXEMBED_WM_RBUTTONDBLCLK:   break;
                case REAPER_FXEMBED_WM_MOUSEWHEEL:      break;
            }

            return 0;
        */
    }
}

impl<'a> Default for ReaperEmbeddedViewDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addParameter (gain = new AudioParameterFloat ("gain", "Gain", 0.0f, 1.0f, 0.5f));
            startTimerHz (60)
        */
    }
}

impl<'a> ReaperEmbeddedViewDemo<'a> {

    pub fn prepare_to_play(&mut self, _0: f64, _1: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn process_block(&mut self, 
        audio: &mut AudioBuffer<f32>,
        _1:    &mut MidiBuffer)  {
        
        todo!();
        /*
            processBlockImpl (audio);
        */
    }
    
    pub fn process_block_f64(
        &mut self, 
        audio: &mut AudioBuffer<f64>,
        _1:    &mut MidiBuffer
    ) {
        
        todo!();
        /*
            processBlockImpl (audio);
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new Editor (*this, *gain, globalBypassFn);
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "ReaperEmbeddedViewDemo";
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
    
    pub fn set_current_program(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_program_name(&mut self, _0: i32) -> String {
        
        todo!();
        /*
            return "None";
        */
    }
    
    pub fn change_program_name(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            MemoryOutputStream (destData, true).writeFloat (*gain);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            gain->setValueNotifyingHost (MemoryInputStream (data,
                                                            static_cast<size_t> (sizeInBytes),
                                                            false).readFloat());
        */
    }
    
    pub fn query_iedit_controller(
        &mut self, 
        tuid: TUID,
        obj:  *mut *mut c_void

    ) -> i32 {
        
        todo!();
        /*
            if (std::memcmp (tuid, embeddedUi.iid, sizeof (Steinberg::TUID)) == 0)
            {
                *obj = &embeddedUi;
                return Steinberg::kResultOk;
            }

            *obj = nullptr;
            return Steinberg::kNoInterface;
        */
    }
    
    pub fn set_ihost_application(&mut self, ptr: *mut dyn FUnknown)  {
        
        todo!();
        /*
            if (ptr == nullptr)
                return;

            void* objPtr = nullptr;

            if (ptr->queryInterface (reaper::IReaperHostApplication::iid, &objPtr) == Steinberg::kResultOk)
            {
                if (void* fnPtr = static_cast<reaper::IReaperHostApplication*> (objPtr)->getReaperApi ("BypassFxAllTracks"))
                    globalBypassFn = reinterpret_cast<void (*) (int)> (fnPtr);
            }
        */
    }
    
    pub fn handle_vst_plugin_can_do(&mut self, 
        _0:  i32,
        _1:  PointerSizedInt,
        ptr: *mut c_void,
        _3:  f32) -> PointerSizedInt {
        
        todo!();
        /*
            if (auto* str = static_cast<const char*> (ptr))
            {
                if (strcmp (str, "hasCockosEmbeddedUI") == 0)
                    return 0xbeef0000;

                if (strcmp (str, "hasCockosExtensions") == 0)
                    return 0xbeef0000;
            }

            return 0;
        */
    }
    
    pub fn handle_vst_manufacturer_specific(&mut self, 
        index: i32,
        value: PointerSizedInt,
        ptr:   *mut c_void,
        opt:   f32) -> PointerSizedInt {
        
        todo!();
        /*
            // The docstring at the top of reaper_plugin_fx_embed.h specifies
            // that the index will always be effEditDraw, which is now deprecated.
            if (index != __effEditDrawDeprecated)
                return 0;

            return (pointer_sized_int) handledEmbeddedUIMessage ((int) opt,
                                                                 (Steinberg::TPtrInt) value,
                                                                 (Steinberg::TPtrInt) ptr);
        */
    }
    
    pub fn handle_vst_host_callback_available(&mut self, hostcb: VstHostCallbackType)  {
        
        todo!();
        /*
            char functionName[] = "BypassFxAllTracks";
            globalBypassFn = reinterpret_cast<void (*) (int)> (hostcb ((int32_t) 0xdeadbeef, (int32_t) 0xdeadf00d, 0, functionName, 0.0));
        */
    }
    
    pub fn process_block_impl<Float>(&mut self, audio: &mut AudioBuffer<f32>)  {
    
        todo!();
        /*
            audio.applyGain (*gain);

            const auto minMax = audio.findMinMax (0, 0, audio.getNumSamples());
            const auto newMax = (float) std::max (std::abs (minMax.getStart()), std::abs (minMax.getEnd()));

            auto loaded = storedLevel.load();
            while (loaded < newMax && ! storedLevel.compare_exchange_weak (loaded, newMax)) {}
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            levelToDraw = std::max (levelToDraw * 0.95f, storedLevel.exchange (0.0f));
        */
    }
    
    pub fn get_size_info(&mut self, size_hints: *mut REAPER_FXEMBED_SizeHints) -> TPtrInt {
        
        todo!();
        /*
            if (sizeHints == nullptr)
                return 0;

            sizeHints->preferred_aspect = 1 << 16;
            sizeHints->minimum_aspect   = 1 << 16;
            sizeHints->min_height = sizeHints->min_width = 50;
            sizeHints->max_height = sizeHints->max_width = 1000;
            return 1;
        */
    }
    
    pub fn do_paint(&mut self, 
        bitmap:    *mut dyn REAPER_FXEMBED_IBitmap,
        draw_info: *mut REAPER_FXEMBED_DrawInfo) -> TPtrInt {
        
        todo!();
        /*
            if (bitmap == nullptr || drawInfo == nullptr || bitmap->getWidth() <= 0 || bitmap->getHeight() <= 0)
                return 0;

            Image img (Image::PixelFormat::ARGB, bitmap->getWidth(), bitmap->getHeight(), true);
            Graphics g (img);

            g.fillAll (Colours::black);

            const auto bounds = g.getClipBounds();
            const auto corner = 3.0f;

            g.setColour (Colours::darkgrey);
            g.fillRoundedRectangle (bounds.withSizeKeepingCentre (20, bounds.getHeight() - 6).toFloat(),
                                    corner);

            const auto minDb = -50.0f;
            const auto maxDb = 6.0f;
            const auto levelInDb = Decibels::gainToDecibels (levelToDraw, minDb);
            const auto fractionOfHeight = jmap (levelInDb, minDb, maxDb, 0.0f, 1.0f);
            const auto trackBounds = bounds.withSizeKeepingCentre (16, bounds.getHeight() - 10).toFloat();

            g.setColour (Colours::black);
            const auto zeroDbIndicatorY = trackBounds.proportionOfHeight (jmap (0.0f,
                                                                                minDb,
                                                                                maxDb,
                                                                                0.0f,
                                                                                1.0f));
            g.drawHorizontalLine ((int) (trackBounds.getBottom() - zeroDbIndicatorY),
                                  trackBounds.getX(),
                                  trackBounds.getRight());

            g.setGradientFill (ColourGradient (Colours::darkgreen,
                                               { 0.0f, (float) bounds.getHeight() },
                                               Colours::darkred,
                                               { 0.0f, 0.0f },
                                               false));

            g.fillRoundedRectangle (trackBounds.withHeight (trackBounds.proportionOfHeight (fractionOfHeight))
                                               .withBottomY (trackBounds.getBottom()),
                                    corner);

            ImageBitmapData imgData { img, ImageBitmapData::readOnly };
            const auto pixelsWidth = imgData.pixelStride * imgData.width;

            auto* px = bitmap->getBits();
            const auto rowSpan = bitmap->getRowSpan();
            const auto numRows = bitmap->getHeight();

            for (int y = 0; y < numRows; ++y)
                std::memcpy (px + (y * rowSpan), imgData.getLinePointer (y), (size_t) pixelsWidth);

            return 1;
        */
    }
    
}
