crate::ix!();

pub trait CreateComponentForPage {

    /**
      | Subclasses must override this to return
      | a component for each preferences page.
      | 
      | The subclass should return a pointer
      | to a new component representing the
      | named page, which the panel will then
      | display.
      | 
      | The panel will delete the component
      | later when the user goes to another page
      | or deletes the panel.
      |
      */
    fn create_component_for_page(&mut self, page_name: &String) -> *mut Component;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_PreferencesPanel.h]

pub trait PreferencesPanelInterface: CreateComponentForPage {}

/**
  | A component with a set of buttons at the
  | top for changing between pages of preferences.
  | 
  | This is just a handy way of writing a Mac-style
  | preferences panel where you have a row
  | of buttons along the top for the different
  | preference categories, each button
  | having an icon above its name. Clicking
  | these will show an appropriate prefs
  | page below it.
  | 
  | You can either put one of these inside
  | your own component, or just use the showInDialogBox()
  | method to show it in a window and run it
  | modally.
  | 
  | To use it, just add a set of named pages
  | with the addSettingsPage() method,
  | and implement the createComponentForPage()
  | method to create suitable components
  | for each of these pages.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct PreferencesPanel<'a> {
    base:              Component<'a>,
    current_page_name: String,
    current_page:      Box<Component<'a>>,
    buttons:           Vec<Box<DrawableButton<'a>>>,
    button_size:       i32,
}

impl<'a> Default for PreferencesPanel<'a> {
    
    /**
      | Creates an empty panel.
      | 
      | Use addSettingsPage() to add some pages
      | to it in your constructor.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : button_size(70),

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_PreferencesPanel.cpp]
impl<'a> PreferencesPanel<'a> {
    
    /**
      | Returns the size of the buttons shown
      | along the top.
      |
      */
    pub fn get_button_size(&self) -> i32 {
        
        todo!();
        /*
            return buttonSize;
        */
    }
    
    /**
      | Changes the size of the buttons shown
      | along the top.
      |
      */
    pub fn set_button_size(&mut self, new_size: i32)  {
        
        todo!();
        /*
            buttonSize = newSize;
        resized();
        */
    }
    
    /**
      | Creates a page using a set of drawables
      | to define the page's icon.
      | 
      | -----------
      | @note
      | 
      | the other version of this method is much
      | easier if you're using an image instead
      | of a custom drawable.
      | 
      | -----------
      | @param pageTitle
      | 
      | the name of this preferences page - you'll
      | need to make sure your createComponentForPage()
      | method creates a suitable component
      | when it is passed this name
      | ----------
      | @param normalIcon
      | 
      | the drawable to display in the page's
      | button normally
      | ----------
      | @param overIcon
      | 
      | the drawable to display in the page's
      | button when the mouse is over
      | ----------
      | @param downIcon
      | 
      | the drawable to display in the page's
      | button when the button is down @see DrawableButton
      |
      */
    pub fn add_settings_page_with_drawables(
        &mut self, 
        title:     &String,
        icon:      *const Drawable,
        over_icon: *const Drawable,
        down_icon: *const Drawable
    ) {
        
        todo!();
        /*
            auto* button = new DrawableButton (title, DrawableButton::ImageAboveTextLabel);
        buttons.add (button);

        button->setImages (icon, overIcon, downIcon);
        button->setRadioGroupId (1);
        button->onClick = [this] { clickedPage(); };
        button->setClickingTogglesState (true);
        button->setWantsKeyboardFocus (false);
        addAndMakeVisible (button);

        resized();

        if (currentPage == nullptr)
            setCurrentPage (title);
        */
    }
    
    /**
      | Creates a page using a set of drawables
      | to define the page's icon.
      | 
      | The other version of this method gives
      | you more control over the icon, but this
      | one is much easier if you're just loading
      | it from a file.
      | 
      | -----------
      | @param pageTitle
      | 
      | the name of this preferences page - you'll
      | need to make sure your createComponentForPage()
      | method creates a suitable component
      | when it is passed this name
      | ----------
      | @param imageData
      | 
      | a block of data containing an image file,
      | e.g. a jpeg, png or gif.
      | 
      | For this to look good, you'll probably
      | want to use a nice transparent png file.
      | ----------
      | @param imageDataSize
      | 
      | the size of the image data, in bytes
      |
      */
    pub fn add_settings_page(&mut self, 
        title:           &String,
        image_data:      *const c_void,
        image_data_size: i32)  {
        
        todo!();
        /*
            DrawableImage icon, iconOver, iconDown;
        icon.setImage (ImageCache::getFromMemory (imageData, imageDataSize));

        iconOver.setImage (ImageCache::getFromMemory (imageData, imageDataSize));
        iconOver.setOverlayColour (Colours::black.withAlpha (0.12f));

        iconDown.setImage (ImageCache::getFromMemory (imageData, imageDataSize));
        iconDown.setOverlayColour (Colours::black.withAlpha (0.25f));

        addSettingsPage (title, &icon, &iconOver, &iconDown);
        */
    }
    
    /**
      | Utility method to display this panel
      | in a DialogWindow.
      | 
      | Calling this will create a DialogWindow
      | containing this panel with the given
      | size and title, and will run it modally,
      | returning when the user closes the dialog
      | box.
      |
      */
    pub fn show_in_dialog_box(
        &mut self, 
        dialog_title:      &String,
        dialog_width:      i32,
        dialog_height:     i32,
        background_colour: Option<Colour>

    ) {

        let background_colour: Colour = background_colour.unwrap_or(colours::white);
        
        todo!();
        /*
            setSize (dialogWidth, dialogHeight);

        DialogWindowLaunchOptions o;
        o.content.setNonOwned (this);
        o.dialogTitle                   = dialogTitle;
        o.dialogBackgroundColour        = backgroundColour;
        o.escapeKeyTriggersCloseButton  = false;
        o.useNativeTitleBar             = false;
        o.resizable                     = false;

        o.launchAsync();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < buttons.size(); ++i)
            buttons.getUnchecked(i)->setBounds (i * buttonSize, 0, buttonSize, buttonSize);

        if (currentPage != nullptr)
            currentPage->setBounds (getLocalBounds().withTop (buttonSize + 5));
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::grey);
        g.fillRect (0, buttonSize + 2, getWidth(), 1);
        */
    }
    
    /**
      | Changes the current page being displayed.
      |
      */
    pub fn set_current_page(&mut self, page_name: &String)  {
        
        todo!();
        /*
            if (currentPageName != pageName)
        {
            currentPageName = pageName;

            currentPage.reset();
            currentPage.reset (createComponentForPage (pageName));

            if (currentPage != nullptr)
            {
                addAndMakeVisible (currentPage.get());
                currentPage->toBack();
                resized();
            }

            for (auto* b : buttons)
            {
                if (b->getName() == pageName)
                {
                    b->setToggleState (true, dontSendNotification);
                    break;
                }
            }
        }
        */
    }
    
    pub fn clicked_page(&mut self)  {
        
        todo!();
        /*
            for (auto* b : buttons)
        {
            if (b->getToggleState())
            {
                setCurrentPage (b->getName());
                break;
            }
        }
        */
    }
}
