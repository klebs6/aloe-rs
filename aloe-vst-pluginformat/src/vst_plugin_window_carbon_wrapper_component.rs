#[cfg(not(any(target_os="ios",target_os="android")))]
crate::ix!();

#[cfg(target_os="macos")]
#[cfg(ALOE_SUPPORT_CARBON)]
#[no_copy]
#[leak_detector]
pub struct VSTPluginWindowCarbonWrapperComponent {
    base:           CarbonViewWrapperComponent,
    owner:          &mut VSTPluginWindow,
    already_inside: bool, // default = false
}

#[cfg(target_os="macos")]
#[cfg(ALOE_SUPPORT_CARBON)]
impl Drop for VSTPluginWindowCarbonWrapperComponent {
    fn drop(&mut self) {
        todo!();
        /*
            deleteWindow();
        */
    }
}

#[cfg(target_os="macos")]
#[cfg(ALOE_SUPPORT_CARBON)]
impl VSTPluginWindowCarbonWrapperComponent {

    pub fn new(w: &mut VSTPluginWindow) -> Self {
    
        todo!();
        /*
        : owner(w),

            keepPluginWindowWhenHidden = w.shouldAvoidDeletingWindow();
                  setRepaintsChildHIViewWhenCreated (w.shouldRepaintCarbonWindowWhenCreated());
        */
    }
    
    pub fn attach_view(&mut self, 
        window_ref: WindowRef,
        root_view:  HIViewRef) -> HIViewRef {
        
        todo!();
        /*
            owner.openPluginWindow (windowRef);
                  return {};
        */
    }
    
    pub fn remove_view(&mut self, _0: HIViewRef)  {
        
        todo!();
        /*
            if (owner.isOpen)
                  {
                      owner.isOpen = false;
                      owner.dispatch (typename Vst2EffEditClose, 0, 0, 0, 0);
                      owner.dispatch (typename Vst2EffEditSleep, 0, 0, 0, 0);
                  }
        */
    }
    
    pub fn get_embedded_view_size(&mut self, 
        w: &mut i32,
        h: &mut i32) -> bool {
        
        todo!();
        /*
            typename Vst2ERect* rect = nullptr;
                  owner.dispatch (typename Vst2EffEditGetRect, 0, 0, &rect, 0);
                  w = rect->right - rect->left;
                  h = rect->bottom - rect->top;
                  return true;
        */
    }
    
    pub fn handle_mouse_down(&mut self, x: i32, y: i32)  {
        
        todo!();
        /*
            if (! alreadyInside)
                  {
                      alreadyInside = true;
                      getTopLevelComponent()->toFront (true);
                      owner.dispatch (typename Vst2EffEditMouse, x, y, 0, 0);
                      alreadyInside = false;
                  }
                  else
                  {
                      PostEvent (::mouseDown, 0);
                  }
        */
    }
    
    pub fn handle_paint(&mut self)  {
        
        todo!();
        /*
            if (auto* peer = getPeer())
                  {
                      auto pos = peer->globalToLocal (getScreenPosition());
                      typename Vst2ERect r;
                      r.left   = (int16) pos.getX();
                      r.top    = (int16) pos.getY();
                      r.right  = (int16) (r.left + getWidth());
                      r.bottom = (int16) (r.top + getHeight());

                      owner.dispatch (typename Vst2EffEditDraw, 0, 0, &r, 0);
                  }
        */
    }
}
