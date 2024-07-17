crate::ix!();

#[derive(PartialEq,Eq)]
pub struct WavefrontObjFileTripleIndex {
    vertex_index:  i32,
    texture_index: i32,
    normal_index:  i32,
}

impl Default for WavefrontObjFileTripleIndex {

    fn default() -> Self {

        Self {
            vertex_index:  -1,
            texture_index: -1,
            normal_index:  -1,
        }
    }
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
