crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_LinkedListPointer.h]

/**
  | Helps to manipulate singly-linked lists of
  | objects.
  |
  | For objects that are designed to contain
  | a pointer to the subsequent item in the list,
  | this class contains methods to deal with the
  | list. To use it, the ObjectType class that it
  | points to must contain a LinkedListPointer
  | called nextListItem, e.g.
  |
  | @code
  | struct MyObject
  | {
  |     int x, y, z;
  |
  |     // A linkable object must contain a member
  |     // with this name and type, which must be
  |     // accessible by the LinkedListPointer
  |     // class. (This doesn't mean it has to be
  |     // public - you could make your class
  |     // a friend of
  |     // a LinkedListPointer<MyObject> instead).
  |     LinkedListPointer<MyObject> nextListItem;
  | };
  |
  | LinkedListPointer<MyObject> myList;
  | myList.append (new MyObject());
  | myList.append (new MyObject());
  |
  | int numItems = myList.size(); // returns 2
  | MyObject* lastInList = myList.getLast();
  | @endcode
  |
  | @tags{Core}
  */
#[no_copy]
pub struct LinkedListPointer<ObjectType> {
    item: *mut ObjectType,
}

pub mod linked_list_pointer {
    use super::*;

    /**
      | Allows efficient repeated insertions
      | into a list.
      | 
      | You can create an Appender object which
      | points to the last element in your list,
      | and then repeatedly call Appender::append()
      | to add items to the end of the list in O(1)
      | time.
      |
      */
    #[no_copy]
    pub struct Appender<ObjectType> {
        end_of_list: *mut LinkedListPointer<ObjectType>,
    }

    impl<ObjectType> Appender<ObjectType> {

        /**
          | Creates an appender which will add items
          | to the given list.
          |
          */
        pub fn new_from_end_of_list_pointer(end_of_list_pointer: &mut LinkedListPointer<ObjectType>) -> Self {
        
            todo!();
            /*
            : end_of_list(&endOfListPointer),

                // This can only be used to add to the end of a list.
                    jassert (endOfListPointer.item == nullptr);
            */
        }

        /**
          | Appends an item to the list.
          |
          */
        pub fn append(&mut self, new_item: *mut ObjectType)  {
            
            todo!();
            /*
                *endOfList = newItem;
                    endOfList = &(newItem->nextListItem);
            */
        }
    }
}
    
impl<ObjectType> Default for LinkedListPointer<ObjectType> {
    
    /**
      | Creates a null pointer to an empty list.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : item(nullptr),

        
        */
    }
}

impl<ObjectType> Index<i32> for LinkedListPointer<ObjectType> {

    type Output = LinkedListPointer<ObjectType>;
    
    /**
      | Returns the item at a given index in the
      | list.
      | 
      | Since the only way to find an item is to
      | iterate the list, this operation can
      | obviously be slow, depending on its
      | size, so you should be careful when using
      | this in algorithms.
      |
      */
    #[inline] fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            auto* l = this;

            while (--index >= 0 && l->item != nullptr)
                l = &(l->item->nextListItem);

            return *l;
        */
    }
}

impl<ObjectType> IndexMut<i32> for LinkedListPointer<ObjectType> {
    
    /**
      | Returns the item at a given index in the
      | list.
      | 
      | Since the only way to find an item is to
      | iterate the list, this operation can
      | obviously be slow, depending on its
      | size, so you should be careful when using
      | this in algorithms.
      |
      */
    #[inline] fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        todo!();
        /*
            auto* l = this;

            while (--index >= 0 && l->item != nullptr)
                l = &(l->item->nextListItem);

            return *l;
        */
    }
}

impl<ObjectType> LinkedListPointer<ObjectType> {

    /**
      | Creates a pointer to a list whose head
      | is the item provided.
      |
      */
    pub fn new_from_head_item(head_item: *mut ObjectType) -> Self {
    
        todo!();
        /*
        : item(headItem),

        
        */
    }

    /**
      | Sets this pointer to point to a new list.
      |
      */
    pub fn assign_from_new_item(&mut self, new_item: *mut ObjectType) -> &mut LinkedListPointer<ObjectType> {
        
        todo!();
        /*
            item = newItem;
            return *this;
        */
    }
    
    pub fn new_from_linked_list(other: LinkedListPointer<ObjectType>) -> Self {
    
        todo!();
        /*
        : item(other.item),

            other.item = nullptr;
        */
    }
    
    pub fn assign_from(&mut self, other: LinkedListPointer<ObjectType>) 
        -> &mut LinkedListPointer<ObjectType> 
    {
        todo!();
        /*
            jassert (this != &other); // hopefully the compiler should make this situation impossible!

            item = other.item;
            other.item = nullptr;
            return *this;
        */
    }

    /**
      | Returns the item which this pointer
      | points to.
      |
      */
    fn into(self) -> ObjectType {
        todo!();
        /*
            return item;
        */
    }

    /**
      | Returns the item which this pointer
      | points to.
      |
      */
    #[inline] pub fn get(&self) -> *mut ObjectType {
        
        todo!();
        /*
            return item;
        */
    }

    /**
      | Returns the last item in the list which
      | this pointer points to.
      | 
      | This will iterate the list and return
      | the last item found. Obviously the speed
      | of this operation will be proportional
      | to the size of the list. If the list is
      | empty the return value will be this object.
      | 
      | If you're planning on appending a number
      | of items to your list, it's much more
      | efficient to use the Appender class
      | than to repeatedly call getLast() to
      | find the end.
      |
      */
    pub fn get_last(&mut self) -> &mut LinkedListPointer<ObjectType> {
        
        todo!();
        /*
            auto* l = this;

            while (l->item != nullptr)
                l = &(l->item->nextListItem);

            return *l;
        */
    }

    /**
      | Returns the number of items in the list.
      | 
      | Obviously with a simple linked list,
      | getting the size involves iterating
      | the list, so this can be a lengthy operation
      | - be careful when using this method in
      | your code.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            int total = 0;

            for (auto* i = item; i != nullptr; i = i->nextListItem)
                ++total;

            return total;
        */
    }

    /**
      | Returns true if the list contains the
      | given item.
      |
      */
    pub fn contains(&self, item_to_look_for: *const ObjectType) -> bool {
        
        todo!();
        /*
            for (auto* i = item; i != nullptr; i = i->nextListItem)
                if (itemToLookFor == i)
                    return true;

            return false;
        */
    }

    /**
      | Inserts an item into the list, placing
      | it before the item that this pointer
      | currently points to.
      |
      */
    pub fn insert_next(&mut self, new_item: *mut ObjectType)  {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011)
            jassert (newItem != nullptr);
            jassert (newItem->nextListItem == nullptr);
            newItem->nextListItem = item;
            item = newItem;
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    /**
      | Inserts an item at a numeric index in
      | the list.
      | 
      | Obviously this will involve iterating
      | the list to find the item at the given
      | index, so be careful about the impact
      | this may have on execution time.
      |
      */
    pub fn insert_at_index(&mut self, 
        index:    i32,
        new_item: *mut ObjectType)  {
        
        todo!();
        /*
            jassert (newItem != nullptr);
            auto* l = this;

            while (index != 0 && l->item != nullptr)
            {
                l = &(l->item->nextListItem);
                --index;
            }

            l->insertNext (newItem);
        */
    }

    /**
      | Replaces the object that this pointer
      | points to, appending the rest of the
      | list to the new object, and returning
      | the old one.
      |
      */
    pub fn replace_next(&mut self, new_item: *mut ObjectType) -> *mut ObjectType {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011 28182)
            jassert (newItem != nullptr);
            jassert (newItem->nextListItem == nullptr);

            auto oldItem = item;
            item = newItem;
            item->nextListItem = oldItem->nextListItem.item;
            oldItem->nextListItem.item = nullptr;
            return oldItem;
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    /**
      | Adds an item to the end of the list.
      | 
      | This operation involves iterating
      | the whole list, so can be slow - if you
      | need to append a number of items to your
      | list, it's much more efficient to use
      | the Appender class than to repeatedly
      | call append().
      |
      */
    pub fn append(&mut self, new_item: *mut ObjectType)  {
        
        todo!();
        /*
            getLast().item = newItem;
        */
    }

    /**
      | Creates copies of all the items in another
      | list and adds them to this one.
      | 
      | This will use the ObjectType's copy
      | constructor to try to create copies
      | of each item in the other list, and appends
      | them to this list.
      |
      */
    pub fn add_copy_of_list(&mut self, other: &LinkedListPointer<ObjectType>)  {
        
        todo!();
        /*
            auto* insertPoint = this;

            for (auto* i = other.item; i != nullptr; i = i->nextListItem)
            {
                insertPoint->insertNext (new ObjectType (*i));
                insertPoint = &(insertPoint->item->nextListItem);
            }
        */
    }

    /**
      | Removes the head item from the list.
      | 
      | This won't delete the object that is
      | removed, but returns it, so the caller
      | can delete it if necessary.
      |
      */
    pub fn remove_next(&mut self) -> *mut ObjectType {
        
        todo!();
        /*
            auto oldItem = item;

            if (oldItem != nullptr)
            {
                item = oldItem->nextListItem;
                oldItem->nextListItem.item = nullptr;
            }

            return oldItem;
        */
    }

    /**
      | Removes a specific item from the list.
      | 
      | Note that this will not delete the item,
      | it simply unlinks it from the list.
      |
      */
    pub fn remove(&mut self, item_to_remove: *mut ObjectType)  {
        
        todo!();
        /*
            if (auto* l = findPointerTo (itemToRemove))
                l->removeNext();
        */
    }

    /**
      | Iterates the list, calling the delete
      | operator on all of its elements and leaving
      | this pointer empty.
      |
      */
    pub fn delete_all(&mut self)  {
        
        todo!();
        /*
            while (item != nullptr)
            {
                auto oldItem = item;
                item = oldItem->nextListItem;
                delete oldItem;
            }
        */
    }

    /**
      | Finds a pointer to a given item.
      | 
      | If the item is found in the list, this
      | returns the pointer that points to it.
      | If the item isn't found, this returns
      | null.
      |
      */
    pub fn find_pointer_to(&mut self, item_to_look_for: *mut ObjectType) -> *mut LinkedListPointer<ObjectType> {
        
        todo!();
        /*
            auto* l = this;

            while (l->item != nullptr)
            {
                if (l->item == itemToLookFor)
                    return l;

                l = &(l->item->nextListItem);
            }

            return nullptr;
        */
    }

    /**
      | Copies the items in the list to an array.
      | 
      | The destArray must contain enough elements
      | to hold the entire list - no checks are
      | made for this!
      |
      */
    pub fn copy_to_array(&self, dest_array: *mut *mut ObjectType)  {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011)
            jassert (destArray != nullptr);

            for (auto* i = item; i != nullptr; i = i->nextListItem)
                *destArray++ = i;

            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    /**
      | Swaps this pointer with another one
      |
      */
    pub fn swap_with(&mut self, other: &mut LinkedListPointer<ObjectType>)  {
        
        todo!();
        /*
            std::swap (item, other.item);
        */
    }
}
