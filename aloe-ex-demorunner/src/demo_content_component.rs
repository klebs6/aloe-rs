crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/UI/DemoContentComponent.h]

pub struct DemoContentComponent<'a> {
    base:                  TabbedComponent<'a>,
    demo_changed_callback: fn(_0: bool) -> (),
    demo_content:          Box<DemoContent<'a>>,

    #[cfg(not(any(target_os="android",target_os="ios")))]
    code_content:          Box<CodeContent<'a>>,

    current_demo_category: String,
    current_demo_index:    i32, // default = -1
    tab_bar_indent:        i32, // default = 0
}

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/UI/DemoContentComponent.cpp]
impl<'a> DemoContentComponent<'a> {

    pub fn get_current_demo_index(&self) -> i32 {
        
        todo!();
        /*
            return currentDemoIndex;
        */
    }
    
    pub fn set_tab_bar_indent(&mut self, indent: i32)  {
        
        todo!();
        /*
            tabBarIndent = indent;
        */
    }
    
    pub fn new(
        main_component: &mut Component,
        callback:       fn(_0: bool) -> ()) -> Self {
    
        todo!();
        /*


            : TabbedComponent (TabbedButtonBar::Orientation::TabsAtTop),
          demoChangedCallback (std::move (callback))
        demoContent.reset (new DemoContent());
        addTab ("Demo",     Colours::transparentBlack, demoContent.get(), false);

       #if ! (ALOE_ANDROID || ALOE_IOS)
        codeContent.reset (new CodeContent());
        addTab ("Code",     Colours::transparentBlack, codeContent.get(), false);
       #endif

        addTab ("Settings", Colours::transparentBlack, new SettingsContent (dynamic_cast<MainComponent&> (mainComponent)), true);

        setTabBarDepth (40);
        lookAndFeelChanged();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            TabbedComponent::resized();

        if (tabBarIndent > 0)
            getTabbedButtonBar().setBounds (getTabbedButtonBar().getBounds().withTrimmedLeft (tabBarIndent));
        */
    }
    
    pub fn set_demo(&mut self, 
        category:            &String,
        selected_demo_index: i32)  {
        
        todo!();
        /*
            if ((currentDemoCategory == category)
            && (currentDemoIndex == selectedDemoIndex))
            return;

        auto demo = ALOEDemos::getCategory (category).demos[(size_t) selectedDemoIndex];

       #if ! (ALOE_ANDROID || ALOE_IOS)
        codeContent->document.replaceAllContent (trimPIP (demo.demoFile.loadFileAsString()));
        codeContent->codeEditor.scrollToLine (0);
       #endif

        auto* content = demo.callback();
        demoContent->setComponent (content);
        demoChangedCallback (demo.isHeavyweight);

        ensureDemoIsShowing();

        currentDemoCategory = category;
        currentDemoIndex = selectedDemoIndex;
        */
    }
    
    pub fn is_showing_home_screen(&self) -> bool {
        
        todo!();
        /*
            return isComponentIntroDemo (demoContent->getComponent()) && getCurrentTabIndex() == 0;
        */
    }
    
    pub fn show_home_screen(&mut self)  {
        
        todo!();
        /*
            demoContent->showHomeScreen();

       #if ! (ALOE_ANDROID || ALOE_IOS)
        codeContent->setDefaultCodeContent();
       #endif

        demoChangedCallback (false);

        ensureDemoIsShowing();

        resized();

        currentDemoCategory = {};
        currentDemoIndex = -1;
        */
    }
    
    pub fn clear_current_demo(&mut self)  {
        
        todo!();
        /*
            demoContent->setComponent (nullptr);
        demoChangedCallback (false);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            auto backgroundColour = findColour (ResizableWindow::backgroundColourId);

        for (int i = 0; i < getNumTabs(); ++i)
            setTabBackgroundColour (i, backgroundColour);
        */
    }
    
    pub fn trimpip(&mut self, file_contents: &String) -> String {
        
        todo!();
        /*
            auto lines = StringArray::fromLines (fileContents);

        auto metadataEndIndex = lines.indexOf (" END_ALOE_PIP_METADATA");

        if (metadataEndIndex == -1)
            return fileContents;

        lines.removeRange (0, metadataEndIndex + 3); // account for newline and comment block end

        return lines.joinIntoString ("\n");
        */
    }
    
    pub fn ensure_demo_is_showing(&mut self)  {
        
        todo!();
        /*
            if (getCurrentTabIndex() == (getNumTabs() - 1))
            setCurrentTabIndex (0);
        */
    }
}
