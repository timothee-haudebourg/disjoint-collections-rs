use std::cell::Cell;

/// Union find.
#[derive(Debug, Clone)]
pub struct DisjointVec<T>(Vec<Item<T>>);

impl<T> Default for DisjointVec<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> DisjointVec<T> {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self(Vec::with_capacity(capacity))
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn push(&mut self, value: T) -> usize {
		let i = self.len();
		self.0.push(Item::Class(value));
		i
	}

	pub fn class_of(&self, i: usize) -> Option<usize> {
		Some(match self.0.get(i)? {
			Item::Class(_) => i,
			Item::Indirection(j) => {
				let k = self.class_of(j.get())?;
				j.set(k);
				k
			}
		})
	}
	
	pub fn get_with_class(&self, i: usize) -> Option<(usize, &T)> {
		Some(match self.0.get(i)? {
			Item::Class(t) => (i, t),
			Item::Indirection(j) => {
				let (k, t) = self.get_with_class(j.get())?;
				j.set(k);
				(k, t)
			}
		})
	}

	pub fn get(&self, i: usize) -> Option<&T> {
		self.get_with_class(i).map(|(_, v)| v)
	}

	pub fn get_mut_with_class(&mut self, mut i: usize) -> Option<(usize, &mut T)> {
		loop {
			match self.0.get(i)? {
				Item::Class(_) => {
					break Some((i, self.0.get_mut(i).unwrap().as_value_mut().unwrap()))
				}
				Item::Indirection(j) => i = j.get(),
			}
		}
	}

	pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
		self.get_mut_with_class(i).map(|(_, v)| v)
	}

	pub fn replace(&mut self, i: usize, value: T) -> Result<T, T> {
		match self.get_mut(i) {
			Some(current_value) => Ok(std::mem::replace(current_value, value)),
			None => Err(value)
		}
	}

	pub fn merge(&mut self, a: usize, b: usize, f: impl FnOnce(T, T) -> T) -> Option<usize> {
		if let Some(mut ac) = self.class_of(a) {
			if let Some(mut bc) = self.class_of(b) {
				if ac == bc {
					return Some(ac);
				} else {
					let av = std::mem::replace(&mut self.0[ac], Item::Indirection(Cell::new(0)))
						.into_value()
						.unwrap();
					let bv = std::mem::replace(&mut self.0[bc], Item::Indirection(Cell::new(0)))
						.into_value()
						.unwrap();
					let value = f(av, bv);

					if bc < ac {
						std::mem::swap(&mut ac, &mut bc);
					}

					self.0[ac] = Item::Class(value);
					self.0[bc] = Item::Indirection(Cell::new(ac));
					return Some(ac);
				}
			}
		}

		None
	}

	pub fn try_merge<E>(
		&mut self,
		a: usize,
		b: usize,
		f: impl FnOnce(T, T) -> Result<T, E>,
	) -> Result<Option<usize>, E> {
		if let Some(mut ac) = self.class_of(a) {
			if let Some(mut bc) = self.class_of(b) {
				if ac == bc {
					return Ok(Some(ac));
				} else {
					let av = std::mem::replace(&mut self.0[ac], Item::Indirection(Cell::new(0)))
						.into_value()
						.unwrap();
					let bv = std::mem::replace(&mut self.0[bc], Item::Indirection(Cell::new(0)))
						.into_value()
						.unwrap();
					match f(av, bv) {
						Ok(value) => {
							if bc < ac {
								std::mem::swap(&mut ac, &mut bc);
							}

							self.0[ac] = Item::Class(value);
							self.0[bc] = Item::Indirection(Cell::new(ac));
							return Ok(Some(ac));
						}
						Err(e) => {
							self.0.clear();
							return Err(e);
						}
					}
				}
			}
		}

		Ok(None)
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> DisjointVec<U> {
		DisjointVec(self.0.into_iter().map(|item| item.map(&mut f)).collect())
	}

	pub fn classes(&self) -> Classes<T> {
		Classes(self.0.iter().enumerate())
	}

	pub fn into_classes(self) -> IntoClasses<T> {
		IntoClasses(self.0.into_iter().enumerate())
	}
}

impl<T> Extend<T> for DisjointVec<T> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		self.0.extend(iter.into_iter().map(Item::Class));
	}
}

impl<T> FromIterator<T> for DisjointVec<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut result = Self::default();
		result.extend(iter);
		result
	}
}

#[derive(Debug, Clone)]
enum Item<T> {
	Class(T),
	Indirection(Cell<usize>),
}

impl<T> Item<T> {
	fn into_value(self) -> Option<T> {
		match self {
			Self::Class(t) => Some(t),
			Self::Indirection(_) => None,
		}
	}

	fn as_value_mut(&mut self) -> Option<&mut T> {
		match self {
			Self::Class(t) => Some(t),
			Self::Indirection(_) => None,
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Item<U> {
		match self {
			Self::Class(t) => Item::Class(f(t)),
			Self::Indirection(c) => Item::Indirection(c)
		}
	}
}

pub struct Classes<'a, T>(std::iter::Enumerate<std::slice::Iter<'a, Item<T>>>);

impl<'a, T> Iterator for Classes<'a, T> {
	type Item = (usize, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		for (i, item) in &mut self.0 {
			if let Item::Class(t) = item {
				return Some((i, t));
			}
		}

		None
	}
}

pub struct IntoClasses<T>(std::iter::Enumerate<std::vec::IntoIter<Item<T>>>);

impl<T> Iterator for IntoClasses<T> {
	type Item = (usize, T);

	fn next(&mut self) -> Option<Self::Item> {
		for (i, item) in &mut self.0 {
			if let Item::Class(t) = item {
				return Some((i, t));
			}
		}

		None
	}
}