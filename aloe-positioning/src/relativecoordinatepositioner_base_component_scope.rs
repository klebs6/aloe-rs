crate::ix!();

/**
  | Used for resolving a RelativeCoordinate
  | expression in the context of a component.
  |
  */
pub struct RelativeCoordinatePositionerBaseComponentScope<'a> {
    component: &'a mut Component<'a>,
}

impl<'a> ExpressionScopeInterface for RelativeCoordinatePositionerBaseComponentScope<'a> {

    fn evaluate_function(&self, _: &String, _: *const f64, _: i32) -> f64 { 
        todo!()
    }

    fn get_symbol_value(&self, symbol: &String) -> Expression {
        
        todo!();
        /*
            switch (RelativeCoordinate::StandardStrings::getTypeOf (symbol))
        {
            case RelativeCoordinate::StandardStrings::x:
            case RelativeCoordinate::StandardStrings::left:   return Expression ((double) component.getX());
            case RelativeCoordinate::StandardStrings::y:
            case RelativeCoordinate::StandardStrings::top:    return Expression ((double) component.getY());
            case RelativeCoordinate::StandardStrings::width:  return Expression ((double) component.getWidth());
            case RelativeCoordinate::StandardStrings::height: return Expression ((double) component.getHeight());
            case RelativeCoordinate::StandardStrings::right:  return Expression ((double) component.getRight());
            case RelativeCoordinate::StandardStrings::bottom: return Expression ((double) component.getBottom());
            case RelativeCoordinate::StandardStrings::parent:
            case RelativeCoordinate::StandardStrings::unknown:
            default: break;
        }

        if (Component* const parent = component.getParentComponent())
        {
            MarkerList* list;

            if (auto* marker = MarkerListScope::findMarker (*parent, symbol, list))
            {
                MarkerListScope scope (*parent);
                return Expression (marker->position.getExpression().evaluate (scope));
            }
        }

        return ExpressionScope::getSymbolValue (symbol);
        */

    }
    
    fn visit_relative_scope(
        &self, 
        scope_name: &String,
        visitor:    &mut dyn ExpressionScopeVisitor

    ) {
        
        todo!();
        /*
            if (auto* targetComp = (scopeName == RelativeCoordinate::Strings::parent)
                                   ? component.getParentComponent()
                                   : findSiblingComponent (scopeName))
            visitor.visit (RelativeCoordinatePositionerBaseComponentScope (*targetComp));
        else
            ExpressionScope::visitRelativeScope (scopeName, visitor);
        */

    }
    
    fn get_scopeuid(&self) -> String {
        
        todo!();
        /*
            return String::toHexString ((pointer_sized_int) (void*) &component);
        */

    }
}

impl<'a> RelativeCoordinatePositionerBaseComponentScope<'a> {
    
    pub fn new(comp: &mut Component) -> Self {
    
        todo!();
        /*
        : component(comp),

        
        */

    }
    
    pub fn find_sibling_component(&self, componentid: &String) -> *mut Component {
        
        todo!();
        /*
            if (Component* const parent = component.getParentComponent())
            return parent->findChildWithID (componentID);

        return nullptr;
        */

    }
}
