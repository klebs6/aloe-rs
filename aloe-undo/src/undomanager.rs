crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/undomanager/aloe_UndoManager.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/undomanager/aloe_UndoManager.cpp]

/**
  | Manages a list of undo/redo commands.
  | 
  | An UndoManager object keeps a list of
  | past actions and can use these actions
  | to move backwards and forwards through
  | an undo history.
  | 
  | To use it, create subclasses of UndoableAction
  | which perform all the actions you need,
  | then when you need to actually perform
  | an action, create one and pass it to the
  | UndoManager's perform() method.
  | 
  | The manager also uses the concept of
  | 'transactions' to group the actions
  | together - all actions performed between
  | calls to beginNewTransaction() are
  | grouped together and are all undone/redone
  | as a group.
  | 
  | The UndoManager is a ChangeBroadcaster,
  | so listeners can register to be told
  | when actions are performed or undone.
  | 
  | @see UndoableAction
  | 
  | @tags{DataStructures}
  |
  */
#[no_copy]
#[leak_detector]
pub struct UndoManager<'a> {
    base:                         ChangeBroadcaster<'a>,
    transactions:                 Vec<UndoManagerActionSet>,
    stashed_future_transactions:  Vec<UndoManagerActionSet>,
    new_transaction_name:         String,
    total_units_stored:           i32, // default = 0
    max_num_units_to_keep:        i32, // default = 0
    minimum_transactions_to_keep: i32, // default = 0
    next_index:                   i32, // default = 0
    new_transaction:              bool, // default = true
    is_inside_undo_redo_call:     bool, // default = false
}

impl<'a> UndoManager<'a> {

    /**
      | Creates an UndoManager.
      | 
      | -----------
      | @param maxNumberOfUnitsToKeep
      | 
      | each UndoableAction object returns
      | a value to indicate how much storage
      | it takes up (UndoableAction::getSizeInUnits()),
      | so this lets you specify the maximum
      | total number of units that the undomanager
      | is allowed to keep in memory before letting
      | the older actions drop off the end of
      | the list.
      | ----------
      | @param minimumTransactionsToKeep
      | 
      | this specifies the minimum number of
      | transactions that will be kept, even
      | if this involves exceeding the amount
      | of space specified in maxNumberOfUnitsToKeep
      |
      */
    pub fn new(
        max_number_of_units_to_keep: Option<i32>,
        minimum_transactions:        Option<i32>

    ) -> Self {

        let max_number_of_units_to_keep: i32 =
            max_number_of_units_to_keep.unwrap_or(30000);

        let minimum_transactions: i32 =
            minimum_transactions.unwrap_or(30);
    
        todo!();
        /*
            setMaxNumberOfStoredUnits (maxNumberOfUnitsToKeep, minimumTransactions);
        */
    }

    /**
      | Deletes all stored actions in the list.
      |
      */
    pub fn clear_undo_history(&mut self)  {
        
        todo!();
        /*
            transactions.clear();
        totalUnitsStored = 0;
        nextIndex = 0;
        sendChangeMessage();
        */
    }

    /**
      | Returns the current amount of space
      | to use for storing UndoableAction objects.
      | @see setMaxNumberOfStoredUnits
      |
      */
    pub fn get_number_of_units_taken_up_by_stored_commands(&self) -> i32 {
        
        todo!();
        /*
            return totalUnitsStored;
        */
    }

    /**
      | Sets the amount of space that can be used
      | for storing UndoableAction objects.
      | 
      | -----------
      | @param maxNumberOfUnitsToKeep
      | 
      | each UndoableAction object returns
      | a value to indicate how much storage
      | it takes up (UndoableAction::getSizeInUnits()),
      | so this lets you specify the maximum
      | total number of units that the undomanager
      | is allowed to keep in memory before letting
      | the older actions drop off the end of
      | the list.
      | ----------
      | @param minimumTransactionsToKeep
      | 
      | this specifies the minimum number of
      | transactions that will be kept, even
      | if this involves exceeding the amount
      | of space specified in maxNumberOfUnitsToKeep
      | @see getNumberOfUnitsTakenUpByStoredCommands
      |
      */
    pub fn set_max_number_of_stored_units(&mut self, 
        max_units:        i32,
        min_transactions: i32)  {
        
        todo!();
        /*
            maxNumUnitsToKeep          = jmax (1, maxUnits);
        minimumTransactionsToKeep  = jmax (1, minTransactions);
        */
    }

    /**
      | Performs an action and also gives it
      | a name.
      | 
      | -----------
      | @param action
      | 
      | the action to perform - this object will
      | be deleted by the UndoManager when no
      | longer needed
      | ----------
      | @param actionName
      | 
      | if this string is non-empty, the current
      | transaction will be given this name;
      | if it's empty, the current transaction
      | name will be left unchanged. See setCurrentTransactionName()
      | 
      | -----------
      | @return
      | 
      | true if the command succeeds - see UndoableAction::perform
      | @see beginNewTransaction
      |
      */
    pub fn perform_with_action_name(
        &mut self, 
        new_action:  Box<dyn UndoableAction>,
        action_name: &String

    ) -> bool {
        
        todo!();
        /*
            if (perform (newAction))
        {
            if (actionName.isNotEmpty())
                setCurrentTransactionName (actionName);

            return true;
        }

        return false;
        */
    }

    /**
      | Performs an action and adds it to the
      | undo history list.
      | 
      | -----------
      | @param action
      | 
      | the action to perform - this object will
      | be deleted by the UndoManager when no
      | longer needed
      | 
      | -----------
      | @return
      | 
      | true if the command succeeds - see UndoableAction::perform
      | @see beginNewTransaction
      |
      */
    pub fn perform(
        &mut self, 
        new_action: Box<dyn UndoableAction>

    ) -> bool {
        
        todo!();
        /*
            if (newAction != nullptr)
        {
            std::unique_ptr<UndoableAction> action (newAction);

            if (isPerformingUndoRedo())
            {
                jassertfalse;  // Don't call perform() recursively from the UndoableAction::perform()
                               // or undo() methods, or else these actions will be discarded!
                return false;
            }

            if (action->perform())
            {
                auto* actionSet = getCurrentSet();

                if (actionSet != nullptr && ! newTransaction)
                {
                    if (auto* lastAction = actionSet->actions.getLast())
                    {
                        if (auto coalescedAction = lastAction->createCoalescedAction (action.get()))
                        {
                            action.reset (coalescedAction);
                            totalUnitsStored -= lastAction->getSizeInUnits();
                            actionSet->actions.removeLast();
                        }
                    }
                }
                else
                {
                    actionSet = new UndoManagerActionSet (newTransactionName);
                    transactions.insert (nextIndex, actionSet);
                    ++nextIndex;
                }

                totalUnitsStored += action->getSizeInUnits();
                actionSet->actions.add (std::move (action));
                newTransaction = false;

                moveFutureTransactionsToStash();
                dropOldTransactionsIfTooLarge();
                sendChangeMessage();
                return true;
            }
        }

        return false;
        */
    }

    pub fn move_future_transactions_to_stash(&mut self)  {
        
        todo!();
        /*
            if (nextIndex < transactions.size())
        {
            stashedFutureTransactions.clear();

            while (nextIndex < transactions.size())
            {
                auto* removed = transactions.removeAndReturn (nextIndex);
                stashedFutureTransactions.add (removed);
                totalUnitsStored -= removed->getTotalSize();
            }
        }
        */
    }

    pub fn restore_stashed_future_transactions(&mut self)  {
        
        todo!();
        /*
            while (nextIndex < transactions.size())
        {
            totalUnitsStored -= transactions.getUnchecked (nextIndex)->getTotalSize();
            transactions.remove (nextIndex);
        }

        for (auto* stashed : stashedFutureTransactions)
        {
            transactions.add (stashed);
            totalUnitsStored += stashed->getTotalSize();
        }

        stashedFutureTransactions.clearQuick (false);
        */
    }

    pub fn drop_old_transactions_if_too_large(&mut self)  {
        
        todo!();
        /*
            while (nextIndex > 0
                && totalUnitsStored > maxNumUnitsToKeep
                && transactions.size() > minimumTransactionsToKeep)
        {
            totalUnitsStored -= transactions.getFirst()->getTotalSize();
            transactions.remove (0);
            --nextIndex;

            // if this fails, then some actions may not be returning
            // consistent results from their getSizeInUnits() method
            jassert (totalUnitsStored >= 0);
        }
        */
    }

    /**
      | Starts a new group of actions that together
      | will be treated as a single transaction.
      | 
      | All actions that are passed to the perform()
      | method between calls to this method
      | are grouped together and undone/redone
      | together by a single call to undo() or
      | redo().
      |
      */
    pub fn begin_new_transaction(&mut self)  {
        
        todo!();
        /*
            beginNewTransaction ({});
        */
    }

    /**
      | Starts a new group of actions that together
      | will be treated as a single transaction.
      | 
      | All actions that are passed to the perform()
      | method between calls to this method
      | are grouped together and undone/redone
      | together by a single call to undo() or
      | redo().
      | 
      | -----------
      | @param actionName
      | 
      | a description of the transaction that
      | is about to be performed
      |
      */
    pub fn begin_new_transaction_with_action_name(&mut self, action_name: &String)  {
        
        todo!();
        /*
            newTransaction = true;
        newTransactionName = actionName;
        */
    }

    /**
      | Changes the name stored for the current
      | transaction.
      | 
      | Each transaction is given a name when
      | the beginNewTransaction() method
      | is called, but this can be used to change
      | that name without starting a new transaction.
      |
      */
    pub fn set_current_transaction_name(&mut self, new_name: &String)  {
        
        todo!();
        /*
            if (newTransaction)
            newTransactionName = newName;
        else if (auto* action = getCurrentSet())
            action->name = newName;
        */
    }

    /**
      | Returns the name of the current transaction.
      | @see setCurrentTransactionName
      |
      */
    pub fn get_current_transaction_name(&self) -> String {
        
        todo!();
        /*
            if (auto* action = getCurrentSet())
            return action->name;

        return newTransactionName;
        */
    }

    /**
      | Returns true if the caller code is in
      | the middle of an undo or redo action.
      |
      */
    pub fn is_performing_undo_redo(&self) -> bool {
        
        todo!();
        /*
            return isInsideUndoRedoCall;
        */
    }

    /**
      | Returns true if there's at least one
      | action in the list to undo. @see getUndoDescription,
      | undo, canRedo
      |
      */
    pub fn can_undo(&self) -> bool {
        
        todo!();
        /*
            return getCurrentSet() != nullptr;
        */
    }

    /**
      | Returns true if there's at least one
      | action in the list to redo. @see getRedoDescription,
      | redo, canUndo
      |
      */
    pub fn can_redo(&self) -> bool {
        
        todo!();
        /*
            return getNextSet()    != nullptr;
        */
    }
    
    /**
      | Tries to roll-back the last transaction.
      | 
      | -----------
      | @return
      | 
      | true if the transaction can be undone,
      | and false if it fails, or if there aren't
      | any transactions to undo @see undoCurrentTransactionOnly
      |
      */
    pub fn undo(&mut self) -> bool {
        
        todo!();
        /*
            if (auto* s = getCurrentSet())
        {
            const ScopedValueSetter<bool> setter (isInsideUndoRedoCall, true);

            if (s->undo())
                --nextIndex;
            else
                clearUndoHistory();

            beginNewTransaction();
            sendChangeMessage();
            return true;
        }

        return false;
        */
    }
    
    /**
      | Tries to redo the last transaction that
      | was undone.
      | 
      | -----------
      | @return
      | 
      | true if the transaction can be redone,
      | and false if it fails, or if there aren't
      | any transactions to redo
      |
      */
    pub fn redo(&mut self) -> bool {
        
        todo!();
        /*
            if (auto* s = getNextSet())
        {
            const ScopedValueSetter<bool> setter (isInsideUndoRedoCall, true);

            if (s->perform())
                ++nextIndex;
            else
                clearUndoHistory();

            beginNewTransaction();
            sendChangeMessage();
            return true;
        }

        return false;
        */
    }
    
    /**
      | Returns the name of the transaction
      | that will be rolled-back when undo()
      | is called. @see undo, canUndo, getUndoDescriptions
      |
      */
    pub fn get_undo_description(&self) -> String {
        
        todo!();
        /*
            if (auto* s = getCurrentSet())
            return s->name;

        return {};
        */
    }
    
    /**
      | Returns the name of the transaction
      | that will be redone when redo() is called.
      | @see redo, canRedo, getRedoDescriptions
      |
      */
    pub fn get_redo_description(&self) -> String {
        
        todo!();
        /*
            if (auto* s = getNextSet())
            return s->name;

        return {};
        */
    }
    
    /**
      | Returns the names of the sequence of
      | transactions that will be performed
      | if undo() is repeatedly called. Note
      | that for transactions where no name
      | was provided, the corresponding string
      | will be empty. @see undo, canUndo, getUndoDescription
      |
      */
    pub fn get_undo_descriptions(&self) -> Vec<String> {
        
        todo!();
        /*
            StringArray descriptions;

        for (int i = nextIndex;;)
        {
            if (auto* t = transactions[--i])
                descriptions.add (t->name);
            else
                return descriptions;
        }
        */
    }
    
    /**
      | Returns the names of the sequence of
      | transactions that will be performed
      | if redo() is repeatedly called. Note
      | that for transactions where no name
      | was provided, the corresponding string
      | will be empty. @see redo, canRedo, getRedoDescription
      |
      */
    pub fn get_redo_descriptions(&self) -> Vec<String> {
        
        todo!();
        /*
            StringArray descriptions;

        for (int i = nextIndex;;)
        {
            if (auto* t = transactions[i++])
                descriptions.add (t->name);
            else
                return descriptions;
        }
        */
    }
    
    /**
      | Returns the time to which the state would
      | be restored if undo() was to be called.
      | 
      | If an undo isn't currently possible,
      | it'll return Time().
      |
      */
    pub fn get_time_of_undo_transaction(&self) -> Time {
        
        todo!();
        /*
            if (auto* s = getCurrentSet())
            return s->time;

        return {};
        */
    }
    
    /**
      | Returns the time to which the state would
      | be restored if redo() was to be called.
      | 
      | If a redo isn't currently possible,
      | it'll return Time::getCurrentTime().
      | @see redo, canRedo
      |
      */
    pub fn get_time_of_redo_transaction(&self) -> Time {
        
        todo!();
        /*
            if (auto* s = getNextSet())
            return s->time;

        return Time::getCurrentTime();
        */
    }
    
    /**
      | Tries to roll-back any actions that
      | were added to the current transaction.
      | 
      | This will perform an undo() only if there
      | are some actions in the undo list that
      | were added after the last call to beginNewTransaction().
      | 
      | This is useful because it lets you call
      | beginNewTransaction(), then perform
      | an operation which may or may not actually
      | perform some actions, and then call
      | this method to get rid of any actions
      | that might have been done without it
      | rolling back the previous transaction
      | if nothing was actually done.
      | 
      | 
      | -----------
      | @return
      | 
      | true if any actions were undone.
      |
      */
    pub fn undo_current_transaction_only(&mut self) -> bool {
        
        todo!();
        /*
            if ((! newTransaction) && undo())
        {
            restoreStashedFutureTransactions();
            return true;
        }

        return false;
        */
    }
    
    /**
      | Returns a list of the UndoableAction
      | objects that have been performed during
      | the transaction that is currently open.
      | 
      | Effectively, this is the list of actions
      | that would be undone if undoCurrentTransactionOnly()
      | were to be called now.
      | 
      | The first item in the list is the earliest
      | action performed.
      |
      */
    pub fn get_actions_in_current_transaction(
        &self, 
        actions_found: &mut Vec<Box<dyn UndoableAction>>

    ) {
        
        todo!();
        /*
            if (! newTransaction)
            if (auto* s = getCurrentSet())
                for (auto* a : s->actions)
                    actionsFound.add (a);
        */
    }

    /**
      | Returns the number of UndoableAction
      | objects that have been performed during
      | the transaction that is currently open.
      | @see getActionsInCurrentTransaction
      |
      */
    pub fn get_num_actions_in_current_transaction(&self) -> i32 {
        
        todo!();
        /*
            if (! newTransaction)
            if (auto* s = getCurrentSet())
                return s->actions.size();

        return 0;
        */
    }
}
