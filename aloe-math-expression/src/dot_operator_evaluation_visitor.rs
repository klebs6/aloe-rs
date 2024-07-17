crate::ix!();

#[no_copy]
pub struct DotOperatorEvaluationVisitor {
    input:           TermPtr,
    output:          TermPtr,
    recursion_count: i32,
}

impl ExpressionScopeVisitor for DotOperatorEvaluationVisitor {

    fn visit(&mut self, scope: &dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            output = input->resolve (scope, recursionCount);
        */
    }
}

impl DotOperatorEvaluationVisitor {

    pub fn new(
        t:         &TermPtr,
        recursion: i32) -> Self {
    
        todo!();
        /*
        : input(t),
        : output(t),
        : recursion_count(recursion),

        
        */
    }
}
