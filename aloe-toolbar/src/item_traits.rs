crate::ix!();

pub trait ToolbarItemComponentInterface: 
    SetStyle 
    + PaintButtonArea
    + ContentAreaChanged
    + GetToolBarItemSizes {}

pub trait PaintButtonArea {

    /**
      | Your subclass should use this method
      | to draw its content area.
      | 
      | The graphics object that is passed-in
      | will have been clipped and had its origin
      | moved to fit the content area as specified
      | get getContentArea(). The width and
      | height parameters are the width and
      | height of the content area.
      | 
      | If the component you're writing isn't
      | a button, you can just do nothing in this
      | method.
      |
      */
    fn paint_button_area(&mut self, 
            g:             &mut Graphics,
            width:         i32,
            height:        i32,
            is_mouse_over: bool,
            is_mouse_down: bool);

}

pub trait ContentAreaChanged {

    /**
      | Callback to indicate that the content
      | area of this item has changed.
      | 
      | This might be because the component
      | was resized, or because the style changed
      | and the space needed for the text label
      | is different.
      | 
      | See getContentArea() for a description
      | of what the area is.
      |
      */
    fn content_area_changed(&mut self, new_bounds: &Rectangle<i32>);

}

pub trait SetStyle {

    /**
      | Changes the current style setting of
      | this item.
      | 
      | Styles are listed in the Toolbar::ToolbarItemStyle
      | enum, and are automatically updated
      | by the toolbar that holds this item.
      | 
      | @see setStyle, Toolbar::setStyle
      |
      */
    fn set_style(&mut self, new_style: &ToolbarItemStyle);
}

pub trait GetToolBarItemSizes {

    /**
      | This method must return the size criteria
      | for this item, based on a given toolbar
      | size and orientation.
      | 
      | The preferredSize, minSize and maxSize
      | values must all be set by your implementation
      | method. If the toolbar is horizontal,
      | these will be the width of the item; for
      | a vertical toolbar, they refer to the
      | item's height.
      | 
      | The preferredSize is the size that the
      | component would like to be, and this
      | must be between the min and max sizes.
      | For a fixed-size item, simply set all
      | three variables to the same value.
      | 
      | The toolbarThickness parameter tells
      | you the depth of the toolbar - the same
      | as calling
      | 
      | Toolbar::getThickness().
      | 
      | The isToolbarVertical parameter tells
      | you whether the bar is oriented horizontally
      | or vertically.
      |
      */
    fn get_toolbar_item_sizes(&mut self, 
            toolbar_thickness:   i32,
            is_toolbar_vertical: bool,
            preferred_size:      &mut i32,
            min_size:            &mut i32,
            max_size:            &mut i32) -> bool;

}
