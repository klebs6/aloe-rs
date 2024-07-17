crate::ix!();

pub struct MenuPage<'a> {
    base:                     Component<'a>,
    random_colour_generator:  Random,
    popup_look_and_feel:      PopupMenuLookAndFeel<'a>,
    short_menu_button:        TextButton<'a>, // default = { "Short"  }
    long_menu_button:         TextButton<'a>, // default = { "Long"  }
    nested_menus_button:      TextButton<'a>, // default = { "Nested Sub-Menus"  }
    multi_column_menu_button: TextButton<'a>, // default = { "Multi Column"  }
    custom_item_button:       TextButton<'a>, // default = { "Custom Items"  }
    fancy_theme_button:       TextButton<'a>, // default = { "Fancy Theme with Column Breaks"  }
}

impl<'a> Default for MenuPage<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (shortMenuButton);
            shortMenuButton.onClick = [&]
            {
                PopupMenu menu;
                menu.addItem ("Single Item", nullptr);
                menu.showMenuAsync (PopupMenu::Options{}.withTargetComponent (shortMenuButton));
            };

            addAndMakeVisible (longMenuButton);
            longMenuButton.onClick = [&]
            {
                PopupMenu menu;

                for (int i = 0; i < 40; ++i)
                    menu.addItem ("Item " + String (i), nullptr);

                menu.showMenuAsync (PopupMenu::Options{}.withTargetComponent (longMenuButton));
            };

            addAndMakeVisible (nestedMenusButton);
            nestedMenusButton.onClick = [&]
            {
                PopupMenu menu;

                for (int i = 0; i < 15; ++i)
                {
                    PopupMenu subMenu;

                    for (int j = 0; j < 10; ++j)
                    {
                        if (j % 2 == 0)
                        {
                            PopupMenu subSubMenu;

                            for (int z = 0; z < 5; ++z)
                                subSubMenu.addItem ("Sub-sub-item " + String (z), nullptr);

                            subMenu.addSubMenu ("Sub-item " + String (j), subSubMenu);
                        }
                        else
                        {
                            subMenu.addItem ("Sub-item " + String (j), nullptr);
                        }
                    }

                    menu.addSubMenu ("Item " + String (i), subMenu);
                }

                menu.showMenuAsync (PopupMenu::Options{}.withTargetComponent (nestedMenusButton));
            };

            addAndMakeVisible (multiColumnMenuButton);
            multiColumnMenuButton.onClick = [&]
            {
                PopupMenu menu;

                for (int i = 0; i < 200; ++i)
                    menu.addItem ("Item " + String (i), nullptr);

                menu.showMenuAsync (PopupMenu::Options{}.withTargetComponent (multiColumnMenuButton)
                                                        .withMinimumNumColumns (2)
                                                        .withMaximumNumColumns (4));
            };

            addAndMakeVisible (customItemButton);
            customItemButton.onClick = [&]
            {
                struct CustomComponent  : public PopupMenu::CustomComponent
                {
                    CustomComponent (int widthIn, int heightIn, Colour backgroundIn)
                        : idealWidth (widthIn), idealHeight (heightIn), background (backgroundIn)
                    {}

                    void getIdealSize (int& width, int& height) override
                    {
                        width = idealWidth;
                        height = idealHeight;
                    }

                    void paint (Graphics& g) override { g.fillAll (background); }

                    int idealWidth = 0;
                    int idealHeight = 0;
                    Colour background;
                };

                PopupMenu menu;

                menu.addCustomItem (-1, std::make_unique<CustomComponent> (100,  20, Colours::darkred));
                menu.addCustomItem (-1, std::make_unique<CustomComponent> (20,  100, Colours::darkgreen));
                menu.addCustomItem (-1, std::make_unique<CustomComponent> (100, 100, Colours::darkblue));
                menu.addCustomItem (-1, std::make_unique<CustomComponent> (100,  50, Colours::darkcyan));
                menu.addCustomItem (-1, std::make_unique<CustomComponent> (50,  100, Colours::darkmagenta));

                menu.showMenuAsync (PopupMenu::Options{}.withTargetComponent (customItemButton)
                                                        .withMinimumNumColumns (5));
            };

            addAndMakeVisible (fancyThemeButton);
            fancyThemeButton.setLookAndFeel (&popupLookAndFeel);
            fancyThemeButton.onClick = [&]
            {
                const auto colour = Colour::fromHSL (randomColourGenerator.nextFloat(), 0.5f, 0.5f, 1.0f);
                fancyThemeButton.setColour (TextButton::buttonColourId, colour);

                PopupMenu menu;
                menu.setLookAndFeel (&popupLookAndFeel);

                for (auto length : { 5, 10, 7, 3 })
                {
                    for (int i = 0; i < length; ++i)
                        menu.addItem ("Item " + String (i), nullptr);

                    menu.addColumnBreak();
                }

                menu.showMenuAsync (PopupMenu::Options{}.withTargetComponent (&fancyThemeButton));
            }
        */
    }
}

impl<'a> Resized for MenuPage<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            const auto makeItem = [] (Component& comp)
            {
                return FlexItem { comp }.withWidth (200).withHeight (24).withMargin ({ 4 });
            };

            FlexBox box;
            box.flexDirection = FlexBox::Direction::column;
            box.items = { makeItem (shortMenuButton),
                          makeItem (longMenuButton),
                          makeItem (nestedMenusButton),
                          makeItem (multiColumnMenuButton),
                          makeItem (customItemButton),
                          makeItem (fancyThemeButton) };

            box.performLayout (getLocalBounds());
        */
    }
}
