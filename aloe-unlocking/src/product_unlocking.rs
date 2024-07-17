crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/aloe_product_unlocking.h]

/**
  | The aloe_product_unlocking module
  | provides simple user-registration
  | classes for allowing you to build apps/plugins
  | with features that are unlocked by a
  | user having a suitable account on a webserver.
  | 
  | Although originally designed for use
  | with products that are sold on the
  | 
  | Tracktion Marketplace web-store,
  | the module itself is fully open, and
  | can be used to connect to your own web-store
  | instead, if you implement your own compatible
  | web-server back-end.
  | 
  | In additional, the module supports
  | in-app purchases both on iOS and Android
  | platforms.
  |
  */

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/aloe_product_unlocking.cpp]

pub const ALOE_CORE_INCLUDE_JNI_HELPERS: usize = 1;
pub const ALOE_CORE_INCLUDE_OBJC_HELPERS: usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS: usize = 1;

/**
  | Set this flag to 1 to use test servers
  | on iOS
  |
  */
#[cfg(not(ALOE_IN_APP_PURCHASES_USE_SANDBOX_ENVIRONMENT))]
pub const ALOE_IN_APP_PURCHASES_USE_SANDBOX_ENVIRONMENT: usize = 0;
