crate::ix!();

pub struct WavefrontObjFileFace {
    triples: Vec<WavefrontObjFileTripleIndex>,
}

impl WavefrontObjFileFace {

    pub fn new(t: CharPointerType) -> Self {
    
        todo!();
        /*


            while (! t.isEmpty())
                    triples.add (parseTriple (t));
        */
    }
    
    pub fn add_indices(&mut self, 
        new_mesh:  &mut WavefrontObjFileMesh,
        src_mesh:  &WavefrontObjFileMesh,
        index_map: &mut WavefrontObjFileIndexMap)  {
        
        todo!();
        /*
            WavefrontObjFileTripleIndex i0 (triples[0]), i1, i2 (triples[1]);

                for (auto i = 2; i < triples.size(); ++i)
                {
                    i1 = i2;
                    i2 = triples.getReference (i);

                    newMesh.indices.add (indexMap.getIndexFor (i0, newMesh, srcMesh));
                    newMesh.indices.add (indexMap.getIndexFor (i1, newMesh, srcMesh));
                    newMesh.indices.add (indexMap.getIndexFor (i2, newMesh, srcMesh));
                }
        */
    }
    
    pub fn parse_triple(t: &mut CharPointerType) -> WavefrontObjFileTripleIndex {
        
        todo!();
        /*
            WavefrontObjFileTripleIndex i;

                t.incrementToEndOfWhitespace();
                i.vertexIndex = t.getIntValue32() - 1;
                t = findEndOfFaceToken (t);

                if (t.isEmpty() || t.getAndAdvance() != '/')
                    return i;

                if (*t == '/')
                {
                    ++t;
                }
                else
                {
                    i.textureIndex = t.getIntValue32() - 1;
                    t = findEndOfFaceToken (t);

                    if (t.isEmpty() || t.getAndAdvance() != '/')
                        return i;
                }

                i.normalIndex = t.getIntValue32() - 1;
                t = findEndOfFaceToken (t);
                return i;
        */
    }
    
    pub fn find_end_of_face_token(t: CharPointerType) -> CharPointerType {
        
        todo!();
        /*
            return CharacterFunctions::findEndOfToken (t, CharPointer_ASCII ("/ \t"), String().getCharPointer());
        */
    }
}
