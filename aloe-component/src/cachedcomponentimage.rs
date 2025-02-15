crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_CachedComponentImage.h]

pub trait InvalidateAll {

    fn invalidate_all(&mut self) -> bool;
}

pub trait Invalidate {

    fn invalidate(&mut self, area: &Rectangle<i32>) -> bool;
}

/**
  | Base class used internally for structures
  | that can store cached images of component
  | state.
  | 
  | Most people are unlikely to ever need
  | to know about this class - it's really
  | only for power-users!
  | 
  | @see Component::setCachedComponentImage
  | 
  | @tags{GUI}
  |
  */
pub trait CachedComponentImage 

    /*: Default*/ 

    /*
     | Called as part of the parent component's
     | paint method, this must draw the given
     | component into the target graphics
     | context, using the cached version where
     | possible.
     |
     */
    : Paint

    /*
      | Invalidates all cached image data.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the peer should also be repainted,
      | or false if this object handles all repaint
      | work internally.
      |
      */
    + InvalidateAll

    /*
      | Invalidates a section of the cached
      | image data.
      | 
      | -----------
      | @return
      | 
      | true if the peer should also be repainted,
      | or false if this object handles all repaint
      | work internally.
      |
      */
    + Invalidate

    /*
      | Called to indicate that the component
      | is no longer active, so any cached data
      | should be released if possible.
      |
      */
    + ReleaseResources
{ }
