crate::ix!();

pub enum ResizableBorderComponentZones
{
    centre  = 0,
    left    = 1,
    top     = 2,
    right   = 4,
    bottom  = 8
}

/**
  | Represents the different sections
  | of a resizable border, which allow it
  | to resized in different ways.
  |
  */
pub struct ResizableBorderComponentZone {
    zone: i32, // default = centre
}

impl ResizableBorderComponentZone {
        
    /**
      | Creates a ResizableBorderComponentZone from a combination of
      | the flags in zoneFlags.
      |
      */
    pub fn new(zone_flags: i32) -> Self {
    
        todo!();
        /*
        
        */
    }
        
    /**
      | Given a point within a rectangle with
      | a resizable border, this returns the
      | zone that the point lies within.
      |
      */
    pub fn from_position_on_border(
        total_size: Rectangle<i32>,
        border:     BorderSize<i32>,
        position:   Point<i32>) -> ResizableBorderComponentZone {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns an appropriate mouse-cursor
      | for this resize zone.
      |
      */
    pub fn get_mouse_cursor(&self) -> MouseCursor {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if dragging this zone will
      | move the enire object without resizing
      | it.
      |
      */
    pub fn is_dragging_whole_object(&self) -> bool {
        
        todo!();
        /*
            return zone == centre;
        */
    }

    /**
      | Returns true if dragging this zone will
      | move the object's left edge.
      |
      */
    pub fn is_dragging_left_edge(&self) -> bool {
        
        todo!();
        /*
            return (zone & left) != 0;
        */
    }

    /**
      | Returns true if dragging this zone will
      | move the object's right edge.
      |
      */
    pub fn is_dragging_right_edge(&self) -> bool {
        
        todo!();
        /*
            return (zone & right) != 0;
        */
    }

    /**
      | Returns true if dragging this zone will
      | move the object's top edge.
      |
      */
    pub fn is_dragging_top_edge(&self) -> bool {
        
        todo!();
        /*
            return (zone & top) != 0;
        */
    }

    /**
      | Returns true if dragging this zone will
      | move the object's bottom edge.
      |
      */
    pub fn is_dragging_bottom_edge(&self) -> bool {
        
        todo!();
        /*
            return (zone & bottom) != 0;
        */
    }

    /**
      | Resizes this rectangle by the given
      | amount, moving just the edges that this
      | zone applies to.
      |
      */
    pub fn resize_rectangle_by<ValueType: Copy>(&self, 
        original: Rectangle<ValueType>,
        distance: &Point<ValueType>) -> Rectangle<ValueType> {
    
        todo!();
        /*
            if (isDraggingWholeObject())
                    return original + distance;

                if (isDraggingLeftEdge())   original.setLeft (jmin (original.getRight(), original.getX() + distance.x));
                if (isDraggingRightEdge())  original.setWidth (jmax (ValueType(), original.getWidth() + distance.x));
                if (isDraggingTopEdge())    original.setTop (jmin (original.getBottom(), original.getY() + distance.y));
                if (isDraggingBottomEdge()) original.setHeight (jmax (ValueType(), original.getHeight() + distance.y));

                return original;
        */
    }

    /**
      | Returns the raw flags for this zone.
      |
      */
    pub fn get_zone_flags(&self) -> i32 {
        
        todo!();
        /*
            return zone;
        */
    }
}
