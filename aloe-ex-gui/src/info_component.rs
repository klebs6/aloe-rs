crate::ix!();

#[no_copy]
#[leak_detector]
pub struct InfoComponent<'a> {
    base:               Component<'a>,
    title_label:        Label<'a>, // default = "Title" 
    description_label:  Label<'a>, // default = "Description" 
    help_label:         Label<'a>, // default = "Help" 
    title_editor:       TextEditor<'a>,
    description_editor: TextEditor<'a>,
    help_editor:        TextEditor<'a>,
    role_label:         Label<'a>, // default = "Role" 
    role_box:           ComboBox<'a>,
}

impl<'a> InfoComponent<'a> {

    pub fn new(owner: &mut CustomWidgetComponent) -> Self {
    
        todo!();
        /*


            titleEditor.setText ("Custom");
                descriptionEditor.setText ("A short description of the custom component.");
                helpEditor.setText ("Some help text for the custom component.");

                titleEditor.onTextChange = [&owner]
                {
                    if (auto* handler = owner.accessibleComponent.getAccessibilityHandler())
                        handler->notifyAccessibilityEvent (AccessibilityEvent::titleChanged);
                };

                for (auto* editor : { &descriptionEditor, &helpEditor })
                {
                    editor->setMultiLine (true);
                    editor->setReturnKeyStartsNewLine (true);
                    editor->setJustification (Justification::centredLeft);
                }

                addAndMakeVisible (titleLabel);
                addAndMakeVisible (titleEditor);

                addAndMakeVisible (descriptionLabel);
                addAndMakeVisible (descriptionEditor);

                addAndMakeVisible (helpLabel);
                addAndMakeVisible (helpEditor);

                setUpAccessibilityRoleSelector (owner);
                addAndMakeVisible (roleBox);
                addAndMakeVisible (roleLabel);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (3)) };

                grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)),
                                         Grid::TrackInfo (Grid::Fr (1)),
                                         Grid::TrackInfo (Grid::Fr (1)),
                                         Grid::TrackInfo (Grid::Fr (1)),
                                         Grid::TrackInfo (Grid::Fr (1)),
                                         Grid::TrackInfo (Grid::Fr (1)) };

                grid.items = { GridItem (titleLabel).withMargin ({ 2 }),
                               GridItem (titleEditor).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }),

                               GridItem (roleLabel).withMargin ({ 2 }),
                               GridItem (roleBox).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }),

                               GridItem (descriptionLabel).withMargin ({ 2 }),
                               GridItem (descriptionEditor).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }),

                               GridItem (helpLabel).withMargin ({ 2 }),
                               GridItem (helpEditor).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }) };

                grid.performLayout (getLocalBounds());
        */
    }
    
    pub fn get_role(&self) -> AccessibilityRole {
        
        todo!();
        /*
            return accessibilityRoles[(size_t) roleBox.getSelectedItemIndex()].role;
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            return titleEditor.getText();
        */
    }
    
    pub fn get_description(&self) -> String {
        
        todo!();
        /*
            return descriptionEditor.getText();
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return helpEditor.getText();
        */
    }
    
    pub fn set_up_accessibility_role_selector(&mut self, owner: &mut CustomWidgetComponent)  {
        
        todo!();
        /*
            int itemId = 1;
                for (const auto& nameAndRole : accessibilityRoles)
                    roleBox.addItem (nameAndRole.name, itemId++);

                roleBox.setSelectedItemIndex (1);
                roleBox.onChange = [&owner] { owner.accessibleComponent.invalidateAccessibilityHandler(); };
        */
    }
}
