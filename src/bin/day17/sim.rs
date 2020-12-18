use std::collections::HashSet;

pub trait Point where Self: Sized {
    fn neighbors(&self) -> Vec<Self>;
    fn points_to_check(&self, other: &Self) -> Vec<Self>;
    fn partwise_min(points: &HashSet<Self>) -> Self;
    fn partwise_max(points: &HashSet<Self>) -> Self;
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
}

#[derive(Debug)]
pub struct Simulation {
    active_points: HashSet<Point3D>,
    min_corner: Point3D,
    max_corner: Point3D,
}

impl Simulation {
    pub fn from(s: &str) -> Self {
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

    pub fn step(&mut self) {
        let Point3D(mut min_x, mut min_y, mut min_z) = self.min_corner;
        let Point3D(mut max_x, mut max_y, mut max_z) = self.max_corner;

        let mut changes: Vec<(Point3D, bool)> = Vec::new();

        for point in self.min_corner.points_to_check(&self.max_corner) {
            if self.active_points.contains(&point) {
                let active_neighbors = self.num_active_neighbors(&point);
                if active_neighbors != 2 && active_neighbors != 3 {
                    changes.push((point, false));
                }
            } else {
                if self.num_active_neighbors(&point) == 3 {
                    let Point3D(x, y, z) = point;
                    changes.push((point, true));

                    if x < min_x {
                        min_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if z < min_z {
                        min_z = z;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                    if z > max_z {
                        max_z = z;
                    }
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

        self.min_corner = Point3D(min_x, min_y, min_z);
        self.max_corner = Point3D(max_x, max_y, max_z);
    }

    pub fn simulate(&mut self, num_cycles: usize) {
        for _ in 0..num_cycles {
            self.step();
        }
    }

    pub fn num_active_points(&self) -> usize {
        self.active_points.len()
    }

    fn num_active_neighbors(&self, point: &Point3D) -> usize {
        point.neighbors().iter()
            .filter(|p| self.active_points.contains(*p))
            .count()
    }
}