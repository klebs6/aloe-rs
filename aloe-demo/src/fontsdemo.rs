crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/FontsDemo.h]

#[no_copy]
#[leak_detector]
pub struct FontsDemo<'a> {
    base:                             Component<'a>,
    base2:                            ListBoxModel,
    fonts:                            Vec<Font>,
    current_style_list:               Vec<String>,
    list_box:                         ListBox<'a>,
    demo_text_box:                    TextEditor<'a>,
    default_scale:                    f64,             // default = 1.0
    default_height:                   f64,             // default = 20.0
    default_kerning:                  f64,             // default = 0.0
    default_bold:                     bool,            // default = false
    default_italic:                   bool,            // default = false
    default_underlined:               bool,            // default = false
    default_style:                    i32,             // default = 0
    default_horizontal_justification: i32,             // default = 0
    default_vertical_justification:   i32,             // default = 0
    height_label:                     Label<'a>,           // default = "Height:" 
    kerning_label:                    Label<'a>,           // default = "Kerning:" 
    scale_label:                      Label<'a>,           // default = "Scale:" 
    style_label:                      Label<'a>,           // default = "Style:" 
    horizontal_justification_label:   Label<'a>,           // default = "Justification (horizontal):" 
    vertical_justification_label:     Label<'a>,           // default = "Justification (vertical):" 
    bold_toggle:                      ToggleButton<'a>,    // default = "Bold" 
    italic_toggle:                    ToggleButton<'a>,    // default = "Italic" 
    underline_toggle:                 ToggleButton<'a>,    // default = "Underlined" 
    reset_button:                     TextButton<'a>,      // default = "Reset" 
    height_slider:                    Slider<'a>,
    kerning_slider:                   Slider<'a>,
    scale_slider:                     Slider<'a>,
    style_box:                        ComboBox<'a>,
    horizontal_justification_box:     ComboBox<'a>,
    vertical_justification_box:       ComboBox<'a>,
    vertical_layout:                  StretchableLayoutManager,
    vertical_divider_bar:             Box<StretchableLayoutResizerBar<'a>>,
    horizontal_justification_strings: Vec<String>,     //{ "Left", "Centred", "Right" };
    vertical_justification_strings:   Vec<String>,     //{ "Top",  "Centred", "Bottom" };
    horizontal_justification_flags:   Vec<i32>,      //{ Justification::left, Justification::horizontallyCentred, Justification::right };
    vertical_justification_flags:     Vec<i32>,      //{ Justification::top,  Justification::verticallyCentred, Justification::bottom};
}

impl<'a> SliderListener for FontsDemo<'a> {

}

impl<'a> SliderDragEnded for FontsDemo<'a> {

    fn slider_drag_ended(&mut self, _: *mut aloe_slider::Slider<'_>) { }
}

impl<'a> SliderDragStarted for FontsDemo<'a> {

    fn slider_drag_started(&mut self, _: *mut aloe_slider::Slider<'_>) { }
}

impl<'a> SliderValueChanged for FontsDemo<'a> {

    fn slider_value_changed(&mut self, slider_that_was_moved: *mut Slider)  {
        
        todo!();
        /*
            if      (sliderThatWasMoved == &heightSlider)   refreshPreviewBoxFont();
            else if (sliderThatWasMoved == &kerningSlider)  refreshPreviewBoxFont();
            else if (sliderThatWasMoved == &scaleSlider)    refreshPreviewBoxFont();
        */
    }
}

impl<'a> Default for FontsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            addAndMakeVisible (listBox);
            addAndMakeVisible (demoTextBox);
            addAndMakeVisible (heightSlider);
            addAndMakeVisible (heightLabel);
            addAndMakeVisible (kerningLabel);
            addAndMakeVisible (kerningSlider);
            addAndMakeVisible (scaleLabel);
            addAndMakeVisible (horizontalJustificationLabel);
            addAndMakeVisible (verticalJustificationLabel);
            addAndMakeVisible (scaleSlider);
            addAndMakeVisible (boldToggle);
            addAndMakeVisible (italicToggle);
            addAndMakeVisible (underlineToggle);
            addAndMakeVisible (styleBox);
            addAndMakeVisible (horizontalJustificationBox);
            addAndMakeVisible (verticalJustificationBox);
            addAndMakeVisible (resetButton);

            kerningLabel                .attachToComponent (&kerningSlider,              true);
            heightLabel                 .attachToComponent (&heightSlider,               true);
            scaleLabel                  .attachToComponent (&scaleSlider,                true);
            styleLabel                  .attachToComponent (&styleBox,                   true);
            horizontalJustificationLabel.attachToComponent (&horizontalJustificationBox, true);
            verticalJustificationLabel  .attachToComponent (&verticalJustificationBox,   true);

            heightSlider .addListener (this);
            kerningSlider.addListener (this);
            scaleSlider  .addListener (this);

            boldToggle     .onClick  = [this] { refreshPreviewBoxFont(); };
            italicToggle   .onClick  = [this] { refreshPreviewBoxFont(); };
            underlineToggle.onClick  = [this] { refreshPreviewBoxFont(); };
            styleBox       .onChange = [this] { refreshPreviewBoxFont(); };

            Font::findFonts (fonts);   // Generate the list of fonts

            listBox.setTitle ("Fonts");
            listBox.setRowHeight (20);
            listBox.setModel (this);   // Tell the listbox where to get its data model
            listBox.setColour (ListBox::textColourId, Colours::black);
            listBox.setColour (ListBox::backgroundColourId, Colours::white);

            heightSlider .setRange (3.0, 150.0, 0.01);
            scaleSlider  .setRange (0.2, 3.0, 0.01);
            kerningSlider.setRange (-2.0, 2.0, 0.01);

            // set up the layout and resizer bars..
            verticalLayout.setItemLayout (0, -0.2, -0.8, -0.35); // width of the font list must be
                                                                 // between 20% and 80%, preferably 50%
            verticalLayout.setItemLayout (1, 8, 8, 8);           // the vertical divider drag-bar thing is always 8 pixels wide
            verticalLayout.setItemLayout (2, 150, -1.0, -0.65);  // the components on the right must be
                                                                 // at least 150 pixels wide, preferably 50% of the total width

            verticalDividerBar.reset (new StretchableLayoutResizerBar (&verticalLayout, 1, true));
            addAndMakeVisible (verticalDividerBar.get());

            // ..and pick a random font to select initially
            listBox.selectRow (Random::getSystemRandom().nextInt (fonts.size()));

            demoTextBox.setMultiLine (true);
            demoTextBox.setReturnKeyStartsNewLine (true);
            demoTextBox.setText ("Aa Bb Cc Dd Ee Ff Gg Hh Ii\n"
                                 "Jj Kk Ll Mm Nn Oo Pp Qq Rr\n"
                                 "Ss Tt Uu Vv Ww Xx Yy Zz\n"
                                 "0123456789\n\n"
                                 "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt "
                                 "ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco "
                                 "laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in "
                                 "voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat "
                                 "non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");

            demoTextBox.setCaretPosition (0);
            demoTextBox.setColour (TextEditor::textColourId, Colours::black);
            demoTextBox.setColour (TextEditor::backgroundColourId, Colours::white);

            demoTextBox.setWhitespaceUnderlined (false);

            resetButton.onClick = [this] { resetToDefaultParameters(); };

            setupJustificationOptions();
            resetToDefaultParameters();

            setSize (750, 750)
        */
    }
}

impl<'a> FontsDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (5);

            // lay out the list box and vertical divider..
            Component* vcomps[] = { &listBox, verticalDividerBar.get(), nullptr };

            verticalLayout.layOutComponents (vcomps, 3,
                                             r.getX(), r.getY(), r.getWidth(), r.getHeight(),
                                             false,     // lay out side-by-side
                                             true);     // resize the components' heights as well as widths

            r.removeFromLeft (verticalDividerBar->getRight());

            resetButton.setBounds (r.removeFromBottom (30).reduced (jmax (20, r.getWidth() / 5), 0));
            r.removeFromBottom (8);

            const int labelWidth = 60;

            auto styleArea = r.removeFromBottom (26);
            styleArea.removeFromLeft (labelWidth);
            styleBox.setBounds (styleArea);
            r.removeFromBottom (8);

            auto row = r.removeFromBottom (30);
            row.removeFromLeft (labelWidth);
            auto toggleWidth = row.getWidth() / 3;
            boldToggle     .setBounds (row.removeFromLeft (toggleWidth));
            italicToggle   .setBounds (row.removeFromLeft (toggleWidth));
            underlineToggle.setBounds (row);

            r.removeFromBottom (8);
            horizontalJustificationBox.setBounds (r.removeFromBottom (30).withTrimmedLeft (labelWidth * 3));
            r.removeFromBottom (8);
            verticalJustificationBox.setBounds (r.removeFromBottom (30).withTrimmedLeft (labelWidth * 3));
            r.removeFromBottom (8);
            scaleSlider.setBounds (r.removeFromBottom (30).withTrimmedLeft (labelWidth));
            r.removeFromBottom (8);
            kerningSlider.setBounds (r.removeFromBottom (30).withTrimmedLeft (labelWidth));
            r.removeFromBottom (8);
            heightSlider.setBounds (r.removeFromBottom (30).withTrimmedLeft (labelWidth));
            r.removeFromBottom (8);
            demoTextBox.setBounds (r);
        */
    }
    

    /**
       The following methods implement the
       ListBoxModel virtual methods:
      */
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return fonts.size();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row_number:      i32,
        g:               &mut Graphics,
        width:           i32,
        height:          i32,
        row_is_selected: bool)  {
        
        todo!();
        /*
            if (rowIsSelected)
                g.fillAll (Colours::lightblue);

            auto font = fonts[rowNumber];

            AttributedString s;
            s.setWordWrap (AttributedString::none);
            s.setJustification (Justification::centredLeft);
            s.append (getNameForRow (rowNumber), font.withHeight ((float) height * 0.7f), Colours::black);
            s.append ("   " + font.getTypefaceName(), Font ((float) height * 0.5f, Font::italic), Colours::grey);

            s.draw (g, Rectangle<int> (width, height).expanded (-4, 50).toFloat());
        */
    }
    
    pub fn get_name_for_row(&mut self, row_number: i32) -> String {
        
        todo!();
        /*
            return fonts[rowNumber].getTypefaceName();
        */
    }
    
    pub fn selected_rows_changed(&mut self, last_rowselected: i32)  {
        
        todo!();
        /*
            refreshPreviewBoxFont();
        */
    }
    
    pub fn reset_to_default_parameters(&mut self)  {
        
        todo!();
        /*
            scaleSlider  .setValue (defaultScale);
            heightSlider .setValue (defaultHeight);
            kerningSlider.setValue (defaultKerning);

            boldToggle     .setToggleState (defaultBold,       sendNotificationSync);
            italicToggle   .setToggleState (defaultItalic,     sendNotificationSync);
            underlineToggle.setToggleState (defaultUnderlined, sendNotificationSync);

            styleBox.setSelectedItemIndex (defaultStyle);
            horizontalJustificationBox.setSelectedItemIndex (defaultHorizontalJustification);
            verticalJustificationBox  .setSelectedItemIndex (defaultVerticalJustification);
        */
    }
    
    pub fn setup_justification_options(&mut self)  {
        
        todo!();
        /*
            horizontalJustificationBox.addItemList (horizontalJustificationStrings, 1);
            verticalJustificationBox  .addItemList (verticalJustificationStrings, 1);

            auto updateJustification = [this]()
            {
                auto horizontalIndex = horizontalJustificationBox.getSelectedItemIndex();
                auto verticalIndex   = verticalJustificationBox.getSelectedItemIndex();

                auto horizontalJustification = horizontalJustificationFlags[horizontalIndex];
                auto verticalJustification   = verticalJustificationFlags[verticalIndex];

                demoTextBox.setJustification (horizontalJustification | verticalJustification);
            };

            horizontalJustificationBox.onChange = updateJustification;
            verticalJustificationBox  .onChange = updateJustification;
        */
    }
    
    pub fn refresh_preview_box_font(&mut self)  {
        
        todo!();
        /*
            auto bold   = boldToggle  .getToggleState();
            auto italic = italicToggle.getToggleState();
            auto useStyle = ! (bold || italic);

            auto font = fonts[listBox.getSelectedRow()];

            font = font.withPointHeight        ((float) heightSlider .getValue())
                       .withExtraKerningFactor ((float) kerningSlider.getValue())
                       .withHorizontalScale    ((float) scaleSlider  .getValue());

            if (bold)    font = font.boldened();
            if (italic)  font = font.italicised();

            updateStylesList (font);

            styleBox.setEnabled (useStyle);

            if (useStyle)
                font = font.withTypefaceStyle (styleBox.getText());

            font.setUnderline (underlineToggle.getToggleState());

            demoTextBox.applyFontToAllText (font);
        */
    }
    
    pub fn update_styles_list(&mut self, new_font: &Font)  {
        
        todo!();
        /*
            auto newStyles = newFont.getAvailableStyles();

            if (newStyles != currentStyleList)
            {
                currentStyleList = newStyles;

                styleBox.clear();
                styleBox.addItemList (newStyles, 1);
                styleBox.setSelectedItemIndex (defaultStyle);
            }
        */
    }
}
