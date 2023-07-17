use parking_lot::*;
use crate::source::*;

DeclareCollectionEnum!{Op; Source}

pub struct Node {
    op: Op,
    size: (usize, usize),
}

pub struct TGraphInner {
    buff: Vec<usize>,
    node: Vec<Node>,
}

impl TGraphInner {
    pub fn new() -> Self { Self { buff: vec![], node: vec![] } }
}

pub struct TGraph {inner: Mutex<TGraphInner>}

impl TGraph {
    pub fn new() -> Self {
        Self { inner: Mutex::new(TGraphInner::new()) }
    }
    pub(crate) fn register<const N: usize>(
        &self, op: Op, shape: [usize; N]
    ) -> usize {
        let mut mu = self.inner.lock();
        let size = (mu.buff.len(), N);
        mu.buff.extend(shape);
        mu.node.push(Node{op, size});
        mu.node.len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_register() {
        let x = [[1,2,3], [4,5,6]];
        let y = [[7,8,9,10], [11,12,13,14]];
        let x: (_, Source) = (x.shape(), x.flatvec().into());
        let y: (_, Source) = (y.shape(), y.flatvec().into());
        let tg = TGraph::new();
        assert!(0 == tg.register(x.1.into(), x.0));
        assert!(1 == tg.register(y.1.into(), y.0));
    }
}
