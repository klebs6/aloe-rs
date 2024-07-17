crate::ix!();

pub struct SvgStateUseImageOp<'a> {
    state:     *const SVGState,
    transform: *mut AffineTransform,
    target:    *mut Drawable<'a>,
}

impl<'a> SvgStateUseImageOp<'a> {

    pub fn invoke(&mut self, xml_path: &SvgStateXmlPath) -> bool {
        
        todo!();
        /*
            target = state->parseImage (xmlPath, true, transform);
                return target != nullptr;
        */
    }
}
