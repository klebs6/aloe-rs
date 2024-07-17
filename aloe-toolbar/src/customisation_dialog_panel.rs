crate::ix!();

pub struct ToolbarCustomisationDialogCustomiserPanel<'a> {
    base:           Component<'a>,
    factory:        &'a mut dyn ToolbarItemFactory,
    toolbar:        &'a mut Toolbar<'a>,
    palette:        ToolbarItemPalette<'a>,
    instructions:   Label<'a>,
    style_box:      ComboBox<'a>,
    default_button: TextButton<'a>,
}

impl<'a> ToolbarCustomisationDialogCustomiserPanel<'a> {

    pub fn new(
        tbf:          &mut dyn ToolbarItemFactory,
        bar:          &mut Toolbar,
        option_flags: i32) -> Self {
    
        todo!();
        /*


            : factory (tbf), toolbar (bar), palette (tbf, bar),
                 instructions ({}, TRANS ("You can drag the items above and drop them onto a toolbar to add them.")
                                     + "\n\n"
                                     + TRANS ("Items on the toolbar can also be dragged around to change their order, or dragged off the edge to delete them.")),
                 defaultButton (TRANS ("Restore to default set of items"))

                addAndMakeVisible (palette);

                if ((optionFlags & (Toolbar::allowIconsOnlyChoice
                                     | Toolbar::allowIconsWithTextChoice
                                     | Toolbar::allowTextOnlyChoice)) != 0)
                {
                    addAndMakeVisible (styleBox);
                    styleBox.setEditableText (false);

                    if ((optionFlags & Toolbar::allowIconsOnlyChoice) != 0)     styleBox.addItem (TRANS("Show icons only"), 1);
                    if ((optionFlags & Toolbar::allowIconsWithTextChoice) != 0) styleBox.addItem (TRANS("Show icons and descriptions"), 2);
                    if ((optionFlags & Toolbar::allowTextOnlyChoice) != 0)      styleBox.addItem (TRANS("Show descriptions only"), 3);

                    int selectedStyle = 0;
                    switch (bar.getStyle())
                    {
                        case Toolbar::iconsOnly:      selectedStyle = 1; break;
                        case Toolbar::iconsWithText:  selectedStyle = 2; break;
                        case Toolbar::textOnly:       selectedStyle = 3; break;
                        default:                      break;
                    }

                    styleBox.setSelectedId (selectedStyle);

                    styleBox.onChange = [this] { updateStyle(); };
                }

                if ((optionFlags & Toolbar::showResetToDefaultsButton) != 0)
                {
                    addAndMakeVisible (defaultButton);
                    defaultButton.onClick = [this] { toolbar.addDefaultItems (factory); };
                }

                addAndMakeVisible (instructions);
                instructions.setFont (Font (13.0f));

                setSize (500, 300);
        */
    }
    
    pub fn update_style(&mut self)  {
        
        todo!();
        /*
            switch (styleBox.getSelectedId())
                {
                    case 1:   toolbar.setStyle (Toolbar::iconsOnly); break;
                    case 2:   toolbar.setStyle (Toolbar::iconsWithText); break;
                    case 3:   toolbar.setStyle (Toolbar::textOnly); break;
                    default:  break;
                }

                palette.resized(); // to make it update the styles
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            Colour background;

                if (auto* dw = findParentComponentOfClass<DialogWindow>())
                    background = dw->getBackgroundColour();

                g.setColour (background.contrasting().withAlpha (0.3f));
                g.fillRect (palette.getX(), palette.getBottom() - 1, palette.getWidth(), 1);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            palette.setBounds (0, 0, getWidth(), getHeight() - 120);
                styleBox.setBounds (10, getHeight() - 110, 200, 22);

                defaultButton.changeWidthToFitText (22);
                defaultButton.setTopLeftPosition (240, getHeight() - 110);

                instructions.setBounds (10, getHeight() - 80, getWidth() - 20, 80);
        */
    }
}
