crate::ix!();

pub struct XWindowSystemVisualAndDepth
{
    visual: *mut Visual,
    depth:  i32,
}

pub struct XWindowSystemDisplayVisuals
{
    visual_16bit: *mut Visual, // default = nullptr
    visual_24bit: *mut Visual, // default = nullptr
    visual_32bit: *mut Visual, // default = nullptr
}

impl XWindowSystemDisplayVisuals {
    
    pub fn new(x_display: *mut Display) -> Self {
    
        todo!();
        /*


            auto findVisualWithDepthOrNull = [&] (int desiredDepth) -> Visual*
        {
            int matchedDepth = 0;
            auto* visual = Visuals::findVisualFormat (xDisplay, desiredDepth, matchedDepth);

            if (desiredDepth == matchedDepth)
                return visual;

            return nullptr;
        };

        visual16Bit = findVisualWithDepthOrNull (16);
        visual24Bit = findVisualWithDepthOrNull (24);
        visual32Bit = findVisualWithDepthOrNull (32);
        */
    }
    
    pub fn get_best_visual_for_window(&self, is_semi_transparent: bool) -> XWindowSystemVisualAndDepth {
        
        todo!();
        /*
            if (isSemiTransparent && visual32Bit != nullptr)
            return { visual32Bit, 32 };

        if (visual24Bit != nullptr)
            return { visual24Bit, 24 };

        return { visual16Bit, 16 };
        */
    }
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return (visual32Bit != nullptr || visual24Bit != nullptr || visual16Bit != nullptr);
        */
    }
}
