use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub struct Space {
    bricks: Vec<Brick>,
}

impl Space {
    pub fn settle(&mut self) {
        let mut ret = vec![];
        for brick in &self.bricks {
            let mut current = brick.clone();
            let mut is_settled = false;
            while !is_settled {
                let supporters = self
                    .bricks
                    .iter()
                    .filter(|supporter| brick.is_supported_by(supporter))
                    .collect::<Vec<_>>();
                if supporters.is_empty() && !current.on_ground() {
                    let next_highest = self
                        .bricks
                        .iter()
                        .filter(|b| b.end.z < current.start.z - 1)
                        .map(|b| b.end.z)
                        .max()
                        .unwrap_or(1);
                    // shift to ground
                    // for supporter in supporters {
                    current = fall_down(&current, next_highest);
                    // shift_down(brick, supporter);
                    // }
                    // fall to ground
                    // brick.start.z = 1;
                } else {
                    is_settled = true;
                    ret.push(current.clone());
                }
            }
        }
        self.bricks = ret;
    }
    pub fn disintegrateable_bricks(&self) -> usize {
        let bricks_by_bottom_z = self
            .bricks
            .iter()
            .group_by(|brick| brick.start.z)
            .into_iter()
            .map(|(ge0, group)| (ge0, group.cloned().collect()))
            .collect::<HashMap<u32, Vec<Brick>>>();
        let bricks_by_top_z = self
            .bricks
            .iter()
            .group_by(|brick| brick.end.z)
            .into_iter()
            .map(|(ge0, group)| (ge0, group.cloned().collect()))
            .collect::<HashMap<u32, Vec<Brick>>>();

        let below_to_above = self
            .bricks
            .iter()
            .map(|brick| {
                if let Some(above) = bricks_by_bottom_z.get(&(brick.end.z + 1)) {
                    (
                        brick.clone(),
                        above
                            .iter()
                            .filter(|a| brick.supports(a))
                            .cloned()
                            .collect(),
                    )
                } else {
                    (brick.clone(), vec![])
                }
            })
            .collect::<HashMap<Brick, Vec<Brick>>>();
        let above_to_below = self
            .bricks
            .iter()
            .map(|brick| {
                if let Some(below) = bricks_by_top_z.get(&(brick.start.z - 1)) {
                    (
                        brick.clone(),
                        below
                            .iter()
                            .filter(|a| brick.is_supported_by(a))
                            .cloned()
                            .collect(),
                    )
                } else {
                    (brick.clone(), vec![])
                }
            })
            .collect::<HashMap<Brick, Vec<Brick>>>();

        let sole_supporters = self
            .bricks
            .iter()
            .filter(|brick| {
                if let Some(above) = below_to_above.get(brick) {
                    above
                        .iter()
                        .any(|above| above_to_below.get(above).unwrap().len() == 1)
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        self.bricks.len() - sole_supporters.len()
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Brick {
    id: usize,
    start: Point3D,
    end: Point3D,
    face: HashSet<(u32, u32)>,
}

impl Hash for Brick {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Brick {
    fn supports(&self, above: &Brick) -> bool {
        above.is_supported_by(self)
    }
    fn is_supported_by(&self, below: &Brick) -> bool {
        self.is_directly_above(below) && self.covers(below)
    }
    fn is_directly_above(&self, below: &Brick) -> bool {
        self.start.z == below.end.z + 1
    }
    fn covers(&self, below: &Brick) -> bool {
        self.face.intersection(&below.face).next().is_some()
    }
    fn on_ground(&self) -> bool {
        self.start.z == 1
    }
}
fn fall_down(a: &Brick, z: u32) -> Brick {
    let mut next = a.clone();
    next.start.z = z;
    next.end.z = z + (a.end.z - a.start.z);
    next
}
fn get_face(start: Point3D, end: Point3D) -> HashSet<(u32, u32)> {
    let mut set = HashSet::new();
    for x in start.x..=end.x {
        for y in start.y..=end.y {
            set.insert((x, y));
        }
    }
    set
}

pub fn parse(input: &str) -> Space {
    Space {
        bricks: input
            .lines()
            .enumerate()
            .map(|(id, line)| {
                line.split('~')
                    .map(|xyz| {
                        xyz.split(',')
                            .map(|s| s.parse::<u32>().unwrap())
                            .collect_tuple::<(u32, u32, u32)>()
                            .map(|t| Point3D {
                                x: t.0,
                                y: t.1,
                                z: t.2,
                            })
                            .unwrap()
                    })
                    .collect_tuple::<(Point3D, Point3D)>()
                    .map(|(start, end)| Brick {
                        id,
                        start,
                        end,
                        face: get_face(start, end),
                    })
                    .unwrap()
            })
            .sorted_by_key(|brick| brick.start.z)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    const INPUT: &str = include_str!("../../input/2023/22.txt");

    #[test]
    fn test_1_sample() {
        let mut space = parse(SAMPLE);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 5);
    }

    #[test]
    #[ignore]
    fn test_1() {
        let mut space = parse(INPUT);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 461);
    }

    #[test]
    #[ignore]
    fn test_2_sample() {
        let mut space = parse(SAMPLE);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 7);
    }

    #[test]
    #[ignore]
    fn test_2() {
        let mut space = parse(INPUT);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 74_074);
    }
}
