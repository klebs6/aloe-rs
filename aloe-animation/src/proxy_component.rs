crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentAnimator.h]
#[no_copy]
#[leak_detector]
pub struct AnimationTaskProxyComponent<'a> {
    base: Component<'a>,
    image: Image,
}

impl<'a> AnimationTaskProxyComponent<'a> {

    pub fn new(c: &mut Component) -> Self {
    
        todo!();
        /*


            setWantsKeyboardFocus (false);
                setBounds (c.getBounds());
                setTransform (c.getTransform());
                setAlpha (c.getAlpha());
                setInterceptsMouseClicks (false, false);

                if (auto* parent = c.getParentComponent())
                    parent->addAndMakeVisible (this);
                else if (c.isOnDesktop() && c.getPeer() != nullptr)
                    addToDesktop (c.getPeer()->getStyleFlags() | ComponentPeer::windowIgnoresKeyPresses);
                else
                    jassertfalse; // seem to be trying to animate a component that's not visible..

                auto scale = (float) Desktop::getInstance().getDisplays().getDisplayForRect (getScreenBounds())->scale
                               * Component::getApproximateScaleFactorForComponent (&c);

                image = c.createComponentSnapshot (c.getLocalBounds(), false, scale);

                setVisible (true);
                toBehind (&c);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setOpacity (1.0f);
                g.drawImageTransformed (image, AffineTransform::scale ((float) getWidth()  / (float) jmax (1, image.getWidth()),
                                                                       (float) getHeight() / (float) jmax (1, image.getHeight())), false);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return createIgnoredAccessibilityHandler (*this);
        */
    }
}
