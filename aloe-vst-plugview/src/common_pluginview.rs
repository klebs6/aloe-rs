crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/common/pluginview.h]

/**
  | Plug-In view default implementation.
  | \ingroup sdkBase
  | 
  | Can be used as base class for an IPlugView
  | implementation.
  |
  */
pub struct CPluginView {
    base:          FObject,
    rect:          ViewRect,
    system_window: *mut c_void,       // default = { nullptr }
    plug_frame:    *mut dyn IPlugFrame, // default = { nullptr }
}

impl FUnknown for CPluginView {

    fn query_interface(
        &mut self, 
        _: [i8; 16], 
        _: *mut *mut aloe_deps::c_void

    ) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }
    fn release(&mut self) -> u32 { todo!() }
}

pub trait AttachedToParent {

    /**
      | Calls when this view will be attached
      | to its parent view.
      |
      */
    fn attached_to_parent(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl AttachedToParent for CPluginView { }

pub trait RemovedFromParent {

    /**
      | Calls when this view will be removed
      | from its parent view.
      |
      */
    fn removed_from_parent(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl RemovedFromParent for CPluginView { }

impl IPlugView for CPluginView {

    fn is_platform_type_supported(&mut self, ty: FIDString) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    fn attached(&mut self, 
        parent: *mut c_void,
        ty:     FIDString) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    fn removed(&mut self) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    fn on_wheel(&mut self, distance: f32) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    fn on_key_down(&mut self, 
        key:       u16,
        key_msg:   i16,
        modifiers: i16) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    fn on_key_up(&mut self, 
        key:       u16,
        key_msg:   i16,
        modifiers: i16) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    fn get_size(&mut self, size: *mut ViewRect) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    fn on_size(&mut self, new_size: *mut ViewRect) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    fn on_focus(&mut self, state: TBool) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    fn set_frame(&mut self, frame: *mut dyn IPlugFrame) -> tresult {
        
        todo!();
        /*
            plugFrame = frame;
            return kResultTrue;
        */
    }
    
    fn can_resize(&mut self) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    fn check_size_constraint(&mut self, rect: *mut ViewRect) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/common/pluginview.cpp]
impl CPluginView {

    //---Interface------
    lazy_static!{
        /*
        OBJ_METHODS (CPluginView, FObject)
            DEFINE_INTERFACES
                DEF_INTERFACE (IPlugView)
            END_DEFINE_INTERFACES (FObject)
            REFCOUNT_METHODS (FObject)
        */
    }

    /**
      | Returns its current frame rectangle.
      |
      */
    pub fn get_rect(&self) -> &ViewRect {
        
        todo!();
        /*
            return rect;
        */
    }

    /**
      | Sets a new frame rectangle.
      |
      */
    pub fn set_rect(&mut self, r: &ViewRect)  {
        
        todo!();
        /*
            rect = r;
        */
    }

    /**
      | Checks if this view is attached to its
      | parent view.
      |
      */
    pub fn is_attached(&self) -> bool {
        
        todo!();
        /*
            return systemWindow != nullptr;
        */
    }
    
    pub fn new(rect: *const ViewRect) -> Self {
    
        todo!();
        /*
        : rect(0, 0, 0, 0),

            if (_rect)
            rect = *_rect;
        */
    }
    
    #[PLUGIN_API]
    pub fn is_platform_type_supported(&mut self, ty: FIDString) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn attached(&mut self, 
        parent: *mut c_void,
        ty:     FIDString) -> tresult {
        
        todo!();
        /*
            systemWindow = parent;

        attachedToParent ();
        return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    pub fn removed(&mut self) -> tresult {
        
        todo!();
        /*
            systemWindow = nullptr;

        removedFromParent ();
        return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    pub fn on_size(&mut self, new_size: *mut ViewRect) -> tresult {
        
        todo!();
        /*
            if (newSize)
            rect = *newSize;
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_size(&mut self, size: *mut ViewRect) -> tresult {
        
        todo!();
        /*
            if (size)
        {
            *size = rect;
            return kResultTrue;
        }
        return kInvalidArgument;
        */
    }
}
