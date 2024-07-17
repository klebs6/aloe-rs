crate::ix!();

pub trait SetText {

    /**
      | Called when the user edits the text.
      | 
      | Your subclass must use this callback
      | to change the value of whatever item
      | this property component represents.
      |
      */
    fn set_text(&mut self, new_text: &String);
}

pub trait GetText {

    /**
      | Returns the text that should be shown
      | in the text editor.
      |
      */
    fn get_text(&self) -> String;
}

pub trait TextWasEdited {

    fn text_was_edited(&mut self);
}

pub trait SetValue {

    /**
      | Called when the user moves the slider
      | to change its value.
      | 
      | Your subclass must use this method to
      | update whatever item this property
      | represents.
      |
      */
    fn set_value(&mut self, new_value: f64);
}

pub trait GetValue {

    /**
      | Returns the value that the slider should
      | show.
      |
      */

    fn get_value(&self) -> f64;
}

pub trait Refresh {

    /**
      | Updates the property component if the
      | item it refers to has changed.
      | 
      | A subclass must implement this method,
      | and other objects may call it to force
      | it to refresh itself.
      | 
      | The subclass should be economical in
      | the amount of work is done, so for example
      | it should check whether it really needs
      | to do a repaint rather than just doing
      | one every time this method is called,
      | as it may be called when the value being
      | displayed hasn't actually changed.
      |
      */
    fn refresh(&mut self);
}

pub trait BooleanPropertyComponentSetState {

    /**
      | Called to change the state of the boolean
      | value.
      |
      */
    fn set_state(&mut self, new_state: bool);
}

pub trait BooleanPropertyComponentGetState {

    /**
      | Must return the current value of the
      | property.
      |
      */
    fn get_state(&self) -> bool;
}

pub trait ButtonClicked {

    /**
      | Called when the user clicks the button.
      |
      */
    fn button_clicked(&mut self);
}

pub trait GetButtonText {

    /**
      | Returns the string that should be displayed
      | in the button.
      | 
      | If you need to change this string, call
      | refresh() to update the component.
      |
      */
    fn get_button_text(&self) -> String;
}

pub trait SetIndex {

    /**
      | Called when the user selects an item
      | from the combo box.
      | 
      | Your subclass must use this callback
      | to update the value that this component
      | represents. The index is the index of
      | the chosen item in the choices
      | 
      | Vec<String>.
      |
      */
    fn set_index(&mut self, new_index: i32);
}

pub trait GetIndex {

    /**
      | Returns the index of the item that should
      | currently be shown.
      | 
      | This is the index of the item in the choices
      | Vec<String> that will be shown.
      |
      */
    fn get_index(&self) -> i32;
}
