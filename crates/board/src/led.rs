// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! LED interface.
//!
//! A LED is an output interface with 2 states: on and off.

use crate::{Error, Id, Support, Unsupported};

/// LED interface.
pub trait Api: Support<usize> + Send {
    /// Returns whether a given LED is on.
    fn get(led: Id<Self>) -> Result<bool, Error>;

    /// Sets the state of a given LED.
    fn set(led: Id<Self>, on: bool) -> Result<(), Error>;
}

impl Api for Unsupported {
    fn get(_: Id<Self>) -> Result<bool, Error> {
        unreachable!()
    }

    fn set(_: Id<Self>, _: bool) -> Result<(), Error> {
        unreachable!()
    }
}
