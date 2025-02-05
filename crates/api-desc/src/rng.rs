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

use crate::*;

pub(crate) fn new() -> Item {
    let docs = docs! {
        /// Random number generators.
    };
    let name = "rng".into();
    let items = vec![item! {
        /// Fills a slice with random bytes.
        fn fill_bytes "rb" {
            /// The slice to fill.
            ptr: *mut u8,

            /// The length of the slice.
            len: usize,
        } -> {
            /// Error code: 0 on success, -1 on error
            ///
            /// The buffer may be modified on error and should not be used.
            res: isize
        }
    }];
    Item::Mod(Mod { docs, name, items })
}
