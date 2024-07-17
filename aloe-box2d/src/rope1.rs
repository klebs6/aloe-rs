crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/Rope.h]

pub struct Rope {
    base:  Test,
    rope:  b2Rope,
    angle: f32,
}

impl Default for Rope {
    
    fn default() -> Self {
        todo!();
        /*
            const int32 N = 40;
            b2Vec2 vertices[N];
            float32 masses[N];

            for (int32 i = 0; i < N; ++i)
            {
                vertices[i].Set(0.0f, 20.0f - 0.25f * i);
                masses[i] = 1.0f;
            }
            masses[0] = 0.0f;
            masses[1] = 0.0f;

            b2RopeDef def;
            def.vertices = vertices;
            def.count = N;
            def.gravity.Set(0.0f, -10.0f);
            def.masses = masses;
            def.damping = 0.1f;
            def.k2 = 1.0f;
            def.k3 = 0.5f;

            m_rope.Initialize(&def);

            m_angle = 0.0f;
            m_rope.SetAngle(m_angle)
        */
    }
}

impl Rope {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'q':
                m_angle = b2Max(-b2_pi, m_angle - 0.05f * b2_pi);
                m_rope.SetAngle(m_angle);
                break;

            case 'e':
                m_angle = b2Min(b2_pi, m_angle + 0.05f * b2_pi);
                m_rope.SetAngle(m_angle);
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            float32 dt = settings->hz > 0.0f ? 1.0f / settings->hz : 0.0f;

            if (settings->pause == 1 && settings->singleStep == 0)
            {
                dt = 0.0f;
            }

            m_rope.Step(dt, 1);

            Test::Step(settings);

            m_rope.Draw(&m_debugDraw);

            m_debugDraw.DrawString(5, m_textLine, "Press (q,e) to adjust target angle");
            m_textLine += 15;
            m_debugDraw.DrawString(5, m_textLine, "Target angle = %g degrees", m_angle * 180.0f / b2_pi);
            m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Rope;
        */
    }
}
