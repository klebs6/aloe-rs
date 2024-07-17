crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Rope/b2Rope.h]

pub struct b2RopeDef {

    vertices: *mut b2Vec2,
    count:    i32,
    masses:   *mut f32,
    gravity:  b2Vec2,
    damping:  f32,

    /**
      Stretching stiffness
      */
    k2:       f32,

    /**
      Bending stiffness. Values above 0.5 can make
      the simulation blow up.
      */
    k3:       f32,
}

impl Default for b2RopeDef {
    
    fn default() -> Self {
        todo!();
        /*


            vertices = NULL;
            count = 0;
            masses = NULL;
            gravity.SetZero();
            damping = 0.1f;
            k2 = 0.9f;
            k3 = 0.1f
        */
    }
}

///------------------------
pub struct b2Rope {
    count:   i32,
    ps:      *mut b2Vec2,
    p0s:     *mut b2Vec2,
    vs:      *mut b2Vec2,
    ims:     *mut f32,
    ls:      *mut f32,
    as_:      *mut f32,
    gravity: b2Vec2,
    damping: f32,
    k2:      f32,
    k3:      f32,
}

impl Drop for b2Rope {
    fn drop(&mut self) {
        todo!();
        /* 
        b2Free(m_ps);
        b2Free(m_p0s);
        b2Free(m_vs);
        b2Free(m_ims);
        b2Free(m_Ls);
        b2Free(m_as);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Rope/b2Rope.cpp]
impl Default for b2Rope {

    fn default() -> Self {
    
        todo!();
        /*


            m_count = 0;
        m_ps = NULL;
        m_p0s = NULL;
        m_vs = NULL;
        m_ims = NULL;
        m_Ls = NULL;
        m_as = NULL;
        m_gravity.SetZero();
        m_k2 = 1.0f;
        m_k3 = 0.1f;
        */
    }
}
    
impl b2Rope {

    pub fn initialize(&mut self, def: *const b2RopeDef)  {
        
        todo!();
        /*
            b2Assert(def->count >= 3);
        m_count = def->count;
        m_ps = (b2Vec2*)b2Alloc(m_count * sizeof(b2Vec2));
        m_p0s = (b2Vec2*)b2Alloc(m_count * sizeof(b2Vec2));
        m_vs = (b2Vec2*)b2Alloc(m_count * sizeof(b2Vec2));
        m_ims = (float32*)b2Alloc(m_count * sizeof(float32));

        for (int32 i = 0; i < m_count; ++i)
        {
            m_ps[i] = def->vertices[i];
            m_p0s[i] = def->vertices[i];
            m_vs[i].SetZero();

            float32 m = def->masses[i];
            if (m > 0.0f)
            {
                m_ims[i] = 1.0f / m;
            }
            else
            {
                m_ims[i] = 0.0f;
            }
        }

        int32 count2 = m_count - 1;
        int32 count3 = m_count - 2;
        m_Ls = (float32*)b2Alloc(count2 * sizeof(float32));
        m_as = (float32*)b2Alloc(count3 * sizeof(float32));

        for (int32 i = 0; i < count2; ++i)
        {
            b2Vec2 p1 = m_ps[i];
            b2Vec2 p2 = m_ps[i+1];
            m_Ls[i] = b2Distance(p1, p2);
        }

        for (int32 i = 0; i < count3; ++i)
        {
            b2Vec2 p1 = m_ps[i];
            b2Vec2 p2 = m_ps[i + 1];
            b2Vec2 p3 = m_ps[i + 2];

            b2Vec2 d1 = p2 - p1;
            b2Vec2 d2 = p3 - p2;

            float32 a = b2Cross(d1, d2);
            float32 b = b2Dot(d1, d2);

            m_as[i] = b2Atan2(a, b);
        }

        m_gravity = def->gravity;
        m_damping = def->damping;
        m_k2 = def->k2;
        m_k3 = def->k3;
        */
    }

    pub fn step(&mut self, 
        h:          f32,
        iterations: i32)  {
        
        todo!();
        /*
            if (h == 0.0)
        {
            return;
        }

        float32 d = expf(- h * m_damping);

        for (int32 i = 0; i < m_count; ++i)
        {
            m_p0s[i] = m_ps[i];
            if (m_ims[i] > 0.0f)
            {
                m_vs[i] += h * m_gravity;
            }
            m_vs[i] *= d;
            m_ps[i] += h * m_vs[i];

        }

        for (int32 i = 0; i < iterations; ++i)
        {
            SolveC2();
            SolveC3();
            SolveC2();
        }

        float32 inv_h = 1.0f / h;
        for (int32 i = 0; i < m_count; ++i)
        {
            m_vs[i] = inv_h * (m_ps[i] - m_p0s[i]);
        }
        */
    }

    pub fn solvec2(&mut self)  {
        
        todo!();
        /*
            int32 count2 = m_count - 1;

        for (int32 i = 0; i < count2; ++i)
        {
            b2Vec2 p1 = m_ps[i];
            b2Vec2 p2 = m_ps[i + 1];

            b2Vec2 d = p2 - p1;
            float32 L = d.Normalize();

            float32 im1 = m_ims[i];
            float32 im2 = m_ims[i + 1];

            if (im1 + im2 == 0.0f)
            {
                continue;
            }

            float32 s1 = im1 / (im1 + im2);
            float32 s2 = im2 / (im1 + im2);

            p1 -= m_k2 * s1 * (m_Ls[i] - L) * d;
            p2 += m_k2 * s2 * (m_Ls[i] - L) * d;

            m_ps[i] = p1;
            m_ps[i + 1] = p2;
        }
        */
    }

    pub fn set_angle(&mut self, angle: f32)  {
        
        todo!();
        /*
            int32 count3 = m_count - 2;
        for (int32 i = 0; i < count3; ++i)
        {
            m_as[i] = angle;
        }
        */
    }

    pub fn solvec3(&mut self)  {
        
        todo!();
        /*
            int32 count3 = m_count - 2;

        for (int32 i = 0; i < count3; ++i)
        {
            b2Vec2 p1 = m_ps[i];
            b2Vec2 p2 = m_ps[i + 1];
            b2Vec2 p3 = m_ps[i + 2];

            float32 m1 = m_ims[i];
            float32 m2 = m_ims[i + 1];
            float32 m3 = m_ims[i + 2];

            b2Vec2 d1 = p2 - p1;
            b2Vec2 d2 = p3 - p2;

            float32 L1sqr = d1.LengthSquared();
            float32 L2sqr = d2.LengthSquared();

            if (L1sqr * L2sqr == 0.0f)
            {
                continue;
            }

            float32 a = b2Cross(d1, d2);
            float32 b = b2Dot(d1, d2);

            float32 angle = b2Atan2(a, b);

            b2Vec2 Jd1 = (-1.0f / L1sqr) * d1.Skew();
            b2Vec2 Jd2 = (1.0f / L2sqr) * d2.Skew();

            b2Vec2 J1 = -Jd1;
            b2Vec2 J2 = Jd1 - Jd2;
            b2Vec2 J3 = Jd2;

            float32 mass = m1 * b2Dot(J1, J1) + m2 * b2Dot(J2, J2) + m3 * b2Dot(J3, J3);
            if (mass == 0.0f)
            {
                continue;
            }

            mass = 1.0f / mass;

            float32 C = angle - m_as[i];

            while (C > b2_pi)
            {
                angle -= 2 * b2_pi;
                C = angle - m_as[i];
            }

            while (C < -b2_pi)
            {
                angle += 2.0f * b2_pi;
                C = angle - m_as[i];
            }

            float32 impulse = - m_k3 * mass * C;

            p1 += (m1 * impulse) * J1;
            p2 += (m2 * impulse) * J2;
            p3 += (m3 * impulse) * J3;

            m_ps[i] = p1;
            m_ps[i + 1] = p2;
            m_ps[i + 2] = p3;
        }
        */
    }

    pub fn draw(&self, draw: *mut b2Draw)  {
        
        todo!();
        /*
            b2Color c(0.4f, 0.5f, 0.7f);

        for (int32 i = 0; i < m_count - 1; ++i)
        {
            draw->DrawSegment(m_ps[i], m_ps[i+1], c);
        }
        */
    }
}
