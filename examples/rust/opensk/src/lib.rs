// Copyright 2023 Google LLC
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

//! TODO: Add docs

#![no_std]
wasefire::applet!();

use opensk::api::customization::{CustomizationImpl, AAGUID_LENGTH, DEFAULT_CUSTOMIZATION};
use opensk::api::rng::Rng;
use opensk::env::Env;
use rand_core::{impls, CryptoRng, Error, RngCore};

fn main() {
    debug!("hello world");
}

struct WasefireRng {}

impl Default for WasefireRng {
    fn default() -> Self {}
}

impl CryptoRng for WasefireRng {}

impl RngCore for WasefireRng {
    fn next_u32(&mut self) -> u32 {
        impls::next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_fill(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rng::fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        rng::fill_bytes(dest)
    }
}

impl Rng for WasefireRng {}

pub const AAGUID: &[u8; AAGUID_LENGTH] = todo!();
// include_bytes!(concat!(env!("OUT_DIR"), "/opensk_aaguid.bin"));

const WASEFIRE_CUSTOMIZATION: CustomizationImpl =
    CustomizationImpl { aaguid: AAGUID, ..DEFAULT_CUSTOMIZATION };

struct WasefireEnv {
    rng: WasefireRng,
    // write: WasefireWrite,
    // store: Store<Storage<S, C>>,
    // upgrade_storage: Option<UpgradeStorage<S, C>>,
    // main_connection: TockHidConnection<S>,
    // vendor_connection: TockHidConnection<S>,
    // blink_pattern: usize,
    // clock: TockClock<S>,
    // c: PhantomData<C>,
}

impl Default for WasefireEnv {
    fn default() -> Self {
        let rng = WasefireRng::default();
        WasefireEnv { rng }
    }
}

impl Env for WasefireEnv {
    type Rng = WasefireRng;
    // type Customization = CustomizationImpl;

    fn rng(&mut self) -> &mut Self::Rng {
        &mut self.rng
    }

    fn customization(&self) -> &Self::Customization {
        &WASEFIRE_CUSTOMIZATION
    }
}

// This depends on `std` features in opensk. But wasefire::applet is `no_std`?
#[cfg(test)]
mod test {
    use opensk::api::customization::is_valid;

    use super::*;

    #[test]
    fn test_invariants() {
        assert!(is_valid(&WASEFIRE_CUSTOMIZATION));
    }
}

// TODO: Add tests for Rng, Customization and others
