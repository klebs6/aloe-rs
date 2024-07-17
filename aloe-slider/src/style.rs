crate::ix!();

/**
  | The types of slider available.
  | 
  | @see setSliderStyle, setRotaryParameters
  |
  */
pub enum SliderStyle
{
    /**
      | A traditional horizontal slider.
      |
      */
    LinearHorizontal,               

    /**
      | A traditional vertical slider.
      |
      */
    LinearVertical,                 

    /**
      | A horizontal bar slider with the text
      | label drawn on top of it.
      |
      */
    LinearBar,                      

    /**
      | A vertical bar slider with the text label
      | drawn on top of it.
      |
      */
    LinearBarVertical,              

    /**
      | A rotary control that you move by dragging
      | the mouse in a circular motion, like
      | a knob. @see setRotaryParameters
      |
      */
    Rotary,                         

    /**
      | A rotary control that you move by dragging
      | the mouse left-to-right. @see setRotaryParameters
      |
      */
    RotaryHorizontalDrag,           

    /**
      | A rotary control that you move by dragging
      | the mouse up-and-down. @see setRotaryParameters
      |
      */
    RotaryVerticalDrag,             

    /**
      | A rotary control that you move by dragging
      | the mouse up-and-down or left-to-right.
      | @see setRotaryParameters
      |
      */
    RotaryHorizontalVerticalDrag,   

    /**
      | A pair of buttons that increment or decrement
      | the slider's value by the increment
      | set in setRange().
      |
      */
    IncDecButtons,                  

    /**
      | A horizontal slider that has two thumbs
      | instead of one, so it can show a minimum
      | and maximum value. @see setMinValue,
      | setMaxValue
      |
      */
    TwoValueHorizontal,             

    /**
      | A vertical slider that has two thumbs
      | instead of one, so it can show a minimum
      | and maximum value. @see setMinValue,
      | setMaxValue
      |
      */
    TwoValueVertical,               

    /**
      | A horizontal slider that has three thumbs
      | instead of one, so it can show a minimum
      | and maximum value, with the current
      | value being somewhere between them.
      | @see setMinValue, setMaxValue
      |
      */
    ThreeValueHorizontal,           

    /**
      | A vertical slider that has three thumbs
      | instead of one, so it can show a minimum
      | and maximum value, with the current
      | value being somewhere between them.
      | @see setMinValue, setMaxValue
      |
      */
    ThreeValueVertical,              
}
