use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point3D(i32, i32, i32);

impl Point3D {
    fn neighbors(&self) -> Vec<Point3D> {
        let &Point3D(x, y, z) = self;
        (-1..=1).flat_map(|dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).map(move |dz| Point3D(x + dx, y + dy, z + dz))
            })
        }).filter(|point| point != self).collect()
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

        let min = Point3D(
            points.iter().map(|Point3D(x, _, _)| *x).min().unwrap(),
            points.iter().map(|Point3D(_, y, _)| *y).min().unwrap(),
            points.iter().map(|Point3D(_, _, z)| *z).min().unwrap(),
        );
        let max = Point3D(
            points.iter().map(|Point3D(x, _, _)| *x).max().unwrap(),
            points.iter().map(|Point3D(_, y, _)| *y).max().unwrap(),
            points.iter().map(|Point3D(_, _, z)| *z).max().unwrap(),
        );

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

        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    let point = Point3D(x, y, z);
                    if self.active_points.contains(&point) {
                        let active_neighbors = self.num_active_neighbors(&point);
                        if active_neighbors != 2 && active_neighbors != 3 {
                            changes.push((point, false));
                        }
                    } else {
                        if self.num_active_neighbors(&point) == 3 {
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