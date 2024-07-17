crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_MultiChoicePropertyComponent.h]

/**
  | A PropertyComponent that shows its
  | value as an expandable list of ToggleButtons.
  | 
  | This type of property component contains
  | a list of options where multiple options
  | can be selected at once.
  | 
  | @see PropertyComponent, PropertyPanel
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MultiChoicePropertyComponent<'a> {

    base:               PropertyComponent<'a>,

    /**
      | You can assign a lambda to this callback
      | object to have it called when the MultiChoicePropertyComponent
      | height changes.
      |
      */
    on_height_change:   fn() -> (),

    value_with_default: WeakReference<ValueWithDefault<'a>>,
    max_height:         i32, // default = 0
    num_hidden:         i32, // default = 0
    expandable:         bool, // default = false
    expanded:           bool, // default = false
    choice_buttons:     Vec<Box<ToggleButton<'a>>>,

    /**
       { "Expand", Colours::transparentBlack, Colours::transparentBlack, Colours::transparentBlack };
      */
    expand_button:      ShapeButton<'a>,
}

impl<'a> Drop for MultiChoicePropertyComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (valueWithDefault != nullptr)
            valueWithDefault->onDefaultChange = nullptr;
        */

    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_MultiChoicePropertyComponent.cpp]
impl<'a> MultiChoicePropertyComponent<'a> {

    pub const COLLAPSED_HEIGHT:   i32 = 125;
    pub const BUTTON_HEIGHT:      i32 = 25;
    pub const EXPAND_AREA_HEIGHT: i32 = 20;

    /**
      | Returns true if the list of options is
      | expanded.
      |
      */
    pub fn is_expanded(&self) -> bool {
        
        todo!();
        /*
            return expanded;
        */
    }

    /**
      | Returns true if the list of options has
      | been truncated and can be expanded.
      |
      */
    pub fn is_expandable(&self) -> bool {
        
        todo!();
        /*
            return expandable;
        */
    }

    pub fn get_total_buttons_height(&mut self, num_buttons: i32) -> i32 {
        
        todo!();
        /*
            return numButtons * buttonHeight + 1;
        */

    }
    
    /**
      | Delegating constructor.
      |
      */
    pub fn new_delegating(
        property_name:        &String,
        choices:              &Vec<String>,
        corresponding_values: &[Var]) -> Self {
    
        todo!();
        /*
        : property_component(propertyName, jmin (getTotalButtonsHeight (choices.size()), collapsedHeight)),

            // The array of corresponding values must contain one value for each of the items in
        // the choices array!
        jassertquiet (choices.size() == correspondingValues.size());

        for (auto choice : choices)
            addAndMakeVisible (choiceButtons.add (new ToggleButton (choice)));

        if (preferredHeight >= collapsedHeight)
        {
            expandable = true;
            maxHeight = getTotalButtonsHeight (choiceButtons.size()) + expandAreaHeight;
        }

        if (isExpandable())
        {
            {
                Path expandShape;
                expandShape.addTriangle ({ 0, 0 }, { 5, 10 }, { 10, 0});
                expandButton.setShape (expandShape, true, true, false);
            }

            expandButton.onClick = [this] { setExpanded (! expanded); };
            addAndMakeVisible (expandButton);

            lookAndFeelChanged();
        }
        */

    }
    
    /**
      | Creates the component. Note that the
      | underlying var object that the Value
      | refers to must be an array.
      | 
      | -----------
      | @param valueToControl
      | 
      | the value that the ToggleButtons will
      | read and control
      | ----------
      | @param propertyName
      | 
      | the name of the property
      | ----------
      | @param choices
      | 
      | the list of possible values that will
      | be represented
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
      | ----------
      | @param maxChoices
      | 
      | the maximum number of values which can
      | be selected at once. The default of
      | 
      | -1 will not limit the number that can
      | be selected
      |
      */
    pub fn new(
        value_to_control:     &Value,
        property_name:        &String,
        choices:              &Vec<String>,
        corresponding_values: &[Var],
        max_choices:          Option<i32>

    ) -> Self {

        let max_choices: i32 = max_choices.unwrap_or(-1);
    
        todo!();
        /*
        : multi_choice_property_component(propertyName, choices, correspondingValues),

            // The value to control must be an array!
        jassert (valueToControl.getValue().isArray());

        for (int i = 0; i < choiceButtons.size(); ++i)
            choiceButtons[i]->getToggleStateValue().referTo (Value (new MultiChoiceRemapperSource (valueToControl,
                                                                                                   correspondingValues[i],
                                                                                                   maxChoices)));
        */

    }
    
    /**
      | Creates the component using a ValueWithDefault
      | object. This will select the default
      | options.
      | 
      | -----------
      | @param valueToControl
      | 
      | the ValueWithDefault object that contains
      | the Value object that the ToggleButtons
      | will read and control.
      | ----------
      | @param propertyName
      | 
      | the name of the property
      | ----------
      | @param choices
      | 
      | the list of possible values that will
      | be represented
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
      | ----------
      | @param maxChoices
      | 
      | the maximum number of values which can
      | be selected at once. The default of
      | 
      | -1 will not limit the number that can
      | be selected
      |
      */
    pub fn new_with_value_with_default(
        value_to_control:     &mut ValueWithDefault,
        property_name:        &String,
        choices:              &Vec<String>,
        corresponding_values: &[Var],
        max_choices:          Option<i32>

    ) -> Self {

        let max_choices: i32 = max_choices.unwrap_or(-1);
    
        todo!();
        /*
        : multi_choice_property_component(propertyName, choices, correspondingValues),

            valueWithDefault = &valueToControl;

        // The value to control must be an array!
        jassert (valueWithDefault->get().isArray());

        for (int i = 0; i < choiceButtons.size(); ++i)
            choiceButtons[i]->getToggleStateValue().referTo (Value (new MultiChoiceRemapperSourceWithDefault (valueWithDefault,
                                                                                                              correspondingValues[i],
                                                                                                              maxChoices,
                                                                                                              choiceButtons[i])));

        valueWithDefault->onDefaultChange = [this] { repaint(); };
        */

    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (findColour (TextEditor::backgroundColourId));
        g.fillRect (getLookAndFeel().getPropertyComponentContentPosition (*this));

        if (isExpandable() && ! isExpanded())
        {
            g.setColour (findColour (TextEditor::backgroundColourId).contrasting().withAlpha (0.4f));
            g.drawFittedText ("+ " + String (numHidden) + " more", getLookAndFeel().getPropertyComponentContentPosition (*this)
                                                                                   .removeFromBottom (expandAreaHeight).withTrimmedLeft (10),
                              Justification::centredLeft, 1);
        }

        PropertyComponent::paint (g);
        */

    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLookAndFeel().getPropertyComponentContentPosition (*this);

        if (isExpandable())
        {
            bounds.removeFromBottom (5);

            auto buttonSlice = bounds.removeFromBottom (10);
            expandButton.setSize (10, 10);
            expandButton.setCentrePosition (buttonSlice.getCentre());
        }

        numHidden = 0;

        for (auto* b : choiceButtons)
        {
            if (bounds.getHeight() >= buttonHeight)
            {
                b->setVisible (true);
                b->setBounds (bounds.removeFromTop (buttonHeight).reduced (5, 2));
            }
            else
            {
                b->setVisible (false);
                ++numHidden;
            }
        }
        */

    }
    
    /**
      | Expands or shrinks the list of options
      | if they are not all visible.
      | 
      | N.B. This will just set the preferredHeight
      | value of the PropertyComponent and
      | attempt to call PropertyPanel::resized(),
      | so if you are not displaying this object
      | in a PropertyPanel then you should use
      | the onHeightChange callback to resize
      | it when the height changes.
      | 
      | @see onHeightChange
      |
      */
    pub fn set_expanded(&mut self, should_be_expanded: bool)  {
        
        todo!();
        /*
            if (! isExpandable() || (isExpanded() == shouldBeExpanded))
            return;

        expanded = shouldBeExpanded;
        preferredHeight = expanded ? maxHeight : collapsedHeight;

        if (auto* propertyPanel = findParentComponentOfClass<PropertyPanel>())
            propertyPanel->resized();

        if (onHeightChange != nullptr)
            onHeightChange();

        expandButton.setTransform (AffineTransform::rotation (expanded ? MathConstants<float>::pi : MathConstants<float>::twoPi,
                                                              (float) expandButton.getBounds().getCentreX(),
                                                              (float) expandButton.getBounds().getCentreY()));

        resized();
        */

    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            auto iconColour = findColour (TextEditor::backgroundColourId).contrasting();
        expandButton.setColours (iconColour, iconColour.darker(), iconColour.darker());

        if (valueWithDefault != nullptr)
        {
            auto usingDefault = valueWithDefault->isUsingDefault();

            for (auto* button : choiceButtons)
                updateButtonTickColour (button, usingDefault);
        }
        */

    }
}

///--------------------
pub fn string_comparator_compare_elements(
    first:  Var,
    second: Var) -> i32 {
    
    todo!();
    /*
        if (first.toString() > second.toString())
            return 1;
        else if (first.toString() < second.toString())
            return -1;

        return 0;
    */

}

pub fn update_button_tick_colour(
        button:        *mut ToggleButton,
        using_default: bool)  {
    
    todo!();
        /*
            button->setColour (ToggleButton::tickColourId, button->getLookAndFeel().findColour (ToggleButton::tickColourId)
                                                                                  .withAlpha (usingDefault ? 0.4f : 1.0f));
        */

}
