crate::ix!();

pub trait ShowPopup {

    /**
      | Pops up the combo box's list.
      | 
      | This is virtual so that you can override
      | it with your own custom popup mechanism
      | if you need some really unusual behaviour.
      |
      */
    fn show_popup(&mut self);
}

pub fn combo_box_popup_menu_finished_callback(
        result: i32,
        combo:  *mut ComboBox)  {
    
    todo!();
        /*
            if (combo != nullptr)
        {
            combo->hidePopup();

            if (result != 0)
                combo->setSelectedId (result);
        }
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ComboBox.h]

/**
  | A class for receiving events from a ComboBox.
  | 
  | You can register a ComboBox::ComboBoxListener
  | with a ComboBox using the ComboBox::addListener()
  | method, and it will be called when the
  | selected item in the box changes.
  | 
  | @see ComboBox::addListener, ComboBox::removeListener
  |
  */
pub trait ComboBoxListener
{
    /**
      | Called when a ComboBox has its selected
      | item changed.
      |
      */
    fn combo_box_changed(&mut self, combo_box_that_has_changed: *mut ComboBox);
}

pub enum ComboBoxEditableState
{
    editableUnknown,
    labelIsNotEditable,
    labelIsEditable
}

/**
  | A component that lets the user choose
  | from a drop-down list of choices.
  | 
  | The combo-box has a list of text strings,
  | each with an associated id number, that
  | will be shown in the drop-down list when
  | the user clicks on the component.
  | 
  | The currently selected choice is displayed
  | in the combo-box, and this can either
  | be read-only text, or editable.
  | 
  | To find out when the user selects a different
  | item or edits the text, you can assign
  | a lambda to the onChange member, or register
  | a ComboBox::ComboBoxListener to receive callbacks.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComboBox<'a> {
    base:                   Component<'a>,
    base2:                  SettableTooltipClient,
    base4:                  AsyncUpdater<'a>,

    /**
      | You can assign a lambda to this callback
      | object to have it called when the selected
      | ID is changed.
      |
      */
    on_change:                  fn() -> (),
    current_menu:               PopupMenu<'a>,
    current_id:                 Value<'a>,
    last_current_id:            i32, // default = 0
    is_button_down:             bool, // default = false
    menu_active:                bool, // default = false
    scroll_wheel_enabled:       bool, // default = false
    mouse_wheel_accumulator:    f32, // default = 0
    listeners:                  ListenerList<Rc<RefCell<dyn ComboBoxListener>>>,
    label:                      Box<Label<'a>>,
    text_when_nothing_selected: String,
    no_choices_message:         String,
    label_editable_state:       ComboBoxEditableState, // default = editableUnknown
}

impl<'a> ValueListener for ComboBox<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            if (lastCurrentId != (int) currentId.getValue())
            setSelectedId (currentId.getValue());
        */
    }
}

pub trait ComboBoxInterface: ShowPopup { }

impl<'a> Drop for ComboBox<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            currentId.removeListener (this);
        hidePopup();
        label.reset();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ComboBox.cpp]
impl<'a> ComboBox<'a> {
    
    /**
      | Returns a Value object that can be used
      | to get or set the selected item's ID.
      | 
      | You can call Value::referTo() on this
      | object to make the combo box control
      | another Value object.
      |
      */
    pub fn get_selected_id_as_value(&mut self) -> &mut Value {
        
        todo!();
        /*
            return currentId;
        */
    }

    /**
      | Returns true if the popup menu is currently
      | being shown.
      |
      */
    pub fn is_popup_active(&self) -> bool {
        
        todo!();
        /*
            return menuActive;
        */
    }

    /**
      | Returns the PopupMenu object associated
      | with the ComboBox.
      | 
      | Can be useful for adding sub-menus to
      | the ComboBox standard PopupMenu
      |
      */
    pub fn get_root_menu_mut(&mut self) -> *mut PopupMenu {
        
        todo!();
        /*
            return &currentMenu;
        */
    }

    /**
      | Returns the PopupMenu object associated
      | with the ComboBox.
      |
      */
    pub fn get_root_menu(&self) -> *const PopupMenu {
        
        todo!();
        /*
            return &currentMenu;
        */
    }

    pub fn get_tooltip(&mut self) -> String {
        
        todo!();
        /*
            return label->getTooltip();
        */
    }
    
    /**
      | Creates a combo-box.
      | 
      | On construction, the text field will
      | be empty, so you should call the setSelectedId()
      | or setText() method to choose the initial
      | value before displaying it.
      | 
      | -----------
      | @param componentName
      | 
      | the name to set for the component (see
      | Component::setName())
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : Component (name),
          noChoicesMessage (TRANS("(no choices)"))

        setRepaintsOnMouseActivity (true);
        lookAndFeelChanged();
        currentId.addListener (this);
        */
    }
    
    /**
      | Sets whether the text in the combo-box
      | is editable.
      | 
      | The default state for a new ComboBox
      | is non-editable, and can only be changed
      | by choosing from the drop-down list.
      |
      */
    pub fn set_editable_text(&mut self, is_editable: bool)  {
        
        todo!();
        /*
            if (label->isEditableOnSingleClick() != isEditable || label->isEditableOnDoubleClick() != isEditable)
        {
            label->setEditable (isEditable, isEditable, false);
            labelEditableState = (isEditable ? labelIsEditable : labelIsNotEditable);

            setWantsKeyboardFocus (labelEditableState == labelIsNotEditable);
            resized();
        }
        */
    }
    
    /**
      | Returns true if the text is directly
      | editable. @see setEditableText
      |
      */
    pub fn is_text_editable(&self) -> bool {
        
        todo!();
        /*
            return label->isEditable();
        */
    }
    
    /**
      | Sets the style of justification to be
      | used for positioning the text.
      | 
      | The default is Justification::centredLeft.
      | The text is displayed using a
      | 
      | Label component inside the ComboBox.
      |
      */
    pub fn set_justification_type(&mut self, justification: Justification)  {
        
        todo!();
        /*
            label->setJustificationType (justification);
        */
    }
    
    /**
      | Returns the current justification
      | for the text box. @see setJustificationType
      |
      */
    pub fn get_justification_type(&self) -> Justification {
        
        todo!();
        /*
            return label->getJustificationType();
        */
    }
    
    /**
      | Gives the ComboBox a tooltip.
      |
      */
    pub fn set_tooltip(&mut self, new_tooltip: &String)  {
        
        todo!();
        /*
            SettableTooltipClient::setTooltip (newTooltip);
        label->setTooltip (newTooltip);
        */
    }
    
    /**
      | Adds an item to be shown in the drop-down
      | list.
      | 
      | -----------
      | @param newItemText
      | 
      | the text of the item to show in the list
      | ----------
      | @param newItemId
      | 
      | an associated ID number that can be set
      | or retrieved - see getSelectedId()
      | and setSelectedId(). Note that this
      | value can not be 0! @see setItemEnabled,
      | addSeparator, addSectionHeading,
      | getNumItems, getItemText, getItemId
      |
      */
    pub fn add_item(&mut self, 
        new_item_text: &String,
        new_item_id:   i32)  {
        
        todo!();
        /*
            // you can't add empty strings to the list..
        jassert (newItemText.isNotEmpty());

        // IDs must be non-zero, as zero is used to indicate a lack of selection.
        jassert (newItemId != 0);

        // you shouldn't use duplicate item IDs!
        jassert (getItemForId (newItemId) == nullptr);

        if (newItemText.isNotEmpty() && newItemId != 0)
            currentMenu.addItem (newItemId, newItemText, true, false);
        */
    }
    
    /**
      | Adds an array of items to the drop-down
      | list.
      | 
      | The item ID of each item will be its index
      | in the StringArray + firstItemIdOffset.
      |
      */
    pub fn add_item_list(
        &mut self, 
        items_to_add: &Vec<String>,
        first_itemid: i32

    ) {
        
        todo!();
        /*
            for (auto& i : itemsToAdd)
            currentMenu.addItem (firstItemID++, i);
        */
    }
    
    /**
      | Adds a separator line to the drop-down
      | list.
      | 
      | This is like adding a separator to a popup
      | menu. See typename PopupMenu::addSeparator().
      |
      */
    pub fn add_separator(&mut self)  {
        
        todo!();
        /*
            currentMenu.addSeparator();
        */
    }
    
    /**
      | Adds a heading to the drop-down list,
      | so that you can group the items into different
      | sections.
      | 
      | The headings are indented slightly
      | differently to set them apart from the
      | items on the list, and obviously can't
      | be selected. You might want to add separators
      | between your sections too.
      | 
      | @see addItem, addSeparator
      |
      */
    pub fn add_section_heading(&mut self, heading_name: &String)  {
        
        todo!();
        /*
            // you can't add empty strings to the list..
        jassert (headingName.isNotEmpty());

        if (headingName.isNotEmpty())
            currentMenu.addSectionHeader (headingName);
        */
    }
    
    /**
      | This allows items in the drop-down list
      | to be selectively disabled.
      | 
      | When you add an item, it's enabled by
      | default, but you can call this method
      | to change its status.
      | 
      | If you disable an item which is already
      | selected, this won't change the current
      | selection - it just stops the user choosing
      | that item from the list.
      |
      */
    pub fn set_item_enabled(
        &mut self, 
        item_id:           i32,
        should_be_enabled: bool

    ) {
        
        todo!();
        /*
            if (auto* item = getItemForId (itemId))
            item->isEnabled = shouldBeEnabled;
        */
    }
    
    /**
      | Returns true if the given item is enabled.
      |
      */
    pub fn is_item_enabled(&self, item_id: i32) -> bool {
        
        todo!();
        /*
            if (auto* item = getItemForId (itemId))
            return item->isEnabled;

        return false;
        */
    }
    
    /**
      | Changes the text for an existing item.
      |
      */
    pub fn change_item_text(
        &mut self, 
        item_id:  i32,
        new_text: &String

    ) {
        
        todo!();
        /*
            if (auto* item = getItemForId (itemId))
            item->text = newText;
        else
            jassertfalse;
        */
    }
    
    /**
      | Removes all the items from the drop-down
      | list.
      | 
      | If this call causes the content to be
      | cleared, and a change-message will
      | be broadcast according to the notification
      | parameter.
      | 
      | @see addItem, getNumItems
      |
      */
    pub fn clear(&mut self, notification: Option<NotificationType>)  {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            currentMenu.clear();

        if (! label->isEditable())
            setSelectedItemIndex (-1, notification);
        */
    }
    
    pub fn get_item_for_id(&self, item_id: i32) -> *mut PopupMenuItem {
        
        todo!();
        /*
            if (itemId != 0)
        {
            for (typename PopupMenu::MenuItemIterator iterator (currentMenu, true); iterator.next();)
            {
                auto& item = iterator.getItem();

                if (item.itemID == itemId)
                    return &item;
            }
        }

        return nullptr;
        */
    }
    
    pub fn get_item_for_index(&self, index: i32) -> *mut PopupMenuItem {
        
        todo!();
        /*
            int n = 0;

        for (typename PopupMenu::MenuItemIterator iterator (currentMenu, true); iterator.next();)
        {
            auto& item = iterator.getItem();

            if (item.itemID != 0)
                if (n++ == index)
                    return &item;
        }

        return nullptr;
        */
    }
    
    /**
      | Returns the number of items that have
      | been added to the list.
      | 
      | -----------
      | @note
      | 
      | this doesn't include headers or separators.
      |
      */
    pub fn get_num_items(&self) -> i32 {
        
        todo!();
        /*
            int n = 0;

        for (typename PopupMenu::MenuItemIterator iterator (currentMenu, true); iterator.next();)
        {
            auto& item = iterator.getItem();

            if (item.itemID != 0)
                n++;
        }

        return n;
        */
    }
    
    /**
      | Returns the text for one of the items
      | in the list.
      | 
      | -----------
      | @note
      | 
      | this doesn't include headers or separators.
      | 
      | -----------
      | @param index
      | 
      | the item's index from 0 to (getNumItems()
      | - 1)
      |
      */
    pub fn get_item_text(&self, index: i32) -> String {
        
        todo!();
        /*
            if (auto* item = getItemForIndex (index))
            return item->text;

        return {};
        */
    }
    
    /**
      | Returns the ID for one of the items in
      | the list.
      | 
      | -----------
      | @note
      | 
      | this doesn't include headers or separators.
      | 
      | -----------
      | @param index
      | 
      | the item's index from 0 to (getNumItems()
      | - 1)
      |
      */
    pub fn get_item_id(&self, index: i32) -> i32 {
        
        todo!();
        /*
            if (auto* item = getItemForIndex (index))
            return item->itemID;

        return 0;
        */
    }
    
    /**
      | Returns the index in the list of a particular
      | item ID.
      | 
      | If no such ID is found, this will return
      | -1.
      |
      */
    pub fn index_of_item_id(&self, item_id: i32) -> i32 {
        
        todo!();
        /*
            if (itemId != 0)
        {
            int n = 0;

            for (typename PopupMenu::MenuItemIterator iterator (currentMenu, true); iterator.next();)
            {
                auto& item = iterator.getItem();

                if (item.itemID == itemId)
                    return n;

                else if (item.itemID != 0)
                    n++;
            }
        }

        return -1;
        */
    }
    
    /**
      | Returns the index of the item that's
      | currently shown in the box.
      | 
      | If no item is selected, or if the text
      | is editable and the user has entered
      | something which isn't one of the items
      | in the list, then this will return -1.
      | 
      | @see setSelectedItemIndex, getSelectedId,
      | getText
      |
      */
    pub fn get_selected_item_index(&self) -> i32 {
        
        todo!();
        /*
            auto index = indexOfItemId (currentId.getValue());

        if (getText() != getItemText (index))
            index = -1;

        return index;
        */
    }
    
    /**
      | Sets one of the items to be the current
      | selection.
      | 
      | This will set the ComboBox's text to
      | that of the item at the given index in
      | the list.
      | 
      | -----------
      | @param newItemIndex
      | 
      | the new item to select
      | ----------
      | @param notification
      | 
      | determines the type of change notification
      | that will be sent to listeners if the
      | value changes @see getSelectedItemIndex,
      | setSelectedId, setText
      |
      */
    pub fn set_selected_item_index(
        &mut self, 
        index:        i32,
        notification: Option<NotificationType>

    ) {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            setSelectedId (getItemId (index), notification);
        */
    }
    
    /**
      | Returns the ID of the item that's currently
      | shown in the box.
      | 
      | If no item is selected, or if the text
      | is editable and the user has entered
      | something which isn't one of the items
      | in the list, then this will return 0.
      | 
      | @see setSelectedId, getSelectedItemIndex,
      | getText
      |
      */
    pub fn get_selected_id(&self) -> i32 {
        
        todo!();
        /*
            if (auto* item = getItemForId (currentId.getValue()))
            if (getText() == item->text)
                return item->itemID;

        return 0;
        */
    }
    
    /**
      | Sets one of the items to be the current
      | selection.
      | 
      | This will set the ComboBox's text to
      | that of the item that matches this ID.
      | 
      | -----------
      | @param newItemId
      | 
      | the new item to select
      | ----------
      | @param notification
      | 
      | determines the type of change notification
      | that will be sent to listeners if the
      | value changes @see getSelectedId,
      | setSelectedItemIndex, setText
      |
      */
    pub fn set_selected_id(
        &mut self, 
        new_item_id:  i32,
        notification: Option<NotificationType>

    )  {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            auto* item = getItemForId (newItemId);
        auto newItemText = item != nullptr ? item->text : String();

        if (lastCurrentId != newItemId || label->getText() != newItemText)
        {
            label->setText (newItemText, dontSendNotification);
            lastCurrentId = newItemId;
            currentId = newItemId;

            repaint();  // for the benefit of the 'none selected' text

            sendChange (notification);
        }
        */
    }
    
    pub fn select_if_enabled(&mut self, index: i32) -> bool {
        
        todo!();
        /*
            if (auto* item = getItemForIndex (index))
        {
            if (item->isEnabled)
            {
                setSelectedItemIndex (index);
                return true;
            }
        }

        return false;
        */
    }
    
    pub fn nudge_selected_item(&mut self, delta: i32) -> bool {
        
        todo!();
        /*
            for (int i = getSelectedItemIndex() + delta; isPositiveAndBelow (i, getNumItems()); i += delta)
            if (selectIfEnabled (i))
                return true;

        return false;
        */
    }
    
    /**
      | Returns the text that is currently shown
      | in the combo-box's text field.
      | 
      | If the ComboBox has editable text, then
      | this text may have been edited by the
      | user; otherwise it will be one of the
      | items from the list, or possibly an empty
      | string if nothing was selected.
      | 
      | @see setText, getSelectedId, getSelectedItemIndex
      |
      */
    pub fn get_text(&self) -> String {
        
        todo!();
        /*
            return label->getText();
        */
    }
    
    /**
      | Sets the contents of the combo-box's
      | text field.
      | 
      | The text passed-in will be set as the
      | current text regardless of whether
      | it is one of the items in the list. If the
      | current text isn't one of the items,
      | then getSelectedId() will return 0,
      | otherwise it will return the appropriate
      | ID.
      | 
      | -----------
      | @param newText
      | 
      | the text to select
      | ----------
      | @param notification
      | 
      | determines the type of change notification
      | that will be sent to listeners if the
      | text changes @see getText
      |
      */
    pub fn set_text(
        &mut self, 
        new_text:     &String,
        notification: Option<NotificationType>

    ) {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            for (typename PopupMenu::MenuItemIterator iterator (currentMenu, true); iterator.next();)
        {
            auto& item = iterator.getItem();

            if (item.itemID != 0
                && item.text == newText)
            {
                setSelectedId (item.itemID, notification);
                return;
            }
        }

        lastCurrentId = 0;
        currentId = 0;
        repaint();

        if (label->getText() != newText)
        {
            label->setText (newText, dontSendNotification);
            sendChange (notification);
        }
        */
    }
    
    /**
      | Programmatically opens the text editor
      | to allow the user to edit the current
      | item.
      | 
      | This is the same effect as when the box
      | is clicked-on. @see Label::showEditor();
      |
      */
    pub fn show_editor(&mut self)  {
        
        todo!();
        /*
            jassert (isTextEditable()); // you probably shouldn't do this to a non-editable combo box?

        label->showEditor();
        */
    }
    
    /**
      | Sets a message to display when there
      | is no item currently selected. @see
      | getTextWhenNothingSelected
      |
      */
    pub fn set_text_when_nothing_selected(&mut self, new_message: &String)  {
        
        todo!();
        /*
            if (textWhenNothingSelected != newMessage)
        {
            textWhenNothingSelected = newMessage;
            repaint();
        }
        */
    }
    
    /**
      | Returns the text that is shown when no
      | item is selected. @see setTextWhenNothingSelected
      |
      */
    pub fn get_text_when_nothing_selected(&self) -> String {
        
        todo!();
        /*
            return textWhenNothingSelected;
        */
    }
    
    /**
      | Sets the message to show when there are
      | no items in the list, and the user clicks
      | on the drop-down box.
      | 
      | By default it just says "no choices",
      | but this lets you change it to something
      | more meaningful.
      |
      */
    pub fn set_text_when_no_choices_available(&mut self, new_message: &String)  {
        
        todo!();
        /*
            noChoicesMessage = newMessage;
        */
    }
    
    /**
      | Returns the text shown when no items
      | have been added to the list. @see setTextWhenNoChoicesAvailable
      |
      */
    pub fn get_text_when_no_choices_available(&self) -> String {
        
        todo!();
        /*
            return noChoicesMessage;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawComboBox (g, getWidth(), getHeight(), isButtonDown,
                                       label->getRight(), 0, getWidth() - label->getRight(), getHeight(),
                                       *this);

        if (textWhenNothingSelected.isNotEmpty() && label->getText().isEmpty() && ! label->isBeingEdited())
            getLookAndFeel().drawComboBoxTextWhenNothingSelected (g, *this, *label);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (getHeight() > 0 && getWidth() > 0)
            getLookAndFeel().positionComboBoxText (*this, *label);
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            lookAndFeelChanged();
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            lookAndFeelChanged();
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            repaint();

        {
            std::unique_ptr<Label> newLabel (getLookAndFeel().createComboBoxTextBox (*this));
            jassert (newLabel != nullptr);

            if (label != nullptr)
            {
                newLabel->setEditable (label->isEditable());
                newLabel->setJustificationType (label->getJustificationType());
                newLabel->setTooltip (label->getTooltip());
                newLabel->setText (label->getText(), dontSendNotification);
            }

            std::swap (label, newLabel);
        }

        addAndMakeVisible (label.get());

        ComboBoxEditableState newEditableState = (label->isEditable() ? labelIsEditable : labelIsNotEditable);

        if (newEditableState != labelEditableState)
        {
            labelEditableState = newEditableState;
            setWantsKeyboardFocus (labelEditableState == labelIsNotEditable);
        }

        label->onTextChange = [this] { triggerAsyncUpdate(); };
        label->addMouseListener (this, false);
        label->setAccessible (labelEditableState == labelIsEditable);

        label->setColour (Label::backgroundColourId, Colours::transparentBlack);
        label->setColour (Label::textColourId, findColour (ComboBox::textColourId));

        label->setColour (TextEditor::textColourId, findColour (ComboBox::textColourId));
        label->setColour (TextEditor::backgroundColourId, Colours::transparentBlack);
        label->setColour (TextEditor::highlightColourId, findColour (TextEditor::highlightColourId));
        label->setColour (TextEditor::outlineColourId, Colours::transparentBlack);

        resized();
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (key == KeyPress::upKey || key == KeyPress::leftKey)
        {
            nudgeSelectedItem (-1);
            return true;
        }

        if (key == KeyPress::downKey || key == KeyPress::rightKey)
        {
            nudgeSelectedItem (1);
            return true;
        }

        if (key == KeyPress::returnKey)
        {
            showPopupIfNotActive();
            return true;
        }

        return false;
        */
    }
    
    pub fn key_state_changed(&mut self, is_key_down: bool) -> bool {
        
        todo!();
        /*
            // only forward key events that aren't used by this component
        return isKeyDown
                && (KeyPress::isKeyCurrentlyDown (KeyPress::upKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::leftKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::downKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::rightKey));
        */
    }
    
    pub fn focus_gained(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn show_popup_if_not_active(&mut self)  {
        
        todo!();
        /*
            if (! menuActive)
        {
            menuActive = true;

            // as this method was triggered by a mouse event, the same mouse event may have
            // exited the modal state of other popups currently on the screen. By calling
            // showPopup asynchronously, we are giving the other popups a chance to properly
            // close themselves
            MessageManager::callAsync ([safePointer = SafePointer<ComboBox> { this }]() mutable { if (safePointer != nullptr) safePointer->showPopup(); });
            repaint();
        }
        */
    }
    
    /**
      | Hides the combo box's popup list, if
      | it's currently visible.
      |
      */
    pub fn hide_popup(&mut self)  {
        
        todo!();
        /*
            if (menuActive)
        {
            menuActive = false;
            typename PopupMenu::dismissAllActiveMenus();
            repaint();
        }
        */
    }
    
    pub fn show_popup(&mut self)  {
        
        todo!();
        /*
            if (! menuActive)
            menuActive = true;

        auto menu = currentMenu;

        if (menu.getNumItems() > 0)
        {
            auto selectedId = getSelectedId();

            for (typename PopupMenu::MenuItemIterator iterator (menu, true); iterator.next();)
            {
                auto& item = iterator.getItem();

                if (item.itemID != 0)
                    item.isTicked = (item.itemID == selectedId);
            }
        }
        else
        {
            menu.addItem (1, noChoicesMessage, false, false);
        }

        auto& lf = getLookAndFeel();

        menu.setLookAndFeel (&lf);
        menu.showMenuAsync (lf.getOptionsForComboBoxPopupMenu (*this, *label),
                            ModalCallbackFunction::forComponent (comboBoxPopupMenuFinishedCallback, this));
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            beginDragAutoRepeat (300);

        isButtonDown = isEnabled() && ! e.mods.isPopupMenu();

        if (isButtonDown && (e.eventComponent == this || ! label->isEditable()))
            showPopupIfNotActive();
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            beginDragAutoRepeat (50);

        if (isButtonDown && e.mouseWasDraggedSinceMouseDown())
            showPopupIfNotActive();
        */
    }
    
    pub fn mouse_up(&mut self, e2: &MouseEvent)  {
        
        todo!();
        /*
            if (isButtonDown)
        {
            isButtonDown = false;
            repaint();

            auto e = e2.getEventRelativeTo (this);

            if (reallyContains (e.getPosition(), true)
                 && (e2.eventComponent == this || ! label->isEditable()))
            {
                showPopupIfNotActive();
            }
        }
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            if (! menuActive && scrollWheelEnabled && e.eventComponent == this && wheel.deltaY != 0.0f)
        {
            mouseWheelAccumulator += wheel.deltaY * 5.0f;

            while (mouseWheelAccumulator > 1.0f)
            {
                mouseWheelAccumulator -= 1.0f;
                nudgeSelectedItem (-1);
            }

            while (mouseWheelAccumulator < -1.0f)
            {
                mouseWheelAccumulator += 1.0f;
                nudgeSelectedItem (1);
            }
        }
        else
        {
            Component::mouseWheelMove (e, wheel);
        }
        */
    }
    
    /**
      | This can be used to allow the scroll-wheel
      | to nudge the chosen item.
      | 
      | By default it's disabled, and I'd recommend
      | leaving it disabled if there's any chance
      | that the control might be inside a scrollable
      | list or viewport.
      |
      */
    pub fn set_scroll_wheel_enabled(&mut self, enabled: bool)  {
        
        todo!();
        /*
            scrollWheelEnabled = enabled;
        */
    }
    
    /**
      | Registers a listener that will be called
      | when the box's content changes.
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn ComboBoxListener)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    /**
      | Deregisters a previously-registered
      | listener.
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn ComboBoxListener)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);
        listeners.callChecked (checker, [this] (ComboBoxListener& l) { l.comboBoxChanged (this); });

        if (checker.shouldBailOut())
            return;

        if (onChange != nullptr)
            onChange();
        */
    }
    
    pub fn send_change(&mut self, notification: NotificationType)  {
        
        todo!();
        /*
            if (notification != dontSendNotification)
            triggerAsyncUpdate();

        if (notification == sendNotificationSync)
            handleUpdateNowIfNeeded();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<ComboBoxAccessibilityHandler> (*this);
        */
    }
}
