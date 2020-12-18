use std::collections::HashSet;
use std::hash::Hash;

pub trait Point where Self: Sized + Eq + Hash {
    fn neighbors(&self) -> Vec<Self>;
    fn points_to_check(&self, other: &Self) -> Vec<Self>;
    fn partwise_min(points: &HashSet<Self>) -> Self;
    fn partwise_max(points: &HashSet<Self>) -> Self;
    fn partwise_decrease(&mut self, other: &Self);
    fn partwise_increase(&mut self, other: &Self);
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point3D(i32, i32, i32);

impl Point for Point3D {
    fn neighbors(&self) -> Vec<Point3D> {
        let &Point3D(x, y, z) = self;
        (-1..=1).flat_map(|dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).map(move |dz| Point3D(x + dx, y + dy, z + dz))
            })
        }).filter(|point| point != self).collect()
    }

    fn points_to_check(&self, other: &Self) -> Vec<Self> {
        let Point3D(x, y, z) = self;
        let Point3D(x2, y2, z2) = other;

        ((x-1)..=(x2+1)).flat_map(|x| {
            ((y-1)..=(y2+1)).flat_map(move |y| {
                ((z-1)..=(z2+1)).map(move |z| {
                    Point3D(x, y, z)
                })
            })
        }).collect()
    }

    fn partwise_min(points: &HashSet<Self>) -> Self {
        Point3D(
            points.iter().map(|Point3D(x, _, _)| *x).min().unwrap(),
            points.iter().map(|Point3D(_, y, _)| *y).min().unwrap(),
            points.iter().map(|Point3D(_, _, z)| *z).min().unwrap(),
        )
    }

    fn partwise_max(points: &HashSet<Self>) -> Self {
        Point3D(
            points.iter().map(|Point3D(x, _, _)| *x).max().unwrap(),
            points.iter().map(|Point3D(_, y, _)| *y).max().unwrap(),
            points.iter().map(|Point3D(_, _, z)| *z).max().unwrap(),
        )
    }

    fn partwise_decrease(&mut self, other: &Self) {
        if other.0 < self.0 {
            self.0 = other.0;
        }
        if other.1 < self.1 {
            self.1 = other.1;
        }
        if other.2 < self.2 {
            self.2 = other.2;
        }
    }

    fn partwise_increase(&mut self, other: &Self) {
        if other.0 > self.0 {
            self.0 = other.0;
        }
        if other.1 > self.1 {
            self.1 = other.1;
        }
        if other.2 > self.2 {
            self.2 = other.2;
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point4D(i32, i32, i32, i32);

impl Point for Point4D {
    fn neighbors(&self) -> Vec<Self> {
        let &Point4D(x, y, z, w) = self;
        (-1..=1).flat_map(|dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).flat_map(move |dz| {
                    (-1..=1).map(move |dw| Point4D(x + dx, y + dy, z + dz, w + dw))
                })
            })
        }).filter(|point| point != self).collect()
    }

    fn points_to_check(&self, other: &Self) -> Vec<Self> {
        let Point4D(x, y, z, w) = self;
        let Point4D(x2, y2, z2, w2) = other;

        ((x-1)..=(x2+1)).flat_map(|x| {
            ((y-1)..=(y2+1)).flat_map(move |y| {
                ((z-1)..=(z2+1)).flat_map(move |z| {
                    ((w-1)..=(w2+1)).map(move |w| {
                        Point4D(x, y, z, w)
                    })
                })
            })
        }).collect()
    }

    fn partwise_min(points: &HashSet<Self>) -> Self {
        Point4D(
            points.iter().map(|Point4D(x, _, _, _)| *x).min().unwrap(),
            points.iter().map(|Point4D(_, y, _, _)| *y).min().unwrap(),
            points.iter().map(|Point4D(_, _, z, _)| *z).min().unwrap(),
            points.iter().map(|Point4D(_, _, _, w)| *w).min().unwrap(),
        )
    }

    fn partwise_max(points: &HashSet<Self>) -> Self {
        Point4D(
            points.iter().map(|Point4D(x, _, _, _)| *x).max().unwrap(),
            points.iter().map(|Point4D(_, y, _, _)| *y).max().unwrap(),
            points.iter().map(|Point4D(_, _, z, _)| *z).max().unwrap(),
            points.iter().map(|Point4D(_, _, _, w)| *w).max().unwrap(),
        )
    }

    fn partwise_decrease(&mut self, other: &Self) {
        if other.0 < self.0 {
            self.0 = other.0;
        }
        if other.1 < self.1 {
            self.1 = other.1;
        }
        if other.2 < self.2 {
            self.2 = other.2;
        }
        if other.3 < self.3 {
            self.3 = other.3;
        }
    }

    fn partwise_increase(&mut self, other: &Self) {
        if other.0 > self.0 {
            self.0 = other.0;
        }
        if other.1 > self.1 {
            self.1 = other.1;
        }
        if other.2 > self.2 {
            self.2 = other.2;
        }
        if other.3 > self.3 {
            self.3 = other.3;
        }
    }
}

#[derive(Debug)]
pub struct Simulation<P: Point> {
    active_points: HashSet<P>,
    min_corner: P,
    max_corner: P,
}

impl Simulation<Point3D> {
    pub fn from_3d(s: &str) -> Self {
        let points: HashSet<Point3D> = s.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                match c {
                    '#' => Some(Point3D(x as i32, y.clone() as i32, 0)),
                    _ => None,
                }
            })
        }).collect();

        let min = Point3D::partwise_min(&points);
        let max = Point3D::partwise_max(&points);

        Simulation {
            active_points: points,
            min_corner: min,
            max_corner: max,
        }
    }
}

impl Simulation<Point4D> {
    pub fn from_4d(s: &str) -> Self {
        let points: HashSet<Point4D> = s.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                match c {
                    '#' => Some(Point4D(x as i32, y.clone() as i32, 0, 0)),
                    _ => None,
                }
            })
        }).collect();

        let min = Point4D::partwise_min(&points);
        let max = Point4D::partwise_max(&points);

        Simulation {
            active_points: points,
            min_corner: min,
            max_corner: max,
        }
    }
}

impl<P: Point> Simulation<P> {
    pub fn step(&mut self) {
        let mut changes: Vec<(P, bool)> = Vec::new();

        for point in self.min_corner.points_to_check(&self.max_corner) {
            if self.active_points.contains(&point) {
                let active_neighbors = self.num_active_neighbors(&point);
                if active_neighbors != 2 && active_neighbors != 3 {
                    changes.push((point, false));
                }
            } else {
                if self.num_active_neighbors(&point) == 3 {
                    self.min_corner.partwise_decrease(&point);
                    self.max_corner.partwise_increase(&point);
                    changes.push((point, true));
                }
            }
        }

        for (point, active) in changes {
            if active {
                self.active_points.insert(point);
            } else {
                self.active_points.remove(&point);
            }
        }
    }

    pub fn simulate(&mut self, num_cycles: usize) {
        for _ in 0..num_cycles {
            self.step();
        }
    }

    pub fn num_active_points(&self) -> usize {
        self.active_points.len()
    }

    fn num_active_neighbors(&self, point: &P) -> usize {
        point.neighbors().iter()
            .filter(|p| self.active_points.contains(*p))
            .count()
    }
}