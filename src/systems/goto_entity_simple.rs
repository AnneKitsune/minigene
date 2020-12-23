use crate::*;

pub fn GotoEntitySimpleSystem(entities: &Entities,
                                 positions: &mut Components<
    Point,
>,
                                 gotos: &Components<
    GotoEntity,
>) {
    let mut v = vec![];
    for (e, _, goto) in join!(&entities && &positions && &gotos){
        v.push((e.unwrap(), goto.unwrap().entity.clone(), goto.unwrap().speed));
    }
    for (e, t, speed) in v {
        if let Some(target) = positions.get(t).map(|p| p.clone()) {
            let mut p = positions.get_mut(e).unwrap();
            // TODO improve when we have a Time struct
            for _i in 0..(speed as usize) {
                let delta_x = target.x - p.x;
                let delta_y = target.y - p.y;
                if delta_x.abs() >= delta_y.abs() {
                    if delta_x > 0 {
                        p.x += 1;
                    } else if delta_x < 0 {
                        p.x -= 1;
                    }
                } else {
                    if delta_y > 0 {
                        p.y += 1;
                    } else if delta_y < 0 {
                        p.y -= 1;
                    }
                }
            }
        }
    }
}
