crate::ix!();

pub fn get_default_password_char() -> wchar_t {
    
    todo!();
        /*
            #if ALOE_LINUX || ALOE_BSD
        return 0x2022;
       #else
        return 0x25cf;
       #endif
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/marketplace/aloe_OnlineUnlockForm.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/marketplace/aloe_OnlineUnlockForm.cpp]

/**
  | Acts as a GUI which asks the user for their
  | details, and calls the appropriate
  | methods on your OnlineUnlockStatus
  | object to attempt to register the app.
  | 
  | You should create one of these components
  | and add it to your parent window, or use
  | a DialogWindow to display it as a pop-up.
  | But if you're writing a plugin, then
  | DO NOT USE A DIALOG WINDOW! Add it as a
  | child component of your plugin's editor
  | component instead. Plugins that pop
  | up external registration windows are
  | incredibly annoying, and cause all
  | sorts of headaches for hosts. Don't
  | be the person who writes that plugin
  | that irritates everyone with a dialog
  | box every time they try to scan for new
  | plugins!
  | 
  | -----------
  | @note
  | 
  | after adding it, you should put the component
  | into a modal state, and it will automatically
  | delete itself when it has completed.
  | 
  | Although it deletes itself, it's also
  | OK to delete it manually yourself if
  | you need to get rid of it sooner.
  | 
  | @see OnlineUnlockStatus
  | 
  | @tags{ProductUnlocking}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OnlineUnlockForm<'a> {
    base:                       Component<'a>,
    message:                    Label<'a>,
    email_box:                  TextEditor<'a>,
    password_box:               TextEditor<'a>,
    register_button:            TextButton<'a>,
    cancel_button:              TextButton<'a>,
    status:                     &'a mut OnlineUnlockStatus,
    bubble:                     Box<BubbleMessageComponent<'a>>,
    show_overlay_cancel_button: bool,
    unlocking_overlay:          ComponentSafePointer<'a, Component<'a>>,
}

impl<'a> ButtonListener for OnlineUnlockForm<'a> {

    fn button_clicked(&mut self, b: *mut Button)  {
        
        todo!();
        /*
            if (b == &registerButton)
            attemptRegistration();
        else if (b == &cancelButton)
            dismiss();
        */
    }
}

pub trait Dismiss {

    fn dismiss(&mut self);
}

impl<'a> Dismiss for OnlineUnlockForm<'a> {

    /**
      | This is called when the form is dismissed
      | (either cancelled or when registration
      | succeeds).
      | 
      | By default it will delete this, but you
      | can override it to do other things.
      |
      */
    fn dismiss(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for OnlineUnlockForm<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            unlockingOverlay.deleteAndZero();
        */
    }
}

impl<'a> OnlineUnlockForm<'a> {

    /**
      | Creates an unlock form that will work
      | with the given status object.
      | 
      | The userInstructions will be displayed
      | above the email and password boxes.
      |
      */
    pub fn new(
        s:                         &mut OnlineUnlockStatus,
        user_instructions:         &String,
        has_cancel_button:         Option<bool>,
        overlay_has_cancel_button: Option<bool>

    ) -> Self {

        let has_cancel_button: bool =
                 has_cancel_button.unwrap_or(true);

        let overlay_has_cancel_button: bool =
                 overlay_has_cancel_button.unwrap_or(false);
    
        todo!();
        /*


            : message (String(), userInstructions),
          passwordBox (String(), getDefaultPasswordChar()),
          registerButton (TRANS("Register")),
          cancelButton (TRANS ("Cancel")),
          status (s),
          showOverlayCancelButton (overlayHasCancelButton)

        // Please supply a message to tell your users what to do!
        jassert (userInstructions.isNotEmpty());

        setOpaque (true);

        emailBox.setText (status.getUserEmail());
        message.setJustificationType (Justification::centred);

        addAndMakeVisible (message);
        addAndMakeVisible (emailBox);
        addAndMakeVisible (passwordBox);
        addAndMakeVisible (registerButton);

        if (hasCancelButton)
            addAndMakeVisible (cancelButton);

        emailBox.setEscapeAndReturnKeysConsumed (false);
        passwordBox.setEscapeAndReturnKeysConsumed (false);

        registerButton.addShortcut (KeyPress (KeyPress::returnKey));

        registerButton.addListener (this);
        cancelButton.addListener (this);

        lookAndFeelChanged();
        setSize (500, 250);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::lightgrey);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            /* If you're writing a plugin, then DO NOT USE A POP-UP A DIALOG WINDOW!
           Plugins that create external windows are incredibly annoying for users, and
           cause all sorts of headaches for hosts. Don't be the person who writes that
           plugin that irritates everyone with a nagging dialog box every time they scan!
        */
        jassert (ALOEApplicationBase::isStandaloneApp() || findParentComponentOfClass<DialogWindow>() == nullptr);

        const int buttonHeight = 22;

        auto r = getLocalBounds().reduced (10, 20);

        auto buttonArea = r.removeFromBottom (buttonHeight);
        registerButton.changeWidthToFitText (buttonHeight);
        cancelButton.changeWidthToFitText (buttonHeight);

        const int gap = 20;
        buttonArea = buttonArea.withSizeKeepingCentre (registerButton.getWidth()
                                                         + (cancelButton.isVisible() ? gap + cancelButton.getWidth() : 0),
                                                       buttonHeight);
        registerButton.setBounds (buttonArea.removeFromLeft (registerButton.getWidth()));
        buttonArea.removeFromLeft (gap);
        cancelButton.setBounds (buttonArea);

        r.removeFromBottom (20);

        // (force use of a default system font to make sure it has the password blob character)
        Font font (Font::getDefaultTypefaceForFont (Font (Font::getDefaultSansSerifFontName(),
                                                          Font::getDefaultStyle(),
                                                          5.0f)));

        const int boxHeight = 24;
        passwordBox.setBounds (r.removeFromBottom (boxHeight));
        passwordBox.setInputRestrictions (64);
        passwordBox.setFont (font);

        r.removeFromBottom (20);
        emailBox.setBounds (r.removeFromBottom (boxHeight));
        emailBox.setInputRestrictions (512);
        emailBox.setFont (font);

        r.removeFromBottom (20);

        message.setBounds (r);

        if (unlockingOverlay != nullptr)
            unlockingOverlay->setBounds (getLocalBounds());
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            Colour labelCol (findColour (TextEditor::backgroundColourId).contrasting (0.5f));

        emailBox.setTextToShowWhenEmpty (TRANS("Email Address"), labelCol);
        passwordBox.setTextToShowWhenEmpty (TRANS("Password"), labelCol);
        */
    }
    
    pub fn show_bubble_message(&mut self, 
        text:   &String,
        target: &mut Component)  {
        
        todo!();
        /*
            bubble.reset (new BubbleMessageComponent (500));
        addChildComponent (bubble.get());

        AttributedString attString;
        attString.append (text, Font (16.0f));

        bubble->showAt (getLocalArea (&target, target.getLocalBounds()),
                        attString, 500,  // numMillisecondsBeforeRemoving
                        true,  // removeWhenMouseClicked
                        false); // deleteSelfAfterUse
        */
    }
    
    pub fn attempt_registration(&mut self)  {
        
        todo!();
        /*
            if (unlockingOverlay == nullptr)
        {
            if (emailBox.getText().trim().length() < 3)
            {
                showBubbleMessage (TRANS ("Please enter a valid email address!"), emailBox);
                return;
            }

            if (passwordBox.getText().trim().length() < 3)
            {
                showBubbleMessage (TRANS ("Please enter a valid password!"), passwordBox);
                return;
            }

            status.setUserEmail (emailBox.getText());

            addAndMakeVisible (unlockingOverlay = new OnlineUnlockFormOverlayComp (*this, showOverlayCancelButton));
            resized();
            unlockingOverlay->enterModalState();
        }
        */
    }
    
    pub fn dismiss(&mut self)  {
        
        todo!();
        /*
            delete this;
        */
    }
}
