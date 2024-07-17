crate::ix!();

/**
  | This class provides some handy utility
  | methods for creating ModalComponentManagerCallback
  | objects that will invoke a static function
  | with some parameters when a modal component
  | is dismissed.
  | 
  | @tags{GUI}
  |
  */
pub struct ModalCallbackFunction {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_ModalComponentManager.cpp]
impl ModalCallbackFunction {

    /**
      | This is a utility function to create
      | a ModalComponentManagerCallback
      | that will call a callable object.
      | 
      | The function that you supply must take
      | an integer parameter, which is the result
      | code that was returned when the modal
      | component was dismissed.
      | 
      | @see ModalComponentManagerCallback
      |
      */
    pub fn create_with_callback<CallbackFn>(fn_: CallbackFn) -> *mut dyn ModalComponentManagerCallback {
    
        todo!();
        /*
            struct Callable  : public ModalComponentManagerCallback
            {
                explicit Callable (CallbackFn&& f)  : fn (std::forward<CallbackFn> (f)) {}
                void modalStateFinished (int result) override  { NullCheckedInvocation::invoke (std::move (fn), result); }

                std::remove_reference_t<CallbackFn> fn;
            };

            return new Callable (std::forward<CallbackFn> (fn));
        */
    }
    
    /**
      | This is a utility function to create
      | a ModalComponentManagerCallback
      | that will call a static function with
      | a parameter.
      | 
      | The function that you supply must take
      | two parameters - the first being an int,
      | which is the result code that was used
      | when the modal component was dismissed,
      | and the second can be a custom type. Note
      | that this custom value will be copied
      | and stored, so it must be a primitive
      | type or a class that provides copy-by-value
      | semantics.
      | 
      | E.g.
      | 
      | -----------
      | @code
      | 
      | static void myCallbackFunction (int modalResult, double customValue)
      | {
      |     if (modalResult == 1)
      |         doSomethingWith (customValue);
      | }
      | 
      | Component* someKindOfComp;
      | ...
      | someKindOfComp->enterModalState (true, ModalCallbackFunction::create (myCallbackFunction, 3.0));
      | 
      | @see ModalComponentManagerCallback
      |
      */
    pub fn create<ParamType>(
        function_to_call: fn(_0: i32, _1: ParamType) -> c_void,
        parameter_value:  ParamType

    ) -> *mut dyn ModalComponentManagerCallback {
    
        todo!();
        /*
            return create ([functionToCall, parameterValue] (int r)
            {
                functionToCall (r, parameterValue);
            });
        */
    }
    
    /**
      | This is a utility function to create
      | a ModalComponentManagerCallback
      | that will call a static function with
      | two custom parameters.
      | 
      | The function that you supply must take
      | three parameters - the first being an
      | int, which is the result code that was
      | used when the modal component was dismissed,
      | and the next two are your custom types.
      | Note that these custom values will be
      | copied and stored, so they must be primitive
      | types or classes that provide copy-by-value
      | semantics.
      | 
      | -----------
      | @code
      | 
      | static void myCallbackFunction (int modalResult, double customValue1, String customValue2)
      | {
      |     if (modalResult == 1)
      |         doSomethingWith (customValue1, customValue2);
      | }
      | 
      | Component* someKindOfComp;
      | ...
      | someKindOfComp->enterModalState (true, ModalCallbackFunction::create (myCallbackFunction, 3.0, String ("xyz")));
      | 
      | @see ModalComponentManagerCallback
      |
      */
    pub fn with_param<ParamType1, ParamType2>(
        function_to_call: fn(
                _0: i32,
                _1: ParamType1,
                _2: ParamType2
        ) -> c_void,
        parameter_value1: ParamType1,
        parameter_value2: ParamType2) -> *mut dyn ModalComponentManagerCallback {
    
        todo!();
        /*
            return create ([functionToCall, parameterValue1, parameterValue2] (int r)
            {
                functionToCall (r, parameterValue1, parameterValue2);
            });
        */
    }
    
    /**
      | This is a utility function to create
      | a ModalComponentManagerCallback
      | that will call a static function with
      | a component.
      | 
      | The function that you supply must take
      | two parameters - the first being an int,
      | which is the result code that was used
      | when the modal component was dismissed,
      | and the second can be a Component class.
      | The component will be stored as a WeakReference,
      | so that if it gets deleted before this
      | callback is invoked, the pointer that
      | is passed to the function will be null.
      | 
      | -----------
      | @code
      | 
      | static void myCallbackFunction (int modalResult, Slider* mySlider)
      | {
      |     if (modalResult == 1 && mySlider != nullptr) // (must check that mySlider isn't null in case it was deleted..)
      |         mySlider->setValue (0.0);
      | }
      | 
      | Component* someKindOfComp;
      | Slider* mySlider;
      | ...
      | someKindOfComp->enterModalState (true, ModalCallbackFunction::forComponent (myCallbackFunction, mySlider));
      | @see ModalComponentManagerCallback
      |
      */
    pub fn for_component<ComponentType>(
        function_to_call: fn(_0: i32, _1: *mut ComponentType) -> c_void,
        component:        *mut ComponentType) -> *mut dyn ModalComponentManagerCallback {
    
        todo!();
        /*
            return create ([functionToCall, comp = WeakReference<Component> { component }] (int r)
            {
                functionToCall (r, static_cast<ComponentType*> (comp.get()));
            });
        */
    }
    
    /**
      | Creates a ModalComponentManagerCallback
      | that will call a static function with
      | a component.
      | 
      | The function that you supply must take
      | three parameters - the first being an
      | int, which is the result code that was
      | used when the modal component was dismissed,
      | the second being a Component class,
      | and the third being a custom type (which
      | must be a primitive type or have copy-by-value
      | semantics).
      | 
      | The component will be stored as a WeakReference,
      | so that if it gets deleted before this
      | callback is invoked, the pointer that
      | is passed into the function will be null.
      | 
      | -----------
      | @code
      | 
      | static void myCallbackFunction (int modalResult, Slider* mySlider, String customParam)
      | {
      |     if (modalResult == 1 && mySlider != nullptr) // (must check that mySlider isn't null in case it was deleted..)
      |         mySlider->setName (customParam);
      | }
      | 
      | Component* someKindOfComp;
      | Slider* mySlider;
      | ...
      | someKindOfComp->enterModalState (true, ModalCallbackFunction::forComponent (myCallbackFunction, mySlider, String ("hello")));
      | @see ModalComponentManagerCallback
      |
      */
    pub fn for_component_with_params<ComponentType, ParamType>(
        function_to_call: fn(
                _0: i32,
                _1: *mut ComponentType,
                _2: ParamType
        ) -> c_void,
        component:        *mut ComponentType,
        param:            ParamType) -> *mut dyn ModalComponentManagerCallback {
    
        todo!();
        /*
            return create ([functionToCall, param, comp = WeakReference<Component> { component }] (int r)
            {
                functionToCall (r, static_cast<ComponentType*> (comp.get()), param);
            });
        */
    }
}
