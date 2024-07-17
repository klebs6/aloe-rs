crate::ix!();

pub trait SetIconImage {

    /**
      | Changes the image shown in the taskbar.
      | 
      | On Windows and Linux a full colour Image
      | is used as an icon.
      | 
      | On macOS a template image is used, where
      | all non-transparent regions will be
      | rendered in a monochrome colour selected
      | dynamically by the operating system.
      | 
      | -----------
      | @param colourImage
      | 
      | An colour image to use as an icon on Windows
      | and Linux
      | ----------
      | @param templateImage
      | 
      | A template image to use as an icon on macOS
      |
      */
    fn set_icon_image(
        &mut self, 
        colour_image:   &Image,
        template_image: &Image
    );
}

pub trait SetIconTooltip {

    /**
      | Changes the icon's tooltip (if the current
      | OS supports this).
      |
      */
    fn set_icon_tooltip(&mut self, tooltip: &String);
}


