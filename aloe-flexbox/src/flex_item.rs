crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_FlexItem.h]

/**
  | This constant can be used for sizes to
  | indicate that 'auto' mode should be
  | used.
  |
  */
pub const FLEX_ITEM_AUTO_VALUE: i32 = -2;

/**
  | This constant can be used for sizes to
  | indicate that no value has been set.
  |
  */
pub const FLEX_ITEM_NOT_ASSIGNED: i32 = -1;

/**
  | Describes the properties of an item
  | inside a FlexBox container.
  | 
  | @see FlexBox
  | 
  | @tags{GUI}
  |
  */
pub struct FlexItem<'a> {

    /**
      | The item's current bounds.
      |
      */
    current_bounds:       Rectangle<f32>,

    /**
      | If this is non-null, it represents a
      | Component whose bounds are controlled
      | by this item.
      |
      */
    associated_component: *mut Component<'a>, // default = nullptr

    /**
      | If this is non-null, it represents a
      | FlexBox whose bounds are controlled
      | by this item.
      |
      */
    associated_flex_box:  *mut FlexBox<'a>, // default = nullptr

    /**
      | Determines the order used to lay out items in their
      | flex container. Elements are laid out in ascending
      | order of thus order value. Elements with the same
      | order value are laid out in the order in which
      | they appear in the array.
      */
    order:                i32, // default = 0

    /**
      | Specifies the flex grow factor of this item. This
      | indicates the amount of space inside the flex container
      | the item should take up.
      */
    flex_grow:            f32, // default = 0.0f

    /**
      | Specifies the flex shrink factor of the item. This
      | indicates the rate at which the item shrinks if
      | there is insufficient space in the container.
      */
    flex_shrink:          f32, // default = 1.0f

    /**
      | Specifies the flex-basis of the item. This is the
      | initial main size of a flex item in the direction
      | of flow. It determines the size of the content-box
      | unless specified otherwise using box-sizing.
      */
    flex_basis:           f32, // default = 0.0f

    /**
      | This is the align-self property of the item. This
      | determines the alignment of the item along the
      | cross-axis (perpendicular to the direction of flow).
      */
    align_self:           FlexItemAlignSelf, // default = FlexItemAlignSelf::stretch

    /**
      | The item's width.
      |
      */
    width:      f32, // default = notAssigned

    /**
      | The item's minimum width
      |
      */
    min_width:  f32, // default = 0.0f

    /**
      | The item's maximum width
      |
      */
    max_width:  f32, // default = notAssigned

    /**
      | The item's height
      |
      */
    height:     f32, // default = notAssigned

    /**
      | The item's minimum height
      |
      */
    min_height: f32, // default = 0.0f

    /**
      | The item's maximum height
      |
      */
    max_height: f32, // default = notAssigned

    /**
      | The margin to leave around this item.
      |
      */
    margin:     FlexItemMargin,
}

impl<'a> Default for FlexItem<'a> {
    
    /**
      | Creates an item with default parameters,
      | and zero size.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<'a> FlexItem<'a> {

    /**
      | Creates an item with the given size.
      |
      */
    pub fn new_from_width_height(
        width:  f32,
        height: f32) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates an item with the given size and
      | target Component.
      |
      */
    pub fn new_from_width_height_and_target_component(
        width:            f32,
        height:           f32,
        target_component: &mut Component) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates an item that represents an embedded
      | FlexBox with a given size.
      |
      */
    pub fn new_from_width_height_and_flexbox(
        width:               f32,
        height:              f32,
        flex_box_to_control: &mut FlexBox) -> Self {
    
        todo!();
        /*
        
        */
    }

    pub fn new_from_wh(w: f32, h: f32) -> Self {
    
        todo!();
        /*
        : current_bounds(w, h),
        : min_width(w),
        : min_height(h),

        
        */
    }
    
    pub fn new_from_wh_and_associated_component(
        w: f32,
        h: f32,
        c: &mut Component) -> Self {
    
        todo!();
        /*
        : flex_item(w, h),

            associatedComponent = &c;
        */
    }
    
    pub fn new_from_wh_and_flexbox(
        w:  f32,
        h:  f32,
        fb: &mut FlexBox) -> Self {
    
        todo!();
        /*
        : flex_item(w, h),

            associatedFlexBox = &fb;
        */
    }
    
    /**
      | Creates an item with a given target Component.
      |
      */
    pub fn new_from_component(c: &mut Component) -> Self {
    
        todo!();
        /*
        : associated_component(&c),

        
        */
    }
    
    /**
      | Creates an item that represents an embedded
      | FlexBox. This class will not create
      | a copy of the supplied flex box. You need
      | to ensure that the life-time of flexBoxToControl
      | is longer than the FlexItem.
      |
      */
    pub fn new_from_flexbox(fb: &mut FlexBox) -> Self {
    
        todo!();
        /*
        : associated_flex_box(&fb),

        
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | flex-grow value.
      |
      */
    pub fn with_flex_with_grow(&self, new_flex_grow: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this;
        fi.flexGrow = newFlexGrow;
        return fi;
        */
    }
    
    /**
      | Returns a copy of this object with new
      | flex-grow and flex-shrink values.
      |
      */
    pub fn with_flex_with_grow_and_shrink(
        &self, 
        new_flex_grow:   f32,
        new_flex_shrink: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = withFlex (newFlexGrow);
        fi.flexShrink = newFlexShrink;
        return fi;
        */
    }
    
    /**
      | Returns a copy of this object with new
      | flex-grow, flex-shrink and flex-basis
      | values.
      |
      */
    pub fn with_flex_with_grow_and_shrink_and_basis(
        &self, 
        new_flex_grow:   f32,
        new_flex_shrink: f32,
        new_flex_basis:  f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = withFlex (newFlexGrow, newFlexShrink);
        fi.flexBasis = newFlexBasis;
        return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | width.
      |
      */
    pub fn with_width(&self, new_width: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.width = newWidth; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | minimum width.
      |
      */
    pub fn with_min_width(&self, new_min_width: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.minWidth = newMinWidth; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | maximum width.
      |
      */
    pub fn with_max_width(&self, new_max_width: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.maxWidth = newMaxWidth; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | minimum height.
      |
      */
    pub fn with_min_height(&self, new_min_height: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.minHeight = newMinHeight; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | maximum height.
      |
      */
    pub fn with_max_height(&self, new_max_height: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.maxHeight = newMaxHeight; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | height.
      |
      */
    pub fn with_height(&self, new_height: f32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.height = newHeight; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | margin.
      |
      */
    pub fn with_margin(&self, m: FlexItemMargin) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.margin = m; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | order.
      |
      */
    pub fn with_order(&self, new_order: i32) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.order = newOrder; return fi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | alignSelf value.
      |
      */
    pub fn with_align_self(&self, a: FlexItemAlignSelf) -> FlexItem {
        
        todo!();
        /*
            auto fi = *this; fi.alignSelf = a; return fi;
        */
    }
}
