crate::ix!();

pub struct MarkerListScope<'a> {
    component: &'a mut Component<'a>,
}

impl<'a> ExpressionScopeInterface for MarkerListScope<'a> {

    fn get_scopeuid(&self) -> String {
        
        todo!();
        /*
            return String::toHexString ((pointer_sized_int) (void*) &component) + "m";
        */

    }
    
    fn get_symbol_value(&self, symbol: &String) -> Expression {
        
        todo!();
        /*
            auto type = RelativeCoordinate::StandardStrings::getTypeOf (symbol);

            if (type == RelativeCoordinate::StandardStrings::width)   return Expression ((double) component.getWidth());
            if (type == RelativeCoordinate::StandardStrings::height)  return Expression ((double) component.getHeight());

            MarkerList* list;

            if (auto* marker = findMarker (component, symbol, list))
                return Expression (marker->position.getExpression().evaluate (*this));

            return ExpressionScope::getSymbolValue (symbol);
        */

    }

    fn evaluate_function(&self, _: &String, _: *const f64, _: i32) -> f64 {
        todo!() 
    }
    
    fn visit_relative_scope(
        &self, 
        scope_name: &String,
        visitor:    &mut dyn ExpressionScopeVisitor

    ) {
        
        todo!();
        /*
            if (scopeName == RelativeCoordinate::Strings::parent)
            {
                if (auto* parent = component.getParentComponent())
                {
                    visitor.visit (MarkerListScope (*parent));
                    return;
                }
            }

            ExpressionScope::visitRelativeScope (scopeName, visitor);
        */

    }
}

impl<'a> MarkerListScope<'a> {

    pub fn new(comp: &mut Component<'a>) -> Self {
    
        todo!();
        /*
        : component(comp),

        
        */

    }
    
    pub fn find_marker(
        component: &mut Component<'a>,
        name:      &String,
        list:      &mut *mut MarkerList) -> *const MarkerListMarker {
        
        todo!();
        /*
            const MarkerList::Marker* marker = nullptr;

            auto* mlh = dynamic_cast<MarkerList::MarkerListHolder*> (&component);

            if (mlh != nullptr)
            {
                list = mlh->getMarkers (true);

                if (list != nullptr)
                    marker = list->getMarker (name);
            }

            if (marker == nullptr)
            {
                if (mlh != nullptr)
                {
                    list = mlh->getMarkers (false);

                    if (list != nullptr)
                        marker = list->getMarker (name);
                }
            }

            return marker;
        */

    }
}
