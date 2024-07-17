crate::ix!();

#[no_copy]
pub struct HueSelectorComp<'a> {
    base:   Component<'a>,
    owner:  &'a mut ColourSelector<'a>,
    h:      &'a mut f32,
    edge:   i32,
    marker: HueSelectorMarker<'a>,
}

impl<'a> HueSelectorComp<'a> {

    pub fn new(
        cs:        &mut ColourSelector<'a>,
        hue:       &mut f32,
        edge_size: i32) -> Self {
    
        todo!();
        /*
        : owner(cs),
        : h(hue),
        : edge(edgeSize),

            addAndMakeVisible (marker);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            ColourGradient cg;
            cg.isRadial = false;
            cg.point1.setXY (0.0f, (float) edge);
            cg.point2.setXY (0.0f, (float) getHeight());

            for (float i = 0.0f; i <= 1.0f; i += 0.02f)
                cg.addColour (i, Colour (i, 1.0f, 1.0f, 1.0f));

            g.setGradientFill (cg);
            g.fillRect (getLocalBounds().reduced (edge));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto markerSize = jmax (14, edge * 2);
            auto area = getLocalBounds().reduced (edge);

            marker.setBounds (Rectangle<int> (getWidth(), markerSize)
                                .withCentre (area.getRelativePoint (0.5f, h)));
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDrag (e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            owner.setHue ((float) (e.y - edge) / (float) (getHeight() - edge * 2));
        */
    }
    
    pub fn update_if_needed(&mut self)  {
        
        todo!();
        /*
            resized();
        */
    }
}
