crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/Pyramid.h]

pub struct Pyramid {
    base: Test,
}

pub const PYRAMID_E_COUNT: usize = 20;

impl Default for Pyramid {
    
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
                float32 a = 0.5f;
                b2PolygonShape shape;
                shape.SetAsBox(a, a);

                b2Vec2 x(-7.0f, 0.75f);
                b2Vec2 y;
                b2Vec2 deltaX(0.5625f, 1.25f);
                b2Vec2 deltaY(1.125f, 0.0f);

                for (int32 i = 0; i < PYRAMID_E_COUNT; ++i)
                {
                    y = x;

                    for (int32 j = i; j < PYRAMID_E_COUNT; ++j)
                    {
                        b2BodyDef bd;
                        bd.type = b2_dynamicBody;
                        bd.position = y;
                        b2Body* body = m_world->CreateBody(&bd);
                        body->CreateFixture(&shape, 5.0f);

                        y += deltaY;
                    }

                    x += deltaX;
                }
        */
    }
}

impl Pyramid {

    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            //b2DynamicTree* tree = &m_world->m_contactManager.m_broadPhase.m_tree;

            //if (m_stepCount == 400)
            //{
            //  tree->RebuildBottomUp();
            //}
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Pyramid;
        */
    }
}
