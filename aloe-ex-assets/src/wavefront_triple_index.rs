crate::ix!();

#[derive(Eq,PartialEq,Default)]
pub struct WavefrontObjFileTripleIndex {
    vertex_index:  i32, // default = -1
    texture_index: i32, // default = -1
    normal_index:  i32, // default = -1
}

impl Ord for WavefrontObjFileTripleIndex {
    
    #[inline] fn cmp(&self, other: &WavefrontObjFileTripleIndex) -> std::cmp::Ordering {
        todo!();
        /*
            if (this == &other)
                    return false;

                if (vertexIndex != other.vertexIndex)
                    return vertexIndex < other.vertexIndex;

                if (textureIndex != other.textureIndex)
                    return textureIndex < other.textureIndex;

                return normalIndex < other.normalIndex;
        */
    }
}

impl PartialOrd<WavefrontObjFileTripleIndex> for WavefrontObjFileTripleIndex {

    #[inline] fn partial_cmp(&self, other: &WavefrontObjFileTripleIndex) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
