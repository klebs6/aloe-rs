crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2WheelJoint.h]

/**
  | Wheel joint definition. This requires defining
  | a line of motion using an axis and an anchor
  | point. The definition uses local anchor points
  | and a local axis so that the initial
  | configuration can violate the constraint
  | slightly. The joint translation is zero when
  | the local anchor points coincide in world
  | space. Using local anchors and a local axis
  | helps when saving and loading a game.
  */
pub struct b2WheelJointDef {
    base: b2JointDef,

    /**
      The local anchor point relative to bodyA's
      origin.
      */
    local_anchora:    b2Vec2,

    /**
      The local anchor point relative to bodyB's
      origin.
      */
    local_anchorb:    b2Vec2,

    /**
      The local translation axis in bodyA.
      */
    local_axisa:      b2Vec2,

    /**
      Enable/disable the joint motor.
      */
    enable_motor:     bool,

    /**
      The maximum motor torque, usually in N-m.
      */
    max_motor_torque: f32,

    /**
      The desired motor speed in radians per
      second.
      */
    motor_speed:      f32,

    /**
      Suspension frequency, zero indicates no
      suspension
      */
    frequency_hz:     f32,

    /**
      Suspension damping ratio, one indicates
      critical damping
      */
    damping_ratio:    f32,
}

impl Default for b2WheelJointDef {
    
    fn default() -> Self {
        todo!();
        /*

            type = e_wheelJoint;
            localAnchorA.SetZero();
            localAnchorB.SetZero();
            localAxisA.Set(1.0f, 0.0f);
            enableMotor = false;
            maxMotorTorque = 0.0f;
            motorSpeed = 0.0f;
            frequencyHz = 2.0f;
            dampingRatio = 0.7f
        */
    }
}

impl b2WheelJointDef {

    /**
      | Initialize the bodies, anchors, axis, and
      | reference angle using the world anchor and
      | world axis.
      |
      | Linear constraint (point-to-line)
      | d = pB - pA = xB + rB - xA - rA
      | C = dot(ay, d)
      | Cdot = dot(d, cross(wA, ay)) + dot(ay, vB + cross(wB, rB) - vA - cross(wA, rA))
      |      = -dot(ay, vA) - dot(cross(d + rA, ay), wA) + dot(ay, vB) + dot(cross(rB, ay), vB)
      | J = [-ay, -cross(d + rA, ay), ay, cross(rB, ay)]
      |
      | Spring linear constraint
      | C = dot(ax, d)
      | Cdot = = -dot(ax, vA) - dot(cross(d + rA, ax), wA) + dot(ax, vB) + dot(cross(rB, ax), vB)
      | J = [-ax -cross(d+rA, ax) ax cross(rB, ax)]
      |
      | Motor rotational constraint
      | Cdot = wB - wA
      | J = [0 0 -1 0 0 1]
      */
    pub fn initialize(&mut self, 
        ba:     *mut b2Body,
        bb:     *mut b2Body,
        anchor: &b2Vec2,
        axis:   &b2Vec2)  {
        
        todo!();
        /*
            bodyA = bA;
        bodyB = bB;
        localAnchorA = bodyA->GetLocalPoint(anchor);
        localAnchorB = bodyB->GetLocalPoint(anchor);
        localAxisA = bodyA->GetLocalVector(axis);
        */
    }
}

/**
  | A wheel joint. This joint provides two degrees
  | of freedom: translation along an axis fixed in
  | bodyA and rotation in the plane. You can use
  | a joint limit to restrict the range of motion
  | and a joint motor to drive the rotation or to
  | model rotational friction.  This joint is
  | designed for vehicle suspensions.
  */
pub struct b2WheelJoint {
    base:             b2Joint,
    frequency_hz:     f32,
    damping_ratio:    f32,

    /* ----------------- Solver shared  ----------------- */
    local_anchora:    b2Vec2,

    local_anchorb:    b2Vec2,
    localx_axisa:     b2Vec2,
    localy_axisa:     b2Vec2,
    impulse:          f32,
    motor_impulse:    f32,
    spring_impulse:   f32,
    max_motor_torque: f32,
    motor_speed:      f32,
    enable_motor:     bool,

    /* ------------------ Solver temp  ------------------ */
    indexa:           i32,
    indexb:           i32,
    local_centera:    b2Vec2,
    local_centerb:    b2Vec2,
    inv_massa:        f32,
    inv_massb:        f32,
    invia:            f32,
    invib:            f32,
    ax:               b2Vec2,
    ay:               b2Vec2,
    sAx:              f32,
    sBx:              f32,
    sAy:              f32,
    sBy:              f32,
    mass:             f32,
    motor_mass:       f32,
    spring_mass:      f32,
    bias:             f32,
    gamma:            f32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2WheelJoint.cpp]
impl b2WheelJoint {

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
      | The local joint axis relative to bodyA.
      |
      */
    pub fn get_local_axisa(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_localXAxisA;
        */
    }

    /**
      | Get the motor speed, usually in radians
      | per second.
      |
      */
    #[inline] pub fn get_motor_speed(&self) -> f32 {
        
        todo!();
        /*
            return m_motorSpeed;
        */
    }

    #[inline] pub fn get_max_motor_torque(&self) -> f32 {
        
        todo!();
        /*
            return m_maxMotorTorque;
        */
    }

    /**
      | Set/Get the spring frequency in hertz.
      | Setting the frequency to zero disables
      | the spring.
      |
      */
    #[inline] pub fn set_spring_frequency_hz(&mut self, hz: f32)  {
        
        todo!();
        /*
            m_frequencyHz = hz;
        */
    }

    #[inline] pub fn get_spring_frequency_hz(&self) -> f32 {
        
        todo!();
        /*
            return m_frequencyHz;
        */
    }

    /**
      | Set/Get the spring damping ratio
      |
      */
    #[inline] pub fn set_spring_damping_ratio(&mut self, ratio: f32)  {
        
        todo!();
        /*
            m_dampingRatio = ratio;
        */
    }

    #[inline] pub fn get_spring_damping_ratio(&self) -> f32 {
        
        todo!();
        /*
            return m_dampingRatio;
        */
    }

    pub fn new(def: *const b2WheelJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            m_localAnchorA = def->localAnchorA;
        m_localAnchorB = def->localAnchorB;
        m_localXAxisA = def->localAxisA;
        m_localYAxisA = b2Cross(1.0f, m_localXAxisA);

        m_mass = 0.0f;
        m_impulse = 0.0f;
        m_motorMass = 0.0f;
        m_motorImpulse = 0.0f;
        m_springMass = 0.0f;
        m_springImpulse = 0.0f;

        m_maxMotorTorque = def->maxMotorTorque;
        m_motorSpeed = def->motorSpeed;
        m_enableMotor = def->enableMotor;

        m_frequencyHz = def->frequencyHz;
        m_dampingRatio = def->dampingRatio;

        m_bias = 0.0f;
        m_gamma = 0.0f;

        m_ax.SetZero();
        m_ay.SetZero();
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

        float32 mA = m_invMassA, mB = m_invMassB;
        float32 iA = m_invIA, iB = m_invIB;

        b2Vec2 cA = data.positions[m_indexA].c;
        float32 aA = data.positions[m_indexA].a;
        b2Vec2 vA = data.velocities[m_indexA].v;
        float32 wA = data.velocities[m_indexA].w;

        b2Vec2 cB = data.positions[m_indexB].c;
        float32 aB = data.positions[m_indexB].a;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        b2Rot qA(aA), qB(aB);

        // Compute the effective masses.
        b2Vec2 rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        b2Vec2 rB = b2Mul(qB, m_localAnchorB - m_localCenterB);
        b2Vec2 d = cB + rB - cA - rA;

        // Point to line constraint
        {
            m_ay = b2Mul(qA, m_localYAxisA);
            m_sAy = b2Cross(d + rA, m_ay);
            m_sBy = b2Cross(rB, m_ay);

            m_mass = mA + mB + iA * m_sAy * m_sAy + iB * m_sBy * m_sBy;

            if (m_mass > 0.0f)
            {
                m_mass = 1.0f / m_mass;
            }
        }

        // Spring constraint
        m_springMass = 0.0f;
        m_bias = 0.0f;
        m_gamma = 0.0f;
        if (m_frequencyHz > 0.0f)
        {
            m_ax = b2Mul(qA, m_localXAxisA);
            m_sAx = b2Cross(d + rA, m_ax);
            m_sBx = b2Cross(rB, m_ax);

            float32 invMass = mA + mB + iA * m_sAx * m_sAx + iB * m_sBx * m_sBx;

            if (invMass > 0.0f)
            {
                m_springMass = 1.0f / invMass;

                float32 C = b2Dot(d, m_ax);

                // Frequency
                float32 omega = 2.0f * b2_pi * m_frequencyHz;

                // Damping coefficient
                float32 damp = 2.0f * m_springMass * m_dampingRatio * omega;

                // Spring stiffness
                float32 k = m_springMass * omega * omega;

                // magic formulas
                float32 h = data.step.dt;
                m_gamma = h * (damp + h * k);
                if (m_gamma > 0.0f)
                {
                    m_gamma = 1.0f / m_gamma;
                }

                m_bias = C * h * k * m_gamma;

                m_springMass = invMass + m_gamma;
                if (m_springMass > 0.0f)
                {
                    m_springMass = 1.0f / m_springMass;
                }
            }
        }
        else
        {
            m_springImpulse = 0.0f;
        }

        // Rotational motor
        if (m_enableMotor)
        {
            m_motorMass = iA + iB;
            if (m_motorMass > 0.0f)
            {
                m_motorMass = 1.0f / m_motorMass;
            }
        }
        else
        {
            m_motorMass = 0.0f;
            m_motorImpulse = 0.0f;
        }

        if (data.step.warmStarting)
        {
            // Account for variable time step.
            m_impulse *= data.step.dtRatio;
            m_springImpulse *= data.step.dtRatio;
            m_motorImpulse *= data.step.dtRatio;

            b2Vec2 P = m_impulse * m_ay + m_springImpulse * m_ax;
            float32 LA = m_impulse * m_sAy + m_springImpulse * m_sAx + m_motorImpulse;
            float32 LB = m_impulse * m_sBy + m_springImpulse * m_sBx + m_motorImpulse;

            vA -= m_invMassA * P;
            wA -= m_invIA * LA;

            vB += m_invMassB * P;
            wB += m_invIB * LB;
        }
        else
        {
            m_impulse = 0.0f;
            m_springImpulse = 0.0f;
            m_motorImpulse = 0.0f;
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
            float32 mA = m_invMassA, mB = m_invMassB;
        float32 iA = m_invIA, iB = m_invIB;

        b2Vec2 vA = data.velocities[m_indexA].v;
        float32 wA = data.velocities[m_indexA].w;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        // Solve spring constraint
        {
            float32 Cdot = b2Dot(m_ax, vB - vA) + m_sBx * wB - m_sAx * wA;
            float32 impulse = -m_springMass * (Cdot + m_bias + m_gamma * m_springImpulse);
            m_springImpulse += impulse;

            b2Vec2 P = impulse * m_ax;
            float32 LA = impulse * m_sAx;
            float32 LB = impulse * m_sBx;

            vA -= mA * P;
            wA -= iA * LA;

            vB += mB * P;
            wB += iB * LB;
        }

        // Solve rotational motor constraint
        {
            float32 Cdot = wB - wA - m_motorSpeed;
            float32 impulse = -m_motorMass * Cdot;

            float32 oldImpulse = m_motorImpulse;
            float32 maxImpulse = data.step.dt * m_maxMotorTorque;
            m_motorImpulse = b2Clamp(m_motorImpulse + impulse, -maxImpulse, maxImpulse);
            impulse = m_motorImpulse - oldImpulse;

            wA -= iA * impulse;
            wB += iB * impulse;
        }

        // Solve point to line constraint
        {
            float32 Cdot = b2Dot(m_ay, vB - vA) + m_sBy * wB - m_sAy * wA;
            float32 impulse = -m_mass * Cdot;
            m_impulse += impulse;

            b2Vec2 P = impulse * m_ay;
            float32 LA = impulse * m_sAy;
            float32 LB = impulse * m_sBy;

            vA -= mA * P;
            wA -= iA * LA;

            vB += mB * P;
            wB += iB * LB;
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

        b2Vec2 rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        b2Vec2 rB = b2Mul(qB, m_localAnchorB - m_localCenterB);
        b2Vec2 d = (cB - cA) + rB - rA;

        b2Vec2 ay = b2Mul(qA, m_localYAxisA);

        float32 sAy = b2Cross(d + rA, ay);
        float32 sBy = b2Cross(rB, ay);

        float32 C = b2Dot(d, ay);

        float32 k = m_invMassA + m_invMassB + m_invIA * m_sAy * m_sAy + m_invIB * m_sBy * m_sBy;

        float32 impulse;
        if (k != 0.0f)
        {
            impulse = - C / k;
        }
        else
        {
            impulse = 0.0f;
        }

        b2Vec2 P = impulse * ay;
        float32 LA = impulse * sAy;
        float32 LB = impulse * sBy;

        cA -= m_invMassA * P;
        aA -= m_invIA * LA;
        cB += m_invMassB * P;
        aB += m_invIB * LB;

        data.positions[m_indexA].c = cA;
        data.positions[m_indexA].a = aA;
        data.positions[m_indexB].c = cB;
        data.positions[m_indexB].a = aB;

        return b2Abs(C) <= b2_linearSlop;
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
            return inv_dt * (m_impulse * m_ay + m_springImpulse * m_ax);
        */
    }

    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            return inv_dt * m_motorImpulse;
        */
    }

    /**
      | Get the current joint translation,
      | usually in meters.
      |
      */
    pub fn get_joint_translation(&self) -> f32 {
        
        todo!();
        /*
            b2Body* bA = m_bodyA;
        b2Body* bB = m_bodyB;

        b2Vec2 pA = bA->GetWorldPoint(m_localAnchorA);
        b2Vec2 pB = bB->GetWorldPoint(m_localAnchorB);
        b2Vec2 d = pB - pA;
        b2Vec2 axis = bA->GetWorldVector(m_localXAxisA);

        float32 translation = b2Dot(d, axis);
        return translation;
        */
    }

    /**
      | Get the current joint translation speed,
      | usually in meters per second.
      |
      */
    pub fn get_joint_speed(&self) -> f32 {
        
        todo!();
        /*
            float32 wA = m_bodyA->m_angularVelocity;
        float32 wB = m_bodyB->m_angularVelocity;
        return wB - wA;
        */
    }

    /**
      | Is the joint motor enabled?
      |
      */
    pub fn is_motor_enabled(&self) -> bool {
        
        todo!();
        /*
            return m_enableMotor;
        */
    }

    /**
      | Enable/disable the joint motor.
      |
      */
    pub fn enable_motor(&mut self, flag: bool)  {
        
        todo!();
        /*
            m_bodyA->SetAwake(true);
        m_bodyB->SetAwake(true);
        m_enableMotor = flag;
        */
    }

    /**
      | Set the motor speed, usually in radians
      | per second.
      |
      */
    pub fn set_motor_speed(&mut self, speed: f32)  {
        
        todo!();
        /*
            m_bodyA->SetAwake(true);
        m_bodyB->SetAwake(true);
        m_motorSpeed = speed;
        */
    }

    /**
      | Set/Get the maximum motor force, usually
      | in N-m.
      |
      */
    pub fn set_max_motor_torque(&mut self, torque: f32)  {
        
        todo!();
        /*
            m_bodyA->SetAwake(true);
        m_bodyB->SetAwake(true);
        m_maxMotorTorque = torque;
        */
    }

    /**
      | Get the current motor torque given the
      | inverse time step, usually in N-m.
      |
      */
    pub fn get_motor_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            return inv_dt * m_motorImpulse;
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

        b2Log("  b2WheelJointDef jd;\n");
        b2Log("  jd.bodyA = bodies[%d];\n", indexA);
        b2Log("  jd.bodyB = bodies[%d];\n", indexB);
        b2Log("  jd.collideConnected = bool(%d);\n", m_collideConnected);
        b2Log("  jd.localAnchorA.Set(%.15lef, %.15lef);\n", m_localAnchorA.x, m_localAnchorA.y);
        b2Log("  jd.localAnchorB.Set(%.15lef, %.15lef);\n", m_localAnchorB.x, m_localAnchorB.y);
        b2Log("  jd.localAxisA.Set(%.15lef, %.15lef);\n", m_localXAxisA.x, m_localXAxisA.y);
        b2Log("  jd.enableMotor = bool(%d);\n", m_enableMotor);
        b2Log("  jd.motorSpeed = %.15lef;\n", m_motorSpeed);
        b2Log("  jd.maxMotorTorque = %.15lef;\n", m_maxMotorTorque);
        b2Log("  jd.frequencyHz = %.15lef;\n", m_frequencyHz);
        b2Log("  jd.dampingRatio = %.15lef;\n", m_dampingRatio);
        b2Log("  joints[%d] = m_world->CreateJoint(&jd);\n", m_index);
        */
    }
}
