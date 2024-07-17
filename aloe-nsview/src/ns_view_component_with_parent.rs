crate::ix!();

/**
  | This is an NSViewComponent which holds
  | a long-lived NSView which acts as the
  | parent view for plugin editors.
  | 
  | -----------
  | @note
  | 
  | this component does not auto-resize
  | depending on the bounds of the owned
  | view. Vst2 and Vst3 plugins have dedicated
  | interfaces to request that the editor
  | bounds are updated. We can call `setSize`
  | on this component from inside those
  | dedicated callbacks.
  |
  */
#[cfg(target_os="macos")]
#[no_copy]
#[no_move]
pub struct NSViewComponentWithParent<'a> {
    base:  NSViewComponent<'a>,
    base2: AsyncUpdater<'a>,
}

#[cfg(target_os="macos")]
impl<'a> Drop for NSViewComponentWithParent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (auto* view = static_cast<NSView*> (getView()))
                object_setInstanceVariable (view, "owner", nullptr);

            cancelPendingUpdate();
        */
    }
}

#[cfg(target_os="macos")]
impl<'a> NSViewComponentWithParent<'a> {

    pub fn new(should_nudge: NSViewComponentWithParentWantsNudge) -> Self {
    
        todo!();
        /*
        : wants_nudge(shouldNudge),

            auto* view = [[getViewClass().createInstance() init] autorelease];
            object_setInstanceVariable (view, "owner", this);
            setView (view);
        */
    }
    
    pub fn new_from_instance(instance: &mut AudioPluginInstance) -> Self {
    
        todo!();
        /*
        : ns_view_component_with_parent(getWantsNudge (instance)),

        
        */
    }
    
    pub fn get_wants_nudge(instance: &mut AudioPluginInstance) -> NSViewComponentWithParentWantsNudge {
        
        todo!();
        /*
            PluginDescription pd;
            instance.fillInPluginDescription (pd);
            return pd.manufacturerName == "FabFilter" ? NSViewComponentWithParentWantsNudge::yes : NSViewComponentWithParentWantsNudge::no;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            if (auto* peer = getTopLevelComponent()->getPeer())
            {
                auto* view = static_cast<NSView*> (getView());
                const auto newArea = peer->getAreaCoveredBy (*this);
                [view setFrame: makeNSRect (newArea.withHeight (newArea.getHeight() + 1))];
                [view setFrame: makeNSRect (newArea)];
            }
        */
    }
    
    pub fn get_view_class() -> &'a mut FlippedNSView {
        
        todo!();
        /*
            static FlippedNSView result;
            return result;
        */
    }
}
