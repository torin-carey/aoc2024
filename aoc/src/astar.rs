use smallvec::SmallVec;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn take_smallest<T: Clone + Eq + Hash, F: Fn(&T) -> usize>(
    set: &mut HashSet<T>,
    f: F,
) -> Option<T> {
    let mut lowest = usize::MAX;
    let mut val = None;
    for p in &*set {
        let m = f(p);
        if m <= lowest {
            lowest = m;
            val = Some(p);
        }
    }
    let val = val.cloned();
    if let Some(k) = &val {
        set.remove(k);
    }
    val
}

pub struct AStar<T> {
    pub open: HashSet<T>,
    pub g_map: HashMap<T, usize>,
    pub f_map: HashMap<T, usize>,
    pub came_from: HashMap<T, SmallVec<[T; 1]>>,
}

impl<T: Clone + Eq + Hash> AStar<T> {
    pub fn run<H, EdgesFrom, Edges, End>(
        start: T,
        h: H,
        edges: EdgesFrom,
        mut end: End,
    ) -> Option<Self>
    where
        H: Fn(&T) -> usize,
        EdgesFrom: Fn(&T) -> Edges,
        Edges: Iterator<Item = (T, usize)>,
        End: FnMut(&T) -> bool,
    {
        let mut open = HashSet::<T>::new();
        let mut g_map = HashMap::<T, usize>::new();
        let mut f_map = HashMap::<T, usize>::new();
        let mut came_from = HashMap::<T, SmallVec<[T; 1]>>::new();
        open.insert(start.clone());
        g_map.insert(start.clone(), 0);
        let h_start = h(&start);
        f_map.insert(start, h_start);

        while let Some(p) = take_smallest(&mut open, |p| f_map[p]) {
            if end(&p) {
                return Some(AStar {
                    open,
                    g_map,
                    f_map,
                    came_from,
                });
            }
            open.remove(&p);
            let gp = g_map[&p];

            for (neigh, d) in edges(&p) {
                let g = gp + d;
                let currentg = *g_map.get(&neigh).unwrap_or(&usize::MAX);
                if g < currentg {
                    g_map.insert(neigh.clone(), g);
                    f_map.insert(neigh.clone(), g + h(&neigh));
                    open.insert(neigh.clone());
                    came_from.insert(neigh, [p.clone()].into());
                } else if g == currentg {
                    came_from.get_mut(&neigh).unwrap().push(p.clone());
                }
            }
        }
        None
    }

    pub fn shortest_paths_nodes<N>(&self, to: N) -> HashSet<T>
    where
        N: IntoIterator,
        N::IntoIter: Iterator<Item = T>,
    {
        let mut set = HashSet::new();
        let mut check = Vec::new();
        to.into_iter().for_each(|n| check.push(n));
        while let Some(point) = check.pop() {
            set.insert(point.clone());
            if let Some(froms) = self.came_from.get(&point) {
                for f in froms {
                    check.push(f.clone());
                }
            }
        }
        set
    }
}
