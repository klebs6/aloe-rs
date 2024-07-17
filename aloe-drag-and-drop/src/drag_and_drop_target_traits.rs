crate::ix!();

pub trait DragAndDropContainerInterface:
ShouldDropFilesWhenDraggedExternally
+ ShouldDropTextWhenDraggedExternally 
+ DragOperationStarted 
+ DragOperationEnded 
{ }

/**
  | Components derived from this class
  | can have things dropped onto them by
  | a DragAndDropContainer.
  | 
  | To create a component that can receive
  | things drag-and-dropped by a DragAndDropContainer,
  | derive your component from this class,
  | and make sure that it is somewhere inside
  | a
  | 
  | DragAndDropContainer component.
  | 
  | -----------
  | @note
  | 
  | If all that you need to do is to respond
  | to files being drag-and-dropped from
  | the operating system onto your component,
  | you don't need any of these classes:
  | instead see the FileDragAndDropTarget
  | class.
  | 
  | @see DragAndDropContainer, FileDragAndDropTarget
  | 
  | @tags{GUI}
  |
  */
pub trait DragAndDropTarget: 
    IsInterestedInDragSource 
    + ItemDragEnter 
    + ItemDragMove 
    + ItemDragExit 
    + ItemDropped 
    + ShouldDrawDragImageWhenOver 
    { }

//---------------------------------------
pub trait IsInterestedInDragSource {

    /**
      | Callback to check whether this target
      | is interested in the type of object being
      | dragged.
      | 
      | -----------
      | @param dragSourceDetails
      | 
      | contains information about the source
      | of the drag operation.
      | 
      | -----------
      | @return
      | 
      | true if this component wants to receive
      | the other callbacks regarding this
      | type of object; if it returns false,
      | no other callbacks will be made.
      |
      */
    fn is_interested_in_drag_source(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) -> bool;
}

pub trait ItemDragEnter {

    /**
      | Callback to indicate that something
      | is being dragged over this component.
      | 
      | This gets called when the user moves
      | the mouse into this component while
      | dragging something.
      | 
      | Use this callback as a trigger to make
      | your component repaint itself to give
      | the user feedback about whether the
      | item can be dropped here or not.
      | 
      | -----------
      | @param dragSourceDetails
      | 
      | contains information about the source
      | of the drag operation. @see itemDragExit
      |
      */
    fn item_drag_enter(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) {}
}

pub trait ItemDragMove {

    /**
      | Callback to indicate that the user is
      | dragging something over this component.
      | 
      | This gets called when the user moves
      | the mouse over this component while
      | dragging something. Normally overriding
      | itemDragEnter() and itemDragExit()
      | are enough, but this lets you know what
      | happens in-between.
      | 
      | -----------
      | @param dragSourceDetails
      | 
      | contains information about the source
      | of the drag operation.
      |
      */
    fn item_drag_move(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) {}
}

pub trait ItemDragExit {

    /**
      | Callback to indicate that something
      | has been dragged off the edge of this
      | component.
      | 
      | This gets called when the user moves
      | the mouse out of this component while
      | dragging something.
      | 
      | If you've used itemDragEnter() to repaint
      | your component and give feedback, use
      | this as a signal to repaint it in its normal
      | state.
      | 
      | -----------
      | @param dragSourceDetails
      | 
      | contains information about the source
      | of the drag operation. @see itemDragEnter
      |
      */
    fn item_drag_exit(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) {}
}

pub trait ItemDropped {

    /**
      | Callback to indicate that the user has
      | dropped something onto this component.
      | 
      | When the user drops an item this get called,
      | and you can use the description to work
      | out whether your object wants to deal
      | with it or not.
      | 
      | -----------
      | @note
      | 
      | after this is called, the itemDragExit
      | method may not be called, so you should
      | clean up in here if there's anything
      | you need to do when the drag finishes.
      | 
      | -----------
      | @param dragSourceDetails
      | 
      | contains information about the source
      | of the drag operation.
      |
      */
    fn item_dropped(&mut self, drag_source_details: &DragAndDropTargetSourceDetails);
}

pub trait ShouldDrawDragImageWhenOver {

    /**
      | Overriding this allows the target to
      | tell the drag container whether to draw
      | the drag image while the cursor is over
      | it.
      | 
      | By default it returns true, but if you
      | return false, then the normal drag image
      | will not be shown when the cursor is over
      | this target.
      |
      */
    fn should_draw_drag_image_when_over(&mut self) -> bool { true }
}

//-------------------------------------------------------
pub trait ShouldDropFilesWhenDraggedExternally {

    /**
      | Override this if you want to be able to
      | perform an external drag of a set of files
      | when the user drags outside of this container
      | component.
      | 
      | This method will be called when a drag
      | operation moves outside the Aloe window,
      | and if you want it to then perform a file
      | drag-and-drop, add the filenames you
      | want to the array passed in, and return
      | true.
      | 
      | -----------
      | @param sourceDetails
      | 
      | information about the source of the
      | drag operation
      | ----------
      | @param files
      | 
      | on return, the filenames you want to
      | drag
      | ----------
      | @param canMoveFiles
      | 
      | on return, true if it's ok for the receiver
      | to move the files; false if it must make
      | a copy of them (see the performExternalDragDropOfFiles()
      | method) @see performExternalDragDropOfFiles,
      | shouldDropTextWhenDraggedExternally
      |
      */
    fn should_drop_files_when_dragged_externally(
        &mut self, 
        source_details: &DragAndDropTargetSourceDetails,
        files:          &mut Vec<String>,
        can_move_files: &mut bool
    ) -> bool;
}

pub trait ShouldDropTextWhenDraggedExternally {

    /**
      | Override this if you want to be able to
      | perform an external drag of text when
      | the user drags outside of this container
      | component.
      | 
      | This method will be called when a drag
      | operation moves outside the Aloe window,
      | and if you want it to then perform a text
      | drag-and-drop, copy the text you want
      | to be dragged into the argument provided
      | and return true.
      | 
      | -----------
      | @param sourceDetails
      | 
      | information about the source of the
      | drag operation
      | ----------
      | @param text
      | 
      | on return, the text you want to drag @see
      | shouldDropFilesWhenDraggedExternally
      |
      */
    fn should_drop_text_when_dragged_externally(
        &mut self, 
        source_details: &DragAndDropTargetSourceDetails,
        text:           &mut String
    ) -> bool;
}

pub trait DragOperationStarted {

    /**
      | Subclasses can override this to be told
      | when a drag starts.
      |
      */
    fn drag_operation_started(&mut self, _0: &DragAndDropTargetSourceDetails);
}

pub trait DragOperationEnded {

    /**
      | Subclasses can override this to be told
      | when a drag finishes.
      |
      */
    fn drag_operation_ended(&mut self, _0: &DragAndDropTargetSourceDetails);
}
