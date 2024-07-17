crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AnimationDemo<'a> {
    base:                  Component<'a>,
    base2:                 Timer,
    components_to_animate: Vec<Box<Component<'a>>>,
    balls:                 Vec<Box<BallComponent<'a>>>,
    ball_generator:        BallGeneratorComponent<'a>,
    animator:              ComponentAnimator<'a>,
    cycle_count:           i32,
    first_callback:        bool, // default = true
}

impl<'a> Paint for AnimationDemo<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (ResizableWindow::backgroundColourId));
        */
    }
}

impl<'a> Resized for AnimationDemo<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            ballGenerator.centreWithSize (80, 50);
            triggerAnimation();
        */
    }
}

impl<'a> Default for AnimationDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            for (auto i = 0; i < 11; ++i)
            {
                auto* b = createButton();
                componentsToAnimate.add (b);
                addAndMakeVisible (b);
                b->onClick = [this] { triggerAnimation(); };
            }

            addAndMakeVisible (ballGenerator);

            cycleCount = 2;
            startTimerHz (60);

            setSize (620, 620)
        */
    }
}

impl<'a> AnimationDemo<'a> {
    
    pub fn create_random_button(&mut self) -> *mut Button {
        
        todo!();
        /*
            DrawablePath normal, over;

            Path star1;
            star1.addStar ({}, 5, 20.0f, 50.0f, 0.2f);
            normal.setPath (star1);
            normal.setFill (Colours::red);

            Path star2;
            star2.addStar ({}, 7, 30.0f, 50.0f, 0.0f);
            over.setPath (star2);
            over.setFill (Colours::pink);
            over.setStrokeFill (Colours::black);
            over.setStrokeThickness (5.0f);

            auto aloeIcon = getImageFromAssets ("aloe_icon.png");

            DrawableImage down;
            down.setImage (aloeIcon);
            down.setOverlayColour (Colours::black.withAlpha (0.3f));

            if (Random::getSystemRandom().nextInt (10) > 2)
            {
                auto type = Random::getSystemRandom().nextInt (3);

                auto* d = new DrawableButton ("Button",
                                              type == 0 ? DrawableButton::ImageOnButtonBackground
                                                        : (type == 1 ? DrawableButton::ImageFitted
                                                                     : DrawableButton::ImageAboveTextLabel));
                d->setImages (&normal,
                              Random::getSystemRandom().nextBool() ? &over : nullptr,
                              Random::getSystemRandom().nextBool() ? &down : nullptr);

                if (Random::getSystemRandom().nextBool())
                {
                    d->setColour (DrawableButton::backgroundColourId,   getRandomBrightColour());
                    d->setColour (DrawableButton::backgroundOnColourId, getRandomBrightColour());
                }

                d->setClickingTogglesState (Random::getSystemRandom().nextBool());
                return d;
            }

            auto* b = new ImageButton ("ImageButton");

            b->setImages (true, true, true,
                          aloeIcon, 0.7f, Colours::transparentBlack,
                          aloeIcon, 1.0f, getRandomDarkColour()  .withAlpha (0.2f),
                          aloeIcon, 1.0f, getRandomBrightColour().withAlpha (0.8f),
                          0.5f);
            return b;
        */
    }
    
    pub fn create_button(&mut self) -> *mut Button {
        
        todo!();
        /*
            auto aloeIcon = getImageFromAssets ("aloe_icon.png").rescaled (128, 128);

            auto* b = new ImageButton ("ImageButton");

            b->setImages (true, true, true,
                          aloeIcon, 1.0f, Colours::transparentBlack,
                          aloeIcon, 1.0f, Colours::white,
                          aloeIcon, 1.0f, Colours::white,
                          0.5f);

            return b;
        */
    }
    
    pub fn trigger_animation(&mut self)  {
        
        todo!();
        /*
            auto width = getWidth();
            auto height = getHeight();

            bool useWidth = (height > width);

            for (auto* component : componentsToAnimate)
            {
                auto newIndex = (componentsToAnimate.indexOf (component) + 3 * cycleCount)
                                 % componentsToAnimate.size();

                auto angle = (float) newIndex * MathConstants<float>::twoPi / (float) componentsToAnimate.size();

                auto radius = useWidth ? (float) width  * 0.35f
                                       : (float) height * 0.35f;

                Rectangle<int> r (getWidth()  / 2 + (int) (radius * std::sin (angle)) - 50,
                                  getHeight() / 2 + (int) (radius * std::cos (angle)) - 50,
                                  100, 100);

                animator.animateComponent (component, r.reduced (10), 1.0f,
                                           900 + (int) (300 * std::sin (angle)),
                                           false, 0.0, 0.0);
            }

            ++cycleCount;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (firstCallback)
            {
                triggerAnimation();
                firstCallback = false;
            }

            // Go through each of our balls and update their position
            for (int i = balls.size(); --i >= 0;)
                if (! balls.getUnchecked (i)->step())
                    balls.remove (i);

            // Randomly generate new balls
            if (Random::getSystemRandom().nextInt (100) < 4)
                addAndMakeVisible (balls.add (new BallComponent (ballGenerator.getBounds().getCentre().toFloat())));
        */
    }
}
