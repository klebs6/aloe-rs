crate::ix!();

pub trait CreateAccessibilityHandler {

    /**
      | Override this method to return a custom
      | AccessibilityHandler for this component.
      | 
      | The default implementation creates
      | and returns a AccessibilityHandler
      | object with an unspecified role, meaning
      | that it will be visible to accessibility
      | clients but without a specific role,
      | action callbacks or interfaces. To
      | control how accessibility clients
      | see and interact with your component
      | subclass AccessibilityHandler, implement
      | the desired behaviours, and return
      | an instance of it from this method in
      | your component subclass.
      | 
      | The accessibility handler you return
      | here is guaranteed to be destroyed before
      | its Component<'a>, so it's safe to store
      | and use a reference back to the Component
      | inside the AccessibilityHandler if
      | necessary.
      | 
      | @see getAccessibilityHandler
      |
      */
    fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler>;

}
