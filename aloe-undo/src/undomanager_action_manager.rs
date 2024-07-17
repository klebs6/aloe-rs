crate::ix!();

pub struct UndoManagerActionSet {
    actions: Vec<Box<dyn UndoableAction>>,
    name:    String,
    time:    Time, // { Time::getCurrentTime() };
}

impl UndoManagerActionSet {

    pub fn new(transaction_name: &String) -> Self {
    
        todo!();
        /*
        : name(transactionName),
        */
    }
    
    pub fn perform(&self) -> bool {
        
        todo!();
        /*
            for (auto* a : actions)
                if (! a->perform())
                    return false;

            return true;
        */
    }
    
    pub fn undo(&self) -> bool {
        
        todo!();
        /*
            for (int i = actions.size(); --i >= 0;)
                if (! actions.getUnchecked(i)->undo())
                    return false;

            return true;
        */
    }
    
    pub fn get_total_size(&self) -> i32 {
        
        todo!();
        /*
            int total = 0;

            for (auto* a : actions)
                total += a->getSizeInUnits();

            return total;
        */
    }
    
    pub fn get_current_set(&self) -> *mut UndoManagerActionSet {
        
        todo!();
        /*
            return transactions[nextIndex - 1];
        */
    }
    
    pub fn get_next_set(&self) -> *mut UndoManagerActionSet {
        
        todo!();
        /*
            return transactions[nextIndex];
        */
    }
}
