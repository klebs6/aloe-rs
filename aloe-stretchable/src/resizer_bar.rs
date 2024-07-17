crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_StretchableLayoutResizerBar.h]

/**
  | A component that acts as one of the vertical
  | or horizontal bars you see being used
  | to resize panels in a window.
  | 
  | One of these acts with a StretchableLayoutManager
  | to resize the other components.
  | 
  | @see StretchableLayoutManager
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct StretchableLayoutResizerBar<'a> {
    base:           Component<'a>,
    layout:         *mut StretchableLayoutManager,
    item_index:     i32,
    mouse_down_pos: i32,
    is_vertical:    bool,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_StretchableLayoutResizerBar.cpp]
impl<'a> StretchableLayoutResizerBar<'a> {

    /**
      | Creates a resizer bar for use on a specified
      | layout.
      | 
      | -----------
      | @param layoutToUse
      | 
      | the layout that will be affected when
      | this bar is dragged
      | ----------
      | @param itemIndexInLayout
      | 
      | the item index in the layout that corresponds
      | to this bar component. You'll need to
      | set up the item properties in a suitable
      | way for a divider bar, e.g. for an 8-pixel
      | wide bar which, you could call myLayout->setItemLayout
      | (barIndex, 8, 8, 8)
      | ----------
      | @param isBarVertical
      | 
      | true if it's an upright bar that you drag
      | left and right; false for a horizontal
      | one that you drag up and down
      |
      */
    pub fn new(
        layout:   *mut StretchableLayoutManager,
        index:    i32,
        vertical: bool) -> Self {
    
        todo!();
        /*
        : layout(layout_),
        : item_index(index),
        : is_vertical(vertical),

            setRepaintsOnMouseActivity (true);
        setMouseCursor (vertical ? MouseCursor::LeftRightResizeCursor
                                 : MouseCursor::UpDownResizeCursor);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawStretchableLayoutResizerBar (g,
                                                          getWidth(), getHeight(),
                                                          isVertical,
                                                          isMouseOver(),
                                                          isMouseButtonDown());
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            mouseDownPos = layout->getItemCurrentPosition (itemIndex);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            const int desiredPos = mouseDownPos + (isVertical ? e.getDistanceFromDragStartX()
                                                          : e.getDistanceFromDragStartY());

        if (layout->getItemCurrentPosition (itemIndex) != desiredPos)
        {
            layout->setItemPosition (itemIndex, desiredPos);
            hasBeenMoved();
        }
        */
    }
    
    pub fn has_been_moved(&mut self)  {
        
        todo!();
        /*
            if (Component* parent = getParentComponent())
            parent->resized();
        */
    }
}
