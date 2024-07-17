crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_MessageBoxOptions.h]

/**
  | The type of icon to show in the dialog
  | box.
  |
  */
pub enum MessageBoxIconType
{
    /**
      | No icon will be shown on the dialog box.
      |
      */
    NoIcon,         

    /**
      | A question-mark icon, for dialog boxes
      | that need the user to answer a question.
      |
      */
    QuestionIcon,   

    /**
      | An exclamation mark to indicate that
      | the dialog is a warning about something
      | and shouldn't be ignored.
      |
      */
    WarningIcon,    

    /**
      | An icon that indicates that the dialog
      | box is just giving the user some information,
      | which doesn't require a response from
      | them.
      |
      */
    InfoIcon,        
}

/**
  | Class used to create a set of options
  | to pass to the AlertWindow and NativeMessageBox
  | methods for showing dialog boxes.
  | 
  | You can chain together a series of calls
  | to this class's methods to create a set
  | of whatever options you want to specify.
  | 
  | -----------
  | @code
  | 
  | AlertWindow::showAsync (MessageBoxOptions()
  |                           .withIconType (MessageBoxIconType::InfoIcon)
  |                           .withTitle ("A Title")
  |                           .withMessage ("A message.")
  |                           .withButton ("OK")
  |                           .withButton ("Cancel")
  |                           .withAssociatedComponent (myComp),
  |                         myCallback);
  | 
  | @tags{GUI}
  |
  */
pub struct MessageBoxOptions<'a> {
    icon_type:            MessageBoxIconType, //= MessageBoxIconType::InfoIcon;
    title:                String,
    message:              String,
    buttons:              Vec<String>,
    associated_component: WeakReference<Component<'a>>,
}

impl<'a> Default for MessageBoxOptions<'a> {

    fn default() -> Self {

        todo!();
    }
}

impl<'a> MessageBoxOptions<'a> {
    
    /**
      | Sets the type of icon that should be used
      | for the dialog box.
      |
      */
    pub fn with_icon_type(&self, ty: MessageBoxIconType) -> MessageBoxOptions {
        
        todo!();
        /*
            return with (*this, &MessageBoxOptions::iconType, type);
        */
    }

    /**
      | Sets the title of the dialog box.
      |
      */
    pub fn with_title(&self, box_title: &String) -> MessageBoxOptions {
        
        todo!();
        /*
            return with (*this, &MessageBoxOptions::title, boxTitle);
        */
    }

    /**
      | Sets the message that should be displayed
      | in the dialog box.
      |
      */
    pub fn with_message(&self, box_message: &String) -> MessageBoxOptions {
        
        todo!();
        /*
            return with (*this, &MessageBoxOptions::message, boxMessage);
        */
    }

    /**
      | If the string passed in is not empty,
      | this will add a button to the dialog box
      | with the specified text.
      | 
      | Generally up to 3 buttons are supported
      | for dialog boxes, so adding any more
      | than this may have no effect.
      |
      */
    pub fn with_button(&self, text: &String) -> MessageBoxOptions {
        
        todo!();
        /*
            auto copy = *this; copy.buttons.add (text); return copy;
        */
    }

    /**
      | The component that the dialog box should
      | be associated with.
      |
      */
    pub fn with_associated_component(&self, component: *mut Component<'a>) -> MessageBoxOptions {
        
        todo!();
        /*
            return with (*this, &MessageBoxOptions::associatedComponent, component);
        */
    }

    
    /**
      | Returns the icon type of the dialog box.
      | 
      | @see withIconType
      |
      */
    pub fn get_icon_type(&self) -> MessageBoxIconType {
        
        todo!();
        /*
            return iconType;
        */
    }

    /**
      | Returns the title of the dialog box.
      | 
      | @see withTitle
      |
      */
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            return title;
        */
    }

    /**
      | Returns the message of the dialog box.
      | 
      | @see withMessage
      |
      */
    pub fn get_message(&self) -> String {
        
        todo!();
        /*
            return message;
        */
    }

    /**
      | Returns the number of buttons that have
      | been added to the dialog box.
      | 
      | @see withButtonText
      |
      */
    pub fn get_num_buttons(&self) -> i32 {
        
        todo!();
        /*
            return buttons.size();
        */
    }

    /**
      | Returns the text that has been set for
      | one of the buttons of the dialog box.
      | 
      | @see withButtonText, getNumButtons
      |
      */
    pub fn get_button_text(&self, button_index: i32) -> String {
        
        todo!();
        /*
            return buttons[buttonIndex];
        */
    }

    /**
      | Returns the component that the dialog
      | box is associated with.
      | 
      | @see withAssociatedComponent
      |
      */
    pub fn get_associated_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return associatedComponent;
        */
    }
    
    pub fn with<Member, Item>(
        options: MessageBoxOptions,
        member:  Member,
        item:    Item) -> MessageBoxOptions {
    
        todo!();
        /*
            options.*member = std::forward<Item> (item);
            return options;
        */
    }
}
