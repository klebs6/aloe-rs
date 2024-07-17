crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/Box2DDemo.h]

/**
   (These classes and random functions are used
   inside the 3rd-party Box2D demo code)
  */
#[inline] pub fn random_float() -> f32 {
    
    todo!();
    /*
        return Random::getSystemRandom().nextFloat() * 2.0f - 1.0f;
    */
}

#[inline] pub fn random_float_in_range(lo: f32, hi: f32) -> f32 {
    
    todo!();
    /*
        return Random::getSystemRandom().nextFloat() * (hi - lo) + lo;
    */
}

pub struct Settings
{
    view_center:          b2Vec2, // default = { 0.0f, 20.0f  }
    hz:                   f32, // default = 60.0f
    velocity_iterations:  i32, // default = 8
    position_iterations:  i32, // default = 3
    draw_shapes:          i32, // default = 1
    draw_joints:          i32, // default = 1
    draw_aab_bs:          i32, // default = 0
    draw_pairs:           i32, // default = 0
    draw_contact_points:  i32, // default = 0
    draw_contact_normals: i32, // default = 0
    draw_contact_forces:  i32, // default = 0
    draw_friction_forces: i32, // default = 0
    draw_co_ms:           i32, // default = 0
    draw_stats:           i32, // default = 0
    draw_profile:         i32, // default = 0
    enable_warm_starting: i32, // default = 1
    enable_continuous:    i32, // default = 1
    enable_sub_stepping:  i32, // default = 0
    pause:                i32, // default = 0
    single_step:          i32, // default = 0
}

pub struct Test {
    world: Box<b2World>, //{ new b2World (b2Vec2 (0.0f, -10.0f)) };
}

pub trait TestInterface {
    fn keyboard(&mut self, key: u8)  { }
    fn keyboard_up(&mut self, key: u8)  { }
}

/**
  | This list box just displays a StringArray
  | and broadcasts a change message when
  | the selected row changes.
  |
  */
#[no_copy]
#[leak_detector]
pub struct Box2DTestList<'a> {
    base:  ListBoxModel,
    base2: ChangeBroadcaster<'a>,
    tests: StringArray,
}

impl<'a> Box2DTestList<'a> {

    pub fn new(test_list: &StringArray) -> Self {
    
        todo!();
        /*
        : tests(testList),

        
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return tests.size();
        */
    }
    
    pub fn selected_rows_changed(&mut self, last_row_selected: i32)  {
        
        todo!();
        /*
            sendChangeMessage();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row:             i32,
        g:               &mut Graphics,
        w:               i32,
        h:               i32,
        row_is_selected: bool)  {
        
        todo!();
        /*
            auto& lf = LookAndFeel::getDefaultLookAndFeel();

            if (rowIsSelected)
                g.fillAll (Colour::contrasting (lf.findColour (ListBox::textColourId),
                                                lf.findColour (ListBox::backgroundColourId)));

            g.setColour (lf.findColour (ListBox::textColourId));
            g.setFont ((float) h * 0.7f);
            g.drawText (tests[row], Rectangle<int> (0, 0, w, h).reduced (2),
                        Justification::centredLeft, true);
        */
    }
}

///--------------------
pub struct Box2DRenderComponent<'a> {
    base:         Component<'a>,
    current_test: Box<Test>,
}

impl<'a> Default for Box2DRenderComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true)
        */
    }
}

impl<'a> Box2DRenderComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::white);

            if (currentTest.get() != nullptr)
            {
                Box2DRenderer renderer;

                renderer.render (g, *currentTest->m_world,
                                 -16.0f, 30.0f, 16.0f, -1.0f,
                                 getLocalBounds().toFloat().reduced (8.0f));
            }
        */
    }
}

///--------------------------
pub enum Box2dDemos
{
    addPair = 0,
    applyForce,
    dominoes,
    chain,
    numTests
}

#[no_copy]
#[leak_detector]
pub struct Box2DDemo<'a> {
    base:             Component<'a>,
    base2:            Timer,
    tests_list:       StringArray,
    tests_list_model: Box2DTestList<'a>, // default = { testsList  }
    render_component: Box2DRenderComponent<'a>,
    tests_list_box:   ListBox<'a>,
    instructions:     TextEditor<'a>,
}

impl<'a> ChangeListener for Box2DDemo<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (source == &testsListModel)
            {
                auto index = testsListBox.getSelectedRow();

                renderComponent.currentTest.reset (createTest (index));
                instructions.setText (getInstructions (index));

                repaint();
            }
        */
    }
}

impl<'a> Default for Box2DDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : tests_list(getTestsList()),

            setOpaque (true);
            setWantsKeyboardFocus (true);

            testsListModel.addChangeListener (this);

            addAndMakeVisible (renderComponent);

            addAndMakeVisible (testsListBox);
            testsListBox.setModel (&testsListModel);
            testsListBox.selectRow (dominoes);

            addAndMakeVisible (instructions);
            instructions.setMultiLine (true);
            instructions.setReadOnly (true);

            startTimerHz (60);

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for Box2DDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            testsListModel.removeChangeListener (this);
         */
    }
}

impl<'a> Box2DDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (4);

            auto area = r.removeFromBottom (150);
            testsListBox.setBounds (area.removeFromLeft (150));

            area.removeFromLeft (4);
            instructions.setBounds (area);

            r.removeFromBottom (6);
            renderComponent.setBounds (r);
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (renderComponent.currentTest.get() != nullptr)
            {
                // We override this to avoid the system beeping for an unused keypress
                switch (key.getTextCharacter())
                {
                    case 'a':
                    case 'w':
                    case 'd':
                        return true;

                    default:
                        break;
                }
            }

            return false;
        */
    }
    
    pub fn create_test(index: i32) -> *mut Test {
        
        todo!();
        /*
            switch (index)
            {
                case addPair:       return new AddPair();
                case applyForce:    return new ApplyForce();
                case dominoes:      return new Dominos();
                case chain:         return new Chain();
                default:            break;
            }

            return nullptr;
        */
    }
    
    pub fn get_instructions(index: i32) -> String {
        
        todo!();
        /*
            switch (index)
            {
                case applyForce:
                    return String ("Keys:") + newLine + "Left: \'a\'" + newLine
                            + "Right: \'d\'" + newLine + "Forward: \'w\'";

                default:
                    break;
            }

            return {};
        */
    }
    
    pub fn check_keys(&mut self)  {
        
        todo!();
        /*
            if (renderComponent.currentTest.get() == nullptr)
                return;

            checkKeyCode ('a');
            checkKeyCode ('w');
            checkKeyCode ('d');
        */
    }
    
    pub fn check_key_code(&mut self, key_code: i32)  {
        
        todo!();
        /*
            if (KeyPress::isKeyCurrentlyDown (keyCode))
                renderComponent.currentTest->Keyboard ((unsigned char) keyCode);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (renderComponent.currentTest.get() == nullptr)
                return;

            if (isShowing())
                grabKeyboardFocus();

            checkKeys();
            renderComponent.currentTest->m_world->Step (1.0f / 60.0f, 6, 2);
            repaint();
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            instructions.applyFontToAllText (instructions.getFont());
        */
    }
    
    pub fn get_tests_list() -> StringArray {
        
        todo!();
        /*
            return { "Add Pair Stress Test", "Apply Force", "Dominoes", "Chain" };
        */
    }
}
