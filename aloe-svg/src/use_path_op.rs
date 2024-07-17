crate::ix!();

pub struct SvgStateUsePathOp {
    state:       *const SVGState,
    target_path: *mut Path,
}

impl SvgStateUsePathOp {

    pub fn invoke(&self, xml_path: &SvgStateXmlPath) -> bool {
        
        todo!();
        /*
            return state->parsePathElement (xmlPath, *targetPath);
        */
    }
}
