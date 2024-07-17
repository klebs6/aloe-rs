crate::ix!();

#[cfg(target_os="android")]
pub struct SystemJavaClassComparator {

}

#[cfg(target_os="android")]
impl SystemJavaClassComparator {

    pub fn compare_elements(
        first:  *mut JNIClassBase,
        second: *mut JNIClassBase) -> i32 {
        
        todo!();
        /*
            auto isSysClassA = isSystemClass (first);
            auto isSysClassB = isSystemClass (second);

            if ((! isSysClassA) && (! isSysClassB))
            {
                return DefaultElementComparator<bool>::compareElements (first != nullptr  ? first->byteCode  != nullptr : false,
                                                                        second != nullptr ? second->byteCode != nullptr : false);
            }

            return DefaultElementComparator<bool>::compareElements (isSystemClass (first),
                                                                    isSystemClass (second));
        */
    }
    
    pub fn is_system_class(cls: *mut JNIClassBase) -> bool {
        
        todo!();
        /*
            if (cls == nullptr)
                return false;

            String path (cls->getClassPath());

            return path.startsWith ("java/")
                || path.startsWith ("android/")
                || path.startsWith ("dalvik/");
        */
    }
}
