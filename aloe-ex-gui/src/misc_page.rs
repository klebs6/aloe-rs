crate::ix!();

pub struct MiscPage<'a> {
    base:         Component<'a>,
    text_editor1: TextEditor<'a>,
    text_editor2: TextEditor<'a>, //{ "Password", (aloe_wchar) 0x2022 };
    combo_box:    ComboBox<'a>, // default = { "Combo"  }
}

impl<'a> Default for MiscPage<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (textEditor1);
            textEditor1.setBounds (10, 25, 200, 24);
            textEditor1.setText ("Single-line text box");

            addAndMakeVisible (textEditor2);
            textEditor2.setBounds (10, 55, 200, 24);
            textEditor2.setText ("Password");

            addAndMakeVisible (comboBox);
            comboBox.setBounds (10, 85, 200, 24);
            comboBox.setEditableText (true);
            comboBox.setJustificationType (Justification::centred);

            for (int i = 1; i < 100; ++i)
                comboBox.addItem ("combo box item " + String (i), i);

            comboBox.setSelectedId (1)
        */
    }
}

impl<'a> MiscPage<'a> {

    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            textEditor1.applyFontToAllText (textEditor1.getFont());
            textEditor2.applyFontToAllText (textEditor2.getFont());
        */
    }
}
