crate::ix!();

pub struct SvgStateUseTextOp<'a> {
    state:     *const SVGState,
    transform: *mut AffineTransform,
    target:    *mut Drawable<'a>,
}

impl<'a> SvgStateUseTextOp<'a> {

    pub fn invoke(&mut self, xml_path: &SvgStateXmlPath) -> bool {
        
        todo!();
        /*
            target = state->parseText (xmlPath, true, transform);
                return target != nullptr;
        */
    }
}
