/*
objective: dynamic tree holding points!
*/
use crate::vector::Vectorial;

#[derive(Clone, Debug)]
enum DNode<const D: usize, T: Vectorial, U, V> {
    Node {
        area: (T, T),
        metadata: U,
        data: V,
        children: [Box<Option<Self>>; D],
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
        if let DNode::Leaf {
            area: _,
            position: npos,
            ..
        } = n
        {
            match self {
                DNode::Node {
                    area: _,
                    metadata: _,
                    data: _,
                    children,
                } => {
                    for child in children {
                        match **child {
                            Some(DNode::Leaf { area, .. }) | Some(DNode::Node { area, .. }) => {
                                if npos.within(area) {
                                    child.insert(n.clone());
                                    break;
                                }
                            }
                            None => continue,
                        }
                    }
                }
                DNode::Leaf {
                    area,
                    position,
                    metadata,
                    data,
                } => {
                    *self = DNode::Node {
                        // FIXME: transition function from leaf to node
                        area: *area,
                        metadata: metadata.clone(),
                        data: data.clone(),
                        children: std::array::from_fn::<_, D, _>(|i| {
                            if position.within(area.quarter(i)) {
                                // FIXME: How to set area to quarter here?
                                Some(self.clone())
                            } else {
                                None
                            }
                        }),
                    }
                }
            }
        } else {
            panic!("Trying to insert either DNode::None or DNode::Node.")
        }
    }
}
