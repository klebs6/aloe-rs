crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ButtonsPage<'a> {
    base:           Component<'a>,
    components:     Vec<Box<Component<'a>>>,
    bubble_message: Box<BubbleMessageComponent<'a>>,
    tooltip_window: TooltipWindow<'a>,
}

impl<'a> ButtonsPage<'a> {

    pub fn new(is_running_component_transform_demo: bool) -> Self {
    
        todo!();
        /*


            {
                auto* group = addToList (new GroupComponent ("group", "Radio buttons"));
                group->setBounds (20, 20, 220, 140);
            }

            for (int i = 0; i < 4; ++i)
            {
                auto* tb = addToList (new ToggleButton ("Radio Button #" + String (i + 1)));

                tb->setRadioGroupId (1234);
                tb->setBounds (45, 46 + i * 22, 180, 22);
                tb->setTooltip ("A set of mutually-exclusive radio buttons");

                if (i == 0)
                    tb->setToggleState (true, dontSendNotification);
            }

            for (int i = 0; i < 4; ++i)
            {
                DrawablePath normal, over;

                Path p;
                p.addStar ({}, i + 5, 20.0f, 50.0f, -0.2f);
                normal.setPath (p);
                normal.setFill (Colours::lightblue);
                normal.setStrokeFill (Colours::black);
                normal.setStrokeThickness (4.0f);

                over.setPath (p);
                over.setFill (Colours::blue);
                over.setStrokeFill (Colours::black);
                over.setStrokeThickness (4.0f);

                auto* db = addToList (new DrawableButton (String (i + 5) + " points", DrawableButton::ImageAboveTextLabel));
                db->setImages (&normal, &over, nullptr);
                db->setClickingTogglesState (true);
                db->setRadioGroupId (23456);

                int buttonSize = 50;
                db->setBounds (25 + i * buttonSize, 180, buttonSize, buttonSize);

                if (i == 0)
                    db->setToggleState (true, dontSendNotification);
            }

            for (int i = 0; i < 4; ++i)
            {
                auto* tb = addToList (new TextButton ("Button " + String (i + 1)));

                tb->setClickingTogglesState (true);
                tb->setRadioGroupId (34567);
                tb->setColour (TextButton::textColourOffId,  Colours::black);
                tb->setColour (TextButton::textColourOnId,   Colours::black);
                tb->setColour (TextButton::buttonColourId,   Colours::white);
                tb->setColour (TextButton::buttonOnColourId, Colours::blueviolet.brighter());

                tb->setBounds (20 + i * 55, 260, 55, 24);
                tb->setConnectedEdges (((i != 0) ? Button::ConnectedOnLeft : 0)
                                        | ((i != 3) ? Button::ConnectedOnRight : 0));

                if (i == 0)
                    tb->setToggleState (true, dontSendNotification);
            }

            {
                auto* colourChangeButton = new ColourChangeButton();
                components.add (colourChangeButton);
                addAndMakeVisible (colourChangeButton);
                colourChangeButton->setTopLeftPosition (20, 320);
            }

            {
                auto* hyperlink = addToList (new HyperlinkButton ("This is a HyperlinkButton",
                                                                  { "http://www.aloe.com" }));
                hyperlink->setBounds (260, 20, 200, 24);
            }

            // create some drawables to use for our drawable buttons...
            DrawablePath normal, over;

            {
                Path p;
                p.addStar ({}, 5, 20.0f, 50.0f, 0.2f);
                normal.setPath (p);
                normal.setFill (getRandomDarkColour());
            }

            {
                Path p;
                p.addStar ({}, 9, 25.0f, 50.0f, 0.0f);
                over.setPath (p);
                over.setFill (getRandomBrightColour());
                over.setStrokeFill (getRandomDarkColour());
                over.setStrokeThickness (5.0f);
            }

            DrawableImage down;
            down.setImage (getImageFromAssets ("aloe_icon.png"));
            down.setOverlayColour (Colours::black.withAlpha (0.3f));

            auto popupMessageCallback = [this, isRunningComponentTransformDemo]
            {
                if (auto* focused = Component::getCurrentlyFocusedComponent())
                    showBubbleMessage (*focused,
                                       "This is a demo of the BubbleMessageComponent, which lets you pop up a message pointing "
                                       "at a component or somewhere on the screen.\n\n"
                                       "The message bubbles will disappear after a timeout period, or when the mouse is clicked.",
                                       this->bubbleMessage,
                                       isRunningComponentTransformDemo);
            };

            {
                // create an image-above-text button from these drawables..
                auto db = addToList (new DrawableButton ("Button 1", DrawableButton::ImageAboveTextLabel));
                db->setImages (&normal, &over, &down);
                db->setBounds (260, 60, 80, 80);
                db->setTooltip ("This is a DrawableButton with a label");
                db->onClick = popupMessageCallback;
            }

            {
                // create an image-only button from these drawables..
                auto db = addToList (new DrawableButton ("Button 2", DrawableButton::ImageFitted));
                db->setImages (&normal, &over, &down);
                db->setClickingTogglesState (true);
                db->setBounds (370, 60, 80, 80);
                db->setTooltip ("This is an image-only DrawableButton");
                db->onClick = popupMessageCallback;
            }

            {
                // create an image-on-button-shape button from the same drawables..
                auto db = addToList (new DrawableButton ("Button 3", DrawableButton::ImageOnButtonBackground));
                db->setImages (&normal, nullptr, nullptr);
                db->setBounds (260, 160, 110, 25);
                db->setTooltip ("This is a DrawableButton on a standard button background");
                db->onClick = popupMessageCallback;
            }

            {
                auto db = addToList (new DrawableButton ("Button 4", DrawableButton::ImageOnButtonBackground));
                db->setImages (&normal, &over, &down);
                db->setClickingTogglesState (true);
                db->setColour (DrawableButton::backgroundColourId,   Colours::white);
                db->setColour (DrawableButton::backgroundOnColourId, Colours::yellow);
                db->setBounds (400, 150, 50, 50);
                db->setTooltip ("This is a DrawableButton on a standard button background");
                db->onClick = popupMessageCallback;
            }

            {
                auto sb = addToList (new ShapeButton ("ShapeButton",
                                                      getRandomDarkColour(),
                                                      getRandomDarkColour(),
                                                      getRandomDarkColour()));
                sb->setShape (getALOELogoPath(), false, true, false);
                sb->setBounds (260, 220, 200, 120);
            }

            {
                auto ib = addToList (new ImageButton ("ImageButton"));

                auto aloeImage = getImageFromAssets ("aloe_icon.png");

                ib->setImages (true, true, true,
                               aloeImage, 0.7f, Colours::transparentBlack,
                               aloeImage, 1.0f, Colours::transparentBlack,
                               aloeImage, 1.0f, getRandomBrightColour().withAlpha (0.8f),
                               0.5f);

                ib->setBounds (45, 380, 100, 100);
                ib->setTooltip ("ImageButton - showing alpha-channel hit-testing and colour overlay when clicked");
            }
        */
    }

    /**
      | This little function avoids a bit of
      | code-duplication by adding a component to
      | our list as well as calling
      | addAndMakeVisible on it..
      */
    pub fn add_to_list<ComponentType>(&mut self, new_comp: *mut ComponentType) -> *mut ComponentType {
    
        todo!();
        /*
            components.add (newComp);
            addAndMakeVisible (newComp);
            return newComp;
        */
    }
}
