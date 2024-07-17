crate::ix!();

#[cfg(not(any(target_os="android",target_os="ios")))]
pub struct CodeContent<'a> {
    base:          Component<'a>,
    document:      CodeDocument<'a>,
    cpp_tokensier: CPlusPlusCodeTokeniser,
    code_editor:   CodeEditorComponent<'a>, // { document, &cppTokensier };
}

impl<'a> Default for CodeContent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (codeEditor);

            codeEditor.setReadOnly (true);
            codeEditor.setScrollbarThickness (8);

            lookAndFeelChanged()
        */
    }
}

impl<'a> CodeContent<'a> {
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            codeEditor.setBounds (getLocalBounds());
        */
    }
    
    pub fn set_default_code_content(&mut self)  {
        
        todo!();
        /*
            document.replaceAllContent ("\n/*******************************************************************************\n"
                                        "          Select one of the demos from the side panel on the left to see\n"
                                        "            its code here and an instance running in the \"Demo\" tab!\n"
                                        "*******************************************************************************/\n");
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            auto* v4 = dynamic_cast <LookAndFeel_V4*> (&Desktop::getInstance().getDefaultLookAndFeel());

            if (v4 != nullptr && (v4->getCurrentColourScheme() != LookAndFeel_V4::getLightColourScheme()))
                codeEditor.setColourScheme (getDarkColourScheme());
            else
                codeEditor.setColourScheme (getLightColourScheme());
        */
    }
}
