crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_TooltipClient.h]

/**
  | Components that want to use pop-up tooltips
  | should implement this interface.
  | 
  | A TooltipWindow will wait for the mouse
  | to hover over a component that implements
  | the TooltipClient interface, and when
  | it finds one, it will display the tooltip
  | returned by its getTooltip() method.
  | 
  | @see TooltipWindow, SettableTooltipClient
  | 
  | @tags{GUI}
  |
  */
pub trait TooltipClient {

    /**
      | Returns the string that this object
      | wants to show as its tooltip.
      |
      */
    fn get_tooltip(&mut self) -> String;
}

pub trait SetTooltip {

    /**
      | Assigns a new tooltip to this object.
      |
      */
    fn set_tooltip(&mut self, new_tooltip: &String);
}

/**
  | An implementation of TooltipClient
  | that stores the tooltip string and a
  | method for changing it.
  | 
  | This makes it easy to add a tooltip to
  | a custom component, by simply adding
  | this as a base class and calling setTooltip().
  | 
  | Many of the Aloe widgets already use
  | this as a base class to implement their
  | tooltips.
  | 
  | @see TooltipClient, TooltipWindow
  | 
  | @tags{GUI}
  |
  */
#[derive(Default)]
pub struct SettableTooltipClient {
    tooltip_string: String,
}

impl TooltipClient for SettableTooltipClient {

    /**
      | Returns the tooltip assigned to this
      | object.
      |
      */
    fn get_tooltip(&mut self) -> String {
        
        todo!();
        /*
            return tooltipString;
        */
    }
}

impl SetTooltip for SettableTooltipClient {
    
    fn set_tooltip(&mut self, new_tooltip: &String)  {
        
        todo!();
        /*
            tooltipString = newTooltip;
        */
    }
}
