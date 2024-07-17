crate::ix!();

pub trait HasEditor {

    fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

pub trait CreateAudioProcessorGraphEditor {
    
    fn create_editor(&mut self) -> *mut dyn AudioProcessorEditorInterface {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

pub trait CreateEditor {

    /**
      | Creates the processor's GUI.
      | 
      | This can return nullptr if you want a
      | GUI-less processor, in which case the
      | host may create a generic UI that lets
      | the user twiddle the parameters directly.
      | 
      | If you do want to pass back a component,
      | the component should be created and
      | set to the correct size before returning
      | it. If you implement this method, you
      | must also implement the hasEditor()
      | method and make it return true.
      | 
      | Remember not to do anything silly like
      | allowing your processor to keep a pointer
      | to the component that gets created -
      | it could be deleted later without any
      | warning, which would make your pointer
      | into a dangler. Use the getActiveEditor()
      | method instead.
      | 
      | The correct way to handle the connection
      | between an editor component and its
      | processor is to use something like a
      | ChangeBroadcaster so that the editor
      | can register itself as a listener, and
      | be told when a change occurs. This lets
      | them safely unregister themselves
      | when they are deleted.
      | 
      | Here are a few things to bear in mind when
      | writing an editor:
      | 
      | - Initially there won't be an editor,
      | until the user opens one, or they might
      | not open one at all. Your processor mustn't
      | rely on it being there.
      | 
      | - An editor object may be deleted and
      | a replacement one created again at any
      | time.
      | 
      | - It's safe to assume that an editor will
      | be deleted before its processor.
      | 
      | @see hasEditor
      |
      */
    fn create_editor(&mut self) -> *mut dyn AudioProcessorEditorInterface;
}

pub trait CheckHasEditor {

    /**
      | Your processor subclass must override
      | this and return true if it can create
      | an editor component. @see createEditor
      |
      */
    fn has_editor(&self) -> bool;
}

pub trait GetActiveEditor {

    /**
      | Returns the active editor, if there
      | is one. Bear in mind this can return nullptr
      | even if an editor has previously been
      | opened.
      | 
      | -----------
      | @note
      | 
      | you should only call this method from
      | the message thread as the active editor
      | may be deleted by the message thread,
      | causing a dangling pointer.
      |
      */
    fn get_active_editor(&self) -> *mut dyn AudioProcessorEditorInterface;
}

pub trait CreateEditorIfNeeded {

    /**
      | Returns the active editor, or if there
      | isn't one, it will create one.
      | 
      | This may call createEditor() internally
      | to create the component.
      |
      */
    fn create_editor_if_needed(&mut self) -> *mut dyn AudioProcessorEditorInterface;
}

pub trait EditorBeingDeleted {

    /**
      | Not for public use - this is called before
      | deleting an editor component.
      |
      */
    fn editor_being_deleted(&mut self, _0: *mut dyn AudioProcessorEditorInterface);
}
