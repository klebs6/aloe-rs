crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_TextDragAndDropTarget.h]

/**
  | Components derived from this class
  | can have text dropped onto them by an
  | external application.
  | 
  | @see DragAndDropContainer
  | 
  | @tags{GUI}
  |
  */
pub trait TextDragAndDropTarget
: IsInterestedInTextDrag 
+ TextDragEnter 
+ TextDragMove 
+ TextDragExit 
+ TextDropped 
{ }

pub trait IsInterestedInTextDrag {

    /**
      | Callback to check whether this target
      | is interested in the set of text being
      | offered.
      | 
      | -----------
      | @note
      | 
      | this will be called repeatedly when
      | the user is dragging the mouse around
      | over your component, so don't do anything
      | time-consuming in here!
      | 
      | -----------
      | @param text
      | 
      | the text that the user is dragging
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
    fn is_interested_in_text_drag(&mut self, text: &str) -> bool;
}

pub trait TextDragEnter {

    /**
      | Callback to indicate that some text
      | is being dragged over this component.
      | 
      | This gets called when the user moves
      | the mouse into this component while
      | dragging.
      | 
      | Use this callback as a trigger to make
      | your component repaint itself to give
      | the user feedback about whether the
      | text can be dropped here or not.
      | 
      | -----------
      | @param text
      | 
      | the text that the user is dragging
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
    fn text_drag_enter(
        &mut self, 
        text: &String,
        x:    i32,
        y:    i32
    ) {}
}

pub trait TextDragMove {

    /**
      | Callback to indicate that the user is
      | dragging some text over this component.
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
      | @param text
      | 
      | the text that the user is dragging
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
    fn text_drag_move(
        &mut self, 
        text: &String,
        x:    i32,
        y:    i32
    ) {}
}

pub trait TextDragExit {

    /**
      | Callback to indicate that the mouse
      | has moved away from this component.
      | 
      | This gets called when the user moves
      | the mouse out of this component while
      | dragging the text.
      | 
      | If you've used textDragEnter() to repaint
      | your component and give feedback, use
      | this as a signal to repaint it in its normal
      | state.
      | 
      | -----------
      | @param text
      | 
      | the text that the user is dragging
      |
      */
    fn text_drag_exit(&mut self, text: &String) {}
}

pub trait TextDropped {

    /**
      | Callback to indicate that the user has
      | dropped the text onto this component.
      | 
      | When the user drops the text, this get
      | called, and you can use the text in whatever
      | way is appropriate.
      | 
      | -----------
      | @note
      | 
      | after this is called, the textDragExit
      | method may not be called, so you should
      | clean up in here if there's anything
      | you need to do when the drag finishes.
      | 
      | -----------
      | @param text
      | 
      | the text that the user is dragging
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
    fn text_dropped(
        &mut self, 
        text: &str,
        x:    i32,
        y:    i32
    );
}
