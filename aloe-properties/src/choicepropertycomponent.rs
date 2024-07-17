crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_ChoicePropertyComponent.h]

/**
  | A PropertyComponent that shows its
  | value as a combo box.
  | 
  | This type of property component contains
  | a list of options and has a combo box to
  | choose one.
  | 
  | Your subclass's constructor must add
  | some strings to the choices Vec<String>
  | and these are shown in the list.
  | 
  | The getIndex() method will be called
  | to find out which option is the currently
  | selected one. If you call refresh()
  | it will call getIndex() to check whether
  | the value has changed, and will update
  | the combo box if needed.
  | 
  | If the user selects a different item
  | from the list, setIndex() will be called
  | to let your class process this.
  | 
  | @see PropertyComponent, PropertyPanel
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ChoicePropertyComponent<'a> {

    base: PropertyComponent<'a>,

    /**
      | The list of options that will be shown
      | in the combo box.
      | 
      | Your subclass must populate this array
      | in its constructor. If any empty strings
      | are added, these will be replaced with
      | horizontal separators (see
      | 
      | ComboBox::addSeparator() for more
      | info).
      |
      */
    choices:            Vec<String>,
    combo_box:          ComboBox<'a>,
    is_custom_class:    bool, // default = false
    value_with_default: WeakReference<ValueWithDefault<'a>>,
}

pub trait ChoicePropertyComponentInterface: SetIndex + GetIndex {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_ChoicePropertyComponent.cpp]
impl<'a> Drop for ChoicePropertyComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (valueWithDefault != nullptr)
            valueWithDefault->onDefaultChange = nullptr;
        */

    }
}

impl<'a> ChoicePropertyComponent<'a> {

    /**
      | Creates the component.
      | 
      | Your subclass's constructor must add
      | a list of options to the choices member
      | variable.
      |
      */
    pub fn new_from_name(name: &String) -> Self {
    
        todo!();
        /*
        : property_component(name),
        : is_custom_class(true),
        */
    }
    
    /**
      | Delegating constructor.
      |
      */
    pub fn new_from_name_and_choice_list(
        name:                 &String,
        choice_list:          &Vec<String>,
        corresponding_values: &[Var]

    ) -> Self {
    
        todo!();
        /*
        : property_component(name),
        : choices(choiceList),

            // The array of corresponding values must contain one value for each of the items in
        // the choices array!
        jassertquiet (correspondingValues.size() == choices.size());
        */

    }
    
    /**
      | Creates the component.
      | 
      | -----------
      | @note
      | 
      | if you call this constructor then you
      | must use the Value to interact with the
      | index, and you can't override the class
      | with your own setIndex or getIndex methods.
      | 
      | If you want to use those methods, call
      | the other constructor instead.
      | 
      | -----------
      | @param valueToControl
      | 
      | the value that the combo box will read
      | and control
      | ----------
      | @param propertyName
      | 
      | the name of the property
      | ----------
      | @param choices
      | 
      | the list of possible values that the
      | drop-down list will contain
      | ----------
      | @param correspondingValues
      | 
      | a list of values corresponding to each
      | item in the 'choices' Vec<String>.
      | 
      | These are the values that will be read
      | and written to the valueToControl value.
      | This array must contain the same number
      | of items as the choices array
      |
      */
    pub fn new_with_value_to_control(
        value_to_control:     &Value,
        name:                 &String,
        choice_list:          &Vec<String>,
        corresponding_values: &[Var]

    ) -> Self {
    
        todo!();
        /*
        : choice_property_component(name, choiceList, correspondingValues),

            refreshChoices();
        initialiseComboBox (Value (new RemapperValueSource (valueToControl, correspondingValues)));
        */

    }
    
    /**
      | Creates the component using a ValueWithDefault
      | object. This will add an item to the ComboBox
      | for the default value with an ID of -1.
      | 
      | -----------
      | @param valueToControl
      | 
      | the ValueWithDefault object that contains
      | the Value object that the combo box will
      | read and control.
      | ----------
      | @param propertyName
      | 
      | the name of the property
      | ----------
      | @param choices
      | 
      | the list of possible values that the
      | drop-down list will contain
      | ----------
      | @param correspondingValues
      | 
      | a list of values corresponding to each
      | item in the 'choices' Vec<String>.
      | 
      | These are the values that will be read
      | and written to the valueToControl value.
      | This array must contain the same number
      | of items as the choices array
      |
      */
    pub fn new_with_value_to_control_mut(
        value_to_control:     &mut ValueWithDefault,
        name:                 &String,
        choice_list:          &Vec<String>,
        corresponding_values: &[Var]

    ) -> Self {
    
        todo!();
        /*
        : choice_property_component(name, choiceList, correspondingValues),

            valueWithDefault = &valueToControl;

        auto getDefaultString = [this, correspondingValues] { return choices [correspondingValues.indexOf (valueWithDefault->getDefault())]; };

        refreshChoices (getDefaultString());
        initialiseComboBox (Value (new RemapperValueSourceWithDefault (valueWithDefault, correspondingValues)));

        valueWithDefault->onDefaultChange = [this, getDefaultString]
        {
            auto selectedId = comboBox.getSelectedId();
            refreshChoices (getDefaultString());
            comboBox.setSelectedId (selectedId);
        };
        */

    }
    
    /**
      | Creates the component using a ValueWithDefault
      | object, adding an item to the ComboBox
      | for the default value with an ID of -1
      | as well as adding separate "Enabled"
      | and "Disabled" options.
      | 
      | This is useful for simple on/off choices
      | that also need a default value.
      |
      */
    pub fn new_with_value_to_control_and_name(
        value_to_control: &mut ValueWithDefault,
        name:             &String

    ) -> Self {
    
        todo!();
        /*


            : PropertyComponent (name),
          choices ({ "Enabled", "Disabled" })

        valueWithDefault = &valueToControl;

        auto getDefaultString = [this] { return valueWithDefault->getDefault() ? "Enabled" : "Disabled"; };

        refreshChoices (getDefaultString());
        initialiseComboBox (Value (new RemapperValueSourceWithDefault (valueWithDefault, { true, false })));

        valueWithDefault->onDefaultChange = [this, getDefaultString]
        {
            auto selectedId = comboBox.getSelectedId();
            refreshChoices (getDefaultString());
            comboBox.setSelectedId (selectedId);
        };
        */

    }
    
    pub fn initialise_combo_box(&mut self, v: &Value)  {
        
        todo!();
        /*
            if (v != Value())
        {
            comboBox.setSelectedId (v.getValue(), dontSendNotification);
            comboBox.getSelectedIdAsValue().referTo (v);
        }

        comboBox.setEditableText (false);
        addAndMakeVisible (comboBox);
        */

    }
    
    pub fn refresh_choices(&mut self)  {
        
        todo!();
        /*
            comboBox.clear();

        for (auto choice : choices)
        {
            if (choice.isNotEmpty())
                comboBox.addItem (choice, choices.indexOf (choice) + 1);
            else
                comboBox.addSeparator();
        }
        */

    }
    
    pub fn refresh_choices_with_default_string(&mut self, default_string: &String)  {
        
        todo!();
        /*
            refreshChoices();
        comboBox.addItem ("Default" + (defaultString.isNotEmpty() ? " (" + defaultString + ")" : ""), -1);
        */

    }
    
    pub fn set_index(&mut self, new_index: i32)  {
        
        todo!();
        /*
            jassertfalse; // you need to override this method in your subclass!
        */

    }
    
    pub fn get_index(&self) -> i32 {
        
        todo!();
        /*
            jassertfalse; // you need to override this method in your subclass!
        return -1;
        */

    }
    
    /**
      | Returns the list of options.
      |
      */
    pub fn get_choices(&self) -> &Vec<String> {
        
        todo!();
        /*
            return choices;
        */

    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            if (isCustomClass)
        {
            if (! comboBox.isVisible())
            {
                refreshChoices();
                initialiseComboBox ({});
                comboBox.onChange = [this] { changeIndex(); };
            }

            comboBox.setSelectedId (getIndex() + 1, dontSendNotification);
        }
        */

    }
    
    pub fn change_index(&mut self)  {
        
        todo!();
        /*
            if (isCustomClass)
        {
            auto newIndex = comboBox.getSelectedId() - 1;

            if (newIndex != getIndex())
                setIndex (newIndex);
        }
        */

    }
}
