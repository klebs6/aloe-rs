crate::ix!();

pub fn perform_remove<T, F>(
    str_:        *mut T,
    length:      u32,
    func:        F,
    func_result: bool) -> u32 {

    todo!();
        /*
            T* p = str;

        while (*p)
        {
            if ((func (*p) != 0) == funcResult)
            {
                size_t toMove = length - (p - str);
                memmove (p, p + 1, toMove * sizeof (T));
                length--;
            }
            else
                p++;
        }
        return length;
        */
}

pub fn perform_trim<T, F>(
    str_:        *mut T,
    length:      u32,
    func:        F,
    func_result: bool) -> u32 {

    todo!();
    /*
            uint32 toRemoveAtHead = 0;
        uint32 toRemoveAtTail = 0;

        T* p = str;

        while ((*p) && ((func (*p) != 0) == funcResult))
            p++;

        toRemoveAtHead = static_cast<uint32> (p - str);

        if (toRemoveAtHead < length)
        {
            p = str + length - 1;

            while (((func (*p) != 0) == funcResult) && (p > str))
            {
                p--;
                toRemoveAtTail++;
            }
        }

        uint32 newLength = length - (toRemoveAtHead + toRemoveAtTail);
        if (newLength != length)
        {
            if (toRemoveAtHead)
                memmove (str, str + toRemoveAtHead, newLength * sizeof (T));
        }
        return newLength;
        */
}

pub fn perform_replace<T>(
    str_:          *mut T,
    to_replace:    *const T,
    to_replace_by: T) -> bool {

    todo!();
        /*
            bool anyReplace = false;
        T* p = str;
        while (*p)
        {
            const T* rep = toReplace;
            while (*rep)
            {
                if (*p == *rep)
                {
                    *p = toReplaceBy;
                    anyReplace = true;
                    break;
                }
                rep++;
            }
            p++;
        }
        return anyReplace;
        */
}

pub fn perform_remove_chars<T>(
    str_:      *mut T,
    length:    u32,
    to_remove: *const T) -> u32 {

    todo!();
        /*
            T* p = str;

        while (*p)
        {
            bool found = false;
            const T* rem = toRemove;
            while (*rem)
            {
                if (*p == *rem)
                {
                    found = true;
                    break;
                }
                rem++;
            }

            if (found)
            {
                size_t toMove = length - (p - str);
                memmove (p, p + 1, toMove * sizeof (T));
                length--;
            }
            else
                p++;
        }
        return length;
        */
}
