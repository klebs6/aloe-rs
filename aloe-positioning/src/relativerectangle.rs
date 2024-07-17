crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeRectangle.h]

/**
  | A rectangle stored as a set of RelativeCoordinate
  | values.
  | 
  | The rectangle's top, left, bottom and
  | right edge positions are each stored
  | as a RelativeCoordinate.
  | 
  | @see RelativeCoordinate, RelativePoint
  | 
  | @tags{GUI}
  |
  */
pub struct RelativeRectangle {

    /** The actual rectangle coords... */
    left:   RelativeCoordinate,
    right:  RelativeCoordinate,
    top:    RelativeCoordinate,
    bottom: RelativeCoordinate,
}

impl Default for RelativeRectangle {
    
    /**
      | Creates a zero-size rectangle at the
      | origin.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl PartialEq<RelativeRectangle> for RelativeRectangle {
    
    #[inline] fn eq(&self, other: &RelativeRectangle) -> bool {
        todo!();
        /*
            return left == other.left && top == other.top && right == other.right && bottom == other.bottom;
        */

    }
}

impl Eq for RelativeRectangle {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeRectangle.cpp]
impl From<&Rectangle<f32>> for RelativeRectangle {
    
    /**
      | Creates an absolute rectangle, relative
      | to the origin.
      |
      */
    fn from(rect: &Rectangle<f32>) -> Self {
    
        todo!();
        /*


            : left (rect.getX()),
          right (Expression::symbol (RelativeCoordinate::Strings::left) + Expression ((double) rect.getWidth())),
          top (rect.getY()),
          bottom (Expression::symbol (RelativeCoordinate::Strings::top) + Expression ((double) rect.getHeight()))
        */
    }
}
    
impl From<&str> for RelativeRectangle {

    /**
      | Creates a rectangle from a stringified
      | representation.
      | 
      | The string must contain a sequence of
      | 4 coordinates, separated by commas,
      | in the order left, top, right, bottom.
      | The syntax for the coordinate strings
      | is explained in the
      | 
      | RelativeCoordinate class. @see toString
      |
      */
    fn from(s: &str) -> Self {
    
        todo!();
        /*


            String error;
        CharPointerType text (s.getCharPointer());
        left = RelativeCoordinate (Expression::parse (text, error));
        RelativeRectangleHelpers::skipComma (text);
        top = RelativeCoordinate (Expression::parse (text, error));
        RelativeRectangleHelpers::skipComma (text);
        right = RelativeCoordinate (Expression::parse (text, error));
        RelativeRectangleHelpers::skipComma (text);
        bottom = RelativeCoordinate (Expression::parse (text, error));
        */
    }
}

impl RelativeRectangle {

    /**
      | Creates a rectangle from four coordinates.
      |
      */
    pub fn new(
        left:   &RelativeCoordinate,
        right:  &RelativeCoordinate,
        top:    &RelativeCoordinate,
        bottom: &RelativeCoordinate) -> Self {
    
        todo!();
        /*
        : left(left_),
        : right(right_),
        : top(top_),
        : bottom(bottom_),

        
        */
    }

    /**
      | Creates and sets an appropriate Component::Positioner
      | object for the given component, which
      | will keep it positioned with this rectangle.
      |
      */
    pub fn apply_to_component(&self, component: &mut Component)  {
        
        todo!();
        /*
            if (isDynamic())
        {
            RelativeRectangleComponentPositioner* current = dynamic_cast<RelativeRectangleComponentPositioner*> (component.getPositioner());

            if (current == nullptr || ! current->isUsingRectangle (*this))
            {
                RelativeRectangleComponentPositioner* p = new RelativeRectangleComponentPositioner (component, *this);

                component.setPositioner (p);
                p->apply();
            }
        }
        else
        {
            component.setPositioner (nullptr);
            component.setBounds (resolve (nullptr).getSmallestIntegerContainer());
        }
        */
    }

    /**
      | Calculates the absolute position of
      | this rectangle.
      | 
      | You'll need to provide a suitable ExpressionScope
      | for looking up any coordinates that
      | may be needed to calculate the result.
      |
      */
    pub fn resolve(&self, scope: *const dyn ExpressionScopeInterface) -> Rectangle<f32> {
        
        todo!();
        /*
            if (scope == nullptr)
        {
            RelativeRectangleLocalScope defaultScope (*this);
            return resolve (&defaultScope);
        }
        else
        {
            const double l = left.resolve (scope);
            const double r = right.resolve (scope);
            const double t = top.resolve (scope);
            const double b = bottom.resolve (scope);

            return Rectangle<float> ((float) l, (float) t, (float) jmax (0.0, r - l), (float) jmax (0.0, b - t));
        }
        */
    }
    
    /**
      | Changes the values of this rectangle's
      | coordinates to make it resolve to the
      | specified position.
      | 
      | Calling this will leave any anchor points
      | unchanged, but will set any absolute
      | or relative positions to whatever values
      | are necessary to make the resultant
      | position match the position that is
      | provided.
      |
      */
    pub fn move_to_absolute(&mut self, 
        new_pos: &Rectangle<f32>,
        scope:   *const dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            left.moveToAbsolute (newPos.getX(), scope);
        right.moveToAbsolute (newPos.getRight(), scope);
        top.moveToAbsolute (newPos.getY(), scope);
        bottom.moveToAbsolute (newPos.getBottom(), scope);
        */
    }
    
    /**
      | Returns true if this rectangle depends
      | on any external symbols for its position.
      | 
      | Coordinates that refer to symbols based
      | on "this" are assumed not to be dynamic.
      |
      */
    pub fn is_dynamic(&self) -> bool {
        
        todo!();
        /*
            using namespace RelativeRectangleHelpers;

        return dependsOnSymbolsOtherThanThis (left.getExpression())
                || dependsOnSymbolsOtherThanThis (right.getExpression())
                || dependsOnSymbolsOtherThanThis (top.getExpression())
                || dependsOnSymbolsOtherThanThis (bottom.getExpression());
        */
    }
    
    /**
      | Returns a string which represents this
      | point.
      | 
      | This returns a comma-separated list
      | of coordinates, in the order left, top,
      | right, bottom.
      | 
      | If you're using this to position a Component,
      | then see the notes for
      | 
      | Component::setBounds (const RelativeRectangle&)
      | for details of the syntax used.
      | 
      | The string that is returned can be passed
      | to the RelativeRectangle constructor
      | to recreate the rectangle.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return left.toString() + ", " + top.toString() + ", " + right.toString() + ", " + bottom.toString();
        */
    }
    
    /**
      | Renames a symbol if it is used by any of
      | the coordinates.
      | 
      | This calls Expression::withRenamedSymbol()
      | on the rectangle's coordinates.
      |
      */
    pub fn rename_symbol(&mut self, 
        old_symbol: &ExpressionSymbol,
        new_name:   &String,
        scope:      &dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            left = left.getExpression().withRenamedSymbol (oldSymbol, newName, scope);
        right = right.getExpression().withRenamedSymbol (oldSymbol, newName, scope);
        top = top.getExpression().withRenamedSymbol (oldSymbol, newName, scope);
        bottom = bottom.getExpression().withRenamedSymbol (oldSymbol, newName, scope);
        */
    }
}

#[inline] pub fn relative_rectangle_skip_comma(s: &mut CharPointerType)  {
    
    todo!();
        /*
            s.incrementToEndOfWhitespace();

            if (*s == ',')
                ++s;
        */

}

pub fn relative_rectangle_depends_on_symbols_other_than_this(e: &Expression) -> bool {
    
    todo!();
        /*
            if (e.getType() == Expression::operatorType && e.getSymbolOrFunction() == ".")
                return true;

            if (e.getType() == Expression::symbolType)
            {
                switch (RelativeCoordinate::StandardStrings::getTypeOf (e.getSymbolOrFunction()))
                {
                    case RelativeCoordinate::StandardStrings::x:
                    case RelativeCoordinate::StandardStrings::y:
                    case RelativeCoordinate::StandardStrings::left:
                    case RelativeCoordinate::StandardStrings::right:
                    case RelativeCoordinate::StandardStrings::top:
                    case RelativeCoordinate::StandardStrings::bottom:   return false;
                    case RelativeCoordinate::StandardStrings::width:
                    case RelativeCoordinate::StandardStrings::height:
                    case RelativeCoordinate::StandardStrings::parent:
                    case RelativeCoordinate::StandardStrings::unknown:

                    default: break;
                }

                return true;
            }
            else
            {
                for (int i = e.getNumInputs(); --i >= 0;)
                    if (dependsOnSymbolsOtherThanThis (e.getInput(i)))
                        return true;
            }

            return false;
        */
}
