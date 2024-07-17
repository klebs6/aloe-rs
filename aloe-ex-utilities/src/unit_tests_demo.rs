crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/UnitTestsDemo.h]

#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct UnitTestsDemo<'a> {
    base:                Component<'a>,
    current_test_thread: Box<TestRunnerThread<'a>>,
    start_test_button:   TextButton<'a>, // default = { "Run Unit Tests..."  }
    categories_box:      ComboBox<'a>,
    test_results_box:    TextEditor<'a>,
}

impl<'a> Default for UnitTestsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*

            setOpaque (true);

            addAndMakeVisible (startTestButton);
            startTestButton.onClick = [this] { start(); };

            addAndMakeVisible (testResultsBox);
            testResultsBox.setMultiLine (true);
            testResultsBox.setFont (Font (Font::getDefaultMonospacedFontName(), 12.0f, Font::plain));

            addAndMakeVisible (categoriesBox);
            categoriesBox.addItem ("All Tests", 1);

            auto categories = UnitTest::getAllCategories();
            categories.sort (true);

            categoriesBox.addItemList (categories, 2);
            categoriesBox.setSelectedId (1);

            logMessage ("This panel runs the built-in Aloe unit-tests from the selected category.\n");
            logMessage ("To add your own unit-tests, see the ALOE_UNIT_TESTS macro.");

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for UnitTestsDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            stopTest();
         */
    }
}

impl<'a> UnitTestsDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colours::grey));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds().reduced (6);

            auto topSlice = bounds.removeFromTop (25);
            startTestButton.setBounds (topSlice.removeFromLeft (200));
            topSlice.removeFromLeft (10);
            categoriesBox  .setBounds (topSlice.removeFromLeft (250));

            bounds.removeFromTop (5);
            testResultsBox.setBounds (bounds);
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            startTest (categoriesBox.getText());
        */
    }
    
    pub fn start_test(&mut self, category: &String)  {
        
        todo!();
        /*
            testResultsBox.clear();
            startTestButton.setEnabled (false);

            currentTestThread.reset (new TestRunnerThread (*this, category));
            currentTestThread->startThread();
        */
    }
    
    pub fn stop_test(&mut self)  {
        
        todo!();
        /*
            if (currentTestThread.get() != nullptr)
            {
                currentTestThread->stopThread (15000);
                currentTestThread.reset();
            }
        */
    }
    
    pub fn log_message(&mut self, message: &String)  {
        
        todo!();
        /*
            testResultsBox.moveCaretToEnd();
            testResultsBox.insertTextAtCaret (message + newLine);
            testResultsBox.moveCaretToEnd();
        */
    }
    
    pub fn test_finished(&mut self)  {
        
        todo!();
        /*
            stopTest();
            startTestButton.setEnabled (true);
            logMessage (newLine + "*** Tests finished ***");
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            testResultsBox.applyFontToAllText (testResultsBox.getFont());
        */
    }
}
