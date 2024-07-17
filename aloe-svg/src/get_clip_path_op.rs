crate::ix!();

pub struct SvgStateGetClipPathOp<'a> {
    state:  *mut SVGState,
    target: *mut Drawable<'a>,
}

impl<'a> SvgStateGetClipPathOp<'a> {

    pub fn invoke(&mut self, xml_path: &SvgStateXmlPath) -> bool {
        
        todo!();
        /*
            return state->applyClipPath (*target, xmlPath);
        */
    }
}
