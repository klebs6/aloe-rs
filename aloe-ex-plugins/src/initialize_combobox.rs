crate::ix!();

pub fn initialise_combo_box_with_consecutive_integers(
        owner:           &mut Component,
        combo_box:       &mut ComboBox,
        label:           &mut Label,
        first_value:     i32,
        num_values:      i32,
        value_to_select: i32)  {
    
    todo!();
    /*
        for (auto i = 0; i < numValues; ++i)
            comboBox.addItem (String (i + firstValue), i + 1);

        comboBox.setSelectedId (valueToSelect - firstValue + 1);

        label.attachToComponent (&comboBox, true);
        owner.addAndMakeVisible (comboBox);
    */
}
