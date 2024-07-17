crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/MultiTouchDemo.h]

pub struct MultiTouchDemo<'a> {
    base:   Component<'a>,
    trails: Vec<Box<MultiTouchDemoTrail<'a>>>,
}

impl<'a> Default for MultiTouchDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            setSize (500, 500)
        */
    }
}

impl<'a> MultiTouchDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colour::greyLevel (0.4f)));

            g.setColour (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::defaultText,
                                                 Colours::lightgrey));
            g.setFont (14.0f);
            g.drawFittedText ("Drag here with as many fingers as you have!",
                              getLocalBounds().reduced (30), Justification::centred, 4);

            for (auto* trail : trails)
                drawTrail (*trail, g);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto* t = getTrail (e.source);

            if (t == nullptr)
            {
                t = new MultiTouchDemoTrail (e.source);
                t->path.startNewSubPath (e.position);
                trails.add (t);
            }

            t->pushPoint (e.position, e.mods, e.pressure);
            repaint();
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            trails.removeObject (getTrail (e.source));
            repaint();
        */
    }
    
    pub fn draw_trail(&mut self, 
        trail: &mut MultiTouchDemoTrail,
        g:     &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (trail.colour);
            g.fillPath (trail.path);

            auto radius = 40.0f;

            g.setColour (Colours::black);
            g.drawEllipse (trail.currentPosition.x - radius,
                           trail.currentPosition.y - radius,
                           radius * 2.0f, radius * 2.0f, 2.0f);

            g.setFont (14.0f);

            String desc ("Mouse #");
            desc << trail.source.getIndex();

            auto pressure = trail.source.getCurrentPressure();

            if (pressure > 0.0f && pressure < 1.0f)
                desc << "  (pressure: " << (int) (pressure * 100.0f) << "%)";

            if (trail.modifierKeys.isCommandDown()) desc << " (CMD)";
            if (trail.modifierKeys.isShiftDown())   desc << " (SHIFT)";
            if (trail.modifierKeys.isCtrlDown())    desc << " (CTRL)";
            if (trail.modifierKeys.isAltDown())     desc << " (ALT)";

            g.drawText (desc,
                        Rectangle<int> ((int) trail.currentPosition.x - 200,
                                        (int) trail.currentPosition.y - 60,
                                        400, 20),
                        Justification::centredTop, false);
        */
    }
    
    pub fn get_trail(&mut self, source: &MouseInputSource) -> *mut MultiTouchDemoTrail {
        
        todo!();
        /*
            for (auto* trail : trails)
            {
                if (trail->source == source)
                    return trail;
            }

            return nullptr;
        */
    }
}
