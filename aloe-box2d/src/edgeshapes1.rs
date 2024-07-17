crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/EdgeShapes.h]

pub struct EdgeShapesCallback {
    fixture: *mut b2Fixture,
    point:   b2Vec2,
    normal:  b2Vec2,
}

impl b2RayCastCallback for EdgeShapesCallback {

    fn report_fixture(
        &mut self, 
        fixture:  *mut b2Fixture,
        point:    &b2Vec2,
        normal:   &b2Vec2,
        fraction: f32

    ) -> f32 {
        
        todo!();
        /*
            m_fixture = fixture;
            m_point = point;
            m_normal = normal;

            return fraction;
        */
    }
}

impl Default for EdgeShapesCallback {
    
    fn default() -> Self {
        todo!();
        /*
            m_fixture = NULL
        */
    }
}

///--------------------
pub struct EdgeShapes {
    base:       Test,
    body_index: i32,
    bodies:     *mut [b2Body; EDGE_SHAPES_MAX_BODIES],
    polygons:   [b2PolygonShape; 4],
    circle:     b2CircleShape,
    angle:      f32,
}

pub const EDGE_SHAPES_MAX_BODIES: usize = 256;

impl Default for EdgeShapes {
    
    fn default() -> Self {
        todo!();
        /*
            // Ground body
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                float32 x1 = -20.0f;
                float32 y1 = 2.0f * cosf(x1 / 10.0f * b2_pi);
                for (int32 i = 0; i < 80; ++i)
                {
                    float32 x2 = x1 + 0.5f;
                    float32 y2 = 2.0f * cosf(x2 / 10.0f * b2_pi);

                    b2EdgeShape shape;
                    shape.Set(b2Vec2(x1, y1), b2Vec2(x2, y2));
                    use super::*;

                    x1 = x2;
                    y1 = y2;
                }
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
            memset(m_bodies, 0, sizeof(m_bodies));

            m_angle = 0.0f
        */
    }
}

impl EdgeShapes {

    pub fn create(&mut self, index: i32)  {
        
        todo!();
        /*
            if (m_bodies[m_bodyIndex] != NULL)
            {
                m_world->DestroyBody(m_bodies[m_bodyIndex]);
                m_bodies[m_bodyIndex] = NULL;
            }

            b2BodyDef bd;

            float32 x = RandomFloat(-10.0f, 10.0f);
            float32 y = RandomFloat(10.0f, 20.0f);
            bd.position.Set(x, y);
            bd.angle = RandomFloat(-b2_pi, b2_pi);
            bd.type = b2_dynamicBody;

            if (index == 4)
            {
                bd.angularDamping = 0.02f;
            }

            m_bodies[m_bodyIndex] = m_world->CreateBody(&bd);

            if (index < 4)
            {
                b2FixtureDef fd;
                fd.shape = m_polygons + index;
                fd.friction = 0.3f;
                fd.density = 20.0f;
                m_bodies[m_bodyIndex]->CreateFixture(&fd);
            }
            else
            {
                b2FixtureDef fd;
                fd.shape = &m_circle;
                fd.friction = 0.3f;
                fd.density = 20.0f;
                m_bodies[m_bodyIndex]->CreateFixture(&fd);
            }

            m_bodyIndex = (m_bodyIndex + 1) % e_maxBodies;
        */
    }
    
    pub fn destroy_body(&mut self)  {
        
        todo!();
        /*
            for (int32 i = 0; i < e_maxBodies; ++i)
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

            case 'd':
                DestroyBody();
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            bool advanceRay = settings->pause == 0 || settings->singleStep;

            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Press 1-5 to drop stuff");
            m_textLine += 15;

            float32 L = 25.0f;
            b2Vec2 point1(0.0f, 10.0f);
            b2Vec2 d(L * cosf(m_angle), -L * b2Abs(sinf(m_angle)));
            b2Vec2 point2 = point1 + d;

            EdgeShapesCallback callback;

            m_world->RayCast(&callback, point1, point2);

            if (callback.m_fixture)
            {
                m_debugDraw.DrawPoint(callback.m_point, 5.0f, b2Color(0.4f, 0.9f, 0.4f));

                m_debugDraw.DrawSegment(point1, callback.m_point, b2Color(0.8f, 0.8f, 0.8f));

                b2Vec2 head = callback.m_point + 0.5f * callback.m_normal;
                m_debugDraw.DrawSegment(callback.m_point, head, b2Color(0.9f, 0.9f, 0.4f));
            }
            else
            {
                m_debugDraw.DrawSegment(point1, point2, b2Color(0.8f, 0.8f, 0.8f));
            }

            if (advanceRay)
            {
                m_angle += 0.25f * b2_pi / 180.0f;
            }
        */
    }
    
    pub fn create_default() -> *mut Test {
        
        todo!();
        /*
            return new EdgeShapes;
        */
    }
}
