use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Space {
    bricks: Vec<Brick>,
}

impl Space {
    pub fn settle(&mut self) {
        let mut ret: Vec<Brick> = vec![];
        for brick in &self.bricks {
            let mut current = *brick;
            let mut is_settled = false;
            while !is_settled {
                let supporters = ret
                    .iter()
                    .filter(|supporter| supporter.supports(&current))
                    .collect::<Vec<_>>();
                if supporters.is_empty() && !current.on_ground() {
                    let next_highest = ret
                        .iter()
                        .filter(|b| b.end.z < current.start.z - 1)
                        .map(|b| b.end.z)
                        .max()
                        .map_or(1, |it| it + 1);

                    current = fall_down(&current, next_highest);
                } else {
                    is_settled = true;
                    ret.push(current);
                }
            }
        }
        self.bricks = ret;
    }

    fn gen_lookups(&self) -> (HashMap<Brick, Vec<Brick>>, HashMap<Brick, Vec<Brick>>) {
        let mut supports: HashMap<Brick, Vec<Brick>> = HashMap::new();
        let mut supported_by: HashMap<Brick, Vec<Brick>> = HashMap::new();
        for a in &self.bricks {
            for b in &self.bricks {
                if a.supports(b) {
                    supports.entry(*a).or_default().push(*b);
                } else if a.is_supported_by(b) {
                    supported_by.entry(*a).or_default().push(*b);
                }
            }
        }
        (supports, supported_by)
    }
    pub fn disintegrateable_bricks(&self) -> usize {
        let (supports, supported_by) = self.gen_lookups();
        self.bricks.len() - self.sole_supporters(&supports, &supported_by).len()
    }

    pub fn sole_supporters(
        &self,
        supports: &HashMap<Brick, Vec<Brick>>,
        supported_by: &HashMap<Brick, Vec<Brick>>,
    ) -> Vec<Brick> {
        self.bricks
            .iter()
            .filter(|brick| {
                let Some(supported) = supports.get(brick) else {
                    return false;
                };
                supported
                    .iter()
                    .any(|above| supported_by.get(above).unwrap().len() == 1)
            })
            .copied()
            .collect::<Vec<_>>()
    }

    pub fn chain_reaction_bricks(&self) -> usize {
        let (supports, supported_by) = self.gen_lookups();
        let sole_supporters = self.sole_supporters(&supports, &supported_by);

        let mut total_count = 0;

        for brick in sole_supporters {
            let mut untoppled = self
                .bricks
                .iter()
                .filter(|b| **b != brick)
                .copied()
                .collect::<HashSet<_>>();
            let mut toppled: HashSet<Brick> = HashSet::from_iter(vec![brick]);
            loop {
                let will_fall = untoppled
                    .iter()
                    .filter(|b| supported_by.get(b).is_some_and(|s| !s.is_empty()))
                    .filter(|b| {
                        supported_by
                            .get(b)
                            .unwrap()
                            .iter()
                            .all(|b| toppled.contains(b))
                    })
                    .copied()
                    .collect::<Vec<_>>();
                for b in &will_fall {
                    untoppled.remove(b);
                    toppled.insert(*b);
                }
                if will_fall.is_empty() {
                    break;
                }
            }
            total_count += toppled.len() - 1;
        }
        total_count
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Brick {
    id: usize,
    start: Point3D,
    end: Point3D,
}

impl Brick {
    fn supports(&self, above: &Brick) -> bool {
        self.covers(above) && self.end.z + 1 == above.start.z
    }
    fn is_supported_by(&self, below: &Brick) -> bool {
        below.covers(self) && self.start.z == below.end.z + 1
    }
    fn covers(&self, above: &Brick) -> bool {
        self.start.x <= above.end.x
            && self.end.x >= above.start.x
            && self.start.y <= above.end.y
            && self.end.y >= above.start.y
    }
    fn on_ground(&self) -> bool {
        self.start.z == 1
    }
}
fn fall_down(a: &Brick, z: u32) -> Brick {
    let mut next = *a;
    next.start.z = z;
    next.end.z = z + (a.end.z - a.start.z);
    next
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
                        // face: get_face(start, end),
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
    fn test_1() {
        let mut space = parse(INPUT);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 461);
    }

    #[test]
    fn test_2_sample() {
        let mut space = parse(SAMPLE);
        space.settle();

        assert_eq!(space.chain_reaction_bricks(), 7);
    }

    #[test]
    fn test_2() {
        let mut space = parse(INPUT);
        space.settle();

        assert_eq!(space.chain_reaction_bricks(), 74_074);
    }
}
