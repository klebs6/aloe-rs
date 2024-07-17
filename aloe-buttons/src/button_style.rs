crate::ix!();

pub enum DrawableButtonStyle
{
    /**
      | The button will just display the images,
      | but will resize and centre them to fit
      | inside it.
      |
      */
    ImageFitted,                            

    /**
      | The button will just display the images
      | in their normal size and position. This
      | leaves it up to the caller to make sure
      | the images are the correct size and position
      | for the button.
      |
      */
    ImageRaw,                               

    /**
      | Draws the button as a text label across
      | the bottom with the image resized and
      | scaled to fit above it.
      |
      */
    ImageAboveTextLabel,                    

    /**
      | Draws the button as a standard rounded-rectangle
      | button with the image on top. The image
      | will be resized to match the button's
      | proportions. Note that if you use this
      | style, the colour IDs that control the
      | button colour are TextButton::buttonColourId
      | and TextButton::buttonOnColourId.
      |
      */
    ImageOnButtonBackground,                

    /**
      | Same as ImageOnButtonBackground,
      | but keeps the original image size.
      |
      */
    ImageOnButtonBackgroundOriginalSize,    

    /**
      | Fills the button with a stretched version
      | of the image.
      |
      */
    ImageStretched,                          
}

