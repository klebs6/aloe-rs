crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ToolbarButton.h]

/**
  | A type of button designed to go on a toolbar.
  | 
  | This simple button can have two Drawable
  | objects specified - one for normal use
  | and another one (optionally) for the
  | button's "on" state if it's a toggle
  | button.
  | 
  | @see Toolbar, ToolbarItemFactory,
  | ToolbarItemComponent, Drawable,
  | Button
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ToolbarButton<'a> {
    normal_image:     Box<Drawable<'a>>,
    toggled_on_image: Box<Drawable<'a>>,
    current_image:    *mut Drawable<'a>, // default = nullptr
}

impl<'a> ToolbarItemComponent for ToolbarButton<'a> {

    fn create_item(&mut self, item_id: i32) -> *mut dyn ToolbarItemComponent {
        todo!();
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ToolbarButton.cpp]
impl<'a> ToolbarButton<'a> {
    
    /**
      | Creates a ToolbarButton.
      | 
      | -----------
      | @param itemId
      | 
      | the ID for this toolbar item type. This
      | is passed through to the ToolbarItemComponent
      | constructor
      | ----------
      | @param labelText
      | 
      | the text to display on the button (if
      | the toolbar is using a style that shows
      | text labels). This is passed through
      | to the ToolbarItemComponent constructor
      | ----------
      | @param normalImage
      | 
      | a drawable object that the button should
      | use as its icon. The object that is passed-in
      | here will be kept by this object and will
      | be deleted when no longer needed or when
      | this button is deleted.
      | ----------
      | @param toggledOnImage
      | 
      | a drawable object that the button can
      | use as its icon if the button is in a toggled-on
      | state (see the Button::getToggleState()
      | method). If nullptr is passed-in here,
      | then the normal image will be used instead,
      | regardless of the toggle state. The
      | object that is passed-in here will be
      | owned by this object and will be deleted
      | when no longer needed or when this button
      | is deleted.
      |
      */
    pub fn new(
        iid:           i32,
        button_text:   &String,
        normal_im:     Box<Drawable>,
        toggled_on_im: Box<Drawable>) -> Self {
    
        todo!();
        /*
        : toolbar_item_component(iid, buttonText, true),
        : normal_image(std::move (normalIm)),
        : toggled_on_image(std::move (toggledOnIm)),

            jassert (normalImage != nullptr);
        */
    }
    
    pub fn get_toolbar_item_sizes(&mut self, 
        toolbar_depth:       i32,
        is_toolbar_vertical: bool,
        preferred_size:      &mut i32,
        min_size:            &mut i32,
        max_size:            &mut i32) -> bool {
        
        todo!();
        /*
            preferredSize = minSize = maxSize = toolbarDepth;
        return true;
        */
    }
    
    pub fn paint_button_area(&mut self, 
        _0:            &mut Graphics,
        width:         i32,
        height:        i32,
        is_mouse_over: bool,
        is_mouse_down: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn content_area_changed(&mut self, _0: &Rectangle<i32>)  {
        
        todo!();
        /*
            buttonStateChanged();
        */
    }
    
    pub fn set_current_image(&mut self, new_image: *mut Drawable)  {
        
        todo!();
        /*
            if (newImage != currentImage)
        {
            removeChildComponent (currentImage);
            currentImage = newImage;

            if (currentImage != nullptr)
            {
                enablementChanged();
                addAndMakeVisible (currentImage);
                updateDrawable();
            }
        }
        */
    }
    
    pub fn update_drawable(&mut self)  {
        
        todo!();
        /*
            if (currentImage != nullptr)
        {
            currentImage->setInterceptsMouseClicks (false, false);
            currentImage->setTransformToFit (getContentArea().toFloat(), RectanglePlacement::centred);
            currentImage->setAlpha (isEnabled() ? 1.0f : 0.5f);
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            ToolbarItemComponent::resized();
        updateDrawable();
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            ToolbarItemComponent::enablementChanged();
        updateDrawable();
        */
    }
    
    pub fn get_image_to_use(&self) -> *mut Drawable {
        
        todo!();
        /*
            if (getStyle() == Toolbar::textOnly)
            return nullptr;

        if (getToggleState() && toggledOnImage != nullptr)
            return toggledOnImage.get();

        return normalImage.get();
        */
    }
    
    pub fn button_state_changed(&mut self)  {
        
        todo!();
        /*
            setCurrentImage (getImageToUse());
        */
    }
}
