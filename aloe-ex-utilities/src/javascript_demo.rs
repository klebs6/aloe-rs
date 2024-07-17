crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/JavaScriptDemo.h]

#[no_copy]
#[leak_detector]
pub struct JavaScriptDemo<'a> {
    base:           Component<'a>,
    base3:          Timer,
    code_document:  CodeDocument<'a>,
    editor:         Box<CodeEditorComponent<'a>>,
    output_display: TextEditor<'a>,
}

impl<'a> CodeDocumentListener for JavaScriptDemo<'a> {
    fn code_document_text_inserted(&mut self, 
        _0: &str,
        _1: i32)  {
        
        todo!();
        /*
            startTimer (300);
        */
    }
    
    fn code_document_text_deleted(&mut self, _0: i32, _1: i32)  {
        
        todo!();
        /*
            startTimer (300);
        */
    }
}

impl<'a> Default for JavaScriptDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*

            setOpaque (true);

            editor.reset (new CodeEditorComponent (codeDocument, nullptr));
            addAndMakeVisible (editor.get());
            editor->setFont ({ Font::getDefaultMonospacedFontName(), 14.0f, Font::plain });
            editor->setTabSize (4, true);

            outputDisplay.setMultiLine (true);
            outputDisplay.setReadOnly (true);
            outputDisplay.setCaretVisible (false);
            outputDisplay.setFont ({ Font::getDefaultMonospacedFontName(), 14.0f, Font::plain });
            addAndMakeVisible (outputDisplay);

            codeDocument.addListener (this);

            editor->loadContent (
                "/*\n"
                "    Javascript! In this simple demo, the native\n"
                "    code provides an object called \'Demo\' which\n"
                "    has a method \'print\' that writes to the\n"
                "    console below...\n"
                "*/\n"
                "\n"
                "Demo.print (\"Hello World in Aloe + Javascript!\");\n"
                "Demo.print (\"\");\n"
                "\n"
                "function factorial (n)\n"
                "{\n"
                "    var total = 1;\n"
                "    while (n > 0)\n"
                "        total = total * n--;\n"
                "    return total;\n"
                "}\n"
                "\n"
                "for (var i = 1; i < 10; ++i)\n"
                "    Demo.print (\"Factorial of \" + i \n"
                "                   + \" = \" + factorial (i));\n");

            setSize (600, 750)
        */
    }
}

impl<'a> JavaScriptDemo<'a> {

    pub fn run_script(&mut self)  {
        
        todo!();
        /*
            outputDisplay.clear();

            JavascriptEngine engine;
            engine.maximumExecutionTime = RelativeTime::seconds (5);
            engine.registerNativeObject ("Demo", new DemoClass (*this));

            auto startTime = Time::getMillisecondCounterHiRes();

            auto result = engine.execute (codeDocument.getAllContent());

            auto elapsedMs = Time::getMillisecondCounterHiRes() - startTime;

            if (result.failed())
                outputDisplay.setText (result.getErrorMessage());
            else
                outputDisplay.insertTextAtCaret ("\n(Execution time: " + String (elapsedMs, 2) + " milliseconds)");
        */
    }
    
    pub fn console_output(&mut self, message: &String)  {
        
        todo!();
        /*
            outputDisplay.moveCaretToEnd();
            outputDisplay.insertTextAtCaret (message + newLine);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();
            runScript();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (8);

            editor->setBounds       (r.removeFromTop (proportionOfHeight (0.6f)));
            outputDisplay.setBounds (r.withTrimmedTop (8));
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            outputDisplay.applyFontToAllText (outputDisplay.getFont());
        */
    }
}
