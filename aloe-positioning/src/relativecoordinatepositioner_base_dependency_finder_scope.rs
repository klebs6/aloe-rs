crate::ix!();

pub struct RelativeCoordinatePositionerBaseDependencyFinderScope<'a> {
    base:       RelativeCoordinatePositionerBaseComponentScope<'a>,
    positioner: &'a mut RelativeCoordinatePositionerBase<'a>,
    ok:         &'a mut bool,
}

impl<'a> RelativeCoordinatePositionerBaseDependencyFinderScope<'a> {

    pub fn new(
        comp:   &mut Component,
        p:      &mut RelativeCoordinatePositionerBase,
        result: &mut bool) -> Self {
    
        todo!();
        /*
        : component_scope(comp),
        : positioner(p),
        : ok(result),

        
        */

    }
    
    pub fn get_symbol_value(&self, symbol: &String) -> Expression {
        
        todo!();
        /*
            switch (RelativeCoordinate::StandardStrings::getTypeOf (symbol))
                {
                    case RelativeCoordinate::StandardStrings::x:
                    case RelativeCoordinate::StandardStrings::left:
                    case RelativeCoordinate::StandardStrings::y:
                    case RelativeCoordinate::StandardStrings::top:
                    case RelativeCoordinate::StandardStrings::width:
                    case RelativeCoordinate::StandardStrings::height:
                    case RelativeCoordinate::StandardStrings::right:
                    case RelativeCoordinate::StandardStrings::bottom:
                        positioner.registerComponentListener (component);
                        break;

                    case RelativeCoordinate::StandardStrings::parent:
                    case RelativeCoordinate::StandardStrings::unknown:
                    default:
                        if (auto* parent = component.getParentComponent())
                        {
                            MarkerList* list;

                            if (MarkerListScope::findMarker (*parent, symbol, list) != nullptr)
                            {
                                positioner.registerMarkerListListener (list);
                            }
                            else
                            {
                                // The marker we want doesn't exist, so watch all lists in case they change and the marker appears later..
                                if (auto* mlh = dynamic_cast<MarkerList::MarkerListHolder*> (parent))
                                {
                                    positioner.registerMarkerListListener (mlh->getMarkers (true));
                                    positioner.registerMarkerListListener (mlh->getMarkers (false));
                                }

                                ok = false;
                            }
                        }
                        break;
                }

                return RelativeCoordinatePositionerBaseComponentScope::getSymbolValue (symbol);
        */

    }
    
    pub fn visit_relative_scope(
        &self, 
        scope_name: &String,
        visitor:    &mut dyn ExpressionScopeVisitor

    ) {
        
        todo!();
        /*
            if (Component* const targetComp = (scopeName == RelativeCoordinate::Strings::parent)
                                                        ? component.getParentComponent()
                                                        : findSiblingComponent (scopeName))
                {
                    visitor.visit (RelativeCoordinatePositionerBaseDependencyFinderScope (*targetComp, positioner, ok));
                }
                else
                {
                    // The named component doesn't exist, so we'll watch the parent for changes in case it appears later..
                    if (Component* const parent = component.getParentComponent())
                        positioner.registerComponentListener (*parent);

                    positioner.registerComponentListener (component);
                    ok = false;
                }
        */

    }
}
