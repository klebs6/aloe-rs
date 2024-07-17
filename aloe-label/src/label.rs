crate::ix!();

pub fn copy_colour_if_specified(
        l:               &mut Label,
        ed:              &mut TextEditor,
        colourid:        i32,
        target_colourid: i32)  {
    
    todo!();
        /*
            if (l.isColourSpecified (colourID) || l.getLookAndFeel().isColourSpecified (colourID))
            ed.setColour (targetColourID, l.findColour (colourID));
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_Label.h]

/**
  | A component that displays a text string,
  | and can optionally become a text editor
  | when clicked.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Label<'a> {
    base:           Component<'a>,
    base2:          SettableTooltipClient,

    /**
      | You can assign a lambda to this callback
      | object to have it called when the label
      | text is changed.
      |
      */
    on_text_change: fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the label's
      | editor is shown.
      |
      */
    on_editor_show: fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the label's
      | editor is hidden.
      |
      */
    on_editor_hide: fn() -> (),

    text_value:                     Value<'a>,
    last_text_value:                String,
    font:                           Font, // default = { 15.0  }
    justification:                  Justification, //= Justification::centredLeft;

    editor:                         Box<TextEditor<'a>>,
    listeners:                      ListenerList<Rc<RefCell<dyn LabelListener>>>,
    owner_component:                WeakReference<Component<'a>>,
    border:                         BorderSize<i32>, //{ 1, 5, 1, 5 };

    minimum_horizontal_scale:       f32, // default = 0

    /**
       = TextInputTarget::textKeyboard;
      */
    keyboard_type:                  VirtualKeyboardType,

    edit_single_click:              bool, // default = false
    edit_double_click:              bool, // default = false
    loss_of_focus_discards_changes: bool, // default = false
    left_of_owner_comp:             bool, // default = false
}

impl<'a> LabelListener for Label<'a> {

}

impl<'a> ComponentListener for Label<'a> {}

impl<'a> ComponentMovedOrResized for Label<'a> {

    fn component_moved_or_resized(&mut self, 
        component:   &mut Component,
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();
        auto f = lf.getLabelFont (*this);
        auto borderSize = lf.getLabelBorderSize (*this);

        if (leftOfOwnerComp)
        {
            auto width = jmin (roundToInt (f.getStringWidthFloat (textValue.toString()) + 0.5f)
                                 + borderSize.getLeftAndRight(),
                               component.getX());

            setBounds (component.getX() - width, component.getY(), width, component.getHeight());
        }
        else
        {
            auto height = borderSize.getTopAndBottom() + 6 + roundToInt (f.getHeight() + 0.5f);

            setBounds (component.getX(), component.getY() - height, component.getWidth(), height);
        }
        */
    }
}

impl<'a> ComponentBroughtToFront for Label<'a> {

}

impl<'a> ComponentVisibilityChanged for Label<'a> {

}

impl<'a> ComponentChildrenChanged for Label<'a> {

}

impl<'a> ComponentParentHierarchyChanged for Label<'a> {

}

impl<'a> ComponentNameChanged for Label<'a> {

}

impl<'a> ComponentBeingDeleted for Label<'a> {

}

impl<'a> ComponentEnablementChanged for Label<'a> {

}

impl<'a> ValueListener for Label<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            if (lastTextValue != textValue.toString())
            setText (textValue.toString(), sendNotification);
        */
    }
}

impl<'a> Drop for Label<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            textValue.removeListener (this);

        if (ownerComponent != nullptr)
            ownerComponent->removeComponentListener (this);

        editor.reset();
        */
    }
}

impl<'a> Label<'a> {

    /**
      | Returns the text content as a Value object.
      | 
      | You can call Value::referTo() on this
      | object to make the label read and control
      | a Value object that you supply.
      |
      */
    pub fn get_text_value(&mut self) -> &mut Value {
        
        todo!();
        /*
            return textValue;
        */
    }

    /**
      | Returns the type of justification,
      | as set in setJustificationType().
      |
      */
    pub fn get_justification_type(&self) -> Justification {
        
        todo!();
        /*
            return justification;
        */
    }

    /**
      | Returns the size of the border to be left
      | around the text.
      |
      */
    pub fn get_border_size(&self) -> BorderSize<i32> {
        
        todo!();
        /*
            return border;
        */
    }

    /**
      | If the label is attached to the left of
      | another component, this returns true.
      | 
      | Returns false if the label is above the
      | other component. This is only relevant
      | if attachToComponent() has been called.
      |
      */
    pub fn is_attached_on_left(&self) -> bool {
        
        todo!();
        /*
            return leftOfOwnerComp;
        */
    }

    /**
      | Specifies the amount that the font can
      | be squashed horizontally.
      |
      */
    pub fn get_minimum_horizontal_scale(&self) -> f32 {
        
        todo!();
        /*
            return minimumHorizontalScale;
        */
    }

    /**
      | Set a keyboard type for use when the text
      | editor is shown.
      |
      */
    pub fn set_keyboard_type(&mut self, ty: VirtualKeyboardType)  {
        
        todo!();
        /*
            keyboardType = type;
        */
    }

    /**
      | Returns true if this option was set using
      | setEditable().
      |
      */
    pub fn is_editable_on_single_click(&self) -> bool {
        
        todo!();
        /*
            return editSingleClick;
        */
    }

    /**
      | Returns true if this option was set using
      | setEditable().
      |
      */
    pub fn is_editable_on_double_click(&self) -> bool {
        
        todo!();
        /*
            return editDoubleClick;
        */
    }

    /**
      | Returns true if this option has been
      | set in a call to setEditable().
      |
      */
    pub fn does_loss_of_focus_discard_changes(&self) -> bool {
        
        todo!();
        /*
            return lossOfFocusDiscardsChanges;
        */
    }

    /**
      | Returns true if the user can edit this
      | label's text.
      |
      */
    pub fn is_editable(&self) -> bool {
        
        todo!();
        /*
            return editSingleClick || editDoubleClick;
        */
    }

    /**
      | Creates a Label.
      | 
      | -----------
      | @param componentName
      | 
      | the name to give the component
      | ----------
      | @param labelText
      | 
      | the text to show in the label
      |
      */
    pub fn new(
        name:       &String,
        label_text: &String) -> Self {
    
        todo!();
        /*
        : component(name),
        : text_value(labelText),
        : last_text_value(labelText),

            setColour (TextEditor::textColourId, Colours::black);
        setColour (TextEditor::backgroundColourId, Colours::transparentBlack);
        setColour (TextEditor::outlineColourId, Colours::transparentBlack);

        textValue.addListener (this);
        */
    }
    
    /**
      | Changes the label text.
      | 
      | The NotificationType parameter indicates
      | whether to send a change message to any
      | Label::LabelListener objects if the new
      | text is different.
      |
      */
    pub fn set_text(&mut self, 
        new_text:     &String,
        notification: NotificationType)  {
        
        todo!();
        /*
            hideEditor (true);

        if (lastTextValue != newText)
        {
            lastTextValue = newText;
            textValue = newText;
            repaint();

            textWasChanged();

            if (ownerComponent != nullptr)
                componentMovedOrResized (*ownerComponent, true, true);

            if (notification != dontSendNotification)
                callChangeListeners();
        }
        */
    }
    
    /**
      | Returns the label's current text.
      | 
      | -----------
      | @param returnActiveEditorContents
      | 
      | if this is true and the label is currently
      | being edited, then this method will
      | return the text as it's being shown in
      | the editor. If false, then the value
      | returned here won't be updated until
      | the user has finished typing and pressed
      | the return key.
      |
      */
    pub fn get_text(&self, return_active_editor_contents: Option<bool>) -> String {

        let return_active_editor_contents: bool = return_active_editor_contents.unwrap_or(false);
        
        todo!();
        /*
            return (returnActiveEditorContents && isBeingEdited())
                    ? editor->getText()
                    : textValue.toString();
        */
    }
    
    /**
      | Changes the font to use to draw the text.
      | @see getFont
      |
      */
    pub fn set_font(&mut self, new_font: &Font)  {
        
        todo!();
        /*
            if (font != newFont)
        {
            font = newFont;
            repaint();
        }
        */
    }
    
    /**
      | Returns the font currently being used.
      | 
      | This may be the one set by setFont(),
      | unless it has been overridden by the
      | current LookAndFeel @see setFont
      |
      */
    pub fn get_font(&self) -> Font {
        
        todo!();
        /*
            return font;
        */
    }
    
    /**
      | Makes the label turn into a TextEditor
      | when clicked.
      | 
      | By default this is turned off.
      | 
      | If turned on, then single- or double-clicking
      | will turn the label into an editor. If
      | the user then changes the text, then
      | the ChangeBroadcaster base class will
      | be used to send change messages to any
      | listeners that have registered.
      | 
      | If the user changes the text, the textWasEdited()
      | method will be called afterwards, and
      | subclasses can override this if they
      | need to do anything special.
      | 
      | -----------
      | @param editOnSingleClick
      | 
      | if true, just clicking once on the label
      | will start editing the text
      | ----------
      | @param editOnDoubleClick
      | 
      | if true, a double-click is needed to
      | start editing
      | ----------
      | @param lossOfFocusDiscardsChanges
      | 
      | if true, clicking somewhere else while
      | the text is being edited will discard
      | any changes; if false, then this will
      | commit the changes. @see showEditor,
      | setEditorColours, TextEditor
      |
      */
    pub fn set_editable(
        &mut self, 
        edit_on_single_click:   bool,
        edit_on_double_click:   Option<bool>,
        loss_of_focus_discards: Option<bool>

    ) {
        let edit_on_double_click:   bool = edit_on_double_click.unwrap_or(false);
        let loss_of_focus_discards: bool = loss_of_focus_discards.unwrap_or(false);
        
        todo!();
        /*
            editSingleClick = editOnSingleClick;
        editDoubleClick = editOnDoubleClick;
        lossOfFocusDiscardsChanges = lossOfFocusDiscards;

        const auto isKeybordFocusable = (editOnSingleClick || editOnDoubleClick);

        setWantsKeyboardFocus (isKeybordFocusable);
        setFocusContainerType (isKeybordFocusable ? FocusContainerType::keyboardFocusContainer
                                                  : FocusContainerType::none);

        invalidateAccessibilityHandler();
        */
    }
    
    /**
      | Sets the style of justification to be
      | used for positioning the text. (The
      | default is Justification::centredLeft)
      |
      */
    pub fn set_justification_type(&mut self, new_justification: Justification)  {
        
        todo!();
        /*
            if (justification != newJustification)
        {
            justification = newJustification;
            repaint();
        }
        */
    }
    
    /**
      | Changes the border that is left between
      | the edge of the component and the text.
      | 
      | By default there's a small gap left at
      | the sides of the component to allow for
      | the drawing of the border, but you can
      | change this if necessary.
      |
      */
    pub fn set_border_size(&mut self, new_border: BorderSize<i32>)  {
        
        todo!();
        /*
            if (border != newBorder)
        {
            border = newBorder;
            repaint();
        }
        */
    }
    
    /**
      | If this label has been attached to another
      | component using attachToComponent,
      | this returns the other component.
      | 
      | Returns nullptr if the label is not attached.
      |
      */
    pub fn get_attached_component(&self) -> *mut Component {
        
        todo!();
        /*
            return ownerComponent.get();
        */
    }
    
    /**
      | Makes this label "stick to" another
      | component.
      | 
      | This will cause the label to follow another
      | component around, staying either to
      | its left or above it.
      | 
      | -----------
      | @param owner
      | 
      | the component to follow
      | ----------
      | @param onLeft
      | 
      | if true, the label will stay on the left
      | of its component; if false, it will stay
      | above it.
      |
      */
    pub fn attach_to_component(&mut self, 
        owner:   *mut Component,
        on_left: bool)  {
        
        todo!();
        /*
            jassert (owner != this); // Not a great idea to try to attach it to itself!

        if (ownerComponent != nullptr)
            ownerComponent->removeComponentListener (this);

        ownerComponent = owner;
        leftOfOwnerComp = onLeft;

        if (ownerComponent != nullptr)
        {
            setVisible (ownerComponent->isVisible());
            ownerComponent->addComponentListener (this);
            componentParentHierarchyChanged (*ownerComponent);
            componentMovedOrResized (*ownerComponent, true, true);
        }
        */
    }
    
    pub fn component_parent_hierarchy_changed(&mut self, component: &mut Component)  {
        
        todo!();
        /*
            if (auto* parent = component.getParentComponent())
            parent->addChildComponent (this);
        */
    }
    
    pub fn component_visibility_changed(&mut self, component: &mut Component)  {
        
        todo!();
        /*
            setVisible (component.isVisible());
        */
    }
    
    pub fn text_was_edited(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn text_was_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn editor_shown(&mut self, text_editor: *mut TextEditor)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);
        listeners.callChecked (checker, [this, textEditor] (Label::LabelListener& l) { l.editorShown (this, *textEditor); });

        if (checker.shouldBailOut())
            return;

        if (onEditorShow != nullptr)
            onEditorShow();
        */
    }
    
    pub fn editor_about_to_be_hidden(&mut self, text_editor: *mut TextEditor)  {
        
        todo!();
        /*
            if (auto* peer = getPeer())
            peer->dismissPendingTextInput();

        Component::BailOutChecker checker (this);
        listeners.callChecked (checker, [this, textEditor] (Label::LabelListener& l) { l.editorHidden (this, *textEditor); });

        if (checker.shouldBailOut())
            return;

        if (onEditorHide != nullptr)
            onEditorHide();
        */
    }
    
    /**
      | Makes the editor appear as if the label
      | had been clicked by the user. @see textWasEdited,
      | setEditable
      |
      */
    pub fn show_editor(&mut self)  {
        
        todo!();
        /*
            if (editor == nullptr)
        {
            editor.reset (createEditorComponent());
            editor->setSize (10, 10);
            addAndMakeVisible (editor.get());
            editor->setText (getText(), false);
            editor->setKeyboardType (keyboardType);
            editor->addListener (this);
            editor->grabKeyboardFocus();

            if (editor == nullptr) // may be deleted by a callback
                return;

            editor->setHighlightedRegion (Range<int> (0, textValue.toString().length()));

            resized();
            repaint();

            editorShown (editor.get());

            enterModalState (false);
            editor->grabKeyboardFocus();
        }
        */
    }
    
    pub fn update_from_text_editor_contents(&mut self, ed: &mut TextEditor) -> bool {
        
        todo!();
        /*
            auto newText = ed.getText();

        if (textValue.toString() != newText)
        {
            lastTextValue = newText;
            textValue = newText;
            repaint();

            textWasChanged();

            if (ownerComponent != nullptr)
                componentMovedOrResized (*ownerComponent, true, true);

            return true;
        }

        return false;
        */
    }
    
    /**
      | Hides the editor if it was being shown.
      | 
      | -----------
      | @param discardCurrentEditorContents
      | 
      | if true, the label's text will be reset
      | to whatever it was before the editor
      | was shown; if false, the current contents
      | of the editor will be used to set the label's
      | text before it is hidden.
      |
      */
    pub fn hide_editor(&mut self, discard_current_editor_contents: bool)  {
        
        todo!();
        /*
            if (editor != nullptr)
        {
            WeakReference<Component> deletionChecker (this);
            std::unique_ptr<TextEditor> outgoingEditor;
            std::swap (outgoingEditor, editor);

            editorAboutToBeHidden (outgoingEditor.get());

            const bool changed = (! discardCurrentEditorContents)
                                   && updateFromTextEditorContents (*outgoingEditor);
            outgoingEditor.reset();

            if (deletionChecker != nullptr)
                repaint();

            if (changed)
                textWasEdited();

            if (deletionChecker != nullptr)
                exitModalState (0);

            if (changed && deletionChecker != nullptr)
                callChangeListeners();
        }
        */
    }
    
    pub fn input_attempt_when_modal(&mut self)  {
        
        todo!();
        /*
            if (editor != nullptr)
        {
            if (lossOfFocusDiscardsChanges)
                textEditorEscapeKeyPressed (*editor);
            else
                textEditorReturnKeyPressed (*editor);
        }
        */
    }
    
    /**
      | Returns true if the editor is currently
      | focused and active.
      |
      */
    pub fn is_being_edited(&self) -> bool {
        
        todo!();
        /*
            return editor != nullptr;
        */
    }
    
    pub fn create_editor_component(&mut self) -> *mut TextEditor {
        
        todo!();
        /*
            auto* ed = new TextEditor (getName());
        ed->applyFontToAllText (getLookAndFeel().getLabelFont (*this));
        copyAllExplicitColoursTo (*ed);

        copyColourIfSpecified (*this, *ed, textWhenEditingColourId, TextEditor::textColourId);
        copyColourIfSpecified (*this, *ed, backgroundWhenEditingColourId, TextEditor::backgroundColourId);
        copyColourIfSpecified (*this, *ed, outlineWhenEditingColourId, TextEditor::focusedOutlineColourId);

        return ed;
        */
    }
    
    /**
      | Returns the currently-visible text
      | editor, or nullptr if none is open.
      |
      */
    pub fn get_current_text_editor(&self) -> *mut TextEditor {
        
        todo!();
        /*
            return editor.get();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawLabel (g, *this);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (editSingleClick
             && isEnabled()
             && contains (e.getPosition())
             && ! (e.mouseWasDraggedSinceMouseDown() || e.mods.isPopupMenu()))
        {
            showEditor();
        }
        */
    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (editDoubleClick
             && isEnabled()
             && ! e.mods.isPopupMenu())
        {
            showEditor();
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (editor != nullptr)
            editor->setBounds (getLocalBounds());
        */
    }
    
    pub fn focus_gained(&mut self, cause: FocusChangeType)  {
        
        todo!();
        /*
            if (editSingleClick
             && isEnabled()
             && cause == focusChangedByTabKey)
        {
            showEditor();
        }
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
            repaint();
        */
    }
    
    /**
      | Specifies the minimum amount that the
      | font can be squashed horizontally before
      | it starts using ellipsis. Use a value
      | of 0 for a default value.
      | 
      | @see Graphics::drawFittedText
      |
      */
    pub fn set_minimum_horizontal_scale(&mut self, new_scale: f32)  {
        
        todo!();
        /*
            if (minimumHorizontalScale != newScale)
        {
            minimumHorizontalScale = newScale;
            repaint();
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<LabelAccessibilityHandler> (*this);
        */
    }
    
    pub fn create_keyboard_focus_traverser(&mut self) -> Box<dyn ComponentTraverser> {
        
        todo!();
        /*
            return std::make_unique<LabelKeyboardFocusTraverser> (*this);
        */
    }
    
    /**
      | Registers a listener that will be called
      | when the label's text changes.
      |
      */
    pub fn add_listener(&mut self, l: Option<&mut dyn LabelListener>)  {
        
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
    pub fn remove_listener(&mut self, l: Option<&mut dyn LabelListener>)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    pub fn call_change_listeners(&mut self)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);
        listeners.callChecked (checker, [this] (LabelListener& l) { l.labelTextChanged (this); });

        if (checker.shouldBailOut())
            return;

        if (onTextChange != nullptr)
            onTextChange();
        */
    }
    
    pub fn text_editor_text_changed(&mut self, ed: &mut TextEditor)  {
        
        todo!();
        /*
            if (editor != nullptr)
        {
            jassert (&ed == editor.get());

            if (! (hasKeyboardFocus (true) || isCurrentlyBlockedByAnotherModalComponent()))
            {
                if (lossOfFocusDiscardsChanges)
                    textEditorEscapeKeyPressed (ed);
                else
                    textEditorReturnKeyPressed (ed);
            }
        }
        */
    }
    
    pub fn text_editor_return_key_pressed(&mut self, ed: &mut TextEditor)  {
        
        todo!();
        /*
            if (editor != nullptr)
        {
            jassert (&ed == editor.get());

            WeakReference<Component> deletionChecker (this);
            bool changed = updateFromTextEditorContents (ed);
            hideEditor (true);

            if (changed && deletionChecker != nullptr)
            {
                textWasEdited();

                if (deletionChecker != nullptr)
                    callChangeListeners();
            }
        }
        */
    }
    
    pub fn text_editor_escape_key_pressed(&mut self, ed: &mut TextEditor)  {
        
        todo!();
        /*
            if (editor != nullptr)
        {
            jassertquiet (&ed == editor.get());

            editor->setText (textValue.toString(), false);
            hideEditor (true);
        }
        */
    }
    
    pub fn text_editor_focus_lost(&mut self, ed: &mut TextEditor)  {
        
        todo!();
        /*
            textEditorTextChanged (ed);
        */
    }
}
