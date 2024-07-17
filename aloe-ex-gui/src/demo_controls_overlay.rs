crate::ix!();

/**
  | This component sits on top of the main
  | GL demo, and contains all the sliders
  | and widgets that control things.
  |
  */
#[no_copy]
#[leak_detector]
pub struct DemoControlsOverlay<'a> {
    base:                   Component<'a>,
    base4:                  Timer,
    status_label:           Label<'a>,
    demo:                   &'a mut OpenGLDemo<'a>,
    speed_label:            Label<'a>, // default = { "Speed:"  }
    zoom_label:             Label<'a>, // default = { "Zoom:"  }
    vertex_document:        CodeDocument<'a>,
    fragment_document:      CodeDocument<'a>,
    vertex_editor_comp:     CodeEditorComponent<'a>, // default = { vertexDocument,   nullptr  }
    fragment_editor_comp:   CodeEditorComponent<'a>, // default = { fragmentDocument, nullptr  }
    tabbed_comp:            TabbedComponent<'a>, // default = { TabbedButtonBar::TabsAtLeft  }
    preset_box:             ComboBox<'a>,
    texture_box:            ComboBox<'a>,
    preset_label:           Label<'a>, // default = { "Shader Preset:"  }
    texture_label:          Label<'a>, // default = { "Texture:"  }
    speed_slider:           Slider<'a>,
    size_slider:            Slider<'a>,
    show_background_toggle: ToggleButton<'a>, // default = { "Draw 2D graphics in background"  }
    textures:               Vec<Box<OpenGLAppDemoTexture>>,
    texture_file_chooser:   Box<FileChooser<'a>>,
}

pub const DEMO_CONTROLS_OVERLAY_SHADER_LINK_DELAY: usize = 500;

impl<'a> CodeDocumentListener for DemoControlsOverlay<'a> {

    fn code_document_text_inserted(&mut self, _: &str, _: i32) { todo!() }

    fn code_document_text_deleted(&mut self, _: i32, _: i32) { todo!() }
}

impl<'a> SliderListener for DemoControlsOverlay<'a> {

}

impl<'a> SliderDragStarted for DemoControlsOverlay<'a> {
    fn slider_drag_started(&mut self, _: *mut aloe_slider::Slider<'_>) { todo!() }
}

impl<'a> SliderDragEnded for DemoControlsOverlay<'a> {

    fn slider_drag_ended(&mut self, _: *mut aloe_slider::Slider<'_>) { todo!() }
}

impl<'a> Initialize for DemoControlsOverlay<'a> {

    fn initialise(&mut self)  {
        
        todo!();
        /*
            showBackgroundToggle.setToggleState (false, sendNotification);
                textureBox.setSelectedItemIndex (0);
                presetBox .setSelectedItemIndex (0);
                speedSlider.setValue (0.01);
                sizeSlider .setValue (0.5);
        */
    }
}

impl<'a> Resized for DemoControlsOverlay<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds().reduced (4);

                auto top = area.removeFromTop (75);

                auto sliders = top.removeFromRight (area.getWidth() / 2);
                showBackgroundToggle.setBounds (sliders.removeFromBottom (25));
                speedSlider         .setBounds (sliders.removeFromBottom (25));
                sizeSlider          .setBounds (sliders.removeFromBottom (25));

                top.removeFromRight (70);
                statusLabel.setBounds (top);

                auto shaderArea = area.removeFromBottom (area.getHeight() / 2);

                auto presets = shaderArea.removeFromTop (25);
                presets.removeFromLeft (100);
                presetBox.setBounds (presets.removeFromLeft (150));
                presets.removeFromLeft (100);
                textureBox.setBounds (presets);

                shaderArea.removeFromTop (4);
                tabbedComp.setBounds (shaderArea);
        */
    }
}

impl<'a> MouseDown for DemoControlsOverlay<'a> {

    fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            demo.draggableOrientation.mouseDown (e.getPosition());
        */
    }
}

impl<'a> MouseDrag for DemoControlsOverlay<'a> {

    fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            demo.draggableOrientation.mouseDrag (e.getPosition());
        */
    }
}

impl<'a> MouseWheelMove for DemoControlsOverlay<'a> {

    fn mouse_wheel_move(&mut self, 
        _0: &MouseEvent,
        d:  &MouseWheelDetails)  {
        
        todo!();
        /*
            sizeSlider.setValue (sizeSlider.getValue() + d.deltaY);
        */
    }
}

impl<'a> MouseMagnify for DemoControlsOverlay<'a> {

    fn mouse_magnify(&mut self, 
        _0:              &MouseEvent,
        magnify_ammount: f32)  {
        
        todo!();
        /*
            sizeSlider.setValue (sizeSlider.getValue() + magnifyAmmount - 1.0f);
        */
    }
}

pub trait SelectPreset {

    fn select_preset(&mut self, preset: i32);
}

impl<'a> SelectPreset for DemoControlsOverlay<'a> {

    fn select_preset(&mut self, preset: i32)  {
        
        todo!();
        /*
            const auto& p = OpenGLUtils::getPresets()[preset];

                vertexDocument  .replaceAllContent (p.vertexShader);
                fragmentDocument.replaceAllContent (p.fragmentShader);

                startTimer (1);
        */
    }
}

pub trait SelectTexture {

    fn select_texture(&mut self, itemid: i32);
}

impl<'a> SelectTexture for DemoControlsOverlay<'a> {

    fn select_texture(&mut self, itemid: i32)  {
        
        todo!();
        /*
            if (itemID == 1000)
                {
                    textureFileChooser = std::make_unique<FileChooser> ("Choose an image to open...",
                                                                        File::getSpecialLocation (File::userPicturesDirectory),
                                                                        "*.jpg;*.jpeg;*.png;*.gif");
                    auto chooserFlags = FileBrowserComponent::openMode | FileBrowserComponent::canSelectFiles;

                    textureFileChooser->launchAsync (chooserFlags, [this] (const FileChooser& fc)
                    {
                        if (fc.getResult() == File{})
                            return;

                        textures.add (new OpenGLUtils::OpenGLAppDemoTextureFromFile (fc.getResult()));
                        updateTexturesList();

                        textureBox.setSelectedId (textures.size());
                    });
                }
                else
                {
                    if (auto* t = textures[itemID - 1])
                        demo.setTexture (t);
                }
        */
    }
}

pub trait UpdateTexturesList {

    fn update_textures_list(&mut self);
}

impl<'a> UpdateTexturesList for DemoControlsOverlay<'a> {

    fn update_textures_list(&mut self)  {
        
        todo!();
        /*
            textureBox.clear();

                for (int i = 0; i < textures.size(); ++i)
                    textureBox.addItem (textures.getUnchecked (i)->name, i + 1);

                textureBox.addSeparator();
                textureBox.addItem ("Load from a file...", 1000);
        */
    }
}

pub trait UpdateShader {

    fn update_shader(&mut self);
}

impl<'a> UpdateShader for DemoControlsOverlay<'a> {

    fn update_shader(&mut self)  {
        
        todo!();
        /*
            startTimer (10);
        */
    }
}

impl<'a> SliderValueChanged for DemoControlsOverlay<'a> {

    fn slider_value_changed(&mut self, _0: *mut Slider)  {
        
        todo!();
        /*
            demo.scale         = (float) sizeSlider .getValue();
                demo.rotationSpeed = (float) speedSlider.getValue();
        */
    }
}

pub trait TimerCallback {

    fn timer_callback(&mut self);
}

impl<'a> TimerCallback for DemoControlsOverlay<'a> {

    fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();
                demo.setShaderProgram (vertexDocument  .getAllContent(),
                                       fragmentDocument.getAllContent());
        */
    }
}

impl<'a> LookAndFeelChanged for DemoControlsOverlay<'a> {

    fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            auto editorBackground = getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                                                Colours::white);

                for (int i = tabbedComp.getNumTabs(); i >= 0; --i)
                    tabbedComp.setTabBackgroundColour (i, editorBackground);

                vertexEditorComp  .setColour (CodeEditorComponent::backgroundColourId, editorBackground);
                fragmentEditorComp.setColour (CodeEditorComponent::backgroundColourId, editorBackground);
        */
    }
}

pub trait CodeDocumentTextInserted {

    fn code_document_text_inserted(
        &mut self, 
        new_text:     &String,
        insert_index: i32
    );
}

impl<'a> CodeDocumentTextInserted for DemoControlsOverlay<'a> {

    fn code_document_text_inserted(&mut self, 
        new_text:     &String,
        insert_index: i32)  {
        
        todo!();
        /*
            startTimer (shaderLinkDelay);
        */
    }
}

pub trait CodeDocumentTextDeleted {

    fn code_document_text_deleted(
        &mut self, 
        start_index: i32,
        end_index:   i32);
}

impl<'a> CodeDocumentTextDeleted for DemoControlsOverlay<'a> {

    fn code_document_text_deleted(&mut self, 
        start_index: i32,
        end_index:   i32)  {
        
        todo!();
        /*
            startTimer (shaderLinkDelay);
        */
    }
}

impl<'a> DemoControlsOverlay<'a> {

    pub fn new(d: &mut OpenGLDemo) -> Self {
    
        todo!();
        /*
        : demo(d),

            addAndMakeVisible (statusLabel);
                statusLabel.setJustificationType (Justification::topLeft);
                statusLabel.setFont (Font (14.0f));

                addAndMakeVisible (sizeSlider);
                sizeSlider.setRange (0.0, 1.0, 0.001);
                sizeSlider.addListener (this);

                addAndMakeVisible (zoomLabel);
                zoomLabel.attachToComponent (&sizeSlider, true);

                addAndMakeVisible (speedSlider);
                speedSlider.setRange (0.0, 0.5, 0.001);
                speedSlider.addListener (this);
                speedSlider.setSkewFactor (0.5f);

                addAndMakeVisible (speedLabel);
                speedLabel.attachToComponent (&speedSlider, true);

                addAndMakeVisible (showBackgroundToggle);
                showBackgroundToggle.onClick = [this] { demo.doBackgroundDrawing = showBackgroundToggle.getToggleState(); };

                addAndMakeVisible (tabbedComp);
                tabbedComp.setTabBarDepth (25);
                tabbedComp.setColour (TabbedButtonBar::tabTextColourId, Colours::grey);
                tabbedComp.addTab ("Vertex", Colours::transparentBlack, &vertexEditorComp, false);
                tabbedComp.addTab ("Fragment", Colours::transparentBlack, &fragmentEditorComp, false);

                vertexDocument.addListener (this);
                fragmentDocument.addListener (this);

                textures.add (new OpenGLUtils::OpenGLAppDemoTextureFromAsset ("portmeirion.jpg"));
                textures.add (new OpenGLUtils::OpenGLAppDemoTextureFromAsset ("tile_background.png"));
                textures.add (new OpenGLUtils::OpenGLAppDemoTextureFromAsset ("aloe_icon.png"));
                textures.add (new OpenGLUtils::OpenGLAppDemoDynamicTexture());

                addAndMakeVisible (textureBox);
                textureBox.onChange = [this] { selectTexture (textureBox.getSelectedId()); };
                updateTexturesList();

                addAndMakeVisible (presetBox);
                presetBox.onChange = [this] { selectPreset (presetBox.getSelectedItemIndex()); };

                auto presets = OpenGLUtils::getPresets();

                for (int i = 0; i < presets.size(); ++i)
                    presetBox.addItem (presets[i].name, i + 1);

                addAndMakeVisible (presetLabel);
                presetLabel.attachToComponent (&presetBox, true);

                addAndMakeVisible (textureLabel);
                textureLabel.attachToComponent (&textureBox, true);

                lookAndFeelChanged();
        */
    }
}
