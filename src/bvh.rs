use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hit::{DynHit, Hit, HitRecord},
    interval::Interval,
    material::Material,
    ray::Ray,
};

#[derive(Debug, Clone)]
pub enum BvhNode {
    Leaf(Arc<dyn DynHit>),
    Node { left: Arc<Bvh>, right: Arc<Bvh> },
}

#[derive(Debug, Clone)]
pub struct Bvh {
    node: BvhNode,
    bbox: Aabb,
}

impl Bvh {
    pub fn new(node: BvhNode, bbox: Aabb) -> Self {
        Self { node, bbox }
    }

    pub fn from_list(hit_list: &mut [Arc<dyn DynHit>]) -> Self {
        assert!(!hit_list.is_empty());

        let bbox = hit_list
            .into_iter()
            .fold(Aabb::default(), |bbox, obj| bbox.merge(obj.aabb()));

        let axis = bbox.longest_axis();

        let node = if hit_list.len() == 1 {
            BvhNode::Leaf(hit_list[0].clone())
        } else {
            hit_list.sort_by(|a, b| a.aabb().compare(&b.aabb(), axis));
            let mid = hit_list.len() / 2;
            let (left, right) = hit_list.split_at_mut(mid);
            let left = Self::from_list(left);
            let right = Self::from_list(right);
            BvhNode::Node {
                left:  Arc::new(left),
                right: Arc::new(right),
            }
        };

        Self::new(node, bbox)
    }
}

impl Hit for Bvh {
    type Material = Material;

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<Self::Material>> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        match self.node {
            BvhNode::Leaf(ref arc) => arc.hit(ray, ray_t),
            BvhNode::Node {
                ref left,
                ref right,
            } => {
                let left = left.hit(ray, ray_t);
                let max = match left {
                    Some(ref hit) => hit.t(),
                    None => ray_t.max(),
                };
                let right = right.hit(ray, Interval::new(ray_t.min(), max));

                match (left, right) {
                    (_, Some(right)) => Some(right),
                    (Some(left), None) => Some(left),
                    (None, None) => None,
                }
            }
        }
    }

    fn aabb(&self) -> Aabb {
        self.bbox
    }

    fn count(&self) -> usize {
        match self.node {
            BvhNode::Leaf(ref arc) => arc.count(),
            BvhNode::Node {
                ref left,
                ref right,
            } => left.count() + right.count(),
        }
    }
}
