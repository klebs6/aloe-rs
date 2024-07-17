crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/SensorTest.h]

pub const SENSOR_TEST_E_COUNT: usize = 7;

/**
  | This is used to test sensor shapes.
  |
  */
pub struct SensorTest {
    base:     Test,
    sensor:   *mut b2Fixture,
    bodies:   *mut [b2Body; SENSOR_TEST_E_COUNT],
    touching: [bool; SENSOR_TEST_E_COUNT],
}

impl Default for SensorTest {
    
    fn default() -> Self {
        todo!();
        /*


            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                {
                    b2EdgeShape shape;
                    shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                    ground->CreateFixture(&shape, 0.0f);
                }

    #if 0
                {
                    b2FixtureDef sd;
                    sd.SetAsBox(10.0f, 2.0f, b2Vec2(0.0f, 20.0f), 0.0f);
                    sd.isSensor = true;
                    m_sensor = ground->CreateFixture(&sd);
                }
    #else
                {
                    b2CircleShape shape;
                    shape.m_radius = 5.0f;
                    shape.m_p.Set(0.0f, 10.0f);

                    b2FixtureDef fd;
                    fd.shape = &shape;
                    fd.isSensor = true;
                    m_sensor = ground->CreateFixture(&fd);
                }
    #endif
            }

            {
                b2CircleShape shape;
                shape.m_radius = 1.0f;

                for (int32 i = 0; i < SENSOR_TEST_E_COUNT; ++i)
                {
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(-10.0f + 3.0f * i, 20.0f);
                    bd.userData = m_touching + i;

                    m_touching[i] = false;
                    m_bodies[i] = m_world->CreateBody(&bd);

                    m_bodies[i]->CreateFixture(&shape, 1.0f);
                }
            
        */
    }
}

impl SensorTest {

    /**
      | Implement contact listener.
      |
      */
    pub fn begin_contact(&mut self, contact: *mut b2Contact)  {
        
        todo!();
        /*
            b2Fixture* fixtureA = contact->GetFixtureA();
            b2Fixture* fixtureB = contact->GetFixtureB();

            if (fixtureA == m_sensor)
            {
                void* userData = fixtureB->GetBody()->GetUserData();
                if (userData)
                {
                    bool* touching = (bool*)userData;
                    *touching = true;
                }
            }

            if (fixtureB == m_sensor)
            {
                void* userData = fixtureA->GetBody()->GetUserData();
                if (userData)
                {
                    bool* touching = (bool*)userData;
                    *touching = true;
                }
            }
        */
    }

    /**
      | Implement contact listener.
      |
      */
    pub fn end_contact(&mut self, contact: *mut b2Contact)  {
        
        todo!();
        /*
            b2Fixture* fixtureA = contact->GetFixtureA();
            b2Fixture* fixtureB = contact->GetFixtureB();

            if (fixtureA == m_sensor)
            {
                void* userData = fixtureB->GetBody()->GetUserData();
                if (userData)
                {
                    bool* touching = (bool*)userData;
                    *touching = false;
                }
            }

            if (fixtureB == m_sensor)
            {
                void* userData = fixtureA->GetBody()->GetUserData();
                if (userData)
                {
                    bool* touching = (bool*)userData;
                    *touching = false;
                }
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            // Traverse the contact results. Apply a force on shapes
            // that overlap the sensor.
            for (int32 i = 0; i < SENSOR_TEST_E_COUNT; ++i)
            {
                if (m_touching[i] == false)
                {
                    continue;
                }

                b2Body* body = m_bodies[i];
                b2Body* ground = m_sensor->GetBody();

                b2CircleShape* circle = (b2CircleShape*)m_sensor->GetShape();
                b2Vec2 center = ground->GetWorldPoint(circle->m_p);

                b2Vec2 position = body->GetPosition();

                b2Vec2 d = center - position;
                if (d.LengthSquared() < FLT_EPSILON * FLT_EPSILON)
                {
                    continue;
                }

                d.Normalize();
                b2Vec2 F = 100.0f * d;
                body->ApplyForce(F, position);
            }
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new SensorTest;
        */
    }
}
