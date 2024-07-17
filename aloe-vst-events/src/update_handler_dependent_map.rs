crate::ix!();

pub type UpdateHandlerDeferedChangeList          = VecDeque<UpdateHandlerDeferedChange>;
pub type UpdateHandlerUpdateDataList             = VecDeque<UpdateHandlerUpdateData>;

///---------------------------
#[cfg(CLASS_NAME_TRACKED)]      pub type UpdateHandlerDependentList = Vec<UpdateHandlerDependency>;
#[cfg(not(CLASS_NAME_TRACKED))] pub type UpdateHandlerDependentList = Vec<*mut dyn IDependent>;

#[cfg(SMTG_CPP11_STDLIBSUPPORT)]
pub type UpdateHandlerDependentMap = HashMap<*const dyn FUnknown,UpdateHandlerDependentList>;

#[cfg(not(SMTG_CPP11_STDLIBSUPPORT))]
pub type UpdateHandlerDependentMap = HashMap<*const dyn FUnknown,UpdateHandlerDependentList>;

///---------------------------
pub struct UpdateHandlerTable
{
    dep_map:     [UpdateHandlerDependentMap; UPDATE_K_HASH_SIZE],
    defered:     UpdateHandlerDeferedChangeList,
    update_data: UpdateHandlerUpdateDataList,
}
