crate::ix!();

pub trait DrawableInterface
    : DrawableDeepCopy 
    + DrawableGetOutlineAsPath 
    + DrawableGetDrawableBounds
    + DrawableReplaceColour { }

pub trait DrawableDeepCopy {

    /**
      | Creates a deep copy of this Drawable
      | object.
      | 
      | Use this to create a new copy of this and
      | any sub-objects in the tree.
      |
      */
    fn create_copy(&self) -> Box<Self>;
}

pub trait DrawableGetOutlineAsPath {

    /**
      | Creates a path that describes the outline
      | of this drawable.
      |
      */
    fn get_outline_as_path(&self) -> Path;
}

pub trait DrawableGetDrawableBounds {

    /**
      | Returns the area that this drawable
      | covers.
      | 
      | The result is expressed in this drawable's
      | own coordinate space, and does not take
      | into account any transforms that may
      | be applied to the component.
      |
      */
    fn get_drawable_bounds(&self) -> Rectangle<f32>;
}

pub trait DrawableReplaceColour {

    /**
      | Recursively replaces a colour that
      | might be used for filling or stroking.
      | return true if any instances of this
      | colour were found.
      |
      */
    fn replace_colour(
        &mut self, 
        original_colour:    Colour,
        replacement_colour: Colour
    ) -> bool;
}
