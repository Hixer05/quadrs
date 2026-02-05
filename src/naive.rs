/*
objective: dynamic tree holding points!
*/
use std::ops::{Add, Mul};
pub trait Vectorial: Sized + Add<Output = Self> + Mul<f64, Output = Self> + Clone + Copy {
    fn within(&self, _: (Self, Self)) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct DefaultVector<const N: usize>([f64; N]);

impl<const N: usize> Add for DefaultVector<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const N: usize> Mul<f64> for DefaultVector<N> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] * rhs))
    }
}

impl<const N: usize> Vectorial for DefaultVector<N> {
    fn within(&self, area: (Self, Self)) -> bool {
        for i in 0..N {
            if !(area.0.0[i].min(area.1.0[i]) <= self.0[i]
                && self.0[i] <= area.0.0[i].max(area.1.0[i]))
            {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_vector_impl() {
    let p = DefaultVector::<2>([1.0, 2.0]);
    assert_eq!((p + p).0, (p * 2.0).0);
}

#[derive(Clone, Debug)]
enum DNode<const D: usize, T: Vectorial, U, V> {
    None,
    Node {
        area: (T, T),
        metadata: U,
        data: V,
        childs: [Box<Self>; D],
    },
    Leaf {
        area: (T, T),
        position: T,
        metadata: U,
        data: V,
    },
}

impl<const D: usize, T: Vectorial, U: Clone, V: Clone> DNode<D, T, U, V> {
    pub fn insert(&mut self, n: &DNode<D, T, U, V>) {
        match n {
            DNode::Leaf {
                area: _,
                position: n_position,
                ..
            } => match self {
                DNode::None => *self = n.clone(),
                DNode::Node {
                    area: _,
                    metadata: _,
                    data: _,
                    childs: self_childs,
                } => {
                    for c in self_childs {
                        match **c {
                            DNode::None => continue,
                            DNode::Leaf {
                                area: child_area, ..
                            }
                            | DNode::Node {
                                area: child_area, ..
                            } => {
                                if n_position.within(child_area) {
                                    c.insert(n);
                                    // TODO: add break here, even if areas should be disjunct
                                }
                            }
                        }
                    }
                }
                DNode::Leaf {
                    area,
                    metadata,
                    data,
                    position: _,
                } => {
                    *self = DNode::Node {
                        area: *area,
                        metadata: metadata.clone(), // REVIEW: Only way?
                        data: data.clone(),         // TODO: add transition method
                        childs: std::array::from_fn::<_, D, _>(|i| match i {
                            // FIXME: finish implementation!
                            0 => Box::new(self.clone()), // NOTE: expensive, but this is naive impl
                            _ => Box::new(DNode::None),
                        }),
                    };
                }
            },
            _ => panic!("Trying to insert either DNode::None or DNode::Node."),
        }
    }
}

#[test]
fn test_quadtree_insert_various_points() {
    const D: usize = 2;
    type Vec2 = DefaultVector<D>;
    type Meta = i32;
    type Data = i32;

    // Points in different locations
    let p1 = DefaultVector::<2>([0.0, 0.0]);
    let p2 = DefaultVector::<2>([2.0, 3.0]);
    let p3 = DefaultVector::<2>([5.0, 1.0]);
    let p4 = DefaultVector::<2>([1.0, 1.0]);

    fn leaf(p: Vec2) -> DNode<D, Vec2, Meta, Data> {
        DNode::Leaf {
            area: (p, p),
            position: p,
            metadata: 0,
            data: 0,
        }
    }

    let n1 = leaf(p1);
    let n2 = leaf(p2);
    let n3 = leaf(p3);
    let n4 = leaf(p4);

    let mut tree: DNode<D, Vec2, Meta, Data> = DNode::None;

    tree.insert(&n1);
    match tree {
        DNode::Leaf { position, .. } => assert_eq!(position.0, [0.0, 0.0]),
        _ => panic!("Expected a leaf after 1st insert"),
    }

    // Insert a second, which should promote to Node
    tree.insert(&n2);
    match tree {
        DNode::Node { ref childs, .. } => {
            // There should be at least 1 child which isn't None
            assert!(
                childs.iter().any(|b| match **b {
                    DNode::Leaf { .. } => true,
                    _ => false,
                }),
                "After promotion to Node, at least one child should be Leaf"
            );
        }
        _ => panic!("Expected a node after 2nd insert"),
    }

    // Insert two more
    tree.insert(&n3);
    tree.insert(&n4);

    fn count_leaves<T: Vectorial + Copy, U: Clone, V: Clone>(node: &DNode<D, T, U, V>) -> usize {
        match node {
            DNode::None => 0,
            DNode::Leaf { .. } => 1,
            DNode::Node { childs, .. } => childs.iter().map(|b| count_leaves::<T, U, V>(b)).sum(),
        }
    }

    let leaf_count = count_leaves(&tree);
    assert!(
        leaf_count >= 1,
        "After all inserts, there should be at least 1 leaf"
    );

    // None of the leaves should have area not containing their position
    fn check_leaf_areas<T: Vectorial + Copy, U: Clone, V: Clone>(node: &DNode<D, T, U, V>) {
        match node {
            DNode::Leaf { area, position, .. } => {
                assert!(
                    position.within(*area),
                    "Leaf position should be within its area"
                );
            }
            DNode::Node { childs, .. } => {
                for c in childs.iter() {
                    check_leaf_areas(&c);
                }
            }
            _ => {}
        }
    }
    check_leaf_areas(&tree);
}
