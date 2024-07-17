/*!
  | This test demonstrates how to use the world
  | ray-cast feature.
  |
  | NOTE: we are intentionally filtering one of the
  | polygons, therefore the ray will always miss
  | one type of polygon.
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/RayCast.h]

/**
   This callback finds the closest hit. Polygon
   0 is filtered.
  */
pub struct RayCastClosestCallback {
    hit:    bool,
    point:  b2Vec2,
    normal: b2Vec2,
}

impl b2RayCastCallback for RayCastClosestCallback {

    fn report_fixture(
        &mut self, 
        fixture:  *mut b2Fixture,
        point:    &b2Vec2,
        normal:   &b2Vec2,
        fraction: f32

    ) -> f32 {
        
        todo!();
        /*
            b2Body* body = fixture->GetBody();
            void* userData = body->GetUserData();
            if (userData)
            {
                int32 index = *(int32*)userData;
                if (index == 0)
                {
                    // filter
                    return -1.0f;
                }
            }

            m_hit = true;
            m_point = point;
            m_normal = normal;
            return fraction;
        */
    }
}

impl Default for RayCastClosestCallback {
    
    fn default() -> Self {
        todo!();
        /*


            m_hit = false
        */
    }
}

/**
   This callback finds any hit. Polygon 0 is
   filtered.
  */
pub struct RayCastAnyCallback {
    hit:    bool,
    point:  b2Vec2,
    normal: b2Vec2,
}

impl b2RayCastCallback for RayCastAnyCallback {

    fn report_fixture(
        &mut self, 
        fixture:  *mut b2Fixture,
        point:    &b2Vec2,
        normal:   &b2Vec2,
        fraction: f32

    ) -> f32 {
        
        todo!();
        /*
            b2Body* body = fixture->GetBody();
            void* userData = body->GetUserData();
            if (userData)
            {
                int32 index = *(int32*)userData;
                if (index == 0)
                {
                    // filter
                    return -1.0f;
                }
            }

            m_hit = true;
            m_point = point;
            m_normal = normal;
            return 0.0f;
        */
    }
}

impl Default for RayCastAnyCallback {
    
    fn default() -> Self {
        todo!();
        /*


            m_hit = false
        */
    }
}

pub const RAY_CAST_MULTIPLE_CALLBACK_E_MAXCOUNT: usize = 3;

/**
  | This ray cast collects multiple hits
  | along the ray. Polygon 0 is filtered.
  |
  */
pub struct RayCastMultipleCallback {
    points:  [b2Vec2; RAY_CAST_MULTIPLE_CALLBACK_E_MAXCOUNT],
    normals: [b2Vec2; RAY_CAST_MULTIPLE_CALLBACK_E_MAXCOUNT],
    count:   i32,
}

impl b2RayCastCallback for RayCastMultipleCallback {

    fn report_fixture(
        &mut self, 
        fixture:  *mut b2Fixture,
        point:    &b2Vec2,
        normal:   &b2Vec2,
        fraction: f32

    ) -> f32 {
        
        todo!();
        /*
            b2Body* body = fixture->GetBody();
            void* userData = body->GetUserData();
            if (userData)
            {
                int32 index = *(int32*)userData;
                if (index == 0)
                {
                    // filter
                    return -1.0f;
                }
            }

            b2Assert(m_count < RAY_CAST_MULTIPLE_CALLBACK_E_MAXCOUNT);

            m_points[m_count] = point;
            m_normals[m_count] = normal;
            ++m_count;

            if (m_count == RAY_CAST_MULTIPLE_CALLBACK_E_MAXCOUNT)
            {
                return 0.0f;
            }

            return 1.0f;
        */
    }
}

impl Default for RayCastMultipleCallback {
    
    fn default() -> Self {
        todo!();
        /*


            m_count = 0
        */
    }
}

///------------------------
pub const RAY_CAST_E_MAXBODIES: usize = 256;

pub enum RayCastMode
{
    e_closest,
    e_any,
    e_multiple
}

pub struct RayCast {
    base:       Test,
    body_index: i32,
    bodies:     *mut [b2Body; RAY_CAST_E_MAXBODIES],
    user_data:  [i32; RAY_CAST_E_MAXBODIES],
    polygons:   [b2PolygonShape; 4],
    circle:     b2CircleShape,
    angle:      f32,
    mode:       RayCastMode,
}

impl Default for RayCast {
    
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
            memset(m_bodies, 0, sizeof(m_bodies));

            m_angle = 0.0f;

            m_mode = e_closest
        */
    }
}

impl RayCast {

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
            float32 y = RandomFloat(0.0f, 20.0f);
            bd.position.Set(x, y);
            bd.angle = RandomFloat(-b2_pi, b2_pi);

            m_userData[m_bodyIndex] = index;
            bd.userData = m_userData + m_bodyIndex;

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
                m_bodies[m_bodyIndex]->CreateFixture(&fd);
            }
            else
            {
                b2FixtureDef fd;
                fd.shape = &m_circle;
                fd.friction = 0.3f;

                m_bodies[m_bodyIndex]->CreateFixture(&fd);
            }

            m_bodyIndex = (m_bodyIndex + 1) % RAY_CAST_E_MAXBODIES;
        */
    }
    
    pub fn destroy_body(&mut self)  {
        
        todo!();
        /*
            for (int32 i = 0; i < RAY_CAST_E_MAXBODIES; ++i)
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

            case 'm':
                if (m_mode == e_closest)
                {
                    m_mode = e_any;
                }
                else if (m_mode == e_any)
                {
                    m_mode = e_multiple;
                }
                else if (m_mode == e_multiple)
                {
                    m_mode = e_closest;
                }
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            bool advanceRay = settings->pause == 0 || settings->singleStep;

            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Press 1-5 to drop stuff, m to change the mode");
            m_textLine += 15;
            m_debugDraw.DrawString(5, m_textLine, "RayCastMode = %d", m_mode);
            m_textLine += 15;

            float32 L = 11.0f;
            b2Vec2 point1(0.0f, 10.0f);
            b2Vec2 d(L * cosf(m_angle), L * sinf(m_angle));
            b2Vec2 point2 = point1 + d;

            if (m_mode == e_closest)
            {
                RayCastClosestCallback callback;
                m_world->RayCast(&callback, point1, point2);

                if (callback.m_hit)
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
            }
            else if (m_mode == e_any)
            {
                RayCastAnyCallback callback;
                m_world->RayCast(&callback, point1, point2);

                if (callback.m_hit)
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
            }
            else if (m_mode == e_multiple)
            {
                RayCastMultipleCallback callback;
                m_world->RayCast(&callback, point1, point2);
                m_debugDraw.DrawSegment(point1, point2, b2Color(0.8f, 0.8f, 0.8f));

                for (int32 i = 0; i < callback.m_count; ++i)
                {
                    b2Vec2 p = callback.m_points[i];
                    b2Vec2 n = callback.m_normals[i];
                    m_debugDraw.DrawPoint(p, 5.0f, b2Color(0.4f, 0.9f, 0.4f));
                    m_debugDraw.DrawSegment(point1, p, b2Color(0.8f, 0.8f, 0.8f));
                    b2Vec2 head = p + 0.5f * n;
                    m_debugDraw.DrawSegment(p, head, b2Color(0.9f, 0.9f, 0.4f));
                }
            }

            if (advanceRay)
            {
                m_angle += 0.25f * b2_pi / 180.0f;
            }

    #if 0
            // This case was failing.
            {
                b2Vec2 vertices[4];
                //vertices[0].Set(-22.875f, -3.0f);
                //vertices[1].Set(22.875f, -3.0f);
                //vertices[2].Set(22.875f, 3.0f);
                //vertices[3].Set(-22.875f, 3.0f);

                b2PolygonShape shape;
                //shape.Set(vertices, 4);
                shape.SetAsBox(22.875f, 3.0f);

                b2RayCastInput input;
                input.p1.Set(10.2725f,1.71372f);
                input.p2.Set(10.2353f,2.21807f);
                //input.maxFraction = 0.567623f;
                input.maxFraction = 0.56762173f;

                b2Transform xf;
                xf.SetIdentity();
                xf.position.Set(23.0f, 5.0f);

                b2RayCastOutput output;
                bool hit;
                hit = shape.RayCast(&output, input, xf);
                hit = false;

                b2Color color(1.0f, 1.0f, 1.0f);
                b2Vec2 vs[4];
                for (int32 i = 0; i < 4; ++i)
                {
                    vs[i] = b2Mul(xf, shape.m_vertices[i]);
                }

                m_debugDraw.DrawPolygon(vs, 4, color);
                m_debugDraw.DrawSegment(input.p1, input.p2, color);
            }
    #endif
        */
    }
    
    pub fn create_default() -> *mut Test {
        
        todo!();
        /*
            return new RayCast;
        */
    }
}
