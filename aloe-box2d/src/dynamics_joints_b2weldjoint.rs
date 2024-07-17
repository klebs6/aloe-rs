crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2WeldJoint.h]

/**
  | Weld joint definition. You need to specify
  | local anchor points where they are attached
  | and the relative body angle. The position of
  | the anchor points is important for computing
  | the reaction torque.
  */
pub struct b2WeldJointDef {

    base:            b2JointDef,

    /**
      The local anchor point relative to bodyA's
      origin.
      */
    local_anchora:   b2Vec2,

    /**
      The local anchor point relative to bodyB's
      origin.
      */
    local_anchorb:   b2Vec2,

    /**
      The bodyB angle minus bodyA angle in the
      reference state (radians).
      */
    reference_angle: f32,

    /**
      | The mass-spring-damper frequency in
      | Hertz. Rotation only.
      |
      | Disable softness with a value of 0.
      */
    frequency_hz:    f32,

    /**
      The damping ratio. 0 = no damping,
      1 = critical damping.
      */
    damping_ratio:   f32,
}

impl Default for b2WeldJointDef {
    
    fn default() -> Self {
        todo!();
        /*

            type = e_weldJoint;
            localAnchorA.Set(0.0f, 0.0f);
            localAnchorB.Set(0.0f, 0.0f);
            referenceAngle = 0.0f;
            frequencyHz = 0.0f;
            dampingRatio = 0.0f
        */
    }
}

impl b2WeldJointDef {

    /**
      | Point-to-point constraint
      | C = p2 - p1
      | Cdot = v2 - v1
      |      = v2 + cross(w2, r2) - v1 - cross(w1, r1)
      | J = [-I -r1_skew I r2_skew ]
      | Identity used:
      | w k % (rx i + ry j) = w * (-ry i + rx j)
      |
      | Angle constraint
      | C = angle2 - angle1 - referenceAngle
      | Cdot = w2 - w1
      | J = [0 0 -1 0 0 1]
      | K = invI1 + invI2
      |
      | Initialize the bodies, anchors, and
      | reference angle using a world anchor point.
      */
    pub fn initialize(&mut self, 
        ba:     *mut b2Body,
        bb:     *mut b2Body,
        anchor: &b2Vec2)  {
        
        todo!();
        /*
            bodyA = bA;
        bodyB = bB;
        localAnchorA = bodyA->GetLocalPoint(anchor);
        localAnchorB = bodyB->GetLocalPoint(anchor);
        referenceAngle = bodyB->GetAngle() - bodyA->GetAngle();
        */
    }
}


/**
  | A weld joint essentially glues two bodies
  | together. A weld joint may distort somewhat
  | because the island constraint solver is
  | approximate.
  */
pub struct b2WeldJoint {
    base:            b2Joint,
    frequency_hz:    f32,
    damping_ratio:   f32,
    bias:            f32,

    /* ----------------- Solver shared  ----------------- */
    local_anchora:   b2Vec2,
    local_anchorb:   b2Vec2,
    reference_angle: f32,
    gamma:           f32,
    impulse:         b2Vec3,

    /* ------------------ Solver temp  ------------------ */
    indexa:          i32,
    indexb:          i32,
    ra:              b2Vec2,
    rb:              b2Vec2,
    local_centera:   b2Vec2,
    local_centerb:   b2Vec2,
    inv_massa:       f32,
    inv_massb:       f32,
    invia:           f32,
    invib:           f32,
    mass:            b2Mat33,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2WeldJoint.cpp]
impl b2WeldJoint {

    /**
      | The local anchor point relative to bodyA's
      | origin.
      |
      */
    pub fn get_local_anchora(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_localAnchorA;
        */
    }

    /**
      | The local anchor point relative to bodyB's
      | origin.
      |
      */
    pub fn get_local_anchorb(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_localAnchorB;
        */
    }

    /**
      | Get the reference angle.
      |
      */
    pub fn get_reference_angle(&self) -> f32 {
        
        todo!();
        /*
            return m_referenceAngle;
        */
    }

    /**
      | Set/get frequency in Hz.
      |
      */
    pub fn set_frequency(&mut self, hz: f32)  {
        
        todo!();
        /*
            m_frequencyHz = hz;
        */
    }

    pub fn get_frequency(&self) -> f32 {
        
        todo!();
        /*
            return m_frequencyHz;
        */
    }

    /**
      | Set/get damping ratio.
      |
      */
    pub fn set_damping_ratio(&mut self, ratio:
        f32)  {
        
        todo!();
        /*
            m_dampingRatio = ratio;
        */
    }

    pub fn get_damping_ratio(&self) -> f32 {
        
        todo!();
        /*
            return m_dampingRatio;
        */
    }

    pub fn new(def: *const b2WeldJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            m_localAnchorA = def->localAnchorA;
        m_localAnchorB = def->localAnchorB;
        m_referenceAngle = def->referenceAngle;
        m_frequencyHz = def->frequencyHz;
        m_dampingRatio = def->dampingRatio;

        m_impulse.SetZero();
        */
    }

    pub fn init_velocity_constraints(&mut self, data: &b2SolverData)  {
        
        todo!();
        /*
            m_indexA = m_bodyA->m_islandIndex;
        m_indexB = m_bodyB->m_islandIndex;
        m_localCenterA = m_bodyA->m_sweep.localCenter;
        m_localCenterB = m_bodyB->m_sweep.localCenter;
        m_invMassA = m_bodyA->m_invMass;
        m_invMassB = m_bodyB->m_invMass;
        m_invIA = m_bodyA->m_invI;
        m_invIB = m_bodyB->m_invI;

        float32 aA = data.positions[m_indexA].a;
        b2Vec2 vA = data.velocities[m_indexA].v;
        float32 wA = data.velocities[m_indexA].w;

        float32 aB = data.positions[m_indexB].a;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        b2Rot qA(aA), qB(aB);

        m_rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        m_rB = b2Mul(qB, m_localAnchorB - m_localCenterB);

        // J = [-I -r1_skew I r2_skew]
        //     [ 0       -1 0       1]
        // r_skew = [-ry; rx]

        // Matlab
        // K = [ mA+r1y^2*iA+mB+r2y^2*iB,  -r1y*iA*r1x-r2y*iB*r2x,          -r1y*iA-r2y*iB]
        //     [  -r1y*iA*r1x-r2y*iB*r2x, mA+r1x^2*iA+mB+r2x^2*iB,           r1x*iA+r2x*iB]
        //     [          -r1y*iA-r2y*iB,           r1x*iA+r2x*iB,                   iA+iB]

        float32 mA = m_invMassA, mB = m_invMassB;
        float32 iA = m_invIA, iB = m_invIB;

        b2Mat33 K;
        K.ex.x = mA + mB + m_rA.y * m_rA.y * iA + m_rB.y * m_rB.y * iB;
        K.ey.x = -m_rA.y * m_rA.x * iA - m_rB.y * m_rB.x * iB;
        K.ez.x = -m_rA.y * iA - m_rB.y * iB;
        K.ex.y = K.ey.x;
        K.ey.y = mA + mB + m_rA.x * m_rA.x * iA + m_rB.x * m_rB.x * iB;
        K.ez.y = m_rA.x * iA + m_rB.x * iB;
        K.ex.z = K.ez.x;
        K.ey.z = K.ez.y;
        K.ez.z = iA + iB;

        if (m_frequencyHz > 0.0f)
        {
            K.GetInverse22(&m_mass);

            float32 invM = iA + iB;
            float32 m = invM > 0.0f ? 1.0f / invM : 0.0f;

            float32 C = aB - aA - m_referenceAngle;

            // Frequency
            float32 omega = 2.0f * b2_pi * m_frequencyHz;

            // Damping coefficient
            float32 d = 2.0f * m * m_dampingRatio * omega;

            // Spring stiffness
            float32 k = m * omega * omega;

            // magic formulas
            float32 h = data.step.dt;
            m_gamma = h * (d + h * k);
            m_gamma = m_gamma != 0.0f ? 1.0f / m_gamma : 0.0f;
            m_bias = C * h * k * m_gamma;

            invM += m_gamma;
            m_mass.ez.z = invM != 0.0f ? 1.0f / invM : 0.0f;
        }
        else
        {
            K.GetSymInverse33(&m_mass);
            m_gamma = 0.0f;
            m_bias = 0.0f;
        }

        if (data.step.warmStarting)
        {
            // Scale impulses to support a variable time step.
            m_impulse *= data.step.dtRatio;

            b2Vec2 P(m_impulse.x, m_impulse.y);

            vA -= mA * P;
            wA -= iA * (b2Cross(m_rA, P) + m_impulse.z);

            vB += mB * P;
            wB += iB * (b2Cross(m_rB, P) + m_impulse.z);
        }
        else
        {
            m_impulse.SetZero();
        }

        data.velocities[m_indexA].v = vA;
        data.velocities[m_indexA].w = wA;
        data.velocities[m_indexB].v = vB;
        data.velocities[m_indexB].w = wB;
        */
    }

    pub fn solve_velocity_constraints(&mut self, data: &b2SolverData)  {
        
        todo!();
        /*
            b2Vec2 vA = data.velocities[m_indexA].v;
        float32 wA = data.velocities[m_indexA].w;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        float32 mA = m_invMassA, mB = m_invMassB;
        float32 iA = m_invIA, iB = m_invIB;

        if (m_frequencyHz > 0.0f)
        {
            float32 Cdot2 = wB - wA;

            float32 impulse2 = -m_mass.ez.z * (Cdot2 + m_bias + m_gamma * m_impulse.z);
            m_impulse.z += impulse2;

            wA -= iA * impulse2;
            wB += iB * impulse2;

            b2Vec2 Cdot1 = vB + b2Cross(wB, m_rB) - vA - b2Cross(wA, m_rA);

            b2Vec2 impulse1 = -b2Mul22(m_mass, Cdot1);
            m_impulse.x += impulse1.x;
            m_impulse.y += impulse1.y;

            b2Vec2 P = impulse1;

            vA -= mA * P;
            wA -= iA * b2Cross(m_rA, P);

            vB += mB * P;
            wB += iB * b2Cross(m_rB, P);
        }
        else
        {
            b2Vec2 Cdot1 = vB + b2Cross(wB, m_rB) - vA - b2Cross(wA, m_rA);
            float32 Cdot2 = wB - wA;
            b2Vec3 Cdot(Cdot1.x, Cdot1.y, Cdot2);

            b2Vec3 impulse = -b2Mul(m_mass, Cdot);
            m_impulse += impulse;

            b2Vec2 P(impulse.x, impulse.y);

            vA -= mA * P;
            wA -= iA * (b2Cross(m_rA, P) + impulse.z);

            vB += mB * P;
            wB += iB * (b2Cross(m_rB, P) + impulse.z);
        }

        data.velocities[m_indexA].v = vA;
        data.velocities[m_indexA].w = wA;
        data.velocities[m_indexB].v = vB;
        data.velocities[m_indexB].w = wB;
        */
    }

    pub fn solve_position_constraints(&mut self, data: &b2SolverData) -> bool {
        
        todo!();
        /*
            b2Vec2 cA = data.positions[m_indexA].c;
        float32 aA = data.positions[m_indexA].a;
        b2Vec2 cB = data.positions[m_indexB].c;
        float32 aB = data.positions[m_indexB].a;

        b2Rot qA(aA), qB(aB);

        float32 mA = m_invMassA, mB = m_invMassB;
        float32 iA = m_invIA, iB = m_invIB;

        b2Vec2 rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        b2Vec2 rB = b2Mul(qB, m_localAnchorB - m_localCenterB);

        float32 positionError, angularError;

        b2Mat33 K;
        K.ex.x = mA + mB + rA.y * rA.y * iA + rB.y * rB.y * iB;
        K.ey.x = -rA.y * rA.x * iA - rB.y * rB.x * iB;
        K.ez.x = -rA.y * iA - rB.y * iB;
        K.ex.y = K.ey.x;
        K.ey.y = mA + mB + rA.x * rA.x * iA + rB.x * rB.x * iB;
        K.ez.y = rA.x * iA + rB.x * iB;
        K.ex.z = K.ez.x;
        K.ey.z = K.ez.y;
        K.ez.z = iA + iB;

        if (m_frequencyHz > 0.0f)
        {
            b2Vec2 C1 =  cB + rB - cA - rA;

            positionError = C1.Length();
            angularError = 0.0f;

            b2Vec2 P = -K.Solve22(C1);

            cA -= mA * P;
            aA -= iA * b2Cross(rA, P);

            cB += mB * P;
            aB += iB * b2Cross(rB, P);
        }
        else
        {
            b2Vec2 C1 =  cB + rB - cA - rA;
            float32 C2 = aB - aA - m_referenceAngle;

            positionError = C1.Length();
            angularError = b2Abs(C2);

            b2Vec3 C(C1.x, C1.y, C2);

            b2Vec3 impulse = -K.Solve33(C);
            b2Vec2 P(impulse.x, impulse.y);

            cA -= mA * P;
            aA -= iA * (b2Cross(rA, P) + impulse.z);

            cB += mB * P;
            aB += iB * (b2Cross(rB, P) + impulse.z);
        }

        data.positions[m_indexA].c = cA;
        data.positions[m_indexA].a = aA;
        data.positions[m_indexB].c = cB;
        data.positions[m_indexB].a = aB;

        return positionError <= b2_linearSlop && angularError <= b2_angularSlop;
        */
    }

    pub fn get_anchora(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_bodyA->GetWorldPoint(m_localAnchorA);
        */
    }

    pub fn get_anchorb(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_bodyB->GetWorldPoint(m_localAnchorB);
        */
    }

    pub fn get_reaction_force(&self, inv_dt: f32) -> b2Vec2 {
        
        todo!();
        /*
            b2Vec2 P(m_impulse.x, m_impulse.y);
        return inv_dt * P;
        */
    }

    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            return inv_dt * m_impulse.z;
        */
    }

    /**
      | Dump to b2Log
      |
      */
    pub fn dump(&mut self)  {
        
        todo!();
        /*
            int32 indexA = m_bodyA->m_islandIndex;
        int32 indexB = m_bodyB->m_islandIndex;

        b2Log("  b2WeldJointDef jd;\n");
        b2Log("  jd.bodyA = bodies[%d];\n", indexA);
        b2Log("  jd.bodyB = bodies[%d];\n", indexB);
        b2Log("  jd.collideConnected = bool(%d);\n", m_collideConnected);
        b2Log("  jd.localAnchorA.Set(%.15lef, %.15lef);\n", m_localAnchorA.x, m_localAnchorA.y);
        b2Log("  jd.localAnchorB.Set(%.15lef, %.15lef);\n", m_localAnchorB.x, m_localAnchorB.y);
        b2Log("  jd.referenceAngle = %.15lef;\n", m_referenceAngle);
        b2Log("  jd.frequencyHz = %.15lef;\n", m_frequencyHz);
        b2Log("  jd.dampingRatio = %.15lef;\n", m_dampingRatio);
        b2Log("  joints[%d] = m_world->CreateJoint(&jd);\n", m_index);
        */
    }
}
