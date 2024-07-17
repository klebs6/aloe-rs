crate::ix!();

#[no_copy]
pub struct ShadowWindow<'a> {
    base:   Component<'a>,
    target: WeakReference<Component<'a>>,
    shadow: DropShadow,
}

impl<'a> ShadowWindow<'a> {

    pub fn new(
        comp: *mut Component<'a>,
        ds:   &DropShadow) -> Self {
    
        todo!();
        /*
        : target(comp),
        : shadow(ds),

            setVisible (true);
                setAccessible (false);
                setInterceptsMouseClicks (false, false);

                if (comp->isOnDesktop())
                {
                    setSize (1, 1); // to keep the OS happy by not having zero-size windows
                    addToDesktop (ComponentPeer::windowIgnoresMouseClicks
                        | ComponentPeer::windowIsTemporary
                        | ComponentPeer::windowIgnoresKeyPresses);
                }
                else if (Component* const parent = comp->getParentComponent())
                {
                    parent->addChildComponent (this);
                }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (Component* c = target)
                    shadow.drawForRectangle (g, getLocalArea (c, c->getLocalBounds()));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            repaint();  // (needed for correct repainting)
        */
    }
    
    pub fn get_desktop_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            if (target != nullptr)
                    return target->getDesktopScaleFactor();

                return Component::getDesktopScaleFactor();
        */
    }
}
