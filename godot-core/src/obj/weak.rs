/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use super::{Gd, RawGd};
use crate::obj::{Inherits, GodotClass};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};


// It's probably possible to simplify this to just <'a, T> and not <'a, T, U>
pub struct WeakGd<'a, T: Inherits<U>, U: GodotClass> {
    phantom: PhantomData<&'a U>,
    raw: *mut RawGd<T>,
}
impl<'a, T: Inherits<U>, U: GodotClass> From<&'a Gd<T>> for WeakGd<'a, T, U> {
    fn from(from: &'a Gd<T>) -> Self {
        // this is probably insane.
        Self {
            phantom: PhantomData,
            raw: unsafe { std::mem::transmute(from.get_ffi()) }
        }
    }
}
impl<'a, T: Inherits<U>, U: GodotClass> Deref for WeakGd<'a, T, U> {
    type Target = Gd<U>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&*self.raw) }
    }
}
impl<'a, T: Inherits<U>, U: GodotClass> DerefMut for WeakGd<'a, T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(&mut *self.raw) }
    }
}
impl<'a, T: Inherits<U>, U: GodotClass> Debug for WeakGd<'a, T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.raw.fmt(f)
    }
}
