crate::ix!();

pub struct PermissionsRequest {
    callback:   RuntimePermissionsCallback,
    permission: RuntimePermissionsPermissionID,
}

impl PermissionsRequest {

    /**
      | using "= default" on the following method
      | triggers an internal compiler error
      | in Android NDK 17
      |
      */
    pub fn new_from_permissions_request_ref(o: &PermissionsRequest) -> Self {
    
        todo!();
        /*
        : callback(o.callback),
        : permission(o.permission),
        */
    }
    
    pub fn new_from_permissions_request(o: PermissionsRequest) -> Self {
    
        todo!();
        /*
        : callback (std::move (o.callback)), permission (o.permission)
        o.permission = static_cast<RuntimePermissions::RuntimePermissionsPermissionID> (-1);
        */
    }
    
    pub fn new(
        callback_to_use:       RuntimePermissionsCallback,
        permission_to_request: RuntimePermissionsPermissionID) -> Self {
    
        todo!();
        /*
        : callback (std::move (callbackToUse)), permission (permissionToRequest)
        */
    }
    
    pub fn assign_from_ref(&mut self, o: &PermissionsRequest) -> &mut PermissionsRequest {
        
        todo!();
        /*
            callback   = o.callback;
            permission = o.permission;
            return *this;
        */
    }
    
    pub fn assign_from(&mut self, o: PermissionsRequest) -> &mut PermissionsRequest {
        
        todo!();
        /*
            callback   = std::move (o.callback);
            permission = o.permission;
            return *this;
        */
    }
}
