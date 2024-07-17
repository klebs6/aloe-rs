crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_KeyMappingEditorComponent.h]

/**
  | A component to allow editing of the keymaps
  | stored by a KeyPressMappingSet object.
  | 
  | @see KeyPressMappingSet
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct KeyMappingEditorComponent<'a> {
    base:         Component<'a>,
    mappings:     &'a mut KeyPressMappingSet<'a>,
    tree:         TreeView<'a>,
    reset_button: TextButton<'a>,
    tree_item:    Box<KeyMappingEditorComponentTopLevelItem<'a>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_KeyMappingEditorComponent.cpp]
impl<'a> Drop for KeyMappingEditorComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            tree.setRootItem (nullptr);
        */
    }
}

impl<'a> KeyMappingEditorComponent<'a> {
    
    /**
      | Returns the KeyPressMappingSet that
      | this component is acting upon.
      |
      */
    pub fn get_mappings(&self) -> &mut KeyPressMappingSet {
        
        todo!();
        /*
            return mappings;
        */
    }

    /**
      | Returns the ApplicationCommandManager
      | that this component is connected to.
      |
      */
    pub fn get_command_manager(&self) -> &mut ApplicationCommandManager {
        
        todo!();
        /*
            return mappings.getCommandManager();
        */
    }
    
    /**
      | Creates a KeyMappingEditorComponent.
      | 
      | -----------
      | @param mappingSet
      | 
      | this is the set of mappings to display
      | and edit. Make sure the mappings object
      | is not deleted before this component!
      | ----------
      | @param showResetToDefaultButton
      | 
      | if true, then at the bottom of the list,
      | the component will include a 'reset
      | to defaults' button.
      |
      */
    pub fn new(
        mapping_manager:              &mut KeyPressMappingSet,
        show_reset_to_default_button: bool) -> Self {
    
        todo!();
        /*
        : mappings(mappingManager),
        : reset_button(TRANS ("reset to defaults")),

            treeItem.reset (new KeyMappingEditorComponentTopLevelItem (*this));

        if (showResetToDefaultButton)
        {
            addAndMakeVisible (resetButton);

            resetButton.onClick = [this]
            {
                AlertWindow::showOkCancelBox (MessageBoxIconType::QuestionIcon,
                                              TRANS("Reset to defaults"),
                                              TRANS("Are you sure you want to reset all the key-mappings to their default state?"),
                                              TRANS("Reset"),
                                              {}, this,
                                              ModalCallbackFunction::forComponent (resetKeyMappingsToDefaultsCallback, this));
            };
        }

        addAndMakeVisible (tree);
        tree.setTitle ("Key Mappings");
        tree.setColour (TreeView::backgroundColourId, findColour (backgroundColourId));
        tree.setRootItemVisible (false);
        tree.setDefaultOpenness (true);
        tree.setRootItem (treeItem.get());
        tree.setIndentSize (12);
        */
    }
    
    /**
      | Sets up the colours to use for parts of
      | the component.
      | 
      | -----------
      | @param mainBackground
      | 
      | colour to use for most of the background
      | ----------
      | @param textColour
      | 
      | colour to use for the text
      |
      */
    pub fn set_colours(&mut self, 
        main_background: Colour,
        text_colour:     Colour)  {
        
        todo!();
        /*
            setColour (backgroundColourId, mainBackground);
        setColour (textColourId, textColour);
        tree.setColour (TreeView::backgroundColourId, mainBackground);
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            treeItem->changeListenerCallback (nullptr);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            int h = getHeight();

        if (resetButton.isVisible())
        {
            const int buttonHeight = 20;
            h -= buttonHeight + 8;
            int x = getWidth() - 8;

            resetButton.changeWidthToFitText (buttonHeight);
            resetButton.setTopRightPosition (x, h + 6);
        }

        tree.setBounds (0, 0, getWidth(), h);
        */
    }
    
    pub fn should_command_be_included(&mut self, commandid: CommandID) -> bool {
        
        todo!();
        /*
            auto* ci = mappings.getCommandManager().getCommandForID (commandID);

        return ci != nullptr && (ci->flags & ApplicationCommandInfo::hiddenFromKeyEditor) == 0;
        */
    }
    
    pub fn is_command_read_only(&mut self, commandid: CommandID) -> bool {
        
        todo!();
        /*
            auto* ci = mappings.getCommandManager().getCommandForID (commandID);

        return ci != nullptr && (ci->flags & ApplicationCommandInfo::readOnlyInKeyEditor) != 0;
        */
    }
    
    pub fn get_description_for_key_press(&mut self, key: &KeyPress) -> String {
        
        todo!();
        /*
            return key.getTextDescription();
        */
    }
}
