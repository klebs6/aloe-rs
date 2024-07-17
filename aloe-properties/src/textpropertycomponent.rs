crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_TextPropertyComponent.h]

/**
  | A PropertyComponent that shows its
  | value as editable text.
  | 
  | @see PropertyComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TextPropertyComponent<'a> {
    base:               PropertyComponent<'a>,
    is_multi_line:      bool,
    text_editor:        Box<TextPropertyComponentLabelComp<'a>>,
    listener_list:      ListenerList<Rc<RefCell<dyn TextPropertyComponentListener>>>,
    value_with_default: WeakReference<ValueWithDefault<'a>>,
}

pub trait TextPropertyComponentInterface: SetText + GetText { }

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_TextPropertyComponent.cpp]
impl<'a> Drop for TextPropertyComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (valueWithDefault != nullptr)
            valueWithDefault->onDefaultChange = nullptr;
        */

    }
}

impl<'a> TextPropertyComponent<'a> {
    
    /**
      | Returns true if the text editor allows
      | carriage returns.
      |
      */
    pub fn is_text_editor_multi_line(&self) -> bool {
        
        todo!();
        /*
            return isMultiLine;
        */

    }
    
    /**
      | Creates a text property component.
      | 
      | -----------
      | @param propertyName
      | 
      | The name of the property
      | ----------
      | @param maxNumChars
      | 
      | If not zero, then this specifies the
      | maximum allowable length of the string.
      | If zero, then the string will have no
      | length limit.
      | ----------
      | @param isMultiLine
      | 
      | Sets whether the text editor allows
      | carriage returns.
      | ----------
      | @param isEditable
      | 
      | Sets whether the text editor is editable.
      | The default is true.
      | 
      | @see TextEditor, setEditable
      |
      */
    pub fn new_from_name_maxchars_multiline_and_is_editable(
        name:          &String,
        max_num_chars: i32,
        multi_line:    bool,
        is_editable:   Option<bool>

    ) -> Self {

        let is_editable: bool = is_editable.unwrap_or(true);
    
        todo!();
        /*
        : property_component(name),
        : is_multi_line(multiLine),

            createEditor (maxNumChars, isEditable);
        */

    }
    
    /**
      | Creates a text property component.
      | 
      | -----------
      | @param valueToControl
      | 
      | The Value that is controlled by the TextPropertyComponent
      | ----------
      | @param propertyName
      | 
      | The name of the property
      | ----------
      | @param maxNumChars
      | 
      | If not zero, then this specifies the
      | maximum allowable length of the string.
      | If zero, then the string will have no
      | length limit.
      | ----------
      | @param isMultiLine
      | 
      | Sets whether the text editor allows
      | carriage returns.
      | ----------
      | @param isEditable
      | 
      | Sets whether the text editor is editable.
      | The default is true.
      | 
      | @see TextEditor, setEditable
      |
      */
    pub fn new_from_value_to_control_name_max_numchars_multiline_and_is_editable(
        value_to_control: &Value,
        name:             &String,
        max_num_chars:    i32,
        multi_line:       bool,
        is_editable:      Option<bool>

    ) -> Self {

        let is_editable: bool = is_editable.unwrap_or(true);
    
        todo!();
        /*
        : text_property_component(name, maxNumChars, multiLine, isEditable),

            textEditor->getTextValue().referTo (valueToControl);
        */

    }
    
    /**
      | Creates a text property component with
      | a default value.
      | 
      | -----------
      | @param valueToControl
      | 
      | The ValueWithDefault that is controlled
      | by the TextPropertyComponent.
      | ----------
      | @param propertyName
      | 
      | The name of the property
      | ----------
      | @param maxNumChars
      | 
      | If not zero, then this specifies the
      | maximum allowable length of the string.
      | If zero, then the string will have no
      | length limit.
      | ----------
      | @param isMultiLine
      | 
      | Sets whether the text editor allows
      | carriage returns.
      | ----------
      | @param isEditable
      | 
      | Sets whether the text editor is editable.
      | The default is true.
      | 
      | @see TextEditor, setEditable
      |
      */
    pub fn new_from_value_with_default(
        value_to_control: &mut ValueWithDefault,
        name:             &String,
        max_num_chars:    i32,
        multi_line:       bool,
        is_editable:      Option<bool>

    ) -> Self {

        let is_editable: bool = is_editable.unwrap_or(true);
    
        todo!();
        /*
        : text_property_component(name, maxNumChars, multiLine, isEditable),

            valueWithDefault = &valueToControl;

        textEditor->getTextValue().referTo (Value (new TextPropertyComponentRemapperValueSourceWithDefault (valueWithDefault)));
        textEditor->setTextToDisplayWhenEmpty (valueWithDefault->getDefault(), 0.5f);

        valueWithDefault->onDefaultChange = [this]
        {
            textEditor->setTextToDisplayWhenEmpty (valueWithDefault->getDefault(), 0.5f);
            repaint();
        };
        */

    }
    
    pub fn set_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            textEditor->setText (newText, sendNotificationSync);
        */

    }
    
    pub fn get_text(&self) -> String {
        
        todo!();
        /*
            return textEditor->getText();
        */

    }
    
    /**
      | Returns the text that should be shown
      | in the text editor as a Value object.
      |
      */
    pub fn get_value(&self) -> &mut Value {
        
        todo!();
        /*
            return textEditor->getTextValue();
        */

    }
    
    pub fn create_editor(&mut self, 
        max_num_chars: i32,
        is_editable:   bool)  {
        
        todo!();
        /*
            textEditor.reset (new TextPropertyComponentLabelComp (*this, maxNumChars, isMultiLine, isEditable));
        addAndMakeVisible (textEditor.get());

        if (isMultiLine)
        {
            textEditor->setJustificationType (Justification::topLeft);
            preferredHeight = 100;
        }
        */

    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            textEditor->setText (getText(), dontSendNotification);
        */

    }
    
    pub fn text_was_edited(&mut self)  {
        
        todo!();
        /*
            auto newText = textEditor->getText();

        if (getText() != newText)
            setText (newText);

        callListeners();
        */

    }
    
    /**
      | Registers a listener to receive events
      | when this button's state changes.
      | 
      | If the listener is already registered,
      | this will not register it again. @see
      | removeListener
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn TextPropertyComponentListener)  {
        
        todo!();
        /*
            listenerList.add (l);
        */

    }
    
    /**
      | Removes a previously-registered button
      | listener @see addListener
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn TextPropertyComponentListener)  {
        
        todo!();
        /*
            listenerList.remove (l);
        */

    }
    
    pub fn call_listeners(&mut self)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);
        listenerList.callChecked (checker, [this] (TextPropertyComponentListener& l) { l.textPropertyComponentChanged (this); });
        */

    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            PropertyComponent::colourChanged();
        textEditor->updateColours();
        */

    }
    
    /**
      | Sets whether the text property component
      | can have files dropped onto it by an external
      | application.
      | 
      | The default setting for this is true
      | but you may want to disable this behaviour
      | if you derive from this class and want
      | your subclass to respond to the file
      | drag.
      |
      */
    pub fn set_interested_in_file_drag(&mut self, is_interested: bool)  {
        
        todo!();
        /*
            if (textEditor != nullptr)
            textEditor->setInterestedInFileDrag (isInterested);
        */

    }
    
    /**
      | Sets whether the text editor is editable.
      | The default setting for this is true.
      |
      */
    pub fn set_editable(&mut self, is_editable: bool)  {
        
        todo!();
        /*
            if (textEditor != nullptr)
            textEditor->setEditable (isEditable, isEditable);
        */

    }
}
