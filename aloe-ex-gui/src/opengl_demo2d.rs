crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/OpenGLDemo2D.h]

#[no_copy]
#[leak_detector]
pub struct OpenGLDemo2D<'a> {
    base:                 Component<'a>,
    base3:                Timer,
    shader:               Box<OpenGLGraphicsContextCustomShader>,
    status_label:         Label<'a>,
    preset_label:         Label<'a>, // default = { {}, "Shader Preset:"  }
    preset_box:           ComboBox<'a>,
    fragment_document:    CodeDocument<'a>,
    fragment_editor_comp: CodeEditorComponent<'a>, // default = { fragmentDocument, nullptr  }
    fragment_code:        String,
    open_gl_context:      OpenGLContext<'a>,
}

impl<'a> CodeDocumentListener for OpenGLDemo2D<'a> {

    fn code_document_text_inserted(
        &mut self, 
        new_text:     &str,
        insert_index: i32
    ) {
        todo!();

        /*
            startTimer (shaderLinkDelay);
        */
    }
    
    fn code_document_text_deleted(
        &mut self, 
        start_index: i32,
        end_index:   i32
    ) {
        todo!();

        /*
            startTimer (shaderLinkDelay);
        */
    }
}

impl<'a> Default for OpenGLDemo2D<'a> {
    
    fn default() -> Self {

        todo!();

        /*


            setOpaque (true);

            if (auto* peer = getPeer())
                peer->setCurrentRenderingEngine (0);

            openGLContext.attachTo (*getTopLevelComponent());

            addAndMakeVisible (statusLabel);
            statusLabel.setJustificationType (Justification::topLeft);
            statusLabel.setFont (Font (14.0f));

            auto presets = getPresets();

            for (int i = 0; i < presets.size(); ++i)
                presetBox.addItem (presets[i].name, i + 1);

            addAndMakeVisible (presetLabel);
            presetLabel.attachToComponent (&presetBox, true);

            addAndMakeVisible (presetBox);
            presetBox.onChange = [this] { selectPreset (presetBox.getSelectedItemIndex()); };

            fragmentEditorComp.setOpaque (false);
            fragmentDocument.addListener (this);
            addAndMakeVisible (fragmentEditorComp);

            presetBox.setSelectedItemIndex (0);

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for OpenGLDemo2D<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            openGLContext.detach();
            shader.reset();
         */
    }
}

impl<'a> OpenGLDemo2D<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillCheckerBoard (getLocalBounds().toFloat(), 48.0f, 48.0f, Colours::lightgrey, Colours::white);

            if (shader.get() == nullptr || shader->getFragmentShaderCode() != fragmentCode)
            {
                shader.reset();

                if (fragmentCode.isNotEmpty())
                {
                    shader.reset (new OpenGLGraphicsContextCustomShader (fragmentCode));

                    auto result = shader->checkCompilation (g.getInternalContext());

                    if (result.failed())
                    {
                        statusLabel.setText (result.getErrorMessage(), dontSendNotification);
                        shader.reset();
                    }
                }
            }

            if (shader.get() != nullptr)
            {
                statusLabel.setText ({}, dontSendNotification);

                shader->fillRect (g.getInternalContext(), getLocalBounds());
            }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds().reduced (4);

            statusLabel.setBounds (area.removeFromTop (75));

            area.removeFromTop (area.getHeight() / 2);

            auto presets = area.removeFromTop (25);
            presets.removeFromLeft (100);
            presetBox.setBounds (presets.removeFromLeft (150));

            area.removeFromTop (4);
            fragmentEditorComp.setBounds (area);
        */
    }
    
    pub fn select_preset(&mut self, preset: i32)  {
        
        todo!();
        /*
            fragmentDocument.replaceAllContent (getPresets()[preset].fragmentShader);
            startTimer (1);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();
            fragmentCode = fragmentDocument.getAllContent();
            repaint();
        */
    }
}
