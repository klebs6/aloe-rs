/*!
  | The contents of this namespace are used
  | to implement ProcessorChain and should
  | not be used elsewhere. Their interfaces
  | (and existence) are liable to change!
  |
  */

// Define a trait that will implement the desired functionality
pub trait ForEachInTuple {
    fn for_each_in_tuple<F: FnMut(&dyn std::any::Any)>(self, f: F);
}

// Implement the trait for an empty tuple
impl ForEachInTuple for () {
    fn for_each_in_tuple<F: FnMut(&dyn std::any::Any)>(self, _: F) {}
}

// Implement the trait for non-empty tuples, using recursion
macro_rules! tuple_impls {
    // Base case: single-element tuple
    ($last:ident,) => {
        impl<$last> ForEachInTuple for ($last,)
        where
            $last: 'static,
        {
            fn for_each_in_tuple<F: FnMut(&dyn std::any::Any)>(self, mut f: F) {
                let ($last,) = self;
                f(&$last as &dyn std::any::Any);
            }
        }
    };
    // Recursive case
    ($head:ident, $($tail:ident,)*) => {
        impl<$head, $($tail,)*> ForEachInTuple for ($head, $($tail,)*)
        where
            $head: 'static,
            $($tail: 'static,)*
        {
            fn for_each_in_tuple<F: FnMut(&dyn std::any::Any)>(self, mut f: F) {
                let ($head, $($tail,)*) = self;
                f(&$head as &dyn std::any::Any);
                ($($tail,)*).for_each_in_tuple(f);
            }
        }

        // Continue the recursion with the tail of the tuple
        tuple_impls!($($tail,)*);
    };
}

// Implement the trait for tuples up to a certain size
tuple_impls!(
    X1, 
    X2, 
    X3, 
    X4, 
    X5, 
    X6, 
    X7, 
    X8, 
    X9, 
    X10, 
    X11, 
    X12, 
    X13, 
    X14, 
    X15, 
    X16, 
    X17, 
    X18, 
    X19, 
    X20, 
    X21, 
    X22, 
    X23, 
    X24, 
    X25, 
    X26,
);
