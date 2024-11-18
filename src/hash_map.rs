use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use crate::{vec::{Classes, IntoClasses}, DisjointVec};

#[derive(Debug)]
pub struct DisjointHashMap<K, V> {
	keys: HashMap<K, usize>,
	inner: DisjointVec<V>,
}

impl<K, V> Default for DisjointHashMap<K, V> {
	fn default() -> Self {
		Self::new()
	}
}

impl<K, V> DisjointHashMap<K, V> {
	pub fn new() -> Self {
		Self {
			keys: HashMap::new(),
			inner: DisjointVec::new(),
		}
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}
}

impl<K: Eq + Hash, V> DisjointHashMap<K, V> {
	pub fn insert(&mut self, key: K, value: V) -> usize {
		let i = self.inner.push(value);
		self.keys.insert(key, i);
		i
	}

	pub fn as_vec(&self) -> &DisjointVec<V> {
		&self.inner
	}

	pub fn index_of<Q>(&self, key: &Q) -> Option<usize>
	where
		Q: ?Sized + Eq + Hash,
		K: Borrow<Q>,
	{
		self.keys.get(key).copied()
	}

	pub fn class_of<Q>(&self, key: &Q) -> Option<usize>
	where
		Q: ?Sized + Eq + Hash,
		K: Borrow<Q>,
	{
		let i = self.index_of(key)?;
		self.inner.class_of(i)
	}

	pub fn get_with_class<Q>(&self, key: &Q) -> Option<(usize, &V)>
	where
		Q: ?Sized + Eq + Hash,
		K: Borrow<Q>,
	{
		let i = self.index_of(key)?;
		self.inner.get_with_class(i)
	}

	pub fn get<Q>(&self, key: &Q) -> Option<&V>
	where
		Q: ?Sized + Eq + Hash,
		K: Borrow<Q>,
	{
		let i = self.index_of(key)?;
		self.inner.get(i)
	}

	pub fn replace<Q>(&mut self, key: &Q, value: V) -> Result<V, V>
	where
		Q: ?Sized + Eq + Hash,
		K: Borrow<Q>,
	{
		match self.index_of(key) {
			Some(i) => self.inner.replace(i, value),
			None => Err(value)
		}
	}

	pub fn merge<Q>(&mut self, a: &Q, b: &Q, f: impl FnOnce(V, V) -> V) -> Option<usize>
	where
		Q: ?Sized + Eq + Hash,
		K: Borrow<Q>,
	{
		if let Some(a) = self.index_of(a) {
			if let Some(b) = self.index_of(b) {
				return self.inner.merge(a, b, f);
			}
		}

		None
	}

	pub fn try_merge<Q, E>(&mut self, a: &Q, b: &Q, f: impl FnOnce(V, V) -> Result<V, E>) -> Result<Option<usize>, E>
	where
		Q: ?Sized + Eq + Hash,
		K: Borrow<Q>,
	{
		if let Some(a) = self.index_of(a) {
			if let Some(b) = self.index_of(b) {
				return self.inner.try_merge(a, b, f);
			}
		}

		Ok(None)
	}

	pub fn classes(&self) -> Classes<V> {
		self.inner.classes()
	}

	pub fn into_classes(self) -> IntoClasses<V> {
		self.inner.into_classes()
	}
}
