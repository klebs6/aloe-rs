crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_FileDragAndDropTarget.h]

/**
  | Components derived from this class
  | can have files dropped onto them by an
  | external application.
  | 
  | @see DragAndDropContainer
  | 
  | @tags{GUI}
  |
  */
pub trait FileDragAndDropTarget: 
IsInterestedInDragSource 
+ ItemDragEnter 
+ ItemDragMove 
+ ItemDragExit 
+ ItemDropped 
+ ShouldDrawDragImageWhenOver  {}

pub trait IsInterestedInFileDrag {

    /**
      | Callback to check whether this target
      | is interested in the set of files being
      | offered.
      | 
      | -----------
      | @note
      | 
      | this will be called repeatedly when
      | the user is dragging the mouse around
      | over your component, so don't do anything
      | time-consuming in here, like opening
      | the files to have a look inside them!
      | 
      | -----------
      | @param files
      | 
      | the set of (absolute) pathnames of the
      | files that the user is dragging
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
    fn is_interested_in_file_drag(&mut self, files: &Vec<String>) -> bool;
}

pub trait FileDragEnter {

    /**
      | Callback to indicate that some files
      | are being dragged over this component.
      | 
      | This gets called when the user moves
      | the mouse into this component while
      | dragging.
      | 
      | Use this callback as a trigger to make
      | your component repaint itself to give
      | the user feedback about whether the
      | files can be dropped here or not.
      | 
      | -----------
      | @param files
      | 
      | the set of (absolute) pathnames of the
      | files that the user is dragging
      | ----------
      | @param x
      | 
      | the mouse x position, relative to this
      | component
      | ----------
      | @param y
      | 
      | the mouse y position, relative to this
      | component
      |
      */
    fn file_drag_enter(
        &mut self, 
        files: &Vec<String>,
        x:     i32,
        y:     i32) {}
}

pub trait FileDragMove {

    /**
      | Callback to indicate that the user is
      | dragging some files over this component.
      | 
      | This gets called when the user moves
      | the mouse over this component while
      | dragging.
      | 
      | Normally overriding itemDragEnter()
      | and itemDragExit() are enough, but
      | this lets you know what happens in-between.
      | 
      | -----------
      | @param files
      | 
      | the set of (absolute) pathnames of the
      | files that the user is dragging
      | ----------
      | @param x
      | 
      | the mouse x position, relative to this
      | component
      | ----------
      | @param y
      | 
      | the mouse y position, relative to this
      | component
      |
      */
    fn file_drag_move(
        &mut self, 
        files: &Vec<String>,
        x:     i32,
        y:     i32) {}
}

pub trait FileDragExit {

    /**
      | Callback to indicate that the mouse
      | has moved away from this component.
      | 
      | This gets called when the user moves
      | the mouse out of this component while
      | dragging the files.
      | 
      | If you've used fileDragEnter() to repaint
      | your component and give feedback, use
      | this as a signal to repaint it in its normal
      | state.
      | 
      | -----------
      | @param files
      | 
      | the set of (absolute) pathnames of the
      | files that the user is dragging
      |
      */
    fn file_drag_exit(&mut self, files: &Vec<String>) {}
}

pub trait FilesDropped {

    /**
      | Callback to indicate that the user has
      | dropped the files onto this component.
      | 
      | When the user drops the files, this get
      | called, and you can use the files in whatever
      | way is appropriate.
      | 
      | -----------
      | @note
      | 
      | after this is called, the fileDragExit
      | method may not be called, so you should
      | clean up in here if there's anything
      | you need to do when the drag finishes.
      | 
      | -----------
      | @param files
      | 
      | the set of (absolute) pathnames of the
      | files that the user is dragging
      | ----------
      | @param x
      | 
      | the mouse x position, relative to this
      | component
      | ----------
      | @param y
      | 
      | the mouse y position, relative to this
      | component
      |
      */
    fn files_dropped(&mut self, 
        files: &Vec<String>,
        x:     i32,
        y:     i32);
}
