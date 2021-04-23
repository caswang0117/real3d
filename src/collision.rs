use crate::geom::*;

#[derive(Clone, Copy, Debug)]
pub struct Contact<T: Copy> {
    pub a: T,
    pub b: T,
    pub mtv: Vec3,
}

pub fn restitute_dyn_stat<S1: Shape, S2: Shape>(
    ashapes: &mut [S1],
    avels: &mut [Vec3],
    bshapes: &[S2],
    contacts: &mut [Contact<usize>],
) where
    S1: Collide<S2>,
{
    contacts.sort_unstable_by(|a, b| b.mtv.magnitude2().partial_cmp(&a.mtv.magnitude2()).unwrap());
    for c in contacts.iter() {
        let a = c.a;
        let b = c.b;
        // Are they still touching?  This way we don't need to track disps or anything
        // at the expense of some extra collision checks
        if let Some(disp) = ashapes[a].disp(&bshapes[b]) {
            // We can imagine we're instantaneously applying a
            // velocity change to pop the object just above the floor.
            ashapes[a].translate(disp);
            // It feels a little weird to be adding displacement (in
            // units) to velocity (in units/frame), but we'll roll
            // with it.  We're not exactly modeling a normal force
            // here but it's something like that.
            avels[a] += disp;
        }
    }
}

pub fn restitute_dyn_dyn<S1: Shape, S2: Shape>(
    ashapes: &mut [S1],
    avels: &mut [Vec3],
    bshapes: &mut [S2],
    bvels: &mut [Vec3],
    contacts: &mut [Contact<usize>],
) where
    S1: Collide<S2>,
{
    contacts.sort_unstable_by(|a, b| b.mtv.magnitude2().partial_cmp(&a.mtv.magnitude2()).unwrap());
    // That can bump into each other in perfectly elastic collisions!
    for c in contacts.iter() {
        let a = c.a;
        let b = c.b;
        // Just split the difference.  In crowded situations this will
        // cause issues, but those will always be hard to solve with
        // this kind of technique.
        if let Some(disp) = ashapes[a].disp(&bshapes[b]) {
            ashapes[a].translate(-disp / 2.0);
            avels[a] -= disp / 2.0;
            bshapes[b].translate(disp / 2.0);
            bvels[b] += disp / 2.0;
        }
    }
}

pub fn restitute_dyns<S1: Shape>(
    ashapes: &mut [S1],
    avels: &mut [Vec3],
    contacts: &mut [Contact<usize>],
) where
    S1: Collide<S1>,
{
    contacts.sort_unstable_by(|a, b| b.mtv.magnitude2().partial_cmp(&a.mtv.magnitude2()).unwrap());
    // That can bump into each other in perfectly elastic collisions!
    for c in contacts.iter() {
        let a = c.a;
        let b = c.b;
        // Just split the difference.  In crowded situations this will
        // cause issues, but those will always be hard to solve with
        // this kind of technique.
        if let Some(disp) = ashapes[a].disp(&ashapes[b]) {
            ashapes[a].translate(-disp / 2.0);
            avels[a] -= disp / 2.0;
            ashapes[b].translate(disp / 2.0);
            avels[b] += disp / 2.0;
        }
    }
}

pub fn gather_contacts_ab<S1: Shape, S2: Shape>(a: &[S1], b: &[S2], into: &mut Vec<Contact<usize>>)
where
    S1: Collide<S2>,
{
    for (ai, a) in a.iter().enumerate() {
        for (bi, b) in b.iter().enumerate() {
            if let Some(disp) = a.disp(b) {
                into.push(Contact {
                    a: ai,
                    b: bi,
                    mtv: disp,
                });
            }
        }
    }
}

pub fn gather_contacts_aa<S1: Shape>(ss: &[S1], into: &mut Vec<Contact<usize>>)
where
    S1: Collide<S1>,
{
    for (ai, a) in ss.iter().enumerate() {
        for (bi, b) in ss[(ai + 1)..].iter().enumerate() {
            let bi = ai + 1 + bi;
            if let Some(disp) = a.disp(b) {
                into.push(Contact {
                    a: ai,
                    b: bi,
                    mtv: disp,
                });
            }
        }
    }
}
