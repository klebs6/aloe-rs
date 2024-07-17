crate::ix!();

bitflags!{
    /**
      | Options for the type of selector to show.
      | These are passed into the constructor.
      |
      */
    pub struct ColourSelectorOptions: u8
    {
        /**
          | if set, the colour's alpha channel can
          | be changed as well as its RGB.
          |
          */
        const showAlphaChannel    = 1 << 0;

        /**
          | if set, a swatch of the colour is shown
          | at the top of the component.
          |
          */
        const showColourAtTop     = 1 << 1;

        /**
          | if set, the colour shows at the top of
          | the component is editable.
          |
          */
        const editableColour      = 1 << 2;

        /**
          | if set, RGB sliders are shown at the bottom
          | of the component.
          |
          */
        const showSliders         = 1 << 3;

        /**
          | if set, a big HSV selector is shown.
          |
          */
        const showColourspace     = 1 << 4;
    }
}
