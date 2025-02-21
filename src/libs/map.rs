/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::{domains::domain::AbstractDomain, symbol::Symbol};
use std::{error::Error, fmt, rc::Rc};

#[derive(Debug)]
pub struct MapError {}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Should not happen..")
    }
}

impl Error for MapError {}

trait MapTrait<K, V>
where K : Ord {
    fn new() -> Self;
    fn singleton(key : &K, value : &V) -> Self;

    fn mem(&self, key : &K) -> bool;
    fn find(&self, key : &K) -> Option<&V>;
    fn is_empty(&self) -> bool;

    fn add(&mut self, key : &K, value : &V) -> Result<(), MapError>;
    fn remove(&mut self, key : &K) -> ();

    fn map<F : FnMut(&V) -> V>(&mut self, f : F) -> ();
    fn iter<F : FnMut(&K, &V) -> ()>(&self, f : F) -> ();
    fn fold<F : FnMut(&K, &V, &V) -> V>(&self, base : &V, f : F) -> V;
    fn filter<F : FnMut(&K, &V) -> bool>(&mut self, f : F) -> ();
    fn mapi<F : FnMut(&K, &V) -> V>(&mut self, f : F) -> ();

    fn map2<F : FnMut(&V, &V) -> V>(&mut self, other : &Self, f : F) -> ();
    fn iter2<F : FnMut(&K, &V, &V) -> ()>(&self, other : &Self, f : F) -> ();
    fn fold2<F : FnMut(&K, &V, &V, &V) -> V>(&mut self, other : &Self, base : &V, f : F) -> V;

    fn min_binding(&self) -> Option<(&K, &V)>;
    fn max_binding(&self) -> Option<(&K, &V)>;
}

#[derive(Clone)]
struct Node<D : AbstractDomain> {
    key : Symbol,
    value : D,
    left : Option<Rc<Node<D>>>,
    right : Option<Rc<Node<D>>>,
    height : u32,
}

impl<D> Node<D>
where D : AbstractDomain {
    fn new(key : &Symbol, value : &D) -> Self {
        Node {
            key: key.clone(),
            value: value.clone(),
            left: None,
            right: None,
            height: 1,
        }
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn mem(&self, key : &Symbol) -> bool {
        if self.key == *key {
            true
        }
        else if self.key < *key {
            match &self.left {
                None => false,
                Some(lhs) => {
                    lhs.mem(key)
                },
            }
        }
        else {
            match &self.right {
                None => false,
                Some(rhs) => {
                    rhs.mem(key)
                },
            }
        }
    }

    fn find(&self, key : &Symbol) -> Option<&D> {
        if self.key == *key {
            Some(&self.value)
        }
        else if self.key < *key {
            match &self.left {
                None => None,
                Some(lhs) => {
                    lhs.find(key)
                },
            }
        }
        else {
            match &self.right {
                None => None,
                Some(rhs) => {
                    rhs.find(key)
                },
            }
        }
    }

    fn create(&self, key : &Symbol, value : &D, r : Option<&Self>) -> Self {
        let hl = self.height;
        let hr = r.map(|x| x.height()).unwrap_or(0);
        let height =
            if (hl >= hr) { hl + 1 } else { hr + 1 };
        Node {
            key: key.clone(),
            value: value.clone(),
            left: Some(Rc::new(self.clone())),
            right: r.map(|node| Rc::new(node.clone())),
            height: height,
        }
    }

    fn bal(&self, key : &Symbol, value : &D, r : Option<&Self>) -> Result<Self, MapError> {
        todo!()
    }

    fn merge(&self, other : Option<&Self>) -> Result<Self, MapError> {
        match other {
            Some(n) => {
                let (s, d) = n.min_binding();
                self.bal(s, d, Some(&n.remove_min_binding()))
            },
            None => Ok(self.clone()),
        }
    }

    fn add(&self, key : &Symbol, value : &D) -> Result<Self, MapError> {
        // is it truly correct ? to verify
        if self.key == *key {
            Ok(Node {
                key : self.key.clone(),
                value : value.clone(),
                left : self.left.clone(),
                right : self.right.clone(),
                height : self.height,
            })
        }
        else if *key < self.key {
            if let Some(n) = &self.left {
                let lhs = n.add(key, value).unwrap();
                lhs.bal(&self.key, &self.value, self.right.as_deref())
            }
            else {
                Ok(Node {
                    key : self.key.clone(),
                    value : value.clone(),
                    left : Some(Self::new(key, value).into()),
                    right : self.right.clone(),
                    height : self.height,
                })
            }
        }
        else {
            if let Some(n) = &self.right {
                let rhs = n.add(key, value).unwrap();
                self.left.clone().unwrap().bal(&self.key, &self.value, Some(&rhs))
            }
            else {
                Ok(Node {
                    key : self.key.clone(),
                    value : value.clone(),
                    left : self.left.clone(),
                    right : Some(Self::new(key, value).into()),
                    height : self.height,
                })
            }
        }
    }

    fn remove(&self, key : &Symbol) -> Option<Self> {
        if self.key == *key {
            if let Some(n) = &self.left {
                match n.merge(self.right.as_deref()) {
                    Ok(node) => Some(node),
                    Err(_) => None,
                }
            }
            else if let Some(n) = &self.right {
                match n.merge(self.left.as_deref()) {
                    Ok(node) => Some(node),
                    Err(_) => None,
                }
            }
            else {
                None
            }
        }
        else if self.key < *key {
            let lhs =
                if let Some(n) = &self.left {
                    n.remove(key)
                }
                else {
                    None
                };
            Some(Node{
                key : self.key.clone(),
                value : self.value.clone(),
                left : lhs.map(|x| x.into()),
                right : self.right.clone(),
                height : self.height,
            })
        }
        else {
            let rhs =
                if let Some(n) = &self.right {
                    n.remove(key)
                }
                else {
                    None
                };
            Some(Node{
                key : self.key.clone(),
                value : self.value.clone(),
                left : self.left.clone(),
                right : rhs.map(|x| x.into()),
                height : self.height,
            })
        }
    }

    fn iter<F : FnMut(&Symbol, &D) -> ()>(&self, mut f : F) -> () {
        f(&self.key, &self.value);
        self.left.as_ref().map(|x| x.iter(&mut f));
        self.right.as_ref().map(|x| x.iter(&mut f));
    }

    fn fold<F : FnMut(&Symbol, &D, &D) -> D>(&self, base : &D, mut f : F) -> D {
        match (&self.left, &self.right) {
            (Some(lhs), Some(rhs)) => {
                let lhs_result = &lhs.fold(base, &mut f);
                let lhs_result= f(&self.key, &self.value, lhs_result);
                rhs.fold(&lhs_result, &mut f)
            }
            (Some(lhs), None) => {
                let lhs_result = lhs.fold(base, &mut f);
                f(&self.key, &self.value, &lhs_result)
            },
            (None, Some(rhs)) =>
                rhs.fold(&f(&self.key, &self.value, base), &mut f),
            (None, None) =>
                f(&self.key, &self.value, base),
        }
    }

    fn filter<F : FnMut(&Symbol, &D) -> bool>(&self, f : &mut F) -> Option<Self> {
        if f(&self.key, &self.value) {
            return None;
        }
        let lhs =
            if let Some(n) = &self.left {
                n.filter(f)
            }
            else {
                None
            };
        let rhs =
            if let Some(n) = &self.right {
                n.filter(f)
            }
            else {
                None
            };
        Some(Node {
            key : self.key.clone(),
            value : self.value.clone(),
            left : lhs.map(|x| x.into()),
            right : rhs.map(|x| x.into()),
            height : self.height,
        })
    }

    fn map<F : FnMut(&D) -> D>(&self, mut f : F) -> Self {
        let lhs =
            &self.left.clone().map(
                |x| x.map(&mut f).into()
            );
        let rhs =
            &self.right.clone().map(
                |x| x.map(&mut f).into()
            );
        Node {
            value : f(&self.value),
            key : self.key.clone(),
            left : lhs.clone(),
            right : rhs.clone(),
            height : self.height,
        }
    }

    fn mapi<F : FnMut(&Symbol, &D) -> D>(&self, mut f : F) -> Self {
        let lhs =
            &self.left.clone().map(
                |x| x.mapi(&mut f).into()
            );
        let rhs =
            &self.right.clone().map(
                |x| x.mapi(&mut f).into()
            );
        Node {
            value : f(&self.key, &self.value),
            key : self.key.clone(),
            left : lhs.clone(),
            right : rhs.clone(),
            height : self.height,
        }
    }

    fn min_binding(&self) -> (&Symbol, &D) {
        if let Some(n) = &self.left {
            n.min_binding()
        }
        else {
            (&self.key, &self.value)
        }
    }

    fn remove_min_binding(&self) -> Self {
        todo!()
    }

    fn max_binding(&self) -> (&Symbol, &D) {
        if let Some(n) = &self.right {
            n.max_binding()
        }
        else {
            (&self.key, &self.value)
        }
    }

    fn map2<F : FnMut(&D, &D) -> D>(&self, other : &Self, f : F) -> Self {
        todo!()
    }

    fn iter2<F : FnMut(&Symbol, &D, &D) -> ()>(&self, other : &Self, f : F) -> () {
        todo!()
    }

    fn fold2<F : FnMut(&Symbol, &D, &D, &D) -> D>(&self, other : &Self, base : &D, f : F) -> &D {
        todo!()
    }
}

pub struct Map<D : AbstractDomain> {
    root : Option<Rc<Node<D>>>,
}

impl<D : AbstractDomain> Map<D> {
    fn get_root(&self) -> Option<Rc<Node<D>>> {
        self.root.clone()
    }
}

impl<D> MapTrait<Symbol, D> for Map<D> 
where D : AbstractDomain {
    fn new() -> Self {
        Map {
            root : None,
        }
    }

    fn singleton(key : &Symbol, value : &D) -> Self {
        Map {
            root : Some(Rc::new(Node::<D>::new(key, value))),
        }
    }

    fn mem(&self, key : &Symbol) -> bool {
        match &self.root {
            None => false,
            Some(node) => node.mem(key),
        }
    }

    fn find(&self, key : &Symbol) -> Option<&D> {
        match &self.root {
            None => None,
            Some(node) => node.find(key),
        }
    }

    fn add(&mut self, key : &Symbol, value : &D) -> Result<(), MapError> {
        match &self.root {
            None => {
                self.root = Some(Rc::new(Node::<D>::new(key, value)));
                Ok(())
            },
            Some(node) => {
                self.root = Some(node.add(key, value)?.into());
                Ok(())
            },
        }
    }

    fn remove(&mut self, key : &Symbol) -> () {
        match &self.root {
            None => (),
            Some(node) => {
                if let Some(res) = node.remove(key) {
                    self.root = Some(res.into());
                }
                else {
                    self.root = None;
                }
            },
        }
    }

    fn iter<F : FnMut(&Symbol, &D) -> ()>(&self, f : F) -> () {
        match &self.root {
            None => (),
            Some(node) => node.iter(f),
        }
    }

    fn fold<F : FnMut(&Symbol, &D, &D) -> D>(&self, base : &D, f : F) -> D {
        match &self.root {
            None => base.clone(),
            Some(node) => node.fold(base, f),
        }
    }

    fn filter<F : FnMut(&Symbol, &D) -> bool>(&mut self, mut f : F) -> () {
        match &self.root {
            None => (),
            Some(node) => {
                if let Some(res) = node.filter(&mut f) {
                    self.root = Some(res.into());
                }
                else {
                    self.root = None;
                }
            }
        }
    }

    fn map<F : FnMut(&D) -> D>(&mut self, f : F) -> () {
        match &self.root {
            None => (),
            Some(node) => {
                self.root = Some(node.map(f).into());
            }
        }
    }

    fn mapi<F : FnMut(&Symbol, &D) -> D>(&mut self, f : F) -> () {
        match &self.root {
            None => (),
            Some(node) => {
                self.root = Some(node.mapi(f).into());
            }
        }
    }
    
    fn map2<F : FnMut(&D, &D) -> D>(&mut self, other : &Self, mut f : F) -> () {
        match (&self.root, &other.root) {
            (None, None) => (),
            (None, Some(node)) =>
                self.root = Some(node.map(|x| { f(x, &D::bottom()) }).into()),
            (Some(node), None) =>
                self.root = Some(node.map(|x| { f(x, &D::bottom()) }).into()),
            (Some(n1), Some(n2)) =>
                self.root = Some(n1.map2(n2, f).into()),
        }
    }
    
    fn iter2<F : FnMut(&Symbol, &D, &D) -> ()>(&self, other : &Self, mut f : F) -> () {
        match (&self.root, &other.root) {
            (None, None) => (),
            (None, Some(node)) =>
                node.iter(|x, d| { f(x, &D::bottom(), d) }),
            (Some(node), None) =>
                node.iter(|x, d| { f(x, d, &D::bottom()) }),
            (Some(n1), Some(n2)) =>
                n1.iter2(n2, f),
        }
    }
    
    fn fold2<F : FnMut(&Symbol, &D, &D, &D) -> D>(&mut self, other : &Self, base : &D, mut f : F) -> D {
        match (&self.root, &other.root) {
            (None, None) => base.clone(),
            (None, Some(n)) => n.fold(base, |x, d, acc| { f(x, &D::bottom(), d, acc) }),
            (Some(n), None) => n.fold(base, |x, d, acc| { f(x, d, &D::bottom(), acc) }),
            (Some(n1), Some(n2)) =>
                n1.fold2(n2, base, f).clone(),
        }
    }
    
    fn min_binding(&self) -> Option<(&Symbol, &D)> {
        match &self.root {
            None => None,
            Some(node) => Some(node.min_binding()),
        }
    }
    
    fn max_binding(&self) -> Option<(&Symbol, &D)> {
        match &self.root {
            None => None,
            Some(node) => Some(node.max_binding()),
        }
    }
    
    fn is_empty(&self) -> bool {
        match &self.root {
            None => true,
            _ => false,
        }
    }
}