crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/WavefrontObjParser.h]

/**
  | This is a quick-and-dirty parser for
  | the 3D OBJ file format.
  | 
  | Just call load() and if there aren't
  | any errors, the 'shapes' array should
  | be filled with all the shape objects
  | that were loaded from the file.
  |
  */
#[no_copy]
#[leak_detector]
pub struct WavefrontObjFile {
    shapes:      Vec<Box<WavefrontObjFileShape>>,
    source_file: File,
}

impl WavefrontObjFile {
    
    pub fn load(&mut self, obj_file_content: &String) -> Result<(),()> {
        
        todo!();
        /*
            shapes.clear();
            return parseObjFile (StringArray::fromLines (objFileContent));
        */
    }
    
    pub fn load_from_file(&mut self, file: &File) -> Result<(),()> {
        
        todo!();
        /*
            sourceFile = file;
            return load (file.loadFileAsString());
        */
    }
    
    pub fn parse_float(t: &mut CharPointerType) -> f32 {
        
        todo!();
        /*
            t.incrementToEndOfWhitespace();
            return (float) CharacterFunctions::readDoubleValue (t);
        */
    }
    
    pub fn parse_vertex(t: CharPointerType) -> WavefrontObjFileVertex {
        
        todo!();
        /*
            WavefrontObjFileVertex v;
            v.x = parseFloat (t);
            v.y = parseFloat (t);
            v.z = parseFloat (t);
            return v;
        */
    }
    
    pub fn parse_texture_coord(t: CharPointerType) -> WavefrontObjFileTextureCoord {
        
        todo!();
        /*
            WavefrontObjFileTextureCoord tc;
            tc.x = parseFloat (t);
            tc.y = parseFloat (t);
            return tc;
        */
    }
    
    pub fn match_token(
        t:     &mut CharPointerType,
        token: *const u8) -> bool {
        
        todo!();
        /*
            auto len = (int) strlen (token);

            if (CharacterFunctions::compareUpTo (CharPointer_ASCII (token), t, len) == 0)
            {
                auto end = t + len;

                if (end.isEmpty() || end.isWhitespace())
                {
                    t = end.findEndOfWhitespace();
                    return true;
                }
            }

            return false;
        */
    }
    
    pub fn parse_face_group(
        src_mesh:   &WavefrontObjFileMesh,
        face_group: &mut Vec<WavefrontObjFileFace>,
        material:   &WavefrontObjFileMaterial,
        name:       &String) -> *mut WavefrontObjFileShape {
        
        todo!();
        /*
            if (faceGroup.size() == 0)
                return nullptr;

            std::unique_ptr<WavefrontObjFileShape> shape (new WavefrontObjFileShape());
            shape->name = name;
            shape->material = material;

            WavefrontObjFileIndexMap indexMap;

            for (auto& f : faceGroup)
                f.addIndices (shape->mesh, srcMesh, indexMap);

            return shape.release();
        */
    }
    
    pub fn parse_obj_file(&mut self, lines: &StringArray) -> Result<(),()> {
        
        todo!();
        /*
            WavefrontObjFileMesh mesh;
            Vec<WavefrontObjFileFace> faceGroup;

            Vec<WavefrontObjFileMaterial> knownMaterials;
            WavefrontObjFileMaterial lastMaterial;
            String lastName;

            for (auto lineNum = 0; lineNum < lines.size(); ++lineNum)
            {
                auto l = lines[lineNum].getCharPointer().findEndOfWhitespace();

                if (matchToken (l, "v"))    { mesh.vertices     .add (parseVertex (l));       continue; }
                if (matchToken (l, "vn"))   { mesh.normals      .add (parseVertex (l));       continue; }
                if (matchToken (l, "vt"))   { mesh.textureCoords.add (parseTextureCoord (l)); continue; }
                if (matchToken (l, "f"))    { faceGroup         .add (WavefrontObjFileFace (l));              continue; }

                if (matchToken (l, "usemtl"))
                {
                    auto name = String (l).trim();

                    for (auto i = knownMaterials.size(); --i >= 0;)
                    {
                        if (knownMaterials.getReference (i).name == name)
                        {
                            lastMaterial = knownMaterials.getReference (i);
                            break;
                        }
                    }

                    continue;
                }

                if (matchToken (l, "mtllib"))
                {
                    auto r = parseMaterial (knownMaterials, String (l).trim());
                    continue;
                }

                if (matchToken (l, "g") || matchToken (l, "o"))
                {
                    if (auto* shape = parseFaceGroup (mesh, faceGroup, lastMaterial, lastName))
                        shapes.add (shape);

                    faceGroup.clear();
                    lastName = StringArray::fromTokens (l, " \t", "")[0];
                    continue;
                }
            }

            if (auto* shape = parseFaceGroup (mesh, faceGroup, lastMaterial, lastName))
                shapes.add (shape);

            return Result::ok();
        */
    }
    
    pub fn parse_material(&mut self, 
        materials: &mut Vec<WavefrontObjFileMaterial>,
        filename:  &String) -> Result<(),()> {
        
        todo!();
        /*
            jassert (sourceFile.exists());
            auto f = sourceFile.getSiblingFile (filename);

            if (! f.exists())
                return Result::fail ("Cannot open file: " + filename);

            auto lines = StringArray::fromLines (f.loadFileAsString());

            materials.clear();
            WavefrontObjFileMaterial material;

            for (auto line : lines)
            {
                auto l = line.getCharPointer().findEndOfWhitespace();

                if (matchToken (l, "newmtl"))   { materials.add (material); material.name = String (l).trim(); continue; }

                if (matchToken (l, "Ka"))       { material.ambient         = parseVertex (l); continue; }
                if (matchToken (l, "Kd"))       { material.diffuse         = parseVertex (l); continue; }
                if (matchToken (l, "Ks"))       { material.specular        = parseVertex (l); continue; }
                if (matchToken (l, "Kt"))       { material.transmittance   = parseVertex (l); continue; }
                if (matchToken (l, "Ke"))       { material.emission        = parseVertex (l); continue; }
                if (matchToken (l, "Ni"))       { material.refractiveIndex = parseFloat (l);  continue; }
                if (matchToken (l, "Ns"))       { material.shininess       = parseFloat (l);  continue; }

                if (matchToken (l, "map_Ka"))   { material.ambientTextureName  = String (l).trim(); continue; }
                if (matchToken (l, "map_Kd"))   { material.diffuseTextureName  = String (l).trim(); continue; }
                if (matchToken (l, "map_Ks"))   { material.specularTextureName = String (l).trim(); continue; }
                if (matchToken (l, "map_Ns"))   { material.normalTextureName   = String (l).trim(); continue; }

                auto tokens = StringArray::fromTokens (l, " \t", "");

                if (tokens.size() >= 2)
                    material.parameters.set (tokens[0].trim(), tokens[1].trim());
            }

            materials.add (material);
            return Result::ok();
        */
    }
}
