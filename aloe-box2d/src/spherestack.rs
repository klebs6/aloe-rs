crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/SphereStack.h]

pub const SPHERE_STACK_E_COUNT: usize = 10;

pub struct SphereStack {
    base:   Test,
    bodies: *mut [b2Body; SPHERE_STACK_E_COUNT],
}

impl Default for SphereStack {
    
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

            {
                b2CircleShape shape;
                shape.m_radius = 1.0f;

                for (int32 i = 0; i < SPHERE_STACK_E_COUNT; ++i)
                {
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(0.0, 4.0f + 3.0f * i);

                    m_bodies[i] = m_world->CreateBody(&bd);

                    m_bodies[i]->CreateFixture(&shape, 1.0f);

                    m_bodies[i]->SetLinearVelocity(b2Vec2(0.0f, -50.0f));
                }
            
        */
    }
}

impl SphereStack {

    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            //for (int32 i = 0; i < SPHERE_STACK_E_COUNT; ++i)
            //{
            //  printf("%g ", m_bodies[i]->GetWorldCenter().y);
            //}

            //for (int32 i = 0; i < SPHERE_STACK_E_COUNT; ++i)
            //{
            //  printf("%g ", m_bodies[i]->GetLinearVelocity().y);
            //}

            //printf("\n");
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new SphereStack;
        */
    }
}
