crate::ix!();

pub struct UniqueBase<ClassType> {
    _p0: PhantomData<ClassType>,
}

pub struct SharedBase<CommonClassType,SourceClassType> {
    _p0: PhantomData<(CommonClassType,SourceClassType)>,
}

pub fn test_for_with_shared_base<ToTest, CommonClassType, SourceClassType>(
        to_test:   &mut ToTest,
        targetiid: TUID,
        _2:        SharedBase<CommonClassType,SourceClassType>) -> InterfaceResultWithDeferredAddRef {

    todo!();
        /*
            if (! doUIDsMatch (targetIID, CommonClassType::iid))
            return {};

        return { typename kResultOk, static_cast<CommonClassType*> (static_cast<SourceClassType*> (std::addressof (toTest))) };
        */
}

pub fn test_for<ToTest, ClassType>(
        to_test:   &mut ToTest,
        targetiid: TUID,
        _2:        UniqueBase<ClassType>) -> InterfaceResultWithDeferredAddRef {

    todo!();
        /*
            return testFor (toTest, targetIID, SharedBase<ClassType, ClassType>{});
        */
}

pub fn test_for_multiple<ToTest>(
        _0: &mut ToTest,
        _1: TUID) -> InterfaceResultWithDeferredAddRef {

    todo!();
        /*
            return {};
        */
}

pub fn test_for_multiple_with_head_and_tail<ToTest, Head, Tail>(
        to_test:   &mut ToTest,
        targetiid: TUID,
        head:      Head,
        tail:      Tail) -> InterfaceResultWithDeferredAddRef {

    todo!();
        /*
            const auto result = testFor (toTest, targetIID, head);

        if (result.isOk())
            return result;

        return testForMultiple (toTest, targetIID, tail...);
        */
}
