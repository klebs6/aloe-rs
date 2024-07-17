crate::ix!();

pub fn expression_check_recursion_depth(depth: i32)  {
    
    todo!();
        /*
            if (depth > 256)
                    throw EvaluationError ("Recursive symbol references");
        */
}

pub fn find_destination_for(
    top_level:  *mut Term,
    input_term: *const Term

) -> *mut Term {
    
    todo!();
        /*
            const int inputIndex = topLevel->getInputIndexFor (inputTerm);
        if (inputIndex >= 0)
            return topLevel;

        for (int i = topLevel->getNumInputs(); --i >= 0;)
        {
            Term* const t = findDestinationFor (topLevel->getInput (i), inputTerm);

            if (t != nullptr)
                return t;
        }

        return nullptr;
        */
}

pub fn find_term_to_adjust(
    term:            *mut Term,
    must_be_flagged: bool

) -> *mut Constant {
    
    todo!();
        /*
            if (term == nullptr)
        {
            jassertfalse;
            return nullptr;
        }

        if (term->getType() == constantType)
        {
            Constant* const c = static_cast<Constant*> (term);
            if (c->isResolutionTarget || ! mustBeFlagged)
                return c;
        }

        if (term->getType() == functionType)
            return nullptr;

        const int numIns = term->getNumInputs();

        for (int i = 0; i < numIns; ++i)
        {
            Term* const input = term->getInput (i);

            if (input->getType() == constantType)
            {
                Constant* const c = static_cast<Constant*> (input);

                if (c->isResolutionTarget || ! mustBeFlagged)
                    return c;
            }
        }

        for (int i = 0; i < numIns; ++i)
            if (auto c = findTermToAdjust (term->getInput (i), mustBeFlagged))
                return c;

        return nullptr;
        */
}

pub fn contains_any_symbols(t: &Term) -> bool {
    
    todo!();
        /*
            if (t.getType() == Expression::symbolType)
                    return true;

                for (int i = t.getNumInputs(); --i >= 0;)
                    if (containsAnySymbols (*t.getInput (i)))
                        return true;

                return false;
        */
}
