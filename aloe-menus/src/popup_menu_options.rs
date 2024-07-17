crate::ix!();

pub enum PopupDirection
{
    upwards,
    downwards
}

/**
  | Class used to create a set of options
  | to pass to the show() method.
  | 
  | You can chain together a series of calls
  | to this class's methods to create a set
  | of whatever options you want to specify.
  | 
  | -----------
  | @code
  | 
  | PopupMenu menu;
  | ...
  | menu.showMenu (typename PopupMenu::PopupMenuOptions().withMinimumWidth (100)
  |                                    .withMaximumNumColumns (3)
  |                                    .withTargetComponent (myComp));
  |
  */
pub struct PopupMenuOptions<'a> {
    target_area:                     Rectangle<i32>,
    target_component:                *mut Component<'a>, // default = nullptr
    parent_component:                *mut Component<'a>, // default = nullptr
    component_to_watch_for_deletion: WeakReference<Component<'a>>,
    visible_itemid:                  i32, // default = 0
    min_width:                       i32, // default = 0
    min_columns:                     i32, // default = 1
    max_columns:                     i32, // default = 0
    standard_height:                 i32, // default = 0
    initially_selected_item_id:      i32, // default = 0
    is_watching_for_deletion:        bool, // default = false
    preferred_popup_direction:       PopupDirection, // default = PopupDirection::downwards
}

pub fn with<Member, PopupMenuItem>(
    options: PopupMenuOptions,
    member:  Member,
    item:    PopupMenuItem
) -> PopupMenuOptions {

    todo!();
    /*
        options.*member = std::forward<PopupMenuItem> (item);
        return options;
    */
}

impl<'a> PopupMenuOptions<'a> {
    
    pub fn get_parent_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return parentComponent;
        */
    }
    
    pub fn get_target_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return targetComponent;
        */
    }
    
    pub fn has_watched_component_been_deleted(&self) -> bool {
        
        todo!();
        /*
            return isWatchingForDeletion && componentToWatchForDeletion == nullptr;
        */
    }
    
    pub fn get_target_screen_area(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return targetArea;
        */
    }
    
    pub fn get_minimum_width(&self) -> i32 {
        
        todo!();
        /*
            return minWidth;
        */
    }
    
    pub fn get_maximum_num_columns(&self) -> i32 {
        
        todo!();
        /*
            return maxColumns;
        */
    }
    
    pub fn get_minimum_num_columns(&self) -> i32 {
        
        todo!();
        /*
            return minColumns;
        */
    }
    
    pub fn get_standard_item_height(&self) -> i32 {
        
        todo!();
        /*
            return standardHeight;
        */
    }
    
    pub fn get_item_that_must_be_visible(&self) -> i32 {
        
        todo!();
        /*
            return visibleItemID;
        */
    }
    
    pub fn get_preferred_popup_direction(&self) -> PopupDirection {
        
        todo!();
        /*
            return preferredPopupDirection;
        */
    }
    
    pub fn get_initially_selected_item_id(&self) -> i32 {
        
        todo!();
        /*
            return initiallySelectedItemId;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*


            targetArea.setPosition (Desktop::getMousePosition());
        */
    }
    
    pub fn with_target_component_ptr(&self, comp: *mut Component<'a>) -> PopupMenuOptions {
        
        todo!();
        /*
            auto o = with (*this, &PopupMenuOptions::targetComponent, comp);

        if (comp != nullptr)
            o.targetArea = comp->getScreenBounds();

        return o;
        */
    }
    
    pub fn with_target_component(&self, comp: &mut Component<'a>) -> PopupMenuOptions {
        
        todo!();
        /*
            return withTargetComponent (&comp);
        */
    }
    
    pub fn with_target_screen_area(&self, area: Rectangle<i32>) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::targetArea, area);
        */
    }
    
    pub fn with_deletion_check(&self, comp: &mut Component<'a>) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (with (*this, &PopupMenuOptions::isWatchingForDeletion, true),
                     &PopupMenuOptions::componentToWatchForDeletion,
                     &comp);
        */
    }
    
    pub fn with_minimum_width(&self, w: i32) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::minWidth, w);
        */
    }
    
    pub fn with_minimum_num_columns(&self, cols: i32) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::minColumns, cols);
        */
    }
    
    pub fn with_maximum_num_columns(&self, cols: i32) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::maxColumns, cols);
        */
    }
    
    pub fn with_standard_item_height(&self, height: i32) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::standardHeight, height);
        */
    }
    
    pub fn with_item_that_must_be_visible(&self, id_of_item_to_be_visible: i32) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::visibleItemID, idOfItemToBeVisible);
        */
    }
    
    pub fn with_parent_component(&self, parent: *mut Component<'a>) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::parentComponent, parent);
        */
    }
    
    pub fn with_preferred_popup_direction(&self, direction: PopupDirection) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::preferredPopupDirection, direction);
        */
    }
    
    pub fn with_initially_selected_item(&self, id_of_item_to_be_selected: i32) -> PopupMenuOptions {
        
        todo!();
        /*
            return with (*this, &PopupMenuOptions::initiallySelectedItemId, idOfItemToBeSelected);
        */
    }
}
