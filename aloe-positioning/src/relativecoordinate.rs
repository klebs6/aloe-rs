crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeCoordinate.h]

/**
  | Expresses a coordinate as a dynamically
  | evaluated expression.
  | 
  | When using relative coordinates to
  | position components, the following
  | symbols are available:
  | 
  | - "left", "right", "top", "bottom"
  | refer to the position of those edges
  | in this component, so e.g. for a component
  | whose width is always 100, you might
  | set the right edge to the "left + 100".
  | 
  | - "[id].left", "[id].right", "[id].top",
  | "[id].bottom", "[id].width", "[id].height",
  | where [id] is the identifier of one of
  | this component's siblings. A component's
  | identifier is set with
  | 
  | Component::setComponentID(). So
  | for example if you want your component
  | to always be 50 pixels to the right of
  | the one called "xyz", you could set your
  | left edge to be "xyz.right + 50".
  | 
  | - Instead of an [id], you can use the name
  | "parent" to refer to this component's
  | parent. Like any other component, these
  | values are relative to their component's
  | parent, so "parent.right" won't be
  | very useful for positioning a component
  | because it refers to a position with
  | the parent's parent.. but "parent.width"
  | can be used for setting positions relative
  | to the parent's size. E.g. to make a 10x10
  | component which remains 1 pixel away
  | from its parent's bottom-right, you
  | could use "right - 10, bottom - 10, parent.width
  | - 1, parent.height - 1".
  | 
  | - The name of one of the parent component's
  | markers can also be used as a symbol.
  | For markers to be used, the parent component
  | must implement its Component::getMarkers()
  | method, and return at least one valid
  | MarkerList. So if you want your component's
  | top edge to be 10 pixels below the marker
  | called "foobar", you'd set it to "foobar
  | + 10".
  | 
  | See the Expression class for details
  | about the operators that are supported,
  | but for example if you wanted to make
  | your component remains centred within
  | its parent with a size of 100, 100, you
  | could express it as:
  | 
  | -----------
  | @code
  | 
  | myComp.setBounds (RelativeBounds (
  |     "parent.width / 2 - 50, 
  |     parent.height / 2 - 50, 
  |     left + 100, 
  |     top + 100"));
  |
  | ..or an alternative way to achieve the
  | same thing:
  | ----------
  | @code
  | 
  | myComp.setBounds (RelativeBounds (
  |       "right - 100, 
  |       bottom - 100, 
  |       parent.width / 2 + 50, 
  |       parent.height / 2 + 50"));
  | 
  | Or if you wanted a 100x100 component
  | whose top edge is lined up to a marker
  | called "topMarker" and which is positioned
  | 50 pixels to the right of another component
  | called "otherComp", you could write:
  | ----------
  | @code
  | 
  | myComp.setBounds (RelativeBounds (
  |       "otherComp.right + 50, 
  |       topMarker, 
  |       left + 100, 
  |       top + 100"));
  | 
  | Be careful not to make your coordinate
  | expressions recursive, though, or
  | exceptions and assertions will be thrown!
  | 
  | @see RelativePoint, RelativeRectangle
  | 
  | @tags{GUI}
  |
  */
pub struct RelativeCoordinate {
    term: Expression,
}

impl Default for RelativeCoordinate {
    
    /**
      | Creates a zero coordinate.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl PartialEq<RelativeCoordinate> for RelativeCoordinate {
    
    #[inline] fn eq(&self, other: &RelativeCoordinate) -> bool {
        todo!();
        /*
            return term.toString() == other.term.toString();
        */

    }
}

impl Eq for RelativeCoordinate {}

impl From<&Expression> for RelativeCoordinate {

    fn from(term: &Expression) -> Self {
    
        todo!();
        /*
        : term(term_),
        */
    }
}

impl From<f64> for RelativeCoordinate {

    /**
      | Creates an absolute position from the
      | parent origin on either the X or Y axis.
      | 
      | -----------
      | @param absoluteDistanceFromOrigin
      | 
      | the distance from the origin
      |
      */
    fn from(absolute_distance_from_origin: f64) -> Self {
    
        todo!();
        /*
        : term(absoluteDistanceFromOrigin),

        
        */

    }
}
    
impl From<&str> for RelativeCoordinate {

    /**
      | Recreates a coordinate from a string
      | description.
      | 
      | The string will be parsed by ExpressionParser::parse().
      | 
      | -----------
      | @param stringVersion
      | 
      | the expression to use @see toString
      |
      */
    fn from(s: &str) -> Self {
    
        todo!();
        /*
        String error;
        term = Expression (s, error);
        */

    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeCoordinate.cpp]
impl RelativeCoordinate {
    
    /**
      | Returns true if this coordinate uses
      | the specified coord name at any level
      | in its evaluation.
      | 
      | This will recursively check any coordinates
      | upon which this one depends.
      |
      */
    pub fn references(&self, 
        coord_name:       &String,
        evaluation_scope: *const dyn ExpressionScopeInterface) -> bool {
        
        todo!();
        /*
        
        */

    }

    /**
      | Returns the expression that defines
      | this coordinate.
      |
      */
    pub fn get_expression(&self) -> &Expression {
        
        todo!();
        /*
            return term;
        */

    }
    
    
    pub fn new_from_ref(other: &RelativeCoordinate) -> Self {
    
        todo!();
        /*
        : term(other.term),

        
        */

    }
    
    pub fn assign_from_ref(&mut self, other: &RelativeCoordinate) -> &mut RelativeCoordinate {
        
        todo!();
        /*
            term = other.term;
        return *this;
        */

    }
    
    pub fn new_from_other(other: RelativeCoordinate) -> Self {
    
        todo!();
        /*
        : term(std::move (other.term)),
        */
    }
    
    pub fn assign_from(&mut self, other: RelativeCoordinate) -> &mut RelativeCoordinate {
        
        todo!();
        /*
            term = std::move (other.term);
        return *this;
        */

    }
    
    
    /**
      | Calculates the absolute position of
      | this coordinate.
      | 
      | You'll need to provide a suitable ExpressionScope
      | for looking up any coordinates that
      | may be needed to calculate the result.
      |
      */
    pub fn resolve(&self, scope: *const dyn ExpressionScopeInterface) -> f64 {
        
        todo!();
        /*
            if (scope != nullptr)
            return term.evaluate (*scope);

        return term.evaluate();
        */

    }
    
    /**
      | Returns true if there's a recursive
      | loop when trying to resolve this coordinate's
      | position.
      |
      */
    pub fn is_recursive(&self, scope: *const dyn ExpressionScopeInterface) -> bool {
        
        todo!();
        /*
            String error;

        if (scope != nullptr)
            term.evaluate (*scope, error);
        else
            term.evaluate (ExpressionScope(), error);

        return error.isNotEmpty();
        */

    }
    
    /**
      | Changes the value of this coord to make
      | it resolve to the specified position.
      | 
      | Calling this will leave the anchor points
      | unchanged, but will set this coordinate's
      | absolute or relative position to whatever
      | value is necessary to make its resultant
      | position match the position that is
      | provided.
      |
      */
    pub fn move_to_absolute(
        &mut self, 
        new_pos: f64,
        scope:   *const dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            if (scope != nullptr)
        {
            term = term.adjustedToGiveNewResult (newPos, *scope);
        }
        else
        {
            ExpressionScope defaultScope;
            term = term.adjustedToGiveNewResult (newPos, defaultScope);
        }
        */

    }
    
    /**
      | Returns true if this coordinate depends
      | on any other coordinates for its position.
      |
      */
    pub fn is_dynamic(&self) -> bool {
        
        todo!();
        /*
            return term.usesAnySymbols();
        */

    }
    
    /**
      | Returns a string which represents this
      | coordinate.
      | 
      | For details of the string syntax, see
      | the constructor notes.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return term.toString();
        */

    }
}
