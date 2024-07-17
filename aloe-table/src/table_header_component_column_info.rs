crate::ix!();

pub struct TableHeaderComponentColumnInfo
{
    name:                  String,
    id:                    i32,
    property_flags:        i32,
    width:                 i32,
    minimum_width:         i32,
    maximum_width:         i32,
    last_deliberate_width: f64,
}

impl TableHeaderComponentColumnInfo {
    
    pub fn is_visible(&self) -> bool {
        
        todo!();
        /*
            return (propertyFlags & TableHeaderComponent::visible) != 0;
        */
    }
}

