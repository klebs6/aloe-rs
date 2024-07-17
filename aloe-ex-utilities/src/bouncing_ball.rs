crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/MultithreadingDemo.h]

#[no_copy]
#[leak_detector]
pub struct BouncingBall<'a> {
    x:                   f32, // default = 0.0f
    y:                   f32, // default = 0.0f
    size:                f32, //= Random::getSystemRandom().nextFloat() * 30.0f + 30.0f;
    dx:                  f32, // default = 0.0f
    dy:                  f32, // default = 0.0f
    parent_width:        f32, // default = 50.0f
    parent_height:       f32, // default = 50.0f
    colour:              Colour,
    thread_id:           ThreadID,
    drawing:             CriticalSection,
    container_component: &'a mut Component<'a>,
}

impl<'a> ComponentListener for BouncingBall<'a> {

}

impl<'a> ComponentEnablementChanged for BouncingBall<'a> {

}

impl<'a> ComponentBeingDeleted for BouncingBall<'a> {

}

impl<'a> ComponentNameChanged for BouncingBall<'a> {

}

impl<'a> ComponentParentHierarchyChanged for BouncingBall<'a> {

}

impl<'a> ComponentChildrenChanged for BouncingBall<'a> {

}

impl<'a> ComponentVisibilityChanged for BouncingBall<'a> {

}

impl<'a> ComponentBroughtToFront for BouncingBall<'a> {

}

impl<'a> ComponentMovedOrResized for BouncingBall<'a> {

}

impl<'a> Drop for BouncingBall<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            containerComponent.removeComponentListener (this);
         */
    }
}

impl<'a> BouncingBall<'a> {

    pub fn new(comp: &mut Component) -> Self {
    
        todo!();
        /*
        : container_component(comp),

            containerComponent.addComponentListener (this);

            auto speed = 5.0f; // give each ball a fixed speed so we can
                               // see the effects of thread priority on how fast
                               // they actually go.

            auto angle = Random::getSystemRandom().nextFloat() * MathConstants<float>::twoPi;

            dx = std::sin (angle) * speed;
            dy = std::cos (angle) * speed;

            colour = Colour ((uint32) Random::getSystemRandom().nextInt())
                        .withAlpha (0.5f)
                        .withBrightness (0.7f);

            componentMovedOrResized (containerComponent, true, true);

            x = Random::getSystemRandom().nextFloat() * parentWidth;
            y = Random::getSystemRandom().nextFloat() * parentHeight;
        */
    }

    /**
       This will be called from the message thread
      */
    pub fn draw(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            const ScopedLock lock (drawing);

            g.setColour (colour);
            g.fillEllipse (x, y, size, size);

            g.setColour (Colours::black);
            g.setFont (10.0f);
            g.drawText (String::toHexString ((int64) threadId), Rectangle<float> (x, y, size, size), Justification::centred, false);
        */
    }
    
    pub fn move_ball(&mut self)  {
        
        todo!();
        /*
            const ScopedLock lock (drawing);

            threadId = Thread::getCurrentThreadId(); // this is so the component can print the thread ID inside the ball

            x += dx;
            y += dy;

            if (x < 0)
                dx = std::abs (dx);

            if (x > parentWidth)
                dx = -std::abs (dx);

            if (y < 0)
                dy = std::abs (dy);

            if (y > parentHeight)
                dy = -std::abs (dy);
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        comp: &mut Component,
        _1:   bool,
        _2:   bool)  {
        
        todo!();
        /*
            const ScopedLock lock (drawing);

            parentWidth  = (float) comp.getWidth()  - size;
            parentHeight = (float) comp.getHeight() - size;
        */
    }
}
