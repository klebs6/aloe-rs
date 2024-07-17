crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_DragAndDropTarget.h]

/**
  | Contains details about the source of
  | a drag-and-drop operation.
  |
  */
pub struct DragAndDropTargetSourceDetails<'a> {

    /**
      | A descriptor for the drag - this is set
      | 
      | DragAndDropContainer::startDragging().
      |
      */
    description:      Var,

    /**
      | The component from the drag operation
      | was started.
      |
      */
    source_component: WeakReference<Component<'a>>,

    /**
      | The local position of the mouse, relative to the
      | target component. Note that for calls such as isInterestedInDragSource(),
      | this may be a null position.
      */
    local_position:   Point<i32>,
}

impl<'a> DragAndDropTargetSourceDetails<'a> {
    
    /**
      | Creates a DragAndDropTargetSourceDetails object from
      | its various settings.
      |
      */
    pub fn new(
        desc: &Var,
        comp: *mut Component<'a>,
        pos:  Point<i32>) -> Self {
    
        todo!();
        /*
        : description(desc),
        : source_component(comp),
        : local_position(pos),

        
        */
    }
}
