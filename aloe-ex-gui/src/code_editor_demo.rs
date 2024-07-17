crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/CodeEditorDemo.h]

#[no_copy]
#[leak_detector]
pub struct CodeEditorDemo<'a> {

    base:          Component<'a>,

    /**
      | this is the document that the editor
      | component is showing
      |
      */
    code_document: CodeDocument<'a>,

    /**
      | this is a tokeniser to apply the C++ syntax
      | highlighting
      |
      */
    cpp_tokeniser: CPlusPlusCodeTokeniser,

    /**
      | the editor component
      |
      */
    editor:        Box<CodeEditorComponent<'a>>,

    file_chooser:  FilenameComponent<'a>, 
}

pub fn code_editor_demo_default_file_chooser<'a>() -> FilenameComponent<'a> {
    todo!();
    /*
    { "File", {}, true, false, false,
    "*.cpp;*.h;*.hpp;*.c;*.mm;*.m", {},
    "Choose a C++ file to open it in the
    editor" };
    */
}

impl<'a> FilenameComponentListener for CodeEditorDemo<'a> {

    fn filename_component_changed(&mut self, _0: *mut FilenameComponent)  {
        
        todo!();
        /*
            editor->loadContent (fileChooser.getCurrentFile().loadFileAsString());
        */
    }
}

impl<'a> Default for CodeEditorDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            // Create the editor..
            editor.reset (new CodeEditorComponent (codeDocument, &cppTokeniser));
            addAndMakeVisible (editor.get());

            editor->loadContent ("\n"
                                 "/* Code editor demo!\n"
                                 "\n"
                                 "   To see a real-world example of the code editor\n"
                                 "   in action, have a look at the Proaloer!\n"
                                 "\n"
                                 "*/\n"
                                 "\n");

            // Create a file chooser control to load files into it..
            addAndMakeVisible (fileChooser);
            fileChooser.addListener (this);

            lookAndFeelChanged();

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for CodeEditorDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            fileChooser.removeListener (this);
         */
    }
}

impl<'a> Paint for CodeEditorDemo<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colours::lightgrey));
        */
    }
}

impl<'a> Resized for CodeEditorDemo<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (8);

            fileChooser.setBounds (r.removeFromTop (25));
            editor->setBounds     (r.withTrimmedTop (8));
        */
    }
}

impl<'a> CodeEditorDemo<'a> {
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            if (auto* v4 = dynamic_cast<LookAndFeel_V4*> (&LookAndFeel::getDefaultLookAndFeel()))
            {
                auto useLight = v4->getCurrentColourScheme() == LookAndFeel_V4::getLightColourScheme();
                editor->setColourScheme (useLight ? getLightCodeEditorColourScheme()
                                                  : getDarkCodeEditorColourScheme());
            }
            else
            {
                editor->setColourScheme (cppTokeniser.getDefaultColourScheme());
            }
        */
    }
    
    pub fn get_dark_code_editor_colour_scheme(&mut self) -> CodeEditorComponentColourScheme {
        
        todo!();
        /*
            struct Type
            {
                const char* name;
                uint32 colour;
            };

            const Type types[] =
            {
                { "Error",              0xffe60000 },
                { "Comment",            0xff72d20c },
                { "Keyword",            0xffee6f6f },
                { "Operator",           0xffc4eb19 },
                { "Identifier",         0xffcfcfcf },
                { "Integer",            0xff42c8c4 },
                { "Float",              0xff885500 },
                { "String",             0xffbc45dd },
                { "Bracket",            0xff058202 },
                { "Punctuation",        0xffcfbeff },
                { "Preprocessor Text",  0xfff8f631 }
            };

            CodeEditorComponent::ColourScheme cs;

            for (auto& t : types)
                cs.set (t.name, Colour (t.colour));

            return cs;
        */
    }
    
    pub fn get_light_code_editor_colour_scheme(&mut self) -> CodeEditorComponentColourScheme {
        
        todo!();
        /*
            struct Type
            {
                const char* name;
                uint32 colour;
            };

            const Type types[] =
            {
                { "Error",              0xffcc0000 },
                { "Comment",            0xff00aa00 },
                { "Keyword",            0xff0000cc },
                { "Operator",           0xff225500 },
                { "Identifier",         0xff000000 },
                { "Integer",            0xff880000 },
                { "Float",              0xff885500 },
                { "String",             0xff990099 },
                { "Bracket",            0xff000055 },
                { "Punctuation",        0xff004400 },
                { "Preprocessor Text",  0xff660000 }
            };

            CodeEditorComponent::ColourScheme cs;

            for (auto& t : types)
                cs.set (t.name, Colour (t.colour));

            return cs;
        */
    }
}
