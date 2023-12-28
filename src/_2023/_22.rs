use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Space {
    bricks: Vec<Rc<RefCell<Brick>>>,
}

impl Space {
    pub fn settle(&mut self) {
        // 🤔
    }
    fn keystones(&self) -> usize {
        self.bricks
            .iter()
            .filter(|brick| {
                brick
                    .borrow()
                    .supporting
                    .iter()
                    .any(|brick| brick.supported_by.len() == 1)
            })
            .count()
    }
    pub fn disintegrateable_bricks(&self) -> usize {
        self.bricks.len() - self.keystones()
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}
pub struct Brick {
    id: usize,
    start: Point3D,
    end: Point3D,
    bottom_face: HashSet<Point3D>,
    top_face: HashSet<Point3D>,
    supporting: Vec<Brick>,
    supported_by: Vec<Brick>,
}

impl Brick {
    fn supports(&self, above: &Brick) -> bool {
        above.is_supported_by(self)
    }
    fn is_supported_by(&self, below: &Brick) -> bool {
        self.is_directly_above(below) && self.faces_overlap(below)
    }
    fn is_directly_below(&self, above: &Brick) -> bool {
        above.is_directly_above(self)
    }
    fn is_directly_above(&self, below: &Brick) -> bool {
        self.start.z == below.end.z + 1
    }
    fn faces_overlap(&self, below: &Brick) -> bool {
        self.bottom_face
            .intersection(&below.bottom_face)
            .next()
            .is_some()
    }
    fn shift_down(&mut self, below: &Brick) {
        let height = self.end.z - self.start.z + 1;
        self.start.z = below.start.z + 1;
        self.end.z = below.start.z + height;
    }
}

#[derive(Copy, Clone)]
enum Face {
    Top,
    Bottom,
}

fn get_face(start: Point3D, end: Point3D, face: Face) -> HashSet<Point3D> {
    let mut set = HashSet::new();
    for x in start.x..=end.x {
        for y in start.y..=end.y {
            set.insert(Point3D {
                x,
                y,
                z: if matches!(face, Face::Bottom) {
                    start.z
                } else {
                    end.z
                },
            });
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
                    .map(|(start, end)| {
                        Rc::new(RefCell::new(Brick {
                            id,
                            start,
                            end,
                            bottom_face: get_face(start, end, Face::Bottom),
                            top_face: get_face(start, end, Face::Top),
                            supporting: Vec::new(),
                            supported_by: Vec::new(),
                        }))
                    })
                    .unwrap()
            })
            .sorted_by_key(|brick| brick.borrow().start.z)
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

        assert_eq!(space.disintegrateable_bricks(), 1 + 1);
    }

    #[test]
    fn test_1() {
        let mut space = parse(INPUT);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 1 + 1);
    }

    #[test]
    fn test_2_sample() {
        let mut space = parse(SAMPLE);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 1 + 1);
    }

    #[test]
    fn test_2() {
        let mut space = parse(INPUT);
        space.settle();

        assert_eq!(space.disintegrateable_bricks(), 1 + 1);
    }
}
