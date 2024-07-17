crate::ix!();

pub trait EscapeKeyPressed {

    /**
      | Called when the escape key is pressed.
      | 
      | This can be overridden to do things other
      | than the default behaviour, which is
      | to hide the window. Return true if the
      | key has been used, or false if it was ignored.
      |
      */
    fn escape_key_pressed(&mut self) -> bool;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_DialogWindow.h]

pub trait DialogWindowInterface: EscapeKeyPressed {}

/**
  | This class defines a collection of settings
  | to be used to open a DialogWindow.
  | 
  | The easiest way to open a DialogWindow
  | is to create yourself a DialogWindowLaunchOptions
  | structure, initialise its fields with
  | the appropriate details, and then call
  | its launchAsync() method to launch
  | the dialog.
  |
  */
#[no_copy]
pub struct DialogWindowLaunchOptions<'a> {

    /**
      | The title to give the window.
      |
      */
    dialog_title:                     String,

    /**
      | The background colour for the window.
      |
      */
    dialog_background_colour:         Colour, // default = Colours_lightgrey

    /**
      | The content component to show in the
      | window. This must not be null!
      | 
      | Using an OptionalScopedPointer to
      | hold this pointer lets you indicate
      | whether you'd like the dialog to automatically
      | delete the component when the dialog
      | has terminated.
      |
      */
    content:                          OptionalScopedPointer<Component<'a>>,

    /**
      | If this is not a nullptr, it indicates
      | a component that you'd like to position
      | this dialog box in front of. See the DocumentWindow::centreAroundComponent()
      | method for more info about this parameter.
      |
      */
    component_to_centre_around:       *mut Component<'a>, // default = nullptr

    /**
      | If true, then the escape key will trigger
      | the dialog's close button.
      |
      */
    escape_key_triggers_close_button: bool, // default = true

    /**
      | If true, the dialog will use a native
      | title bar. See TopLevelWindow::setUsingNativeTitleBar()
      |
      */
    use_native_title_bar:             bool, // default = true

    /**
      | If true, the window will be resizable.
      | See ResizableWindow::setResizable()
      |
      */
    resizable:                        bool, // default = true

    /**
      | Indicates whether to use a border or
      | corner resizer component. See ResizableWindow::setResizable()
      |
      */
    use_bottom_right_corner_resizer:  bool, // default = false
}

impl<'a> Default for DialogWindowLaunchOptions<'a> {

    fn default() -> Self {
        todo!();
    }
}

impl<'a> DialogWindowLaunchOptions<'a> {

    /**
      | Creates a new DialogWindow instance
      | with these settings.
      | 
      | This method simply creates the window,
      | it doesn't run it modally. In most cases
      | you'll want to use launchAsync() or
      | runModal() instead.
      |
      */
    pub fn create(&mut self) -> *mut DialogWindow {
        
        todo!();
        /*
            jassert (content != nullptr); // You need to provide some kind of content for the dialog!

        return new DefaultDialogWindow (*this);
        */
    }
    
    /**
      | Launches a new modal dialog window.
      | 
      | This will create a dialog based on the
      | settings in this structure, launch
      | it modally, and return immediately.
      | The window that is returned will be automatically
      | deleted when the modal state is terminated.
      | 
      | When the dialog's close button is clicked,
      | it'll automatically terminate its
      | modal state, but you can also do this
      | programmatically by calling exitModalState
      | (returnValue) on the DialogWindow.
      | 
      | If your content component needs to find
      | the dialog window that it is contained
      | in, a quick trick is to do this:
      | 
      | -----------
      | @code
      | 
      | if (DialogWindow* dw = contentComponent->findParentComponentOfClass<DialogWindow>())
      |     dw->exitModalState (1234);
      |
      */
    pub fn launch_async(&mut self) -> *mut DialogWindow {
        
        todo!();
        /*
            auto* d = create();
        d->enterModalState (true, nullptr, true);
        return d;
        */
    }

    /**
      | Launches and runs the dialog modally,
      | returning the status code that was used
      | to terminate the modal loop.
      | 
      | -----------
      | @note
      | 
      | running modal loops inline is a BAD technique.
      | If possible, always use launchAsync()
      | instead of this method.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn run_modal(&mut self) -> i32 {
        
        todo!();
        /*
            return launchAsync()->runModalLoop();
        */
    }
}

/**
  | A dialog-box style window.
  | 
  | This class is a convenient way of creating
  | a DocumentWindow with a close button
  | that can be triggered by pressing the
  | escape key.
  | 
  | Any of the methods available to a DocumentWindow
  | or ResizableWindow are also available
  | to this, so it can be made resizable,
  | have a menu bar, etc.
  | 
  | You can either override or use an instance
  | of the DialogWindow class directly,
  | or you can use a DialogWindow::DialogWindowLaunchOptions
  | structure to quickly set up and launch
  | a box containing a content component.
  | 
  | If you use the class directly, you'll
  | need to override the
  | 
  | DocumentWindow::closeButtonPressed()
  | method to handle the user clicking the
  | close button - for more info, see the
  | DocumentWindow help.
  | 
  | @see DocumentWindow, ResizableWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DialogWindow<'a> {
    base:                             DocumentWindow<'a>,
    desktop_scale:                    f32, // default = 1.0f
    escape_key_triggers_close_button: bool,
}

impl<'a> Drop for DialogWindow<'a> {

    /**
      | Destructor.
      | 
      | If a content component has been set with
      | setContentOwned(), it will be deleted.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_DialogWindow.cpp]
impl<'a> DialogWindow<'a> {

    pub fn get_desktop_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            return desktopScale;
        */
    }
    
    /**
      | Creates a DialogWindow.
      | 
      | -----------
      | @param name
      | 
      | the name to give the component - this
      | is also the title shown at the top of the
      | window. To change this later, use setName()
      | ----------
      | @param backgroundColour
      | 
      | the colour to use for filling the window's
      | background.
      | ----------
      | @param escapeKeyTriggersCloseButton
      | 
      | if true, then pressing the escape key
      | will cause the close button to be triggered
      | ----------
      | @param addToDesktop
      | 
      | if true, the window will be automatically
      | added to the desktop; if false, you can
      | use it as a child component
      | ----------
      | @param desktopScale
      | 
      | specifies the scale to use when drawing
      | the window. In a plugin, the host controls
      | the scale used to render the plugin editor.
      | 
      | You should query the editor scale with
      | 
      | Component::getApproximateScaleFactorForComponent()
      | and pass the result here. You can ignore
      | this parameter in a standalone app
      |
      */
    pub fn new(
        name:          &String,
        colour:        Colour,
        escape_closes: bool,
        on_desktop:    Option<bool>,
        scale:         Option<f32>) -> Self {

        let on_desktop: bool = on_desktop.unwrap_or(true);
        let scale: f32 = scale.unwrap_or(1.0);
    
        todo!();
        /*


            : DocumentWindow (name, colour, DocumentWindow::closeButton, onDesktop),
          desktopScale (scale),
          escapeKeyTriggersCloseButton (escapeCloses)
        */
    }
    
    pub fn escape_key_pressed(&mut self) -> bool {
        
        todo!();
        /*
            if (escapeKeyTriggersCloseButton)
        {
            setVisible (false);
            return true;
        }

        return false;
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (key == KeyPress::escapeKey && escapeKeyPressed())
            return true;

        return DocumentWindow::keyPressed (key);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            DocumentWindow::resized();

        if (escapeKeyTriggersCloseButton)
        {
            if (auto* close = getCloseButton())
            {
                const KeyPress esc (KeyPress::escapeKey, 0, 0);

                if (! close->isRegisteredForShortcut (esc))
                    close->addShortcut (esc);
            }
        }
        */
    }

    /**
      | Easy way of quickly showing a dialog
      | box containing a given component.
      | 
      | -----------
      | @note
      | 
      | This method has been superseded by the
      | DialogWindow::DialogWindowLaunchOptions structure,
      | which does the same job with some extra
      | flexibility. The showDialog method
      | is here for backwards compatibility,
      | but please use DialogWindow::DialogWindowLaunchOptions
      | in new code.
      | 
      | This will open and display a DialogWindow
      | containing a given component, making
      | it modal, but returning immediately
      | to allow the dialog to finish in its own
      | time. If you want to block and run a modal
      | loop until the dialog is dismissed,
      | use showModalDialog() instead.
      | 
      | To close the dialog programmatically,
      | you should call exitModalState (returnValue)
      | on the DialogWindow that is created.
      | To find a pointer to this window from
      | your contentComponent, you can do something
      | like this:
      | 
      | -----------
      | @param dialogTitle
      | 
      | the dialog box's title
      | ----------
      | @param contentComponent
      | 
      | the content component for the dialog
      | box. Make sure that this has been set
      | to the size you want it to be before calling
      | this method. The component won't be
      | deleted by this call, so you can re-use
      | it or delete it afterwards
      | ----------
      | @param componentToCentreAround
      | 
      | if this is not a nullptr, it indicates
      | a component that you'd like to show this
      | dialog box in front of. See the
      | 
      | DocumentWindow::centreAroundComponent()
      | method for more info on this parameter
      | ----------
      | @param backgroundColour
      | 
      | a colour to use for the dialog box's background
      | colour
      | ----------
      | @param escapeKeyTriggersCloseButton
      | 
      | if true, then pressing the escape key
      | will cause the close button to be triggered
      | ----------
      | @param shouldBeResizable
      | 
      | if true, the dialog window has either
      | a resizable border, or a corner resizer
      | ----------
      | @param useBottomRightCornerResizer
      | 
      | if shouldBeResizable is true, this
      | indicates whether to use a border or
      | corner resizer component. See ResizableWindow::setResizable()
      | 
      | -----------
      | @code
      | 
      | if (DialogWindow* dw = contentComponent->findParentComponentOfClass<DialogWindow>())
      |     dw->exitModalState (1234);
      |
      */
    pub fn show_dialog(&mut self, 
        dialog_title:                     &String,
        content_component:                *mut Component<'a>,
        component_to_centre_around:       *mut Component<'a>,
        background_colour:                Colour,
        escape_key_triggers_close_button: bool,
        resizable:                        Option<bool>,
        use_bottom_right_corner_resizer:  Option<bool>)  {

        let resizable:                       bool = resizable.unwrap_or(false);
        let use_bottom_right_corner_resizer: bool = use_bottom_right_corner_resizer.unwrap_or(false);
        
        todo!();
        /*
            DialogWindowLaunchOptions o;
        o.dialogTitle = dialogTitle;
        o.content.setNonOwned (contentComponent);
        o.componentToCentreAround = componentToCentreAround;
        o.dialogBackgroundColour = backgroundColour;
        o.escapeKeyTriggersCloseButton = escapeKeyTriggersCloseButton;
        o.useNativeTitleBar = false;
        o.resizable = resizable;
        o.useBottomRightCornerResizer = useBottomRightCornerResizer;

        o.launchAsync();
        */
    }

    /**
      | Easy way of quickly showing a dialog
      | box containing a given component.
      | 
      | -----------
      | @note
      | 
      | This method has been superseded by the
      | DialogWindow::DialogWindowLaunchOptions structure,
      | which does the same job with some extra
      | flexibility. The showDialog method
      | is here for backwards compatibility,
      | but please use DialogWindow::DialogWindowLaunchOptions
      | in new code.
      | 
      | This will open and display a DialogWindow
      | containing a given component, returning
      | when the user clicks its close button.
      | 
      | It returns the value that was returned
      | by the dialog box's runModalLoop()
      | call.
      | 
      | To close the dialog programmatically,
      | you should call exitModalState (returnValue)
      | on the DialogWindow that is created.
      | To find a pointer to this window from
      | your contentComponent, you can do something
      | like this:
      | 
      | -----------
      | @param dialogTitle
      | 
      | the dialog box's title
      | ----------
      | @param contentComponent
      | 
      | the content component for the dialog
      | box. Make sure that this has been set
      | to the size you want it to be before calling
      | this method. The component won't be
      | deleted by this call, so you can re-use
      | it or delete it afterwards
      | ----------
      | @param componentToCentreAround
      | 
      | if this is not a nullptr, it indicates
      | a component that you'd like to show this
      | dialog box in front of. See the
      | 
      | DocumentWindow::centreAroundComponent()
      | method for more info on this parameter
      | ----------
      | @param backgroundColour
      | 
      | a colour to use for the dialog box's background
      | colour
      | ----------
      | @param escapeKeyTriggersCloseButton
      | 
      | if true, then pressing the escape key
      | will cause the close button to be triggered
      | ----------
      | @param shouldBeResizable
      | 
      | if true, the dialog window has either
      | a resizable border, or a corner resizer
      | ----------
      | @param useBottomRightCornerResizer
      | 
      | if shouldBeResizable is true, this
      | indicates whether to use a border or
      | corner resizer component. See ResizableWindow::setResizable()
      | 
      | -----------
      | @code
      | 
      | if (DialogWindow* dw = contentComponent->findParentComponentOfClass<DialogWindow>())
      |     dw->exitModalState (1234);
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_modal_dialog(&mut self, 
        dialog_title:                     &String,
        content_component:                *mut Component<'a>,
        component_to_centre_around:       *mut Component<'a>,
        background_colour:                Colour,
        escape_key_triggers_close_button: bool,
        resizable:                        bool,
        use_bottom_right_corner_resizer:  bool) -> i32 {

        let resizable:                       bool = resizable.unwrap_or(false);
        let use_bottom_right_corner_resizer: bool = use_bottom_right_corner_resizer.unwrap_or(false);
        
        todo!();
        /*
            DialogWindowLaunchOptions o;
        o.dialogTitle = dialogTitle;
        o.content.setNonOwned (contentComponent);
        o.componentToCentreAround = componentToCentreAround;
        o.dialogBackgroundColour = backgroundColour;
        o.escapeKeyTriggersCloseButton = escapeKeyTriggersCloseButton;
        o.useNativeTitleBar = false;
        o.resizable = resizable;
        o.useBottomRightCornerResizer = useBottomRightCornerResizer;

        return o.runModal();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::dialogWindow);
        */
    }
}

///-------------------------
#[no_copy]
pub struct DefaultDialogWindow<'a> {
    base: DialogWindow<'a>,
}

impl<'a> DefaultDialogWindow<'a> {

    pub fn new(options: &mut DialogWindowLaunchOptions) -> Self {
    
        todo!();
        /*
            : DialogWindow (options.dialogTitle, options.dialogBackgroundColour,
                            options.escapeKeyTriggersCloseButton, true,
                            options.componentToCentreAround != nullptr
                                ? Component::getApproximateScaleFactorForComponent (options.componentToCentreAround)
                                : 1.0f)

            setUsingNativeTitleBar (options.useNativeTitleBar);
            setAlwaysOnTop (aloe_areThereAnyAlwaysOnTopWindows());

            if (options.content.willDeleteObject())
                setContentOwned (options.content.release(), true);
            else
                setContentNonOwned (options.content.release(), true);

            centreAroundComponent (options.componentToCentreAround, getWidth(), getHeight());
            setResizable (options.resizable, options.useBottomRightCornerResizer);
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            setVisible (false);
        */
    }
}
