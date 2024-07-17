crate::ix!();

pub struct SvgStateGetFillTypeOp {
    state:     *const SVGState,
    path:      *const Path,
    opacity:   f32,
    fill_type: FillType,
}

impl SvgStateGetFillTypeOp {

    pub fn invoke(&mut self, xml: &SvgStateXmlPath) -> bool {
        
        todo!();
        /*
            if (xml->hasTagNameIgnoringNamespace ("linearGradient")
                     || xml->hasTagNameIgnoringNamespace ("radialGradient"))
                {
                    fillType = state->getGradientFillType (xml, *path, opacity);
                    return true;
                }

                return false;
        */
    }
}

