crate::ix!();

pub trait SetScaleFactor {

    /**
      | Can be called by a host to tell the editor
      | that it should use a non-unity
      | 
      | GUI scale.
      |
      */
    fn set_scale_factor(&mut self, new_scale: f32)  {
        
        todo!();
        /*
        hostScaleTransform = AffineTransform::scale (newScale);
        setTransform (hostScaleTransform);

        editorResized (true);
        */
    }
}
