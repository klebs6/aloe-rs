crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2TimeOfImpact.h]

/**
   Input parameters for b2TimeOfImpact
  */
pub struct b2TOIInput
{
    proxya: b2DistanceProxy,
    proxyb: b2DistanceProxy,
    sweepa: b2Sweep,
    sweepb: b2Sweep,

    /**
       defines sweep interval [0, tMax]
      */
    t_max:  f32,
}

pub enum B2ToiOutputState
{
    e_unknown,
    e_failed,
    e_overlapped,
    e_touching,
    e_separated
}

/**
  | Output parameters for b2TimeOfImpact.
  |
  */
pub struct b2TOIOutput {
    state: B2ToiOutputState,
    t:     f32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2TimeOfImpact.cpp]

lazy_static!{
    /*
    int32 b2_toiCalls, b2_toiIters, b2_toiMaxIters;
    int32 b2_toiRootIters, b2_toiMaxRootIters;
    */
}

pub enum B2SeparationFunctionType
{
    Points,
    FaceA,
    FaceB
}

pub struct b2SeparationFunction {
    proxya:      *const b2DistanceProxy,
    proxyb:      *const b2DistanceProxy,
    sweepa:      b2Sweep,
    sweepb:      b2Sweep,
    ty:          B2SeparationFunctionType,
    local_point: b2Vec2,
    axis:        b2Vec2,
}

impl b2SeparationFunction {

    /**
      | TODO_ERIN might not need to return the
      | separation
      |
      */
    pub fn initialize(&mut self, 
        cache:  *const b2SimplexCache,
        proxya: *const b2DistanceProxy,
        sweepa: &b2Sweep,
        proxyb: *const b2DistanceProxy,
        sweepb: &b2Sweep,
        t1:     f32) -> f32 {
        
        todo!();
        /*
            m_proxyA = proxyA;
            m_proxyB = proxyB;
            int32 count = cache->count;
            b2Assert(0 < count && count < 3);

            m_sweepA = sweepA;
            m_sweepB = sweepB;

            b2Transform xfA, xfB;
            m_sweepA.GetTransform(&xfA, t1);
            m_sweepB.GetTransform(&xfB, t1);

            if (count == 1)
            {
                m_type = e_points;
                b2Vec2 localPointA = m_proxyA->GetVertex(cache->indexA[0]);
                b2Vec2 localPointB = m_proxyB->GetVertex(cache->indexB[0]);
                b2Vec2 pointA = b2Mul(xfA, localPointA);
                b2Vec2 pointB = b2Mul(xfB, localPointB);
                m_axis = pointB - pointA;
                float32 s = m_axis.Normalize();
                return s;
            }
            else if (cache->indexA[0] == cache->indexA[1])
            {
                // Two points on B and one on A.
                m_type = e_faceB;
                b2Vec2 localPointB1 = proxyB->GetVertex(cache->indexB[0]);
                b2Vec2 localPointB2 = proxyB->GetVertex(cache->indexB[1]);

                m_axis = b2Cross(localPointB2 - localPointB1, 1.0f);
                m_axis.Normalize();
                b2Vec2 normal = b2Mul(xfB.q, m_axis);

                m_localPoint = 0.5f * (localPointB1 + localPointB2);
                b2Vec2 pointB = b2Mul(xfB, m_localPoint);

                b2Vec2 localPointA = proxyA->GetVertex(cache->indexA[0]);
                b2Vec2 pointA = b2Mul(xfA, localPointA);

                float32 s = b2Dot(pointA - pointB, normal);
                if (s < 0.0f)
                {
                    m_axis = -m_axis;
                    s = -s;
                }
                return s;
            }
            else
            {
                // Two points on A and one or two points on B.
                m_type = e_faceA;
                b2Vec2 localPointA1 = m_proxyA->GetVertex(cache->indexA[0]);
                b2Vec2 localPointA2 = m_proxyA->GetVertex(cache->indexA[1]);

                m_axis = b2Cross(localPointA2 - localPointA1, 1.0f);
                m_axis.Normalize();
                b2Vec2 normal = b2Mul(xfA.q, m_axis);

                m_localPoint = 0.5f * (localPointA1 + localPointA2);
                b2Vec2 pointA = b2Mul(xfA, m_localPoint);

                b2Vec2 localPointB = m_proxyB->GetVertex(cache->indexB[0]);
                b2Vec2 pointB = b2Mul(xfB, localPointB);

                float32 s = b2Dot(pointB - pointA, normal);
                if (s < 0.0f)
                {
                    m_axis = -m_axis;
                    s = -s;
                }
                return s;
            }
        */
    }
    
    pub fn find_min_separation(&self, 
        indexa: *mut i32,
        indexb: *mut i32,
        t:      f32) -> f32 {
        
        todo!();
        /*
            b2Transform xfA, xfB;
            m_sweepA.GetTransform(&xfA, t);
            m_sweepB.GetTransform(&xfB, t);

            switch (m_type)
            {
            case e_points:
                {
                    b2Vec2 axisA = b2MulT(xfA.q,  m_axis);
                    b2Vec2 axisB = b2MulT(xfB.q, -m_axis);

                    *indexA = m_proxyA->GetSupport(axisA);
                    *indexB = m_proxyB->GetSupport(axisB);

                    b2Vec2 localPointA = m_proxyA->GetVertex(*indexA);
                    b2Vec2 localPointB = m_proxyB->GetVertex(*indexB);

                    b2Vec2 pointA = b2Mul(xfA, localPointA);
                    b2Vec2 pointB = b2Mul(xfB, localPointB);

                    float32 separation = b2Dot(pointB - pointA, m_axis);
                    return separation;
                }

            case e_faceA:
                {
                    b2Vec2 normal = b2Mul(xfA.q, m_axis);
                    b2Vec2 pointA = b2Mul(xfA, m_localPoint);

                    b2Vec2 axisB = b2MulT(xfB.q, -normal);

                    *indexA = -1;
                    *indexB = m_proxyB->GetSupport(axisB);

                    b2Vec2 localPointB = m_proxyB->GetVertex(*indexB);
                    b2Vec2 pointB = b2Mul(xfB, localPointB);

                    float32 separation = b2Dot(pointB - pointA, normal);
                    return separation;
                }

            case e_faceB:
                {
                    b2Vec2 normal = b2Mul(xfB.q, m_axis);
                    b2Vec2 pointB = b2Mul(xfB, m_localPoint);

                    b2Vec2 axisA = b2MulT(xfA.q, -normal);

                    *indexB = -1;
                    *indexA = m_proxyA->GetSupport(axisA);

                    b2Vec2 localPointA = m_proxyA->GetVertex(*indexA);
                    b2Vec2 pointA = b2Mul(xfA, localPointA);

                    float32 separation = b2Dot(pointA - pointB, normal);
                    return separation;
                }

            default:
                b2Assert(false);
                *indexA = -1;
                *indexB = -1;
                return 0.0f;
            }
        */
    }
    
    pub fn evaluate(&self, 
        indexa: i32,
        indexb: i32,
        t:      f32) -> f32 {
        
        todo!();
        /*
            b2Transform xfA, xfB;
            m_sweepA.GetTransform(&xfA, t);
            m_sweepB.GetTransform(&xfB, t);

            switch (m_type)
            {
            case e_points:
                {
                    b2Vec2 localPointA = m_proxyA->GetVertex(indexA);
                    b2Vec2 localPointB = m_proxyB->GetVertex(indexB);

                    b2Vec2 pointA = b2Mul(xfA, localPointA);
                    b2Vec2 pointB = b2Mul(xfB, localPointB);
                    float32 separation = b2Dot(pointB - pointA, m_axis);

                    return separation;
                }

            case e_faceA:
                {
                    b2Vec2 normal = b2Mul(xfA.q, m_axis);
                    b2Vec2 pointA = b2Mul(xfA, m_localPoint);

                    b2Vec2 localPointB = m_proxyB->GetVertex(indexB);
                    b2Vec2 pointB = b2Mul(xfB, localPointB);

                    float32 separation = b2Dot(pointB - pointA, normal);
                    return separation;
                }

            case e_faceB:
                {
                    b2Vec2 normal = b2Mul(xfB.q, m_axis);
                    b2Vec2 pointB = b2Mul(xfB, m_localPoint);

                    b2Vec2 localPointA = m_proxyA->GetVertex(indexA);
                    b2Vec2 pointA = b2Mul(xfA, localPointA);

                    float32 separation = b2Dot(pointA - pointB, normal);
                    return separation;
                }

            default:
                b2Assert(false);
                return 0.0f;
            }
        */
    }
}

/**
  | Compute the upper bound on time before two
  | shapes penetrate. Time is represented as
  | a fraction between [0,tMax]. This uses a swept
  | separating axis and may miss some
  | intermediate, non-tunneling collision. If you
  | change the time interval, you should call this
  | function again.
  |
  | Note: use b2Distance to compute the contact
  | point and normal at the time of impact.
  |
  | CCD via the local separating axis method. This
  | seeks progression by computing the largest time
  | at which separation is maintained.
  */
pub fn b2time_of_impact(
        output: *mut b2TOIOutput,
        input:  *const b2TOIInput)  {
    
    todo!();
    /*
        ++b2_toiCalls;

        output->state = b2TOIOutput::e_unknown;
        output->t = input->tMax;

        const b2DistanceProxy* proxyA = &input->proxyA;
        const b2DistanceProxy* proxyB = &input->proxyB;

        b2Sweep sweepA = input->sweepA;
        b2Sweep sweepB = input->sweepB;

        // Large rotations can make the root finder fail, so we normalize the
        // sweep angles.
        sweepA.Normalize();
        sweepB.Normalize();

        float32 tMax = input->tMax;

        float32 totalRadius = proxyA->m_radius + proxyB->m_radius;
        float32 target = b2Max(b2_linearSlop, totalRadius - 3.0f * b2_linearSlop);
        float32 tolerance = 0.25f * b2_linearSlop;
        b2Assert(target > tolerance);

        float32 t1 = 0.0f;
        const int32 k_maxIterations = 20;   // TODO_ERIN b2Settings
        int32 iter = 0;

        // Prepare input for distance query.
        b2SimplexCache cache;
        cache.count = 0;
        b2DistanceInput distanceInput;
        distanceInput.proxyA = input->proxyA;
        distanceInput.proxyB = input->proxyB;
        distanceInput.useRadii = false;

        // The outer loop progressively attempts to compute new separating axes.
        // This loop terminates when an axis is repeated (no progress is made).
        for(;;)
        {
            b2Transform xfA, xfB;
            sweepA.GetTransform(&xfA, t1);
            sweepB.GetTransform(&xfB, t1);

            // Get the distance between shapes. We can also use the results
            // to get a separating axis.
            distanceInput.transformA = xfA;
            distanceInput.transformB = xfB;
            b2DistanceOutput distanceOutput;
            b2Distance(&distanceOutput, &cache, &distanceInput);

            // If the shapes are overlapped, we give up on continuous collision.
            if (distanceOutput.distance <= 0.0f)
            {
                // Failure!
                output->state = b2TOIOutput::e_overlapped;
                output->t = 0.0f;
                break;
            }

            if (distanceOutput.distance < target + tolerance)
            {
                // Victory!
                output->state = b2TOIOutput::e_touching;
                output->t = t1;
                break;
            }

            // Initialize the separating axis.
            b2SeparationFunction fcn;
            fcn.Initialize(&cache, proxyA, sweepA, proxyB, sweepB, t1);
    #if 0
            // Dump the curve seen by the root finder
            {
                const int32 N = 100;
                float32 dx = 1.0f / N;
                float32 xs[N+1];
                float32 fs[N+1];

                float32 x = 0.0f;

                for (int32 i = 0; i <= N; ++i)
                {
                    sweepA.GetTransform(&xfA, x);
                    sweepB.GetTransform(&xfB, x);
                    float32 f = fcn.Evaluate(xfA, xfB) - target;

                    printf("%g %g\n", x, f);

                    xs[i] = x;
                    fs[i] = f;

                    x += dx;
                }
            }
    #endif

            // Compute the TOI on the separating axis. We do this by successively
            // resolving the deepest point. This loop is bounded by the number of vertices.
            bool done = false;
            float32 t2 = tMax;
            int32 pushBackIter = 0;
            for (;;)
            {
                // Find the deepest point at t2. Store the witness point indices.
                int32 indexA, indexB;
                float32 s2 = fcn.FindMinSeparation(&indexA, &indexB, t2);

                // Is the final configuration separated?
                if (s2 > target + tolerance)
                {
                    // Victory!
                    output->state = b2TOIOutput::e_separated;
                    output->t = tMax;
                    done = true;
                    break;
                }

                // Has the separation reached tolerance?
                if (s2 > target - tolerance)
                {
                    // Advance the sweeps
                    t1 = t2;
                    break;
                }

                // Compute the initial separation of the witness points.
                float32 s1 = fcn.Evaluate(indexA, indexB, t1);

                // Check for initial overlap. This might happen if the root finder
                // runs out of iterations.
                if (s1 < target - tolerance)
                {
                    output->state = b2TOIOutput::e_failed;
                    output->t = t1;
                    done = true;
                    break;
                }

                // Check for touching
                if (s1 <= target + tolerance)
                {
                    // Victory! t1 should hold the TOI (could be 0.0).
                    output->state = b2TOIOutput::e_touching;
                    output->t = t1;
                    done = true;
                    break;
                }

                // Compute 1D root of: f(x) - target = 0
                int32 rootIterCount = 0;
                float32 a1 = t1, a2 = t2;
                for (;;)
                {
                    // Use a mix of the secant rule and bisection.
                    float32 t;
                    if (rootIterCount & 1)
                    {
                        // Secant rule to improve convergence.
                        t = a1 + (target - s1) * (a2 - a1) / (s2 - s1);
                    }
                    else
                    {
                        // Bisection to guarantee progress.
                        t = 0.5f * (a1 + a2);
                    }

                    float32 s = fcn.Evaluate(indexA, indexB, t);

                    if (b2Abs(s - target) < tolerance)
                    {
                        // t2 holds a tentative value for t1
                        t2 = t;
                        break;
                    }

                    // Ensure we continue to bracket the root.
                    if (s > target)
                    {
                        a1 = t;
                        s1 = s;
                    }
                    else
                    {
                        a2 = t;
                        s2 = s;
                    }

                    ++rootIterCount;
                    ++b2_toiRootIters;

                    if (rootIterCount == 50)
                    {
                        break;
                    }
                }

                b2_toiMaxRootIters = b2Max(b2_toiMaxRootIters, rootIterCount);

                ++pushBackIter;

                if (pushBackIter == b2_maxPolygonVertices)
                {
                    break;
                }
            }

            ++iter;
            ++b2_toiIters;

            if (done)
            {
                break;
            }

            if (iter == k_maxIterations)
            {
                // Root finder got stuck. Semi-victory.
                output->state = b2TOIOutput::e_failed;
                output->t = t1;
                break;
            }
        }

        b2_toiMaxIters = b2Max(b2_toiMaxIters, iter);
    */
}
