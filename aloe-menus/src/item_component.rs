crate::ix!();

#[no_copy]
#[leak_detector]
pub struct PopupMenuItemComponent<'a> {
    base:           Component<'a>,
    item:           PopupMenuItem<'a>,

    parent_window:  &'a mut MenuWindow<'a>,
    options:        &'a PopupMenuOptions<'a>,

    /**
      | NB: we use a copy of the one from the item
      | info in case we're using our own section
      | comp
      |
      */
    custom_comp:    ReferenceCountedObjectPtr<PopupMenuCustomComponent<'a>>,

    is_highlighted: bool, // default = false
}

impl<'a> Drop for PopupMenuItemComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                if (customComp != nullptr)
                    setItem (*customComp, nullptr);

                removeChildComponent (customComp.get());
             */
    }
}

impl<'a> PopupMenuItemComponent<'a> {

    pub fn new(
        i:      &PopupMenuItem,
        o:      &PopupMenuOptions,
        parent: &mut MenuWindow) -> Self {
    
        todo!();
        /*
        : item(i),
        : parent_window(parent),
        : options(o),
        : custom_comp(i.customComponent),

            if (item.isSectionHeader)
                    customComp = *new PopupMenuHeaderItemComponent (item.text, options);

                if (customComp != nullptr)
                {
                    setItem (*customComp, &item);
                    addAndMakeVisible (*customComp);
                }

                parent.addAndMakeVisible (this);

                updateShortcutKeyDescription();

                int itemW = 80;
                int itemH = 16;
                getIdealSize (itemW, itemH, options.getStandardItemHeight());
                setSize (itemW, jlimit (1, 600, itemH));

                addMouseListener (&parent, false);
        */
    }
    
    pub fn get_ideal_size(&mut self, 
        ideal_width:          &mut i32,
        ideal_height:         &mut i32,
        standard_item_height: i32)  {
        
        todo!();
        /*
            if (customComp != nullptr)
                    customComp->getIdealSize (idealWidth, idealHeight);
                else
                    getLookAndFeel().getIdealPopupMenuItemSizeWithOptions (getTextForMeasurement(),
                                                                           item.isSeparator,
                                                                           standardItemHeight,
                                                                           idealWidth, idealHeight,
                                                                           options);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (customComp == nullptr)
                    getLookAndFeel().drawPopupMenuItemWithOptions (g, getLocalBounds(),
                                                                   isHighlighted,
                                                                   item,
                                                                   options);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (auto* child = getChildComponent (0))
                {
                    const auto border = getLookAndFeel().getPopupMenuBorderSizeWithOptions (options);
                    child->setBounds (getLocalBounds().reduced (border, 0));
                }
        */
    }
    
    pub fn set_highlighted(&mut self, should_be_highlighted: bool)  {
        
        todo!();
        /*
            shouldBeHighlighted = shouldBeHighlighted && item.isEnabled;

                if (isHighlighted != shouldBeHighlighted)
                {
                    isHighlighted = shouldBeHighlighted;

                    if (customComp != nullptr)
                        customComp->setHighlighted (shouldBeHighlighted);

                    if (isHighlighted)
                        if (auto* handler = getAccessibilityHandler())
                            handler->grabFocus();

                    repaint();
                }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return item.isSeparator ? nullptr : std::make_unique<PopupMenuItemComponentItemAccessibilityHandler> (*this);
        */
    }
    
    pub fn update_shortcut_key_description(&mut self)  {
        
        todo!();
        /*
            if (item.commandManager != nullptr
                     && item.itemID != 0
                     && item.shortcutKeyDescription.isEmpty())
                {
                    String shortcutKey;

                    for (auto& keypress : item.commandManager->getKeyMappings()
                                            ->getKeyPressesAssignedToCommand (item.itemID))
                    {
                        auto key = keypress.getTextDescriptionWithIcons();

                        if (shortcutKey.isNotEmpty())
                            shortcutKey << ", ";

                        if (key.length() == 1 && key[0] < 128)
                            shortcutKey << "shortcut: '" << key << '\'';
                        else
                            shortcutKey << key;
                    }

                    item.shortcutKeyDescription = shortcutKey.trim();
                }
        */
    }
    
    pub fn get_text_for_measurement(&self) -> String {
        
        todo!();
        /*
            return item.shortcutKeyDescription.isNotEmpty() ? item.text + "   " + item.shortcutKeyDescription
                                                                : item.text;
        */
    }
}
