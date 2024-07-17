crate::ix!();

pub struct WavefrontObjFileIndexMap {
    map: HashMap<WavefrontObjFileTripleIndex,WavefrontObjFileIndex>,
}

impl WavefrontObjFileIndexMap {

    pub fn get_index_for(&mut self, 
        i:        WavefrontObjFileTripleIndex,
        new_mesh: &mut WavefrontObjFileMesh,
        src_mesh: &WavefrontObjFileMesh) -> WavefrontObjFileIndex {
        
        todo!();
        /*
            const std::map<WavefrontObjFileTripleIndex, WavefrontObjFileIndex>::iterator it (map.find (i));

                if (it != map.end())
                    return it->second;

                auto index = (WavefrontObjFileIndex) newMesh.vertices.size();

                if (isPositiveAndBelow (i.vertexIndex, srcMesh.vertices.size()))
                    newMesh.vertices.add (srcMesh.vertices.getReference (i.vertexIndex));

                if (isPositiveAndBelow (i.normalIndex, srcMesh.normals.size()))
                    newMesh.normals.add (srcMesh.normals.getReference (i.normalIndex));

                if (isPositiveAndBelow (i.textureIndex, srcMesh.textureCoords.size()))
                    newMesh.textureCoords.add (srcMesh.textureCoords.getReference (i.textureIndex));

                map[i] = index;
                return index;
        */
    }
}

