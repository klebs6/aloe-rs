crate::ix!();

#[cfg(not(target_os="macos"))]
pub struct AloeVideoViewerClass {
    base: ObjCClass<UIView>,
}

#[cfg(not(target_os="macos"))]
impl Default for AloeVideoViewerClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<UIView> ("AloeVideoViewerClass_")
                       addMethod (@selector (layoutSubviews), layoutSubviews, "v@:");

                       registerClass();
        */
    }
}

#[cfg(not(target_os="macos"))]
impl AloeVideoViewerClass {

    pub fn layout_subviews(
        self_: id,
        _1:    SEL)  {
        
        todo!();
        /*
            sendSuperclassMessage<void> (self, @selector (layoutSubviews));

                       UIView* asUIView = (UIView*) self;

                       if (auto* previewLayer = getPreviewLayer (self))
                           previewLayer.frame = asUIView.bounds;
        */
    }
    
    pub fn get_preview_layer(self_: id) -> *mut AVPlayerLayer {
        
        todo!();
        /*
            UIView* asUIView = (UIView*) self;

                if (asUIView.layer.sublayers != nil && [asUIView.layer.sublayers count] > 0)
                    if ([asUIView.layer.sublayers[0] isKindOfClass: [AVPlayerLayer class]])
                        return (AVPlayerLayer*) asUIView.layer.sublayers[0];

                return nil;
        */
    }
}
