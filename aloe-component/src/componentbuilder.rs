crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentBuilder.h]

/**
  | Loads and maintains a tree of Components
  | from a ValueTree that represents them.
  | 
  | To allow the state of a tree of components
  | to be saved as a ValueTree and re-loaded,
  | this class lets you register a set of
  | type-handlers for the different components
  | that are involved, and then uses these
  | types to re-create a set of components
  | from its stored state.
  | 
  | Essentially, to use this, you need to
  | create a ComponentBuilder with your
  | ValueTree, then use registerTypeHandler()
  | to give it a set of type handlers that
  | can cope with all the items in your tree.
  | Then you can call getComponent() to
  | build the component.
  | 
  | Once you've got the component you can
  | either take it and delete the ComponentBuilder
  | object, or if you keep the ComponentBuilder
  | around, it'll monitor any changes in
  | the
  | 
  | ValueTree and automatically update
  | the component to reflect these changes.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentBuilder<'a> {

    /**
      | This is the ValueTree data object that
      | the builder is working with.
      |
      */
    state:          ValueTree,
    types:          Vec<ComponentBuilderTypeHandler<'a>>,
    component:      Box<Component<'a>>,
    image_provider: *mut dyn ComponentBuilderImageProvider,

    #[cfg(ALOE_DEBUG)]
    component_ref: WeakReference<Component>,
}

impl<'a> ValueTreeListener for ComponentBuilder<'a> {

}

impl<'a> ComponentBuilder<'a> {

    /**
      | An identifier for the property of the
      | ValueTrees that is used to store a unique
      | ID for that component.
      |
      */
    lazy_static!{
        /*
        static const Identifier idProperty;
        const Identifier ComponentBuilder::idProperty ("id");
        */
    }
    
    pub fn get_state_id(state: &ValueTree) -> String {
        
        todo!();
        /*
            return state [ComponentBuilder::idProperty].toString();
        */
    }

    pub fn remove_component_withid(
        components: &mut Vec<Component<'a>>,
        comp_id:    &String

    ) -> *mut Component<'a> {
        
        todo!();
        /*
            jassert (compId.isNotEmpty());

                for (int i = components.size(); --i >= 0;)
                {
                    Component* const c = components.getUnchecked (i);

                    if (c->getComponentID() == compId)
                        return components.removeAndReturn (i);
                }

                return nullptr;
        */
    }

    pub fn find_component_withid(
            c:       &mut Component<'a>,
            comp_id: &String) -> *mut Component<'a> {
        
        todo!();
        /*
            jassert (compId.isNotEmpty());
                if (c.getComponentID() == compId)
                    return &c;

                for (auto* child : c.getChildren())
                    if (auto* found = findComponentWithID (*child, compId))
                        return found;

                return nullptr;
        */
    }

    pub fn create_new_component(
            ty:     &mut ComponentBuilderTypeHandler,
            state:  &ValueTree,
            parent: *mut Component<'a>) -> *mut Component<'a> {
        
        todo!();
        /*
            Component* const c = type.addNewComponentFromState (state, parent);
                jassert (c != nullptr && c->getParentComponent() == parent);
                c->setComponentID (getStateId (state));
                return c;
        */
    }

    pub fn update_component(
            builder: &mut ComponentBuilder,
            state:   &ValueTree)  {
        
        todo!();
        /*
            if (Component* topLevelComp = builder.getManagedComponent())
                {
                    ComponentBuilder::ComponentBuilderTypeHandler* const type = builder.getHandlerForState (state);
                    const String uid (getStateId (state));

                    if (type == nullptr || uid.isEmpty())
                    {
                        // ..handle the case where a child of the actual state node has changed.
                        if (state.getParent().isValid())
                            updateComponent (builder, state.getParent());
                    }
                    else
                    {
                        if (Component* const changedComp = findComponentWithID (*topLevelComp, uid))
                            type->updateComponentFromState (changedComp, state);
                    }
                }
        */
    }
}

/**
  | The class is a base class for objects
  | that manage the loading of a type of component
  | from a ValueTree.
  | 
  | To store and re-load a tree of components
  | as a ValueTree, each component type
  | must have a ComponentBuilderTypeHandler to represent
  | it.
  | 
  | @see ComponentBuilder::registerTypeHandler(),
  | Drawable::registerDrawableTypeHandlers()
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentBuilderTypeHandler<'a> {

    /**
     | Returns the type of the ValueTrees that
     | this handler can parse.
     |
     */
    ty:      Identifier,
    builder: *mut ComponentBuilder<'a>,
}

pub trait ComponentBuilderTypeHandlerAddNewComponentFromState {

    /**
      | This method must create a new component
      | from the given state, add it to the specified
      | parent component (which may be null),
      | and return it.
      | 
      | The ValueTree will have been pre-checked
      | to make sure that its type matches the
      | type that this handler supports.
      | 
      | There's no need to set the new Component's
      | ID to match that of the state - the builder
      | will take care of that itself.
      |
      */
    fn add_new_component_from_state(&mut self, 
        state:  &ValueTree,
        parent: *mut Component) -> *mut Component;

}

pub trait ComponentBuilderUpdateComponentFromState {

    /**
      | This method must update an existing
      | component from a new ValueTree state.
      | 
      | A component that has been created with
      | addNewComponentFromState() may need
      | to be updated if the ValueTree changes,
      | so this method is used to do that. Your
      | implementation must do whatever's
      | necessary to update the component from
      | the new state provided.
      | 
      | The ValueTree will have been pre-checked
      | to make sure that its type matches the
      | type that this handler supports, and
      | the component will have been created
      | by this type's addNewComponentFromState()
      | method.
      |
      */
    fn update_component_from_state(&mut self, 
        component: *mut Component,
        state:     &ValueTree);

}

impl<'a> ComponentBuilderTypeHandler<'a> {

    /**
      | Creates a ComponentBuilderTypeHandler.
      | 
      | The valueTreeType must be the type name
      | of the ValueTrees that this handler
      | can parse.
      |
      */
    pub fn new(value_tree_type: &Identifier) -> Self {
    
        todo!();
        /*
        : ty(valueTreeType),
        : builder(nullptr),

        
        */
    }
    
    /**
      | Returns the builder that this type is
      | registered with.
      |
      */
    pub fn get_builder(&self) -> *mut ComponentBuilder<'a> {
        
        todo!();
        /*
            // A type handler needs to be registered with a ComponentBuilder before using it!
        jassert (builder != nullptr);
        return builder;
        */
    }
}

/**
  | This class is used when references to
  | images need to be stored in ValueTrees.
  | 
  | An instance of an ImageProvider provides
  | a mechanism for converting an Image
  | to/from a reference, which may be a file,
  | Url, ID string, or whatever system is
  | appropriate in your app.
  | 
  | When you're loading components from
  | a ValueTree that may need a way of loading
  | images, you should call ComponentBuilder::setImageProvider()
  | to supply a suitable provider before
  | trying to load the component.
  | 
  | @see ComponentBuilder::setImageProvider()
  |
  */
pub trait ComponentBuilderImageProvider {

    /**
      | Retrieves the image associated with
      | this identifier, which could be any
      | kind of string, number, filename, etc.
      | 
      | The image that is returned will be owned
      | by the caller, but it may come from the
      | ImageCache.
      |
      */
    fn get_image_for_identifier(&mut self, image_identifier: &Var) -> Image;

    /**
      | Returns an identifier to be used to refer
      | to a given image.
      | 
      | This is used when a reference to an image
      | is stored in a ValueTree.
      |
      */
    fn get_identifier_for_image(&mut self, image: &Image) -> Var;

}

impl<'a> Drop for ComponentBuilder<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
        state.removeListener (this);

       #if ALOE_DEBUG
        // Don't delete the managed component!! The builder owns that component, and will delete
        // it automatically when it gets deleted.
        jassert (componentRef.get() == component.get());
       #endif
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentBuilder.cpp]

impl<'a> Default for ComponentBuilder<'a> {

    /**
      | Creates a builder that doesn't have
      | a state object.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : image_provider(nullptr),

        
        */
    }
}

impl<'a> ComponentBuilder<'a> {
    
    /**
      | Creates a ComponentBuilder that will
      | use the given state.
      | 
      | Once you've created your builder, you
      | should use registerTypeHandler()
      | to register some type handlers for it,
      | and then you can call createComponent()
      | or getManagedComponent() to get the
      | actual component.
      |
      */
    pub fn new(state: &ValueTree) -> Self {
    
        todo!();
        /*
        : state(state_),
        : image_provider(nullptr),

            state.addListener (this);
        */
    }
    
    /**
      | Returns the builder's component (creating
      | it if necessary).
      | 
      | The first time that this method is called,
      | the builder will attempt to create a
      | component from the ValueTree, so you
      | must have registered some suitable
      | type handlers before calling this.
      | If there's a problem and the component
      | can't be created, this method returns
      | nullptr.
      | 
      | The component that is returned is owned
      | by this ComponentBuilder, so you can
      | put it inside your own parent components,
      | but don't delete it! The ComponentBuilder
      | will delete it automatically when the
      | builder is destroyed. If you want to
      | get a component that you can delete yourself,
      | call createComponent() instead.
      | 
      | The ComponentBuilder will update this
      | component if any changes are made to
      | the ValueTree, so if there's a chance
      | that the tree might change, be careful
      | not to keep any pointers to sub-components,
      | as they may be changed or removed.
      |
      */
    pub fn get_managed_component(&mut self) -> *mut Component<'a> {
        
        todo!();
        /*
            if (component == nullptr)
        {
            component.reset (createComponent());

           #if ALOE_DEBUG
            componentRef = component.get();
           #endif
        }

        return component.get();
        */
    }
    
    /**
      | Creates and returns a new instance of
      | the component that the ValueTree represents.
      | 
      | The caller is responsible for using
      | and deleting the object that is returned.
      | Unlike getManagedComponent(), the
      | component that is returned will not
      | be updated by the builder.
      |
      */
    pub fn create_component(&mut self) -> *mut Component<'a> {
        
        todo!();
        /*
            jassert (types.size() > 0);  // You need to register all the necessary types before you can load a component!

        if (ComponentBuilderTypeHandler* const type = getHandlerForState (state))
            return ComponentBuilderHelpers::createNewComponent (*type, state, nullptr);

        jassertfalse; // trying to create a component from an unknown type of ValueTree
        return nullptr;
        */
    }
    
    /**
      | Adds a type handler that the builder
      | can use when trying to load components.
      | @see Drawable::registerDrawableTypeHandlers()
      |
      */
    pub fn register_type_handler(&mut self, ty: *mut ComponentBuilderTypeHandler)  {
        
        todo!();
        /*
            jassert (type != nullptr);

        // Don't try to move your types around! Once a type has been added to a builder, the
        // builder owns it, and you should leave it alone!
        jassert (type->builder == nullptr);

        types.add (type);
        type->builder = this;
        */
    }
    
    /**
      | Tries to find a registered type handler
      | that can load a component from the given
      | ValueTree.
      |
      */
    pub fn get_handler_for_state(&self, s: &ValueTree) -> *mut ComponentBuilderTypeHandler {
        
        todo!();
        /*
            const Identifier targetType (s.getType());

        for (int i = 0; i < types.size(); ++i)
        {
            ComponentBuilderTypeHandler* const t = types.getUnchecked(i);

            if (t->type == targetType)
                return t;
        }

        return nullptr;
        */
    }
    
    /**
      | Returns the number of registered type
      | handlers. @see getHandler, registerTypeHandler
      |
      */
    pub fn get_num_handlers(&self) -> i32 {
        
        todo!();
        /*
            return types.size();
        */
    }
    
    /**
      | Returns one of the registered type handlers.
      | @see getNumHandlers, registerTypeHandler
      |
      */
    pub fn get_handler(&self, index: i32) -> *mut ComponentBuilderTypeHandler {
        
        todo!();
        /*
            return types [index];
        */
    }
    
    /**
      | Registers handlers for various standard
      | aloe components.
      |
      */
    pub fn register_standard_component_types(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Gives the builder an ImageProvider
      | object that the type handlers can use
      | when loading images from stored references.
      | 
      | The object that is passed in is not owned
      | by the builder, so the caller must delete
      | it when it is no longer needed, but not
      | while the builder may still be using
      | it. To clear the image provider, just
      | call setImageProvider (nullptr).
      |
      */
    pub fn set_image_provider(&mut self, new_image_provider: *mut dyn ComponentBuilderImageProvider)  {
        
        todo!();
        /*
            imageProvider = newImageProvider;
        */
    }
    
    /**
      | Returns the current image provider
      | that this builder is using, or nullptr
      | if none has been set.
      |
      */
    pub fn get_image_provider(&self) -> *mut dyn ComponentBuilderImageProvider {
        
        todo!();
        /*
            return imageProvider;
        */
    }
    
    pub fn value_tree_property_changed(&mut self, 
        tree: &mut ValueTree,
        _1:   &Identifier)  {
        
        todo!();
        /*
            ComponentBuilderHelpers::updateComponent (*this, tree);
        */
    }
    
    pub fn value_tree_child_added(&mut self, 
        tree: &mut ValueTree,
        _1:   &mut ValueTree)  {
        
        todo!();
        /*
            ComponentBuilderHelpers::updateComponent (*this, tree);
        */
    }
    
    pub fn value_tree_child_removed(&mut self, 
        tree: &mut ValueTree,
        _1:   &mut ValueTree,
        _2:   i32)  {
        
        todo!();
        /*
            ComponentBuilderHelpers::updateComponent (*this, tree);
        */
    }
    
    pub fn value_tree_child_order_changed(&mut self, 
        tree: &mut ValueTree,
        _1:   i32,
        _2:   i32)  {
        
        todo!();
        /*
            ComponentBuilderHelpers::updateComponent (*this, tree);
        */
    }
    
    pub fn value_tree_parent_changed(&mut self, tree: &mut ValueTree)  {
        
        todo!();
        /*
            ComponentBuilderHelpers::updateComponent (*this, tree);
        */
    }
    
    /**
      | Updates the children of a parent component
      | by updating them from the children of
      | a given ValueTree.
      |
      */
    pub fn update_child_components(&mut self, 
        parent:   &mut Component<'a>,
        children: &ValueTree)  {
        
        todo!();
        /*
            using namespace ComponentBuilderHelpers;

        auto numExistingChildComps = parent.getNumChildComponents();

        Vec<Component*> componentsInOrder;
        componentsInOrder.ensureStorageAllocated (numExistingChildComps);

        {
            Vec<Box<Component>> existingComponents;
            existingComponents.ensureStorageAllocated (numExistingChildComps);

            for (int i = 0; i < numExistingChildComps; ++i)
                existingComponents.add (parent.getChildComponent (i));

            auto newNumChildren = children.getNumChildren();

            for (int i = 0; i < newNumChildren; ++i)
            {
                auto childState = children.getChild (i);
                auto* c = removeComponentWithID (existingComponents, getStateId (childState));

                if (c == nullptr)
                {
                    if (auto* type = getHandlerForState (childState))
                        c = ComponentBuilderHelpers::createNewComponent (*type, childState, &parent);
                    else
                        jassertfalse;
                }

                if (c != nullptr)
                    componentsInOrder.add (c);
            }

            // (remaining unused items in existingComponents get deleted here as it goes out of scope)
        }

        // Make sure the z-order is correct..
        if (componentsInOrder.size() > 0)
        {
            componentsInOrder.getLast()->toFront (false);

            for (int i = componentsInOrder.size() - 1; --i >= 0;)
                componentsInOrder.getUnchecked(i)->toBehind (componentsInOrder.getUnchecked (i + 1));
        }
        */
    }
}
