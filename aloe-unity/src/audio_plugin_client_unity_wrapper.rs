crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/Unity/aloe_Unity_Wrapper.cpp]

#[cfg(target_feature = "unity")]
pub type createUnityPeerFunctionType = fn(&mut Component);

#[cfg(target_feature = "unity")]
pub fn create_unity_peer(c: &mut Component) -> *mut ComponentPeer {
    
    todo!();
    /*
        return new UnityPeer (c);
    */
}

#[cfg(target_feature = "unity")]
lazy_static!{
    /*
    extern createUnityPeerFunctionType aloe_createUnityPeerFn;
    */
}

#[no_copy]
#[leak_detector]
#[cfg(target_feature = "unity")]
pub struct UnityPeer {
    base:           ComponentPeer,
    base2:          AsyncUpdater,
    bounds:         Rectangle<i32>,
    mouse_watcher:  MouseWatcher,
    pixel_data:     *mut u8, // default = nullptr
    texture_width:  i32,
    texture_height: i32,
    render_image:   Image,
}

#[cfg(target_feature = "unity")]
pub mod unity_peer {
    use super::*;

    #[no_copy]
    #[leak_detector]
    pub struct UnityPeerUnityBitmapImage {
        base:         ImagePixelData,
        image_data:   *mut u8,
        pixel_stride: i32, // default = 4
        line_stride:  i32,
    }

    impl UnityPeerUnityBitmapImage {

        pub fn new(
            data: *mut u8,
            w:    i32,
            h:    i32) -> Self {
        
            todo!();
            /*


                : ImagePixelData (Image::PixelFormat::ARGB, w, h),
                      imageData (data),
                      lineStride (width * pixelStride)
            */
        }
        
        pub fn create_type(&self) -> Box<ImageType> {
            
            todo!();
            /*
                return std::make_unique<SoftwareImageType>();
            */
        }
        
        pub fn create_low_level_context(&mut self) -> Box<LowLevelGraphicsContext> {
            
            todo!();
            /*
                return std::make_unique<LowLevelGraphicsSoftwareRenderer> (Image (this));
            */
        }
        
        pub fn initialise_bitmap_data(&mut self, 
            bitmap: &mut ImageBitmapData,
            x:      i32,
            y:      i32,
            mode:   ImageBitmapData::ReadWriteMode)  {
            
            todo!();
            /*
                ignoreUnused (mode);

                    bitmap.data = imageData + x * pixelStride + y * lineStride;
                    bitmap.pixelFormat = pixelFormat;
                    bitmap.lineStride = lineStride;
                    bitmap.pixelStride = pixelStride;
            */
        }
        
        pub fn clone(&mut self) -> ImagePixelData::Ptr {
            
            todo!();
            /*
                auto im = new UnityPeerUnityBitmapImage (imageData, width, height);

                    for (int i = 0; i < height; ++i)
                        memcpy (im->imageData + i * lineStride, imageData + i * lineStride, (size_t) lineStride);

                    return im;
            */
        }
    }

    pub struct MouseWatcher {
        base:            Timer,
        owner:           &mut ComponentPeer,
        bounds_to_watch: Rectangle<i32>,
        last_mouse_pos:  Point<i32>,
    }

    impl MouseWatcher {

        pub fn new(o: &mut ComponentPeer) -> Self {
        
            todo!();
            /*
            : owner(o),

            
            */
        }
        
        pub fn timer_callback(&mut self)  {
            
            todo!();
            /*
                auto pos = Desktop::getMousePosition();

                    if (boundsToWatch.contains (pos) && pos != lastMousePos)
                    {
                        auto ms = Desktop::getInstance().getMainMouseSource();

                        if (! ms.getCurrentModifiers().isLeftButtonDown())
                            owner.handleMouseEvent (MouseInputSource::mouse, owner.globalToLocal (pos.toFloat()), {},
                                                    MouseInputSource::invalidPressure, MouseInputSource::invalidOrientation, Time::currentTimeMillis());

                        lastMousePos = pos;
                    }
            */
        }
        
        pub fn set_bounds_to_watch(&mut self, b: Rectangle<i32>)  {
            
            todo!();
            /*
                if (boundsToWatch != b)
                        boundsToWatch = b;

                    startTimer (250);
            */
        }
    }
}

#[cfg(target_feature = "unity")]
impl UnityPeer {

    pub fn new(ed: &mut Component) -> Self {
    
        todo!();
        /*


            : ComponentPeer (ed, 0),
              mouseWatcher (*this)
            getEditor().setResizable (false, false);
        */
    }
    
    pub fn get_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return bounds;
        */
    }
    
    pub fn local_to_global(&mut self, relative_position: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            return relativePosition + getBounds().getPosition().toFloat();
        */
    }
    
    pub fn global_to_local(&mut self, screen_position: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            return screenPosition - getBounds().getPosition().toFloat();
        */
    }
    
    pub fn get_available_rendering_engines(&mut self) -> StringArray {
        
        todo!();
        /*
            return StringArray ("Software Renderer");
        */
    }
    
    pub fn set_bounds(&mut self, 
        new_bounds: &Rectangle<i32>,
        _1:         bool)  {
        
        todo!();
        /*
            bounds = newBounds;
            mouseWatcher.setBoundsToWatch (bounds);
        */
    }
    
    pub fn contains(&self, 
        local_pos: Point<i32>,
        _1:        bool) -> bool {
        
        todo!();
        /*
            if (isPositiveAndBelow (localPos.getX(), getBounds().getWidth())
                   && isPositiveAndBelow (localPos.getY(), getBounds().getHeight()))
                return true;

            return false;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            fillPixels();
        */
    }
    
    pub fn get_editor(&mut self) -> &mut AudioProcessorEditor {
        
        todo!();
        /*
            return *dynamic_cast<AudioProcessorEditor*> (&getComponent());
        */
    }
    
    pub fn set_pixel_data_handle(&mut self, 
        handle: *mut u8,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            pixelData = handle;

            textureWidth = width;
            textureHeight = height;

            renderImage = Image (new UnityPeerUnityBitmapImage (pixelData, width, height));
        */
    }

    /**
      | N.B. This is NOT an efficient way to do
      | this and you shouldn't use this method in
      | your own code.  It works for our purposes
      | here but a much more efficient way would be
      | to use a GL texture.
      */
    pub fn fill_pixels(&mut self)  {
        
        todo!();
        /*
            if (pixelData == nullptr)
                return;

            LowLevelGraphicsSoftwareRenderer renderer (renderImage);
            renderer.addTransform (AffineTransform::verticalFlip ((float) getComponent().getHeight()));

            handlePaint (renderer);

            for (int i = 0; i < textureWidth * textureHeight * 4; i += 4)
            {
                auto r = pixelData[i + 2];
                auto g = pixelData[i + 1];
                auto b = pixelData[i + 0];

                pixelData[i + 0] = r;
                pixelData[i + 1] = g;
                pixelData[i + 2] = b;
            }
        */
    }
    
    pub fn forward_mouse_event(&mut self, 
        position: Point<f32>,
        mods:     ModifierKeys)  {
        
        todo!();
        /*
            ModifierKeys::currentModifiers = mods;

            handleMouseEvent (MouseInputSource::mouse, position, mods, MouseInputSource::invalidPressure,
                              MouseInputSource::invalidOrientation, Time::currentTimeMillis());
        */
    }
    
    pub fn forward_key_press(&mut self, 
        code: i32,
        name: String,
        mods: ModifierKeys)  {
        
        todo!();
        /*
            ModifierKeys::currentModifiers = mods;

            handleKeyPress (getKeyPress (code, name));
        */
    }
    
    pub fn get_key_press(&mut self, 
        key_code: i32,
        name:     String) -> KeyPress {
        
        todo!();
        /*
            if (keyCode >= 32 && keyCode <= 64)
                return { keyCode, ModifierKeys::currentModifiers, aloe_wchar (keyCode) };

            if (keyCode >= 91 && keyCode <= 122)
                return { keyCode, ModifierKeys::currentModifiers, name[0] };

            if (keyCode >= 256 && keyCode <= 265)
                return { KeyPress::numberPad0 + (keyCode - 256), ModifierKeys::currentModifiers, String (keyCode - 256).getCharPointer()[0] };

            if (keyCode == 8)      return { KeyPress::backspaceKey,          ModifierKeys::currentModifiers, {} };
            if (keyCode == 127)    return { KeyPress::deleteKey,             ModifierKeys::currentModifiers, {} };
            if (keyCode == 9)      return { KeyPress::tabKey,                ModifierKeys::currentModifiers, {} };
            if (keyCode == 13)     return { KeyPress::returnKey,             ModifierKeys::currentModifiers, {} };
            if (keyCode == 27)     return { KeyPress::escapeKey,             ModifierKeys::currentModifiers, {} };
            if (keyCode == 32)     return { KeyPress::spaceKey,              ModifierKeys::currentModifiers, {} };
            if (keyCode == 266)    return { KeyPress::numberPadDecimalPoint, ModifierKeys::currentModifiers, {} };
            if (keyCode == 267)    return { KeyPress::numberPadDivide,       ModifierKeys::currentModifiers, {} };
            if (keyCode == 268)    return { KeyPress::numberPadMultiply,     ModifierKeys::currentModifiers, {} };
            if (keyCode == 269)    return { KeyPress::numberPadSubtract,     ModifierKeys::currentModifiers, {} };
            if (keyCode == 270)    return { KeyPress::numberPadAdd,          ModifierKeys::currentModifiers, {} };
            if (keyCode == 272)    return { KeyPress::numberPadEquals,       ModifierKeys::currentModifiers, {} };
            if (keyCode == 273)    return { KeyPress::upKey,                 ModifierKeys::currentModifiers, {} };
            if (keyCode == 274)    return { KeyPress::downKey,               ModifierKeys::currentModifiers, {} };
            if (keyCode == 275)    return { KeyPress::rightKey,              ModifierKeys::currentModifiers, {} };
            if (keyCode == 276)    return { KeyPress::leftKey,               ModifierKeys::currentModifiers, {} };

            return {};
        */
    }
    
    pub fn set_minimised(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_minimised(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn set_full_screen(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_full_screen(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn set_always_on_top(&mut self, _0: bool) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn to_front(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn to_behind(&mut self, _0: *mut ComponentPeer)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_focused(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn grab_focus(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_native_handle(&self)  {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    pub fn get_frame_size(&self) -> BorderSize<i32> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn set_visible(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_title(&mut self, _0: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_icon(&mut self, _0: &Image)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn text_input_required(&mut self, 
        _0: Point<i32>,
        _1: &mut TextInputTarget)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_alpha(&mut self, _0: f32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn perform_any_pending_repaints_now(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn repaint(&mut self, _0: &Rectangle<i32>)  {
        
        todo!();
        /*
        
        */
    }
}


///----------------------------
#[no_copy]
#[leak_detector]
#[cfg(target_feature = "unity")]
pub struct AudioProcessorUnityWrapper {
    plugin_instance:        Box<AudioProcessor>,
    plugin_instance_editor: Box<AudioProcessorEditor>,
    samples_per_block:      i32, // default = 1024
    parameter_descriptions: StringArray,
    scratch_buffer:         AudioBuffer<f32>,
    aloe_parameters:        LegacyAudioParametersWrapper,
}

#[cfg(target_feature = "unity")]
impl Drop for AudioProcessorUnityWrapper {
    fn drop(&mut self) {
        todo!();
        /* 
            if (pluginInstanceEditor != nullptr)
            {
                pluginInstanceEditor->removeFromDesktop();

                PopupMenu::dismissAllActiveMenus();
                pluginInstanceEditor->processor.editorBeingDeleted (pluginInstanceEditor.get());
                pluginInstanceEditor = nullptr;
            }
         */
    }
}

#[cfg(target_feature = "unity")]
impl AudioProcessorUnityWrapper {

    pub fn new(is_temporary: bool) -> Self {
    
        todo!();
        /*


            pluginInstance.reset (createPluginFilterOfType (AudioProcessor::wrapperType_Unity));

            if (! isTemporary && pluginInstance->hasEditor())
            {
                pluginInstanceEditor.reset (pluginInstance->createEditorIfNeeded());
                pluginInstanceEditor->setVisible (true);
                pluginInstanceEditor->addToDesktop (0);
            }

            aloeParameters.update (*pluginInstance, false);
        */
    }
    
    pub fn create(&mut self, state: *mut UnityAudioEffectState)  {
        
        todo!();
        /*
            // only supported in Unity plugin API > 1.0
            if (state->structSize >= sizeof (UnityAudioEffectState))
                samplesPerBlock = static_cast<int> (state->dspBufferSize);

           #ifdef AloePlugin_PreferredChannelConfigurations
            short configs[][2] = { AloePlugin_PreferredChannelConfigurations };
            const int numConfigs = sizeof (configs) / sizeof (short[2]);

            jassertquiet (numConfigs > 0 && (configs[0][0] > 0 || configs[0][1] > 0));

            pluginInstance->setPlayConfigDetails (configs[0][0], configs[0][1], state->sampleRate, samplesPerBlock);
           #else
            pluginInstance->setRateAndBufferSizeDetails (state->sampleRate, samplesPerBlock);
           #endif

            pluginInstance->prepareToPlay (state->sampleRate, samplesPerBlock);

            scratchBuffer.setSize (jmax (pluginInstance->getTotalNumInputChannels(), pluginInstance->getTotalNumOutputChannels()), samplesPerBlock);
        */
    }
    
    pub fn release(&mut self)  {
        
        todo!();
        /*
            pluginInstance->releaseResources();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            pluginInstance->reset();
        */
    }
    
    pub fn process(&mut self, 
        in_buffer:        *mut f32,
        out_buffer:       *mut f32,
        buffer_size:      i32,
        num_in_channels:  i32,
        num_out_channels: i32,
        is_bypassed:      bool)  {
        
        todo!();
        /*
            for (int pos = 0; pos < bufferSize;)
            {
                auto max = jmin (bufferSize - pos, samplesPerBlock);
                processBuffers (inBuffer + (pos * numInChannels), outBuffer + (pos * numOutChannels), max, numInChannels, numOutChannels, isBypassed);

                pos += max;
            }
        */
    }
    
    pub fn declare_parameters(&mut self, definition: &mut UnityAudioEffectDefinition)  {
        
        todo!();
        /*
            static std::unique_ptr<UnityAudioParameterDefinition> parametersPtr;
            static int numParams = 0;

            if (parametersPtr == nullptr)
            {
                numParams = aloeParameters.params.size();

                parametersPtr.reset (static_cast<UnityAudioParameterDefinition*> (std::calloc (static_cast<size_t> (numParams),
                                                                                  sizeof (UnityAudioParameterDefinition))));

                parameterDescriptions.clear();

                for (int i = 0; i < numParams; ++i)
                {
                    auto* parameter = aloeParameters.params[i];
                    auto& paramDef = parametersPtr.get()[i];

                    const auto nameLength = (size_t) numElementsInArray (paramDef.name);
                    const auto unitLength = (size_t) numElementsInArray (paramDef.unit);

                    parameter->getName ((int) nameLength - 1).copyToUTF8 (paramDef.name, nameLength);

                    if (parameter->getLabel().isNotEmpty())
                        parameter->getLabel().copyToUTF8 (paramDef.unit, unitLength);

                    parameterDescriptions.add (parameter->getName (15));
                    paramDef.description = parameterDescriptions[i].toRawUTF8();

                    paramDef.defaultVal = parameter->getDefaultValue();
                    paramDef.min = 0.0f;
                    paramDef.max = 1.0f;
                    paramDef.displayScale = 1.0f;
                    paramDef.displayExponent = 1.0f;
                }
            }

            definition.numParameters = static_cast<uint32> (numParams);
            definition.parameterDefintions = parametersPtr.get();
        */
    }
    
    pub fn set_parameter(&mut self, 
        index: i32,
        value: f32)  {
        
        todo!();
        /*
            aloeParameters.getParamForIndex (index)->setValueNotifyingHost (value);
        */
    }
    
    pub fn get_parameter(&self, index: i32) -> f32 {
        
        todo!();
        /*
            return aloeParameters.getParamForIndex (index)->getValue();
        */
    }
    
    pub fn get_parameter_string(&self, index: i32) -> String {
        
        todo!();
        /*
            auto* param = aloeParameters.getParamForIndex (index);
            return param->getText (param->getValue(), 16);
        */
    }
    
    pub fn get_num_input_channels(&self) -> i32 {
        
        todo!();
        /*
            return pluginInstance->getTotalNumInputChannels();
        */
    }
    
    pub fn get_num_output_channels(&self) -> i32 {
        
        todo!();
        /*
            return pluginInstance->getTotalNumOutputChannels();
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return pluginInstance->hasEditor();
        */
    }
    
    pub fn get_editor_peer(&self) -> &mut UnityPeer {
        
        todo!();
        /*
            auto* peer = dynamic_cast<UnityPeer*> (pluginInstanceEditor->getPeer());

            jassert (peer != nullptr);
            return *peer;
        */
    }
    
    pub fn process_buffers(&mut self, 
        in_buffer:        *mut f32,
        out_buffer:       *mut f32,
        buffer_size:      i32,
        num_in_channels:  i32,
        num_out_channels: i32,
        is_bypassed:      bool)  {
        
        todo!();
        /*
            int ch;
            for (ch = 0; ch < numInChannels; ++ch)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::Interleaved,    AudioData::Const>;

                DstSampleType dstData (scratchBuffer.getWritePointer (ch));
                SrcSampleType srcData (inBuffer + ch, numInChannels);
                dstData.convertSamples (srcData, bufferSize);
            }

            for (; ch < numOutChannels; ++ch)
                scratchBuffer.clear (ch, 0, bufferSize);

            {
                const ScopedLock sl (pluginInstance->getCallbackLock());

                if (pluginInstance->isSuspended())
                {
                    scratchBuffer.clear();
                }
                else
                {
                    MidiBuffer mb;

                    if (isBypassed)
                        pluginInstance->processBlockBypassed (scratchBuffer, mb);
                    else
                        pluginInstance->processBlock (scratchBuffer, mb);
                }
            }

            for (ch = 0; ch < numOutChannels; ++ch)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::Interleaved,    AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::Const>;

                DstSampleType dstData (outBuffer + ch, numOutChannels);
                SrcSampleType srcData (scratchBuffer.getReadPointer (ch));
                dstData.convertSamples (srcData, bufferSize);
            }
        */
    }
}

#[cfg(target_feature = "unity")]
pub fn get_wrapper_map() -> &mut HashMap<i32,*mut AudioProcessorUnityWrapper> {
    
    todo!();
    /*
        static HashMap<int, AudioProcessorUnityWrapper*> wrapperMap;
        return wrapperMap;
    */
}

#[cfg(target_feature = "unity")]
pub fn on_wrapper_creation(wrapper_to_add: *mut AudioProcessorUnityWrapper)  {
    
    todo!();
    /*
        getWrapperMap().set (std::abs (Random::getSystemRandom().nextInt (65536)), wrapperToAdd);
    */
}

#[cfg(target_feature = "unity")]
pub fn on_wrapper_deletion(wrapper_to_remove: *mut AudioProcessorUnityWrapper)  {
    
    todo!();
    /*
        getWrapperMap().removeValue (wrapperToRemove);
    */
}

pub mod unity_callbacks {
    use super::*;


    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn create_callback(state: *mut UnityAudioEffectState) -> i32 {
        
        todo!();
        /*
            auto* pluginInstance = new AudioProcessorUnityWrapper (false);
                pluginInstance->create (state);

                state->effectData = pluginInstance;

                onWrapperCreation (pluginInstance);

                return 0;
        */
    }

    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn release_callback(state: *mut UnityAudioEffectState) -> i32 {
        
        todo!();
        /*
            auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();
                pluginInstance->release();

                onWrapperDeletion (pluginInstance);
                delete pluginInstance;

                if (getWrapperMap().size() == 0)
                    shutdownAloe_GUI();

                return 0;
        */
    }

    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn reset_callback(state: *mut UnityAudioEffectState) -> i32 {
        
        todo!();
        /*
            auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();
                pluginInstance->reset();

                return 0;
        */
    }

    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn set_position_callback(
            state: *mut UnityAudioEffectState,
            pos:   u32) -> i32 {
        
        todo!();
        /*
            ignoreUnused (state, pos);

                return 0;
        */
    }

    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn set_float_parameter_callback(
            state: *mut UnityAudioEffectState,
            index: i32,
            value: f32) -> i32 {
        
        todo!();
        /*
            auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();
                pluginInstance->setParameter (index, value);

                return 0;
        */
    }

    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn get_float_parameter_callback(
            state:     *mut UnityAudioEffectState,
            index:     i32,
            value:     *mut f32,
            value_str: *mut u8) -> i32 {
        
        todo!();
        /*
            auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();
                *value = pluginInstance->getParameter (index);

                pluginInstance->getParameterString (index).copyToUTF8 (valueStr, 15);

                return 0;
        */
    }

    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn get_float_buffer_callback(
            state:       *mut UnityAudioEffectState,
            name:        *const u8,
            buffer:      *mut f32,
            num_samples: i32) -> i32 {
        
        todo!();
        /*
            ignoreUnused (numSamples);

                auto nameStr = String (name);

                if (nameStr == "Editor")
                {
                    auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();

                    buffer[0] = pluginInstance->hasEditor() ? 1.0f : 0.0f;
                }
                else if (nameStr == "ID")
                {
                    auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();

                    for (HashMap<int, AudioProcessorUnityWrapper*>::Iterator i (getWrapperMap()); i.next();)
                    {
                        if (i.getValue() == pluginInstance)
                        {
                            buffer[0] = (float) i.getKey();
                            break;
                        }
                    }

                    return 0;
                }
                else if (nameStr == "Size")
                {
                    auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();

                    auto& editor = pluginInstance->getEditorPeer().getEditor();

                    buffer[0] = (float) editor.getBounds().getWidth();
                    buffer[1] = (float) editor.getBounds().getHeight();
                    buffer[2] = (float) editor.getConstrainer()->getMinimumWidth();
                    buffer[3] = (float) editor.getConstrainer()->getMinimumHeight();
                    buffer[4] = (float) editor.getConstrainer()->getMaximumWidth();
                    buffer[5] = (float) editor.getConstrainer()->getMaximumHeight();
                }

                return 0;
        */
    }

    #[UNITY_INTERFACE_API]
    #[cfg(target_feature = "unity")]
    pub fn process_callback(
            state:            *mut UnityAudioEffectState,
            in_buffer:        *mut f32,
            out_buffer:       *mut f32,
            buffer_size:      u32,
            num_in_channels:  i32,
            num_out_channels: i32) -> i32 {
        
        todo!();
        /*
            auto* pluginInstance = state->getEffectData<AudioProcessorUnityWrapper>();

                if (pluginInstance != nullptr)
                {
                    auto isPlaying = ((state->flags & stateIsPlaying) != 0);
                    auto isMuted   = ((state->flags & stateIsMuted)   != 0);
                    auto isPaused  = ((state->flags & stateIsPaused)  != 0);

                    auto bypassed = ! isPlaying || (isMuted || isPaused);

                    pluginInstance->process (inBuffer, outBuffer, static_cast<int> (bufferSize), numInChannels, numOutChannels, bypassed);
                }
                else
                {
                    FloatVectorOperations::clear (outBuffer, static_cast<int> (bufferSize) * numOutChannels);
                }

                return 0;
        */
    }
}

#[cfg(target_feature = "unity")]
pub fn declare_effect(definition: &mut UnityAudioEffectDefinition)  {
    
    todo!();
    /*
        memset (&definition, 0, sizeof (definition));

        std::unique_ptr<AudioProcessorUnityWrapper> wrapper = std::make_unique<AudioProcessorUnityWrapper> (true);

        String name (AloePlugin_Name);
        if (! name.startsWithIgnoreCase ("audioplugin"))
            name = "audioplugin_" + name;

        name.copyToUTF8 (definition.name, (size_t) numElementsInArray (definition.name));

        definition.structSize = sizeof (UnityAudioEffectDefinition);
        definition.parameterStructSize = sizeof (UnityAudioParameterDefinition);

        definition.apiVersion = UNITY_AUDIO_PLUGIN_API_VERSION;
        definition.pluginVersion = AloePlugin_VersionCode;

        // effects must set this to 0, generators > 0
        definition.channels = (wrapper->getNumInputChannels() != 0 ? 0
                                                                   : static_cast<uint32> (wrapper->getNumOutputChannels()));

        wrapper->declareParameters (definition);

        definition.create            = UnityCallbacks::createCallback;
        definition.release           = UnityCallbacks::releaseCallback;
        definition.reset             = UnityCallbacks::resetCallback;
        definition.setPosition       = UnityCallbacks::setPositionCallback;
        definition.process           = UnityCallbacks::processCallback;
        definition.setFloatParameter = UnityCallbacks::setFloatParameterCallback;
        definition.getFloatParameter = UnityCallbacks::getFloatParameterCallback;
        definition.getFloatBuffer    = UnityCallbacks::getFloatBufferCallback;
    */
}

#[UNITY_INTERFACE_API]
#[UNITY_INTERFACE_EXPORT] 
#[cfg(target_feature = "unity")]
pub fn unity_get_audio_effect_definitions(definitions_ptr: *mut *mut *mut UnityAudioEffectDefinition) -> i32 {
    
    todo!();
    /*
        if (getWrapperMap().size() == 0)
            initialiseAloe_GUI();

        static bool hasInitialised = false;

        if (! hasInitialised)
        {
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_Unity;
            aloe_createUnityPeerFn = createUnityPeer;

            hasInitialised = true;
        }

        auto* definition = new UnityAudioEffectDefinition();
        declareEffect (*definition);

        *definitionsPtr = &definition;

        return 1;
    */
}

#[cfg(target_feature = "unity")]
pub fn unity_modifiers_toaloe(
        mods:         UnityEventModifiers,
        mouse_down:   bool,
        mouse_button: i32) -> ModifierKeys {
    let mouse_button: i32 = mouse_button.unwrap_or(-1);

    todo!();
    /*
        int flags = 0;

        if (mouseDown)
        {
            if (mouseButton == 0)
                flags |= ModifierKeys::leftButtonModifier;
            else if (mouseButton == 1)
                flags |= ModifierKeys::rightButtonModifier;
            else if (mouseButton == 2)
                flags |= ModifierKeys::middleButtonModifier;
        }

        if (mods == 0)
            return flags;

        if ((mods & UnityEventModifiers::shift) != 0)        flags |= ModifierKeys::shiftModifier;
        if ((mods & UnityEventModifiers::control) != 0)      flags |= ModifierKeys::ctrlModifier;
        if ((mods & UnityEventModifiers::alt) != 0)          flags |= ModifierKeys::altModifier;
        if ((mods & UnityEventModifiers::command) != 0)      flags |= ModifierKeys::commandModifier;

        return { flags };
    */
}

#[cfg(target_feature = "unity")]
pub fn get_wrapper_checked(id: i32) -> *mut AudioProcessorUnityWrapper {
    
    todo!();
    /*
        auto* wrapper = getWrapperMap()[id];
        jassert (wrapper != nullptr);

        return wrapper;
    */
}

#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn on_render_event(id: i32)  {
    
    todo!();
    /*
        getWrapperChecked (id)->getEditorPeer().triggerAsyncUpdate();
    */
}

#[UNITY_INTERFACE_EXPORT] 
#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn get_render_callback() -> renderCallback {
    
    todo!();
    /*
        return onRenderEvent;
    */
}

#[UNITY_INTERFACE_EXPORT] 
#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn unity_initialise_texture(
        id:   i32,
        data: *mut c_void,
        w:    i32,
        h:    i32)  {
    
    todo!();
    /*
        getWrapperChecked (id)->getEditorPeer().setPixelDataHandle (reinterpret_cast<uint8*> (data), w, h);
    */
}

#[UNITY_INTERFACE_EXPORT] 
#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn unity_mouse_down(
        id:         i32,
        x:          f32,
        y:          f32,
        unity_mods: UnityEventModifiers,
        button:     i32)  {
    
    todo!();
    /*
        getWrapperChecked (id)->getEditorPeer().forwardMouseEvent ({ x, y }, unityModifiersToALOE (unityMods, true, button));
    */
}

#[UNITY_INTERFACE_EXPORT] 
#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn unity_mouse_drag(
        id:         i32,
        x:          f32,
        y:          f32,
        unity_mods: UnityEventModifiers,
        button:     i32)  {
    
    todo!();
    /*
        getWrapperChecked (id)->getEditorPeer().forwardMouseEvent ({ x, y }, unityModifiersToALOE (unityMods, true, button));
    */
}

#[UNITY_INTERFACE_EXPORT] 
#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn unity_mouse_up(
        id:         i32,
        x:          f32,
        y:          f32,
        unity_mods: UnityEventModifiers)  {
    
    todo!();
    /*
        getWrapperChecked (id)->getEditorPeer().forwardMouseEvent ({ x, y }, unityModifiersToALOE (unityMods, false));
    */
}

#[UNITY_INTERFACE_EXPORT] 
#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn unity_key_event(
        id:   i32,
        code: i32,
        mods: UnityEventModifiers,
        name: *const u8)  {
    
    todo!();
    /*
        getWrapperChecked (id)->getEditorPeer().forwardKeyPress (code, name, unityModifiersToALOE (mods, false));
    */
}

#[UNITY_INTERFACE_EXPORT] 
#[UNITY_INTERFACE_API]
#[cfg(target_feature = "unity")]
pub fn unity_set_screen_bounds(
        id: i32,
        x:  f32,
        y:  f32,
        w:  f32,
        h:  f32)  {
    
    todo!();
    /*
        getWrapperChecked (id)->getEditorPeer().getEditor().setBounds ({ (int) x, (int) y, (int) w, (int) h });
    */
}

#[cfg(target_os="windows")]
#[cfg(target_feature = "unity")]
extern "C"  {

    #[WINAPI]
    pub fn dll_main(
            instance: HINSTANCE,
            reason:   u64,
            _2:       LPVOID) -> bool {
        
        todo!();
        /*
            if (reason == DLL_PROCESS_ATTACH)
                 Process::setCurrentModuleInstanceHandle (instance);

             return true;
        */
    }
}

