crate::ix!();

pub struct SvgStateSetGradientStopsOp {
    state:    *const SVGState,
    gradient: *mut ColourGradient,
}

impl SvgStateSetGradientStopsOp {

    pub fn invoke(&self, xml: &SvgStateXmlPath) -> bool {
        
        todo!();
        /*
            return state->addGradientStopsIn (*gradient, xml);
        */
    }
}
