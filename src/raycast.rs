use crate::line_seg::LineSeg;
use crate::object::Object;
use crate::object_manager::om;
use crate::transform::Transform;
use crate::vec2::Vec2;
use std::cell::RefCell;
use std::rc::Rc;

pub enum FilterType {
    None,
    Whitelist,
    Blacklist,
}

pub struct Raycast {
    pub ray: LineSeg,
    pub hit: Option<RaycastHit>,
    filter: Vec<String>,
}

pub struct RaycastHit {
    pub object: Rc<RefCell<dyn Object>>,
    pub pos: Vec2,
    pub distance: f64,
}

impl Raycast {
    pub fn new(ray: LineSeg, filter: Vec<String>) -> Self {
        Self {
            ray,
            hit: None,
            filter,
        }
    }

    pub fn fire(&mut self, filter_type: FilterType) {
        let mut has_hit = false;
        let mut hit_distance: f64 = 0.0;
        let mut hit = RaycastHit {
            object: om().objects[0].clone(),
            pos: [0.0, 0.0].into(),
            distance: 0.0,
        };

        let bounds_to_check = Transform::new(
            [
                self.ray.p1.x + self.ray.x_diff() / 2.0,
                self.ray.p1.y + self.ray.y_diff() / 2.0,
            ]
                .into(),
            [self.ray.x_diff(), self.ray.y_diff()].into(),
            0.0,
        );

        let objects_in_bounds = om().objects_in_bounds(&bounds_to_check);
        let mut objects_to_check: Vec<Rc<RefCell<dyn Object>>> = Vec::new();

        match filter_type {
            FilterType::None => {}
            FilterType::Whitelist => {
                for object in objects_in_bounds {
                    for tag in &self.filter {
                        if object.borrow().tags().contains(tag) {
                            objects_to_check.push(object.clone());
                            break;
                        }
                    }
                }
            }
            FilterType::Blacklist => {
                for object in objects_in_bounds {
                    let mut tag_found = false;
                    for tag in &object.borrow().tags() {
                        if self.filter.contains(tag) {
                            tag_found = true;
                            break;
                        }
                    }
                    
                    if tag_found {
                        continue;
                    }

                    objects_to_check.push(object.clone());
                }
            }
        }

        for object in objects_to_check {
            for line in object.borrow().transform().lines() {
                match line.intersects(&self.ray) {
                    None => {}
                    Some(pos) => {
                        if !has_hit {
                            has_hit = true;
                            hit_distance = (&pos - &self.ray.p1).len();
                            hit = RaycastHit {
                                object: object.clone(),
                                pos,
                                distance: hit_distance,
                            };
                        } else if (&pos - &self.ray.p1).len() < hit_distance {
                            hit_distance = (&pos - &self.ray.p1).len();
                            hit = RaycastHit {
                                object: object.clone(),
                                pos,
                                distance: hit_distance,
                            };
                        }
                    }
                }
            }
        }

        if has_hit {
            self.hit = Some(hit);
        } else {
            self.hit = None;
        }
    }

    pub fn set_filter(&mut self, filter: Vec<String>) {
        self.filter = filter;
    }
}
