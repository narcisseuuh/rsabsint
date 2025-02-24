/*
 * author : Narcisse.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::symbol::Symbol;
use std::{error::Error, fmt, rc::Rc};

#[derive(Debug)]
pub struct MapError {}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map library misuse")
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
    fn for_all<F : FnMut(&K, &V) -> bool>(&self, f : F) -> bool;

    fn map2z<F : FnMut(&V, &V) -> V>
    (&mut self, other : &Self, f : F)
        -> Result<(), MapError>;
    fn iter2z<F : FnMut(&K, &V, &V) -> ()>
    (&self, other : &Self, f : F)
        -> Result<(), MapError>;
    fn fold2z<F : FnMut(&K, &V, &V, &V) -> V>
    (&mut self, other : &Self, base : &V, f : F)
        -> Result<V, MapError>;
    fn for_all2z<F : FnMut(&K, &V, &V) -> bool>
    (&mut self, other : &Self, f : F)
        -> Result<bool, MapError>;

    fn min_binding(&self) -> Option<(&K, &V)>;
    fn max_binding(&self) -> Option<(&K, &V)>;
}

#[derive(Clone)]
struct Node<D> {
    key : Symbol,
    value : D,
    left : Option<Rc<Node<D>>>,
    right : Option<Rc<Node<D>>>,
    height : u32,
}

impl<D> Node<D>
where D : Clone + Eq {
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
            if hl >= hr { hl + 1 } else { hr + 1 };
        Node {
            key: key.clone(),
            value: value.clone(),
            left: Some(Rc::new(self.clone())),
            right: r.map(|node| Rc::new(node.clone())),
            height: height,
        }
    }

    fn bal(&self, key : &Symbol, value : &D, r : Option<&Self>) -> Result<Self, MapError> {
        let hl = self.height;
        let hr = r.map(|x| x.height()).unwrap_or(0);

        if hl > hr + 2 {
            match &self.left {
                None => Err(MapError {}),
                Some(lhs) => {
                    if lhs.height() >= lhs.right.as_ref().map_or(0, |x| x.height()) {
                        Ok(lhs.create(
                            &lhs.key,
                            &lhs.value,
                            Some(&self.create(key, value, r))
                        ))
                    } else {
                        match &lhs.right {
                            None => Err(MapError {}),
                            Some(lr) => {
                                Ok(lhs.create(
                                    &lhs.key,
                                    &lhs.value,
                                    Some(&lr.create(&lr.key,
                                        &lr.value,
                                        Some(&self.create(key, value, r)))
                                    )
                                ))
                            }
                        }
                    }
                }
            }
        } else if hr > hl + 2 {
            match r {
                None => Err(MapError {}),
                Some(rhs) => {
                    if rhs.height() >= rhs.left.as_ref().map_or(0, |x| x.height()) {
                        Ok(self.create(
                            key,
                            value,
                            Some(&rhs.create(
                                &rhs.key,
                                &rhs.value,
                                rhs.right.as_deref()
                            ))
                        ))
                    } else {
                        match &rhs.left {
                            None => return Err(MapError {}),
                            Some(rl) => {
                                Ok(self.create(
                                    key,
                                    value,
                                    Some(&rl.create(
                                        &rl.key,
                                        &rl.value,
                                        Some(&rhs.create(
                                            &rhs.key,
                                            &rhs.value,
                                            rhs.right.as_deref()
                                        ))
                                    ))
                                ))
                            }
                        }
                    }
                }
            }
        } else {
            Ok(Node {
                key: key.clone(),
                value: value.clone(),
                left: Some(Rc::new(self.clone())),
                right: r.map(|node| Rc::new(node.clone())),
                height: if hl >= hr { hl + 1 } else { hr + 1 },
            })
        }
    }

    fn merge(&self, other : Option<&Self>) -> Result<Self, MapError> {
        match other {
            Some(n) => {
                let (s, d) = n.min_binding();
                self.bal(s, d, n.remove_min_binding().as_ref())
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
        if let Some(lhs) = &self.left {
            lhs.iter(&mut f);
        }
        if let Some(rhs) = &self.right {
            rhs.iter(&mut f);
        }
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

    fn for_all<F : FnMut(&Symbol, &D) -> bool>
    (&self, mut f : F) -> bool {
        f(&self.key, &self.value)
        && if let Some(n) = &self.left {
            n.for_all(&mut f)
        } else {
            true
        }
        && if let Some(n) = &self.right {
            n.for_all(&mut f)
        } else {
            true
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

    fn remove_min_binding(&self) -> Option<Self> {
        match (&self.left, &self.right) {
            (None, None) => None,
            (None, Some(n)) => Some((**n).clone()),
            (Some(n), _) => {
                let lhs = n.remove_min_binding()?;
                Some(lhs.bal(&self.key, &self.value, self.right.as_deref()).unwrap())
            },
        }
    }

    fn max_binding(&self) -> (&Symbol, &D) {
        if let Some(n) = &self.right {
            n.max_binding()
        }
        else {
            (&self.key, &self.value)
        }
    }
    
    fn cut(&self, key : &Symbol)
     -> Result<(Option<&Self>, Option<&D>, Option<&Self>), MapError> {
        if self.key == *key {
            Ok(
                (self.left.as_deref(),
                Some(&self.value),
                self.right.as_deref())
            )
        }
        else if self.key > *key {
            match &self.left {
                None => Ok((None, None, self.right.as_deref())),
                Some(n) => n.cut(key),
            }
        }
        else if self.key < *key {
            match &self.right {
                None => Ok((self.left.as_deref(), None, None)),
                Some(n) => n.cut(key),
            }
        }
        else {
            Err(MapError {})
        }
    }

    fn map2z<F : FnMut(&D, &D) -> D>
    (&self, other : Option<&Self>, mut f : F) -> Result<Self, MapError> {
        if let Some(other) = other {
            let (lhs, val, rhs) =
                other.cut(&self.key.clone())?;
            if let Some(val) = val {
                Ok(Node {
                    key: self.key.clone(),
                    value: f(&self.value, val),
                    left: self.left
                        .as_ref()
                        .map(|x| x.map2z(lhs, &mut f).unwrap().into()),
                    right: self.right
                        .as_ref()
                        .map(|x| x.map2z(rhs, &mut f).unwrap().into()),
                    height: self.height,
                })
            }
            else {
                Err(MapError {})
            }
        }
        else {
            Err(MapError {})
        }
    }

    fn iter2z<F : FnMut(&Symbol, &D, &D) -> ()>
    (&self, other : Option<&Self>, mut f : F) -> Result<(), MapError> {
        if let Some(other) = other {
            let (lhs, val, rhs) =
                other.cut(&self.key.clone())?;
            if let Some(val) = val {
                self.left
                    .as_ref()
                    .map(|x| x.iter2z(lhs, &mut f));
                if self.value != *val {
                    f(&self.key, &self.value, val);
                }
                self.right
                    .as_ref()
                    .map(|x| x.iter2z(rhs, &mut f));
                Ok(())
            }
            else {
                Err(MapError {})
            }
        }
        else {
            Err(MapError {})
        }
    }

    fn fold2z<F : FnMut(&Symbol, &D, &D, &D) -> D>
    (&self, other : Option<&Self>, base : &D, f : &mut F) -> Result<&D, MapError> {
        if let Some(other) = other {
            let (lhs, val, rhs) =
                other.cut(&self.key.clone())?;
            if let Some(val) = val {
                let acc =
                    self.left
                    .as_ref()
                    .map(|x| x.fold2z(lhs, base, f))
                    .unwrap();
                let acc = if self.value == *val {
                    acc?
                }
                else {
                    &f(&self.key, &self.value, val, acc?)
                };
                self.right
                    .as_ref()
                    .map(|x|  x.fold2z(rhs, acc, f))
                    .unwrap_or(Err(MapError {}))
            }
            else {
                Err(MapError {})
            }
        }
        else {
            Err(MapError {})
        }
    }

    fn for_all2z<F : FnMut(&Symbol, &D, &D) -> bool>
    (&self, other : Option<&Self>, f : &mut F) -> Result<bool, MapError> {
        if let Some(other) = other {
            let (lhs, val, rhs) =
                other.cut(&self.key.clone())?;
            if let Some(val) = val {
                Ok(
                    self.left
                        .as_ref()
                        .map(|x| x.for_all2z(lhs, f).unwrap())
                        .unwrap_or(true)
                    && (self.value == *val || f(&self.key, &self.value, val))
                    && self.left
                    .as_ref()
                    .map(|x| x.for_all2z(rhs, f).unwrap())
                    .unwrap_or(true)
                )
            }
            else {
                Err(MapError {})
            }
        }
        else {
            Err(MapError {})
        }
    }
}

pub struct Map<D> {
    root : Option<Rc<Node<D>>>,
}

impl<D> Map<D> {
    fn get_root(&self) -> Option<Rc<Node<D>>> {
        self.root.clone()
    }
}

impl<D> MapTrait<Symbol, D> for Map<D>
where D : Clone + Eq {
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
    
    fn map2z<F : FnMut(&D, &D) -> D>
    (&mut self, other : &Self, mut f : F) -> Result<(), MapError> {
        match (&self.root, &other.root) {
            (None, None) => Ok(()),
            (None, Some(_)) =>
                Err(MapError {}),
            (Some(_), None) =>
                Err(MapError {}),
            (Some(n1), Some(n2)) => {
                self.root = Some(n1.map2z(Some(n2), &mut f)?.into());
                Ok(())
            },
        }
    }
    
    fn iter2z<F : FnMut(&Symbol, &D, &D) -> ()>
    (&self, other : &Self, mut f : F) -> Result<(), MapError> {
        match (&self.root, &other.root) {
            (None, None) => Ok(()),
            (None, Some(_)) =>
                Err(MapError {}),
            (Some(_), None) =>
                Err(MapError {}),
            (Some(n1), Some(n2)) => {
                n1.iter2z(Some(n2), &mut f)?;
                Ok(())
            },
        }
    }
    
    fn fold2z<F : FnMut(&Symbol, &D, &D, &D) -> D>
    (&mut self, other : &Self, base : &D, mut f : F) -> Result<D, MapError> {
        match (&self.root, &other.root) {
            (None, None) => Ok(base.clone()),
            (None, Some(_)) =>
                Err(MapError {}),
            (Some(_), None) =>
                Err(MapError {}),
            (Some(n1), Some(n2)) =>
                Ok(n1.fold2z(Some(n2), base, &mut f)?.clone()),
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
    
    fn for_all<F : FnMut(&Symbol, &D) -> bool>(&self, mut f : F) -> bool {
        match &self.root {
            None => true,
            Some(n) => n.for_all(&mut f),
        }
    }
    
    fn for_all2z<F : FnMut(&Symbol, &D, &D) -> bool>
    (&mut self, other : &Self, mut f : F) -> Result<bool, MapError> {
        match (&self.root, &other.root) {
            (None, None) => Ok(true),
            (None, Some(_)) => Err(MapError {}),
            (Some(_), None) => Err(MapError {}),
            (Some(n1), Some(n2)) => {
                Ok(n1.for_all2z(Some(n2), &mut f)?)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typing::Type;

    impl Symbol {
        pub fn new(name : &str) -> Self {
            Self::Variable { name: name.to_string(), dtype: Type::Int }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestDomain(i32);

    #[test]
    fn test_new() {
        let map: Map<TestDomain> = Map::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_singleton() {
        let key = Symbol::new("key");
        let value = TestDomain(42);
        let map = Map::singleton(&key, &value);
        assert!(!map.is_empty());
        assert!(map.mem(&key));
        assert_eq!(map.find(&key), Some(&value));
    }

    #[test]
    fn test_add() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        assert!(map.mem(&key1));
        assert!(map.mem(&key2));
        assert_eq!(map.find(&key1), Some(&value1));
        assert_eq!(map.find(&key2), Some(&value2));
    }

    #[test]
    fn test_remove() {
        let key = Symbol::new("key");
        let value = TestDomain(42);
        let mut map = Map::singleton(&key, &value);
        map.remove(&key);
        assert!(map.is_empty());
    }

    #[test]
    fn test_iter() {
        let key1 = Symbol::new("key1");
        let key2 = Symbol::new("key2");
        let value1 = TestDomain(42);
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        let mut keys = Vec::new();
        let mut values = Vec::new();
        map.iter(|k, v| {
            keys.push(k.clone());
            values.push(v.clone());
        });
        assert_eq!(keys, vec![key1.clone(), key2.clone()]);
        assert_eq!(values, vec![value1.clone(), value2.clone()]);
    }

    #[test]
    fn test_fold() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        let result = map.fold(&TestDomain(0), |_, v, acc| TestDomain(v.0 + acc.0));
        assert_eq!(result, TestDomain(85));
    }

    #[test]
    fn test_filter() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        map.filter(|_, v| v.0 > 42);
        assert!(map.mem(&key2));
        assert!(!map.mem(&key1));
    }

    #[test]
    fn test_map() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        map.map(|v| TestDomain(v.0 + 1));
        assert_eq!(map.find(&key1), Some(&TestDomain(43)));
        assert_eq!(map.find(&key2), Some(&TestDomain(44)));
    }

    #[test]
    fn test_mapi() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        map.mapi(|_, v| TestDomain(v.0 + 1));
        assert_eq!(map.find(&key1), Some(&TestDomain(43)));
        assert_eq!(map.find(&key2), Some(&TestDomain(44)));
    }

    #[test]
    fn test_min_binding() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        let (min_key, min_value) = map.min_binding().unwrap();
        assert_eq!(min_key, &key1);
        assert_eq!(min_value, &value1);
    }

    #[test]
    fn test_max_binding() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        let (max_key, max_value) = map.max_binding().unwrap();
        assert_eq!(max_key, &key2);
        assert_eq!(max_value, &value2);
    }

    #[test]
    fn test_for_all() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map = Map::new();
        map.add(&key1, &value1).unwrap();
        map.add(&key2, &value2).unwrap();
        assert!(map.for_all(|_, v| v.0 > 40));
        assert!(!map.for_all(|_, v| v.0 > 42));
    }

    #[test]
    fn test_map2z() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map1 = Map::new();
        map1.add(&key1, &value1).unwrap();
        map1.add(&key2, &value2).unwrap();
        let mut map2 = Map::new();
        map2.add(&key1, &TestDomain(1)).unwrap();
        map2.add(&key2, &TestDomain(2)).unwrap();
        map1.map2z(&map2, |v1, v2| TestDomain(v1.0 + v2.0)).unwrap();
        assert_eq!(map1.find(&key1), Some(&TestDomain(43)));
        assert_eq!(map1.find(&key2), Some(&TestDomain(45)));
    }

    #[test]
    fn test_iter2z() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map1 = Map::new();
        map1.add(&key1, &value1).unwrap();
        map1.add(&key2, &value2).unwrap();
        let mut map2 = Map::new();
        map2.add(&key1, &TestDomain(1)).unwrap();
        map2.add(&key2, &TestDomain(2)).unwrap();
        let mut results = Vec::new();
        map1.iter2z(&map2, |k, v1, v2| {
            results.push((k.clone(), v1.clone(), v2.clone()));
        }).unwrap();
        assert_eq!(results, vec![(key1.clone(), value1.clone(), TestDomain(1)), (key2.clone(), value2.clone(), TestDomain(2))]);
    }

    #[test]
    fn test_fold2z() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map1 = Map::new();
        map1.add(&key1, &value1).unwrap();
        map1.add(&key2, &value2).unwrap();
        let mut map2 = Map::new();
        map2.add(&key1, &TestDomain(1)).unwrap();
        map2.add(&key2, &TestDomain(2)).unwrap();
        let result = map1.fold2z(&map2, &TestDomain(0), |_, v1, v2, acc| TestDomain(v1.0 + v2.0 + acc.0)).unwrap();
        assert_eq!(result, TestDomain(88));
    }

    #[test]
    fn test_for_all2z() {
        let key1 = Symbol::new("key1");
        let value1 = TestDomain(42);
        let key2 = Symbol::new("key2");
        let value2 = TestDomain(43);
        let mut map1 = Map::new();
        map1.add(&key1, &value1).unwrap();
        map1.add(&key2, &value2).unwrap();
        let mut map2 = Map::new();
        map2.add(&key1, &TestDomain(1)).unwrap();
        map2.add(&key2, &TestDomain(2)).unwrap();
        assert!(map1.for_all2z(&map2, |_, v1, v2| v1.0 > v2.0).unwrap());
        assert!(!map1.for_all2z(&map2, |_, v1, v2| v1.0 < v2.0).unwrap());
    }
}
