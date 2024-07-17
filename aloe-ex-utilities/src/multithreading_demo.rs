crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MultithreadingDemo<'a> {
    base:           Component<'a>,
    base2:          Timer,
    pool:           ThreadPool<'a>, // default = { 3  }
    control_button: TextButton<'a>, // default = { "Thread type"  }
    is_using_pool:  bool,       // default = false
    balls:          Vec<Box<BouncingBall<'a>>>,
}

impl<'a> Default for MultithreadingDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            addAndMakeVisible (controlButton);
            controlButton.changeWidthToFitText (24);
            controlButton.setTopLeftPosition (20, 20);
            controlButton.setTriggeredOnMouseDown (true);
            controlButton.setAlwaysOnTop (true);
            controlButton.onClick = [this] { showMenu(); };

            setSize (500, 500);

            resetAllBalls();

            startTimerHz (60)
        */
    }
}

impl<'a> Drop for MultithreadingDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            pool.removeAllJobs (true, 2000);
         */
    }
}

impl<'a> MultithreadingDemo<'a> {
    
    pub fn reset_all_balls(&mut self)  {
        
        todo!();
        /*
            pool.removeAllJobs (true, 4000);
            balls.clear();

            for (int i = 0; i < 5; ++i)
                addABall();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));

            for (auto* ball : balls)
                ball->draw (g);
        */
    }
    
    pub fn set_using_pool(&mut self, use_pool: bool)  {
        
        todo!();
        /*
            isUsingPool = usePool;
            resetAllBalls();
        */
    }
    
    pub fn add_aball(&mut self)  {
        
        todo!();
        /*
            if (isUsingPool)
            {
                auto newBall = std::make_unique<DemoThreadPoolJob> (*this);
                pool.addJob (newBall.get(), false);
                balls.add (newBall.release());

            }
            else
            {
                balls.add (new DemoThread (*this));
            }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn show_menu(&mut self)  {
        
        todo!();
        /*
            PopupMenu m;
            m.addItem (1, "Use one thread per ball", true, ! isUsingPool);
            m.addItem (2, "Use a thread pool",       true,   isUsingPool);

            m.showMenuAsync (PopupMenu::Options().withTargetComponent (controlButton),
                             ModalCallbackFunction::forComponent (menuItemChosenCallback, this));
        */
    }
    
    pub fn menu_item_chosen_callback(
        result:         i32,
        demo_component: *mut MultithreadingDemo)  {
        
        todo!();
        /*
            if (result != 0 && demoComponent != nullptr)
                demoComponent->setUsingPool (result == 2);
        */
    }
}
