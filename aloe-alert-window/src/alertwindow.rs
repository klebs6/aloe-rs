crate::ix!();

pub type AlertIconType = MessageBoxIconType;

pub const ALERT_WINDOW_NO_ICON:       AlertIconType = MessageBoxIconType::NoIcon;
pub const ALERT_WINDOW_QUESTION_ICON: AlertIconType = MessageBoxIconType::QuestionIcon;
pub const ALERT_WINDOW_WARNING_ICON:  AlertIconType = MessageBoxIconType::WarningIcon;
pub const ALERT_WINDOW_INFO_ICON:     AlertIconType = MessageBoxIconType::InfoIcon;

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_AlertWindow.h]

/**
  | A window that displays a message and
  | has buttons for the user to react to it.
  | 
  | For simple dialog boxes with just a couple
  | of buttons on them, there are some static
  | methods for running these.
  | 
  | For more complex dialogs, an AlertWindow
  | can be created, then it can have some
  | buttons and components added to it,
  | and its runModalLoop() method is then
  | used to show it. The value returned by
  | runModalLoop() shows which button
  | the user pressed to dismiss the box.
  | 
  | @see ThreadWithProgressWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AlertWindow<'a> {
    base:                     TopLevelWindow<'a>,
    text:                     String,
    text_layout:              TextLayout,
    accessible_message_label: Label<'a>,
    alert_icon_type:          MessageBoxIconType,
    constrainer:              ComponentBoundsConstrainer,
    dragger:                  ComponentDragger,
    text_area:                Rectangle<i32>,
    buttons:                  Vec<Box<TextButton<'a>>>,
    text_boxes:               Vec<Box<TextEditor<'a>>>,
    combo_boxes:              Vec<Box<ComboBox<'a>>>,
    progress_bars:            Vec<Box<ProgressBar<'a>>>,
    custom_comps:             Vec<*mut Component<'a>>,
    text_blocks:              Vec<Box<Component<'a>>>,
    all_comps:                Vec<*mut Component<'a>>,
    textbox_names:            Vec<String>,
    combo_box_names:          Vec<String>,
    associated_component:     *const Component<'a>,
    escape_key_cancels:       bool, // default = true
    desktop_scale:            f32, // default = 1.0f
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_AlertWindow.cpp]
impl<'a> Drop for AlertWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            // Ensure that the focus does not jump to another TextEditor while we
        // remove children.
        for (auto* t : textBoxes)
            t->setWantsKeyboardFocus (false);

        // Give away focus before removing the editors, so that any TextEditor
        // with focus has a chance to dismiss native keyboard if shown.
        giveAwayKeyboardFocus();

        removeAllChildren();
        */
    }
}

impl<'a> AlertWindow<'a> {

    /**
      | Returns the type of alert icon that was
      | specified when the window was created.
      |
      */
    pub fn get_alert_type(&self) -> MessageBoxIconType {
        
        todo!();
        /*
            return alertIconType;
        */
    }
    
    /**
      | Shows a dialog box using the specified
      | options.
      | 
      | The box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and the callback
      | will be invoked later when the user dismisses
      | the box.
      | 
      | -----------
      | @param options
      | 
      | the options to use when creating the
      | dialog.
      | ----------
      | @param callback
      | 
      | if this is non-null, the callback will
      | receive a call to its modalStateFinished()
      | when the box is dismissed with the index
      | of the button that was clicked as its
      | argument.
      | 
      | The callback object will be owned and
      | deleted by the system, so make sure that
      | it works safely and doesn't keep any
      | references to objects that might be
      | deleted before it gets called.
      | 
      | @see MessageBoxOptions
      |
      */
    pub fn show_async_with_modal_callback(
        &mut self, 
        options:  &MessageBoxOptions,
        callback: *mut dyn ModalComponentManagerCallback

    )  {
        
        todo!();
        /*
            if (LookAndFeel::getDefaultLookAndFeel().isUsingNativeAlertWindows())
        {
            NativeMessageBox::showAsync (options, callback);
        }
        else
        {
            AlertWindowInfo info (options, rawToUniquePtr (callback), Async::yes);
            info.invoke();
        }
        */
    }
    
    /**
      | Shows a dialog box using the specified
      | options.
      | 
      | The box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and the callback
      | will be invoked later when the user dismisses
      | the box.
      | 
      | -----------
      | @param options
      | 
      | the options to use when creating the
      | dialog.
      | ----------
      | @param callback
      | 
      | if this is non-null, the callback will
      | be called when the box is dismissed with
      | the index of the button that was clicked
      | as its argument.
      | 
      | @see MessageBoxOptions
      |
      */
    pub fn show_async(&mut self, 
        options:  &MessageBoxOptions,
        callback: fn(_0: i32) -> ())  {
        
        todo!();
        /*
            showAsync (options, ModalCallbackFunction::create (callback));
        */
    }

    pub fn get_desktop_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            return desktopScale;
        */
    }
    
    /**
      | Creates an AlertWindow.
      | 
      | -----------
      | @param title
      | 
      | the headline to show at the top of the
      | dialog box
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the headline
      | ----------
      | @param iconType
      | 
      | the type of icon to display
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      |
      */
    pub fn new(
        title:     &String,
        message:   &String,
        icon_type: MessageBoxIconType,
        comp:      *mut Component<'a>) -> Self {

        todo!();
        /*


            : TopLevelWindow (title, true),
         alertIconType (iconType),
         associatedComponent (comp),
         desktopScale (comp != nullptr ? Component::getApproximateScaleFactorForComponent (comp) : 1.0f)

        setAlwaysOnTop (aloe_areThereAnyAlwaysOnTopWindows());

        accessibleMessageLabel.setColour (Label::textColourId, Colours::transparentBlack);
        addAndMakeVisible (accessibleMessageLabel);

        if (message.isEmpty())
            text = " "; // to force an update if the message is empty

        setMessage (message);

        AlertWindow::lookAndFeelChanged();
        constrainer.setMinimumOnscreenAmounts (0x10000, 0x10000, 0x10000, 0x10000);
        */
    }
    
    pub fn user_tried_to_close_window(&mut self)  {
        
        todo!();
        /*
            if (escapeKeyCancels || buttons.size() > 0)
            exitModalState (0);
        */
    }
    
    /**
      | Changes the dialog box's message.
      | 
      | This will also resize the window to fit
      | the new message if required.
      |
      */
    pub fn set_message(&mut self, message: &String)  {
        
        todo!();
        /*
            auto newMessage = message.substring (0, 2048);

        if (text != newMessage)
        {
            text = newMessage;

            auto accessibleText = getName() + ". " + text;
            accessibleMessageLabel.setText (accessibleText, NotificationType::dontSendNotification);
            setDescription (accessibleText);

            updateLayout (true);
            repaint();
        }
        */
    }
    
    pub fn exit_alert(&mut self, button: *mut Button)  {
        
        todo!();
        /*
            if (auto* parent = button->getParentComponent())
            parent->exitModalState (button->getCommandID());
        */
    }
    
    /**
      | Adds a button to the window.
      | 
      | -----------
      | @param name
      | 
      | the text to show on the button
      | ----------
      | @param returnValue
      | 
      | the value that should be returned from
      | runModalLoop() if this is the button
      | that the user presses.
      | ----------
      | @param shortcutKey1
      | 
      | an optional key that can be pressed to
      | trigger this button
      | ----------
      | @param shortcutKey2
      | 
      | a second optional key that can be pressed
      | to trigger this button
      |
      */
    pub fn add_button(&mut self, 
        name:          &String,
        return_value:  i32,
        shortcut_key1: Option<&KeyPress>,
        shortcut_key2: Option<&KeyPress>)  {

        let shortcut_key1: &KeyPress = shortcut_key1.unwrap_or(&KeyPress::default());
        let shortcut_key2: &KeyPress = shortcut_key2.unwrap_or(&KeyPress::default());
        
        todo!();
        /*
            auto* b = new TextButton (name, {});
        buttons.add (b);

        b->setWantsKeyboardFocus (true);
        b->setExplicitFocusOrder (1);
        b->setMouseClickGrabsKeyboardFocus (false);
        b->setCommandToTrigger (nullptr, returnValue, false);
        b->addShortcut (shortcutKey1);
        b->addShortcut (shortcutKey2);
        b->onClick = [this, b] { exitAlert (b); };

        Vec<TextButton*> buttonsArray (buttons.begin(), buttons.size());
        auto& lf = getLookAndFeel();

        auto buttonHeight = lf.getAlertWindowButtonHeight();
        auto buttonWidths = lf.getWidthsForTextButtons (*this, buttonsArray);

        jassert (buttonWidths.size() == buttons.size());
        int i = 0;

        for (auto* button : buttons)
            button->setSize (buttonWidths[i++], buttonHeight);

        addAndMakeVisible (b, 0);
        updateLayout (false);
        */
    }
    
    /**
      | Returns the number of buttons that the
      | window currently has.
      |
      */
    pub fn get_num_buttons(&self) -> i32 {
        
        todo!();
        /*
            return buttons.size();
        */
    }
    
    /**
      | Invokes a click of one of the buttons.
      |
      */
    pub fn trigger_button_click(&mut self, button_name: &String)  {
        
        todo!();
        /*
            for (auto* b : buttons)
        {
            if (buttonName == b->getName())
            {
                b->triggerClick();
                break;
            }
        }
        */
    }
    
    /**
      | If set to true and the window contains
      | no buttons, then pressing the escape
      | key will make the alert cancel its modal
      | state.
      | 
      | By default this setting is true - turn
      | it off if you don't want the box to respond
      | to the escape key. Note that it is ignored
      | if you have any buttons, and in that case
      | you should give the buttons appropriate
      | keypresses to trigger cancelling if
      | you want to.
      |
      */
    pub fn set_escape_key_cancels(&mut self, should_escape_key_cancel: bool)  {
        
        todo!();
        /*
            escapeKeyCancels = shouldEscapeKeyCancel;
        */
    }
    
    /**
      | Adds a textbox to the window for entering
      | strings.
      | 
      | -----------
      | @param name
      | 
      | an internal name for the text-box. This
      | is the name to pass to the getTextEditorContents()
      | method to find out what the user typed-in.
      | ----------
      | @param initialContents
      | 
      | a string to show in the text box when it's
      | first shown
      | ----------
      | @param onScreenLabel
      | 
      | if this is non-empty, it will be displayed
      | next to the text-box to label it.
      | ----------
      | @param isPasswordBox
      | 
      | if true, the text editor will display
      | asterisks instead of the actual text
      | @see getTextEditorContents
      |
      */
    pub fn add_text_editor(
        &mut self, 
        name:             &String,
        initial_contents: &String,
        on_screen_label:  Option<&String>,
        is_password_box:  Option<bool>

    )  {
        let on_screen_label = on_screen_label.unwrap_or(&String::new());
        let is_password_box: bool = is_password_box.unwrap_or(false);
        
        todo!();
        /*
            auto* ed = new TextEditor (name, isPasswordBox ? getDefaultPasswordChar() : 0);
        ed->setSelectAllWhenFocused (true);
        ed->setEscapeAndReturnKeysConsumed (false);
        textBoxes.add (ed);
        allComps.add (ed);

        ed->setColour (TextEditor::outlineColourId, findColour (ComboBox::outlineColourId));
        ed->setFont (getLookAndFeel().getAlertWindowMessageFont());
        addAndMakeVisible (ed);
        ed->setText (initialContents);
        ed->setCaretPosition (initialContents.length());
        textboxNames.add (onScreenLabel);

        updateLayout (false);
        */
    }
    
    /**
      | Returns a pointer to a textbox that was
      | added with addTextEditor().
      |
      */
    pub fn get_text_editor(&self, name_of_text_editor: &String) -> *mut TextEditor {
        
        todo!();
        /*
            for (auto* tb : textBoxes)
            if (tb->getName() == nameOfTextEditor)
                return tb;

        return nullptr;
        */
    }
    
    /**
      | Returns the contents of a named textbox.
      | 
      | After showing an AlertWindow that contains
      | a text editor, this can be used to find
      | out what the user has typed into it.
      | 
      | -----------
      | @param nameOfTextEditor
      | 
      | the name of the text box that you're interested
      | in @see addTextEditor
      |
      */
    pub fn get_text_editor_contents(&self, name_of_text_editor: &String) -> String {
        
        todo!();
        /*
            if (auto* t = getTextEditor (nameOfTextEditor))
            return t->getText();

        return {};
        */
    }
    
    /**
      | Adds a drop-down list of choices to the
      | box.
      | 
      | After the box has been shown, the getComboBoxComponent()
      | method can be used to find out which item
      | the user picked.
      | 
      | -----------
      | @param name
      | 
      | the label to use for the drop-down list
      | ----------
      | @param items
      | 
      | the list of items to show in it
      | ----------
      | @param onScreenLabel
      | 
      | if this is non-empty, it will be displayed
      | next to the combo-box to label it. @see
      | getComboBoxComponent
      |
      */
    pub fn add_combo_box(
        &mut self, 
        name:            &String,
        items:           &Vec<String>,
        on_screen_label: Option<&String>

    )  {

        let on_screen_label = on_screen_label.unwrap_or(&String::new());
        
        todo!();
        /*
            auto* cb = new ComboBox (name);
        comboBoxes.add (cb);
        allComps.add (cb);

        cb->addItemList (items, 1);

        addAndMakeVisible (cb);
        cb->setSelectedItemIndex (0);

        comboBoxNames.add (onScreenLabel);
        updateLayout (false);
        */
    }
    
    /**
      | Returns a drop-down list that was added
      | to the AlertWindow.
      | 
      | -----------
      | @param nameOfList
      | 
      | the name that was passed into the addComboBox()
      | method when creating the drop-down
      | 
      | -----------
      | @return
      | 
      | the ComboBox component, or nullptr
      | if none was found for the given name.
      |
      */
    pub fn get_combo_box_component(&self, name_of_list: &String) -> *mut ComboBox {
        
        todo!();
        /*
            for (auto* cb : comboBoxes)
            if (cb->getName() == nameOfList)
                return cb;

        return nullptr;
        */
    }
    
    /**
      | Adds a block of text.
      | 
      | This is handy for adding a multi-line
      | note next to a textbox or combo-box,
      | to provide more details about what's
      | going on.
      |
      */
    pub fn add_text_block(&mut self, text_block: &String)  {
        
        todo!();
        /*
            auto* c = new AlertTextComp (*this, textBlock, getLookAndFeel().getAlertWindowMessageFont());
        textBlocks.add (c);
        allComps.add (c);
        addAndMakeVisible (c);

        updateLayout (false);
        */
    }
    
    /**
      | Adds a progress-bar to the window.
      | 
      | -----------
      | @param progressValue
      | 
      | a variable that will be repeatedly checked
      | while the dialog box is visible, to see
      | how far the process has got. The value
      | should be in the range 0 to 1.0
      |
      */
    pub fn add_progress_bar_component(&mut self, progress_value: &mut f64)  {
        
        todo!();
        /*
            auto* pb = new ProgressBar (progressValue);
        progressBars.add (pb);
        allComps.add (pb);
        addAndMakeVisible (pb);

        updateLayout (false);
        */
    }
    
    /**
      | Adds a user-defined component to the
      | dialog box.
      | 
      | -----------
      | @param component
      | 
      | the component to add - its size should
      | be set up correctly before it is passed
      | in. The caller is responsible for deleting
      | the component later on - the AlertWindow
      | won't delete it.
      |
      */
    pub fn add_custom_component(&mut self, component: *mut Component<'a>)  {
        
        todo!();
        /*
            customComps.add (component);
        allComps.add (component);
        addAndMakeVisible (component);

        updateLayout (false);
        */
    }
    
    /**
      | Returns the number of custom components
      | in the dialog box. @see getCustomComponent,
      | addCustomComponent
      |
      */
    pub fn get_num_custom_components(&self) -> i32 {
        
        todo!();
        /*
            return customComps.size();
        */
    }
    
    /**
      | Returns one of the custom components
      | in the dialog box.
      | 
      | -----------
      | @param index
      | 
      | a value 0 to (getNumCustomComponents()
      | - 1).
      | 
      | Out-of-range indexes will return nullptr
      | @see getNumCustomComponents, addCustomComponent
      |
      */
    pub fn get_custom_component(&self, index: i32) -> *mut Component {
        
        todo!();
        /*
            return customComps [index];
        */
    }
    
    /**
      | Removes one of the custom components
      | in the dialog box.
      | 
      | -----------
      | @note
      | 
      | this won't delete it, it just removes
      | the component from the window
      | 
      | -----------
      | @param index
      | 
      | a value 0 to (getNumCustomComponents()
      | - 1).
      | 
      | Out-of-range indexes will return nullptr
      | 
      | -----------
      | @return
      | 
      | the component that was removed (or null)
      | @see getNumCustomComponents, addCustomComponent
      |
      */
    pub fn remove_custom_component(&mut self, index: i32) -> *mut Component {
        
        todo!();
        /*
            auto* c = getCustomComponent (index);

        if (c != nullptr)
        {
            customComps.removeFirstMatchingValue (c);
            allComps.removeFirstMatchingValue (c);
            removeChildComponent (c);

            updateLayout (false);
        }

        return c;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();
        lf.drawAlertBox (g, *this, textArea, textLayout);

        g.setColour (findColour (textColourId));
        g.setFont (lf.getAlertWindowFont());

        for (int i = textBoxes.size(); --i >= 0;)
        {
            auto* te = textBoxes.getUnchecked(i);

            g.drawFittedText (textboxNames[i],
                              te->getX(), te->getY() - 14,
                              te->getWidth(), 14,
                              Justification::centredLeft, 1);
        }

        for (int i = comboBoxNames.size(); --i >= 0;)
        {
            auto* cb = comboBoxes.getUnchecked(i);

            g.drawFittedText (comboBoxNames[i],
                              cb->getX(), cb->getY() - 14,
                              cb->getWidth(), 14,
                              Justification::centredLeft, 1);
        }

        for (auto* c : customComps)
            g.drawFittedText (c->getName(),
                              c->getX(), c->getY() - 14,
                              c->getWidth(), 14,
                              Justification::centredLeft, 1);
        */
    }
    
    pub fn update_layout(&mut self, only_increase_size: bool)  {
        
        todo!();
        /*
            const int titleH = 24;
        const int iconWidth = 80;

        auto& lf = getLookAndFeel();
        auto messageFont (lf.getAlertWindowMessageFont());

        auto wid = jmax (messageFont.getStringWidth (text),
                         messageFont.getStringWidth (getName()));

        auto sw = (int) std::sqrt (messageFont.getHeight() * (float) wid);
        auto w = jmin (300 + sw * 2, (int) ((float) getParentWidth() * 0.7f));
        const int edgeGap = 10;
        const int labelHeight = 18;
        int iconSpace = 0;

        AttributedString attributedText;
        attributedText.append (getName(), lf.getAlertWindowTitleFont());

        if (text.isNotEmpty())
            attributedText.append ("\n\n" + text, messageFont);

        attributedText.setColour (findColour (textColourId));

        if (alertIconType == NoIcon)
        {
            attributedText.setJustification (Justification::centredTop);
            textLayout.createLayoutWithBalancedLineLengths (attributedText, (float) w);
        }
        else
        {
            attributedText.setJustification (Justification::topLeft);
            textLayout.createLayoutWithBalancedLineLengths (attributedText, (float) w);
            iconSpace = iconWidth;
        }

        w = jmax (350, (int) textLayout.getWidth() + iconSpace + edgeGap * 4);
        w = jmin (w, (int) ((float) getParentWidth() * 0.7f));

        auto textLayoutH = (int) textLayout.getHeight();
        auto textBottom = 16 + titleH + textLayoutH;
        int h = textBottom;

        int buttonW = 40;

        for (auto* b : buttons)
            buttonW += 16 + b->getWidth();

        w = jmax (buttonW, w);

        h += (textBoxes.size() + comboBoxes.size() + progressBars.size()) * 50;

        if (auto* b = buttons[0])
            h += 20 + b->getHeight();

        for (auto* c : customComps)
        {
            w = jmax (w, (c->getWidth() * 100) / 80);
            h += 10 + c->getHeight();

            if (c->getName().isNotEmpty())
                h += labelHeight;
        }

        for (auto* tb : textBlocks)
            w = jmax (w, static_cast<const AlertTextComp*> (tb)->bestWidth);

        w = jmin (w, (int) ((float) getParentWidth() * 0.7f));

        for (auto* tb : textBlocks)
        {
            auto* ac = static_cast<AlertTextComp*> (tb);
            ac->updateLayout ((int) ((float) w * 0.8f));
            h += ac->getHeight() + 10;
        }

        h = jmin (getParentHeight() - 50, h);

        if (onlyIncreaseSize)
        {
            w = jmax (w, getWidth());
            h = jmax (h, getHeight());
        }

        if (! isVisible())
            centreAroundComponent (associatedComponent, w, h);
        else
            setBounds (getBounds().withSizeKeepingCentre (w, h));

        textArea.setBounds (edgeGap, edgeGap, w - (edgeGap * 2), h - edgeGap);
        accessibleMessageLabel.setBounds (textArea);

        const int spacer = 16;
        int totalWidth = -spacer;

        for (auto* b : buttons)
            totalWidth += b->getWidth() + spacer;

        auto x = (w - totalWidth) / 2;
        auto y = (int) ((float) getHeight() * 0.95f);

        for (auto* c : buttons)
        {
            int ny = proportionOfHeight (0.95f) - c->getHeight();
            c->setTopLeftPosition (x, ny);
            if (ny < y)
                y = ny;

            x += c->getWidth() + spacer;

            c->toFront (false);
        }

        y = textBottom;

        for (auto* c : allComps)
        {
            h = 22;

            const int comboIndex = comboBoxes.indexOf (dynamic_cast<ComboBox*> (c));
            if (comboIndex >= 0 && comboBoxNames [comboIndex].isNotEmpty())
                y += labelHeight;

            const int tbIndex = textBoxes.indexOf (dynamic_cast<TextEditor*> (c));
            if (tbIndex >= 0 && textboxNames[tbIndex].isNotEmpty())
                y += labelHeight;

            if (customComps.contains (c))
            {
                if (c->getName().isNotEmpty())
                    y += labelHeight;

                c->setTopLeftPosition (proportionOfWidth (0.1f), y);
                h = c->getHeight();
            }
            else if (textBlocks.contains (c))
            {
                c->setTopLeftPosition ((getWidth() - c->getWidth()) / 2, y);
                h = c->getHeight();
            }
            else
            {
                c->setBounds (proportionOfWidth (0.1f), y, proportionOfWidth (0.8f), h);
            }

            y += h + 10;
        }

        setWantsKeyboardFocus (getNumChildComponents() == 0);
        */
    }
    
    /**
      | Returns true if the window contains
      | any components other than just buttons.
      |
      */
    pub fn contains_any_extra_components(&self) -> bool {
        
        todo!();
        /*
            return allComps.size() > 0;
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            dragger.startDraggingComponent (this, e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            dragger.dragComponent (this, e, &constrainer);
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            for (auto* b : buttons)
        {
            if (b->isRegisteredForShortcut (key))
            {
                b->triggerClick();
                return true;
            }
        }

        if (key.isKeyCode (KeyPress::escapeKey) && escapeKeyCancels)
        {
            exitModalState (0);
            return true;
        }

        if (key.isKeyCode (KeyPress::returnKey) && buttons.size() == 1)
        {
            buttons.getUnchecked(0)->triggerClick();
            return true;
        }

        return false;
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            const int newFlags = getLookAndFeel().getAlertBoxWindowFlags();

        setUsingNativeTitleBar ((newFlags & ComponentPeer::windowHasTitleBar) != 0);
        setDropShadowEnabled (isOpaque() && (newFlags & ComponentPeer::windowHasDropShadow) != 0);
        updateLayout (false);
        */
    }
    
    pub fn get_desktop_window_style_flags(&self) -> i32 {
        
        todo!();
        /*
            return getLookAndFeel().getAlertBoxWindowFlags();
        */
    }
    
    /**
      | Shows a dialog box that just has a message
      | and a single button to get rid of it.
      | 
      | The box is shown modally, and the method
      | will block until the user has clicked
      | the button (or pressed the escape or
      | return keys).
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the headline
      | ----------
      | @param buttonText
      | 
      | the text to show in the button - if this
      | string is empty, the default string
      | "OK" (or a localised version) will be
      | used.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_message_box(&mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        button_text:          &String,
        associated_component: *mut Component<'a>)  {

        let button_text: &String = button_text.unwrap_or(String);
        let associated_component: *mut Component = associated_component.unwrap_or(nullptr);
        
        todo!();
        /*
            show (MessageBoxOptions()
                .withIconType (iconType)
                .withTitle (title)
                .withMessage (message)
                .withButton (buttonText.isEmpty() ? TRANS("OK") : buttonText)
                .withAssociatedComponent (associatedComponent));
        */
    }
    
    /**
      | Shows a dialog box using the specified
      | options.
      | 
      | The box is shown modally, and the method
      | will block until the user dismisses
      | it.
      | 
      | -----------
      | @param options
      | 
      | the options to use when creating the
      | dialog.
      | 
      | -----------
      | @return
      | 
      | the index of the button that was clicked.
      | 
      | @see MessageBoxOptions
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show(&mut self, options: &MessageBoxOptions) -> i32 {
        
        todo!();
        /*
            if (LookAndFeel::getDefaultLookAndFeel().isUsingNativeAlertWindows())
            return NativeMessageBox::show (options);

        AlertWindowInfo info (options, nullptr, Async::no);
        return info.invoke();
        */
    }
    
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_native_dialog_box(&mut self, 
        title:        &String,
        body_text:    &String,
        is_ok_cancel: bool) -> bool {
        
        todo!();
        /*
            if (isOkCancel)
            return NativeMessageBox::showOkCancelBox (AlertWindow::NoIcon, title, bodyText);

        NativeMessageBox::showMessageBox (AlertWindow::NoIcon, title, bodyText);
        return true;
        */
    }
    
    /**
      | Shows a dialog box that just has a message
      | and a single button to get rid of it.
      | 
      | The box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and if a callback
      | was supplied, it will be invoked later
      | when the user dismisses the box.
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the headline
      | ----------
      | @param buttonText
      | 
      | the text to show in the button - if this
      | string is empty, the default string
      | "OK" (or a localised version) will be
      | used.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      | ----------
      | @param callback
      | 
      | if this is non-null, the callback will
      | receive a call to its modalStateFinished()
      | when the box is dismissed. The callback
      | object will be owned and deleted by the
      | system, so make sure that it works safely
      | and doesn't keep any references to objects
      | that might be deleted before it gets
      | called.
      |
      */
    pub fn show_message_box_async(
        &mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        button_text:          Option<&String>,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback

    )  {

        let button_text = button_text.unwrap_or(&String::new());
        
        todo!();
        /*
            showAsync (MessageBoxOptions()
                     .withIconType (iconType)
                     .withTitle (title)
                     .withMessage (message)
                     .withButton (buttonText.isEmpty() ? TRANS("OK") : buttonText)
                     .withAssociatedComponent (associatedComponent),
                   callback);
        */
    }

    /**
      | Shows a dialog box with two buttons.
      | 
      | Ideal for ok/cancel or yes/no choices.
      | The return key can also be used to trigger
      | the first button, and the escape key
      | for the second button.
      | 
      | If the callback parameter is null, the
      | box is shown modally, and the method
      | will block until the user has clicked
      | the button (or pressed the escape or
      | return keys).
      | 
      | If the callback parameter is non-null,
      | the box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and the callback
      | will be invoked later when the user dismisses
      | the box.
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the headline
      | ----------
      | @param button1Text
      | 
      | the text to show in the first button -
      | if this string is empty, the default
      | string "OK" (or a localised version
      | of it) will be used.
      | ----------
      | @param button2Text
      | 
      | the text to show in the second button
      | - if this string is empty, the default
      | string "cancel" (or a localised version
      | of it) will be used.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      | ----------
      | @param callback
      | 
      | if this is non-null, the menu will be
      | launched asynchronously, returning
      | immediately, and the callback will
      | receive a call to its modalStateFinished()
      | when the box is dismissed, with its parameter
      | being 1 if the ok button was pressed,
      | or 0 for cancel. The callback object
      | will be owned and deleted by the system,
      | so make sure that it works safely and
      | doesn't keep any references to objects
      | that might be deleted before it gets
      | called.
      | 
      | -----------
      | @return
      | 
      | true if button 1 was clicked, false if
      | it was button 2. If the callback parameter
      | is not null, the method always returns
      | false, and the user's choice is delivered
      | later by the callback.
      |
      */
    pub fn show_ok_cancel_box(
        &mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        button_1text:         &String,
        button_2text:         &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback

    ) -> bool {
        
        todo!();
        /*
            return showMaybeAsync (MessageBoxOptions()
                                 .withIconType (iconType)
                                 .withTitle (title)
                                 .withMessage (message)
                                 .withButton (button1Text.isEmpty() ? TRANS("OK")     : button1Text)
                                 .withButton (button2Text.isEmpty() ? TRANS("Cancel") : button2Text)
                                 .withAssociatedComponent (associatedComponent),
                               callback,
                               LookAndFeel::getDefaultLookAndFeel().isUsingNativeAlertWindows()
                                   ? AlertWindowMappings::okCancel
                                   : AlertWindowMappings::noMapping) == 1;
        */
    }

     /**
      | Shows a dialog box with three buttons.
      | 
      | Ideal for yes/no/cancel boxes.
      | 
      | The escape key can be used to trigger
      | the third button.
      | 
      | If the callback parameter is null, the
      | box is shown modally, and the method
      | will block until the user has clicked
      | the button (or pressed the escape or
      | return keys).
      | 
      | If the callback parameter is non-null,
      | the box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and the callback
      | will be invoked later when the user dismisses
      | the box.
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the headline
      | ----------
      | @param button1Text
      | 
      | the text to show in the first button -
      | if an empty string, then "yes" will be
      | used (or a localised version of it)
      | ----------
      | @param button2Text
      | 
      | the text to show in the first button -
      | if an empty string, then "no" will be
      | used (or a localised version of it)
      | ----------
      | @param button3Text
      | 
      | the text to show in the first button -
      | if an empty string, then "cancel" will
      | be used (or a localised version of it)
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      | ----------
      | @param callback
      | 
      | if this is non-null, the menu will be
      | launched asynchronously, returning
      | immediately, and the callback will
      | receive a call to its modalStateFinished()
      | when the box is dismissed, with its parameter
      | being 1 if the "yes" button was pressed,
      | 2 for the "no" button, or 0 if it was cancelled.
      | The callback object will be owned and
      | deleted by the system, so make sure that
      | it works safely and doesn't keep any
      | references to objects that might be
      | deleted before it gets called.
      | 
      | -----------
      | @return
      | 
      | If the callback parameter has been set,
      | this returns 0. Otherwise, it returns
      | one of the following values:
      | 
      | - 0 if the third button was pressed (normally
      | used for 'cancel')
      | 
      | - 1 if the first button was pressed (normally
      | used for 'yes')
      | 
      | - 2 if the middle button was pressed (normally
      | used for 'no')
      |
      */
    pub fn show_yes_no_cancel_box(
        &mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        button_1text:         &String,
        button_2text:         &String,
        button_3text:         &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback

    ) -> i32 {
        
        todo!();
        /*
            return showMaybeAsync (MessageBoxOptions()
                                 .withIconType (iconType)
                                 .withTitle (title)
                                 .withMessage (message)
                                 .withButton (button1Text.isEmpty() ? TRANS("Yes")    : button1Text)
                                 .withButton (button2Text.isEmpty() ? TRANS("No")     : button2Text)
                                 .withButton (button3Text.isEmpty() ? TRANS("Cancel") : button3Text)
                                 .withAssociatedComponent (associatedComponent),
                               callback,
                               LookAndFeel::getDefaultLookAndFeel().isUsingNativeAlertWindows()
                                   ? AlertWindowMappings::yesNoCancel
                                   : AlertWindowMappings::noMapping);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::dialogWindow);
        */
    }
}
