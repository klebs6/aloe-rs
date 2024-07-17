crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/PolyShapes.h]

/**
   This tests stacking. It also shows how to use
   b2World::Query and b2TestOverlap.
  */
pub const k_maxBodies: usize = 256;

pub const POLY_SHAPES_CALLBACK_E_MAXCOUNT: usize = 4;

/**
  | This callback is called by
  | b2World::QueryAABB. We find all the fixtures
  | that overlap an AABB. Of those, we use
  | b2TestOverlap to determine which fixtures
  | overlap a circle. Up to 4 overlapped fixtures
  | will be highlighted with a yellow border.
  */
pub struct PolyShapesCallback {
    circle:     b2CircleShape,
    transform:  b2Transform,
    debug_draw: *mut b2Draw,
    count:      i32,
}

impl b2QueryCallback for PolyShapesCallback {

    /**
      | Called for each fixture found in the
      | query AABB.
      | 
      | -----------
      | @return
      | 
      | false to terminate the query.
      |
      */
    fn report_fixture(&mut self, fixture: *mut b2Fixture) -> bool {
        
        todo!();
        /*
            if (m_count == POLY_SHAPES_CALLBACK_E_MAXCOUNT)
            {
                return false;
            }

            b2Body* body = fixture->GetBody();
            b2Shape* shape = fixture->GetShape();

            bool overlap = b2TestOverlap(shape, 0, &m_circle, 0, body->GetTransform(), m_transform);

            if (overlap)
            {
                DrawFixture(fixture);
                ++m_count;
            }

            return true;
        */
    }
}

impl Default for PolyShapesCallback {
    
    fn default() -> Self {
        todo!();
        /*


            m_count = 0
        */
    }
}

impl PolyShapesCallback {

    pub fn draw_fixture(&mut self, fixture: *mut b2Fixture)  {
        
        todo!();
        /*
            b2Color color(0.95f, 0.95f, 0.6f);
            const b2Transform& xf = fixture->GetBody()->GetTransform();

            switch (fixture->GetType())
            {
            case b2Shape::e_circle:
                {
                    b2CircleShape* circle = (b2CircleShape*)fixture->GetShape();

                    b2Vec2 center = b2Mul(xf, circle->m_p);
                    float32 radius = circle->m_radius;

                    m_debugDraw->DrawCircle(center, radius, color);
                }
                break;

            case b2Shape::e_polygon:
                {
                    b2PolygonShape* poly = (b2PolygonShape*)fixture->GetShape();
                    int32 vertexCount = poly->m_vertexCount;
                    b2Assert(vertexCount <= b2_maxPolygonVertices);
                    b2Vec2 vertices[b2_maxPolygonVertices];

                    for (int32 i = 0; i < vertexCount; ++i)
                    {
                        vertices[i] = b2Mul(xf, poly->m_vertices[i]);
                    }

                    m_debugDraw->DrawPolygon(vertices, vertexCount, color);
                }
                break;

            default:
                break;
            }
        */
    }

}

///---------------------
pub struct PolyShapes {
    base:       Test,
    body_index: i32,
    bodies:     *mut [b2Body; k_maxBodies],
    polygons:   [b2PolygonShape; 4],
    circle:     b2CircleShape,
}

impl Default for PolyShapes {
    
    fn default() -> Self {
        todo!();
        /*
            // Ground body
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            {
                b2Vec2 vertices[3];
                vertices[0].Set(-0.5f, 0.0f);
                vertices[1].Set(0.5f, 0.0f);
                vertices[2].Set(0.0f, 1.5f);
                m_polygons[0].Set(vertices, 3);
            }

            {
                b2Vec2 vertices[3];
                vertices[0].Set(-0.1f, 0.0f);
                vertices[1].Set(0.1f, 0.0f);
                vertices[2].Set(0.0f, 1.5f);
                m_polygons[1].Set(vertices, 3);
            }

            {
                float32 w = 1.0f;
                float32 b = w / (2.0f + b2Sqrt(2.0f));
                float32 s = b2Sqrt(2.0f) * b;

                b2Vec2 vertices[8];
                vertices[0].Set(0.5f * s, 0.0f);
                vertices[1].Set(0.5f * w, b);
                vertices[2].Set(0.5f * w, b + s);
                vertices[3].Set(0.5f * s, w);
                vertices[4].Set(-0.5f * s, w);
                vertices[5].Set(-0.5f * w, b + s);
                vertices[6].Set(-0.5f * w, b);
                vertices[7].Set(-0.5f * s, 0.0f);

                m_polygons[2].Set(vertices, 8);
            }

            {
                m_polygons[3].SetAsBox(0.5f, 0.5f);
            }

            {
                m_circle.m_radius = 0.5f;
            }

            m_bodyIndex = 0;
            memset(m_bodies, 0, sizeof(m_bodies))
        */
    }
}

impl PolyShapes {
    
    pub fn create(&mut self, index: i32)  {
        
        todo!();
        /*
            if (m_bodies[m_bodyIndex] != NULL)
            {
                m_world->DestroyBody(m_bodies[m_bodyIndex]);
                m_bodies[m_bodyIndex] = NULL;
            }

            b2BodyDef bd;
            bd.type = b2_dynamicBody;

            float32 x = RandomFloat(-2.0f, 2.0f);
            bd.position.Set(x, 10.0f);
            bd.angle = RandomFloat(-b2_pi, b2_pi);

            if (index == 4)
            {
                bd.angularDamping = 0.02f;
            }

            m_bodies[m_bodyIndex] = m_world->CreateBody(&bd);

            if (index < 4)
            {
                b2FixtureDef fd;
                fd.shape = m_polygons + index;
                fd.density = 1.0f;
                fd.friction = 0.3f;
                m_bodies[m_bodyIndex]->CreateFixture(&fd);
            }
            else
            {
                b2FixtureDef fd;
                fd.shape = &m_circle;
                fd.density = 1.0f;
                fd.friction = 0.3f;

                m_bodies[m_bodyIndex]->CreateFixture(&fd);
            }

            m_bodyIndex = (m_bodyIndex + 1) % k_maxBodies;
        */
    }
    
    pub fn destroy_body(&mut self)  {
        
        todo!();
        /*
            for (int32 i = 0; i < k_maxBodies; ++i)
            {
                if (m_bodies[i] != NULL)
                {
                    m_world->DestroyBody(m_bodies[i]);
                    m_bodies[i] = NULL;
                    return;
                }
            }
        */
    }
    
    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case '1':
            case '2':
            case '3':
            case '4':
            case '5':
                Create(key - '1');
                break;

            case 'a':
                for (int32 i = 0; i < k_maxBodies; i += 2)
                {
                    if (m_bodies[i])
                    {
                        bool active = m_bodies[i]->IsActive();
                        m_bodies[i]->SetActive(!active);
                    }
                }
                break;

            case 'd':
                DestroyBody();
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            PolyShapesCallback callback;
            callback.m_circle.m_radius = 2.0f;
            callback.m_circle.m_p.Set(0.0f, 1.1f);
            callback.m_transform.SetIdentity();
            callback.m_debugDraw = &m_debugDraw;

            b2AABB aabb;
            callback.m_circle.ComputeAABB(&aabb, callback.m_transform, 0);

            m_world->QueryAABB(&callback, aabb);

            b2Color color(0.4f, 0.7f, 0.8f);
            m_debugDraw.DrawCircle(callback.m_circle.m_p, callback.m_circle.m_radius, color);

            m_debugDraw.DrawString(5, m_textLine, "Press 1-5 to drop stuff");
            m_textLine += 15;
            m_debugDraw.DrawString(5, m_textLine, "Press 'a' to (de)activate some bodies");
            m_textLine += 15;
            m_debugDraw.DrawString(5, m_textLine, "Press 'd' to destroy a body");
            m_textLine += 15;
        */
    }
    
    pub fn create_default() -> *mut Test {
        
        todo!();
        /*
            return new PolyShapes;
        */
    }
}
