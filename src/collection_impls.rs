// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementations of serialization for structures found in libcollections

use std::default::Default;
use std::hash::{Hash, Hasher};

use {Decodable, Encodable, Decoder, Encoder};
use std::collections::{DList, RingBuf, BTreeMap, BTreeSet, HashMap, HashSet, VecMap};

impl<
    T: Encodable
> Encodable for DList<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_seq(self.len(), |s| {
            for (i, e) in self.iter().enumerate() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
            }
            Ok(())
        })
    }
}

impl<T:Decodable> Decodable for DList<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<DList<T>, D::Error> {
        d.read_seq(|d, len| {
            let mut list = DList::new();
            for i in range(0u, len) {
                list.push_back(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(list)
        })
    }
}

impl<T: Encodable> Encodable for RingBuf<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_seq(self.len(), |s| {
            for (i, e) in self.iter().enumerate() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
            }
            Ok(())
        })
    }
}

impl<T:Decodable> Decodable for RingBuf<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<RingBuf<T>, D::Error> {
        d.read_seq(|d, len| {
            let mut deque: RingBuf<T> = RingBuf::new();
            for i in range(0u, len) {
                deque.push_back(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(deque)
        })
    }
}

impl<
    K: Encodable + PartialEq + Ord,
    V: Encodable + PartialEq
> Encodable for BTreeMap<K, V> {
    fn encode<S: Encoder>(&self, e: &mut S) -> Result<(), S::Error> {
        e.emit_map(self.len(), |e| {
            let mut i = 0;
            for (key, val) in self.iter() {
                try!(e.emit_map_elt_key(i, |e| key.encode(e)));
                try!(e.emit_map_elt_val(i, |e| val.encode(e)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    K: Decodable + PartialEq + Ord,
    V: Decodable + PartialEq
> Decodable for BTreeMap<K, V> {
    fn decode<D: Decoder>(d: &mut D) -> Result<BTreeMap<K, V>, D::Error> {
        d.read_map(|d, len| {
            let mut map = BTreeMap::new();
            for i in range(0u, len) {
                let key = try!(d.read_map_elt_key(i, |d| Decodable::decode(d)));
                let val = try!(d.read_map_elt_val(i, |d| Decodable::decode(d)));
                map.insert(key, val);
            }
            Ok(map)
        })
    }
}

impl<
    T: Encodable + PartialEq + Ord
> Encodable for BTreeSet<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_seq(self.len(), |s| {
            let mut i = 0;
            for e in self.iter() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    T: Decodable + PartialEq + Ord
> Decodable for BTreeSet<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<BTreeSet<T>, D::Error> {
        d.read_seq(|d, len| {
            let mut set = BTreeSet::new();
            for i in range(0u, len) {
                set.insert(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(set)
        })
    }
}

impl<
    K: Encodable + Hash<X> + Eq,
    V: Encodable,
    X,
    H: Hasher<X>
> Encodable for HashMap<K, V, H> {
    fn encode<S: Encoder>(&self, e: &mut S) -> Result<(), S::Error> {
        e.emit_map(self.len(), |e| {
            let mut i = 0;
            for (key, val) in self.iter() {
                try!(e.emit_map_elt_key(i, |e| key.encode(e)));
                try!(e.emit_map_elt_val(i, |e| val.encode(e)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    K: Decodable + Hash<S> + Eq,
    V: Decodable,
    S,
    H: Hasher<S> + Default
> Decodable for HashMap<K, V, H> {
    fn decode<D: Decoder>(d: &mut D) -> Result<HashMap<K, V, H>, D::Error> {
        d.read_map(|d, len| {
            let hasher = Default::default();
            let mut map = HashMap::with_capacity_and_hasher(len, hasher);
            for i in range(0u, len) {
                let key = try!(d.read_map_elt_key(i, |d| Decodable::decode(d)));
                let val = try!(d.read_map_elt_val(i, |d| Decodable::decode(d)));
                map.insert(key, val);
            }
            Ok(map)
        })
    }
}

impl<
    T: Encodable + Hash<X> + Eq,
    X,
    H: Hasher<X>
> Encodable for HashSet<T, H> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_seq(self.len(), |s| {
            let mut i = 0;
            for e in self.iter() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    T: Decodable + Hash<S> + Eq,
    S,
    H: Hasher<S> + Default
> Decodable for HashSet<T, H> {
    fn decode<D: Decoder>(d: &mut D) -> Result<HashSet<T, H>, D::Error> {
        d.read_seq(|d, len| {
            let mut set = HashSet::with_capacity_and_hasher(len, Default::default());
            for i in range(0u, len) {
                set.insert(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(set)
        })
    }
}

impl<V: Encodable> Encodable for VecMap<V> {
    fn encode<S: Encoder>(&self, e: &mut S) -> Result<(), S::Error> {
        e.emit_map(self.len(), |e| {
                for (i, (key, val)) in self.iter().enumerate() {
                    try!(e.emit_map_elt_key(i, |e| key.encode(e)));
                    try!(e.emit_map_elt_val(i, |e| val.encode(e)));
                }
                Ok(())
            })
    }
}

impl<V: Decodable> Decodable for VecMap<V> {
    fn decode<D: Decoder>(d: &mut D) -> Result<VecMap<V>, D::Error> {
        d.read_map(|d, len| {
            let mut map = VecMap::new();
            for i in range(0u, len) {
                let key = try!(d.read_map_elt_key(i, |d| Decodable::decode(d)));
                let val = try!(d.read_map_elt_val(i, |d| Decodable::decode(d)));
                map.insert(key, val);
            }
            Ok(map)
        })
    }
}
