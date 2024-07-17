crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_GridItem.h]

pub enum GridItemKeyword { 
    autoValue 
}

/**
  | Defines an item in a Grid @see Grid
  | 
  | @tags{GUI}
  |
  */
pub struct GridItem<'a> {

    /**
      | If this is non-null, it represents a
      | Component whose bounds are controlled
      | by this item.
      |
      */
    associated_component: *mut Component<'a>, // default = nullptr

    /**
      | Determines the order used to lay out
      | items in their grid container.
      |
      */
    order:                i32, // default = 0

    /**
      | This is the justify-self property of the item.
      | This determines the alignment of the item along
      | the row.
      */
    justify_self:         GridItemJustifySelf, // default = GridItemJustifySelf::autoValue

    /**
      | This is the align-self property of the item. This
      | determines the alignment of the item along the
      | column.
      */
    align_self:           GridItemAlignSelf, // default = GridItemAlignSelf::autoValue

    /**
      | These are the start and end properties
      | of the column.
      |
      */
    column:               GridItemStartAndEndProperty,

    /**
      | These are the start and end properties
      | of the row.
      |
      */
    row:                  GridItemStartAndEndProperty,

    area:                 String,

    /**
      | TODO: move all of this into a common class
      | that is shared with the FlexItem
      |
      */
    width:                f32, // default = notAssigned

    min_width:            f32, // default = 0.0f
    max_width:            f32, // default = notAssigned
    height:               f32, // default = notAssigned
    min_height:           f32, // default = 0.0f
    max_height:           f32, // default = notAssigned

    /**
      | The margin to leave around this item.
      |
      */
    margin:               GridItemMargin,

    /**
      | The item's current bounds.
      |
      */
    current_bounds:       Rectangle<f32>,
}

/**
  | TODO: useDefaultValue should be named
  | useAuto
  |
  */
pub const useDefaultValue: isize = -2; 
pub const notAssigned:     isize = -1;

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_GridItem.cpp]
impl<'a> GridItem<'a> {

    /**
      | Creates an item with default parameters.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }

    /**
      | Creates an item with a given Component
      | to use.
      |
      */
    pub fn new_with_ref(component_to_use: &mut Component<'a>) -> Self {
    
        todo!();
        /*
        : associated_component(&componentToUse),

        
        */
    }
    
    /**
      | Creates an item with a given Component
      | to use.
      |
      */
    pub fn new_with_ptr(component_to_use: *mut Component<'a>) -> Self {
    
        todo!();
        /*
        : associated_component(componentToUse),

        
        */
    }
    
    /**
      | Short-hand
      |
      */
    pub fn set_area(&mut self, 
        row_start:    GridItemProperty,
        column_start: GridItemProperty,
        row_end:      GridItemProperty,
        column_end:   GridItemProperty)  {
        
        todo!();
        /*
            column.start = columnStart;
        column.end = columnEnd;
        row.start = rowStart;
        row.end = rowEnd;
        */
    }
    
    /**
      | Short-hand, span of 1 by default
      |
      */
    pub fn set_area_span_of_one(
        &mut self, 
        row_start:    GridItemProperty,
        column_start: GridItemProperty)  {
        
        todo!();
        /*
            column.start = columnStart;
        row.start = rowStart;
        */
    }
    
    /**
      | Short-hand
      |
      */
    pub fn set_area_shorthand(&mut self, area_name: &String)  {
        
        todo!();
        /*
            area = areaName;
        */
    }
    
    /**
      | Short-hand
      |
      */
    pub fn with_area(&self, 
        row_start:    GridItemProperty,
        column_start: GridItemProperty,
        row_end:      GridItemProperty,
        column_end:   GridItemProperty) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.setArea (rowStart, columnStart, rowEnd, columnEnd);
        return gi;
        */
    }
    
    /**
      | Short-hand, span of 1 by default
      |
      */
    pub fn with_area_with_span_of_one(&self, 
        row_start:    GridItemProperty,
        column_start: GridItemProperty) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.setArea (rowStart, columnStart);
        return gi;
        */
    }
    
    /**
      | Short-hand
      |
      */
    pub fn with_area_shorthand(&self, area_name: &String) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.setArea (areaName);
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | row property.
      |
      */
    pub fn with_row(&self, new_row: GridItemStartAndEndProperty) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.row = newRow;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | column property.
      |
      */
    pub fn with_column(&self, new_column: GridItemStartAndEndProperty) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.column = newColumn;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | alignSelf property.
      |
      */
    pub fn with_align_self(&self, new_align_self: GridItemAlignSelf) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.alignSelf = newAlignSelf;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | justifySelf property.
      |
      */
    pub fn with_justify_self(&self, new_justify_self: GridItemJustifySelf) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.justifySelf = newJustifySelf;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | width.
      |
      */
    pub fn with_width(&self, new_width: f32) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.width = newWidth;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | height.
      |
      */
    pub fn with_height(&self, new_height: f32) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.height = newHeight;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | size.
      |
      */
    pub fn with_size(&self, 
        new_width:  f32,
        new_height: f32) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.width = newWidth;
        gi.height = newHeight;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | margin.
      |
      */
    pub fn with_margin(&self, new_height: GridItemMargin) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.margin = newHeight;
        return gi;
        */
    }
    
    /**
      | Returns a copy of this object with a new
      | order.
      |
      */
    pub fn with_order(&self, new_order: i32) -> GridItem {
        
        todo!();
        /*
            auto gi = *this;
        gi.order = newOrder;
        return gi;
        */
    }
}
