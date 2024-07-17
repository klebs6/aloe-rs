crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/ShapeEditing.h]

pub struct ShapeEditing {
    base:     Test,
    body:     *mut b2Body,
    fixture1: *mut b2Fixture,
    fixture2: *mut b2Fixture,
    sensor:   bool,
}

impl Default for ShapeEditing {
    
    fn default() -> Self {
        todo!();
        /*
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            b2BodyDef bd;
            bd.type = b2_dynamicBody;
            bd.position.Set(0.0f, 10.0f);
            m_body = m_world->CreateBody(&bd);

            b2PolygonShape shape;
            shape.SetAsBox(4.0f, 4.0f, b2Vec2(0.0f, 0.0f), 0.0f);
            m_fixture1 = m_body->CreateFixture(&shape, 10.0f);

            m_fixture2 = NULL;

            m_sensor = false
        */
    }
}

impl ShapeEditing {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'c':
                if (m_fixture2 == NULL)
                {
                    b2CircleShape shape;
                    shape.m_radius = 3.0f;
                    shape.m_p.Set(0.5f, -4.0f);
                    m_fixture2 = m_body->CreateFixture(&shape, 10.0f);
                    m_body->SetAwake(true);
                }
                break;

            case 'd':
                if (m_fixture2 != NULL)
                {
                    m_body->DestroyFixture(m_fixture2);
                    m_fixture2 = NULL;
                    m_body->SetAwake(true);
                }
                break;

            case 's':
                if (m_fixture2 != NULL)
                {
                    m_sensor = !m_sensor;
                    m_fixture2->SetSensor(m_sensor);
                }
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Press: (c) create a shape, (d) destroy a shape.");
            m_textLine += 15;
            m_debugDraw.DrawString(5, m_textLine, "sensor = %d", m_sensor);
            m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new ShapeEditing;
        */
    }
}

