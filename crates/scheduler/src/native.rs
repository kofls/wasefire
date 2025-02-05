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

use alloc::boxed::Box;
use core::ffi::{c_char, CStr};

use wasefire_board_api::Api as Board;
use wasefire_logger as log;
use wasefire_mutex::Mutex;

#[cfg(feature = "debug")]
use crate::perf::Slot;
use crate::Scheduler;

pub(crate) trait ErasedScheduler: Send {
    fn dispatch(&mut self, link: &CStr, params: *const u32, results: *mut u32);
    fn flush_events(&mut self);
    fn process_event(&mut self);
    #[cfg(feature = "debug")]
    fn perf_record(&mut self, slot: Slot);
}

impl<B: Board> ErasedScheduler for Scheduler<B> {
    fn dispatch(&mut self, link: &CStr, params: *const u32, results: *mut u32) {
        self.dispatch(link, params, results);
    }

    fn flush_events(&mut self) {
        self.flush_events();
    }

    fn process_event(&mut self) {
        self.process_event();
    }

    #[cfg(feature = "debug")]
    fn perf_record(&mut self, slot: Slot) {
        self.perf.record(slot);
    }
}

static SCHEDULER: Mutex<Option<Box<dyn ErasedScheduler>>> = Mutex::new(None);
static CALLBACK: Mutex<Option<Box<dyn Fn() + Send>>> = Mutex::new(None);

pub(crate) fn set_scheduler<B: Board>(scheduler: Scheduler<B>) {
    *SCHEDULER.lock() = Some(Box::new(scheduler));
}

pub(crate) fn with_scheduler<R>(f: impl FnOnce(&mut dyn ErasedScheduler) -> R) -> R {
    f(SCHEDULER.lock().as_deref_mut().unwrap())
}

pub(crate) fn schedule_callback(callback: Box<dyn Fn() + Send>) {
    assert!(CALLBACK.lock().replace(callback).is_none());
}

pub(crate) fn execute_callback() {
    if let Some(callback) = CALLBACK.lock().take() {
        #[cfg(feature = "debug")]
        with_scheduler(|x| x.perf_record(Slot::Platform));
        callback();
        #[cfg(feature = "debug")]
        with_scheduler(|x| x.perf_record(Slot::Applets));
        log::debug!("Callback executed.");
    }
}

#[no_mangle]
extern "C" fn env_dispatch(link: *const c_char, params: *const u32, results: *mut u32) {
    let link = unsafe { CStr::from_ptr(link) };
    with_scheduler(|scheduler| {
        #[cfg(feature = "debug")]
        scheduler.perf_record(Slot::Applets);
        scheduler.flush_events();
        scheduler.dispatch(link, params, results);
    });
    execute_callback();
    #[cfg(feature = "debug")]
    with_scheduler(|x| x.perf_record(Slot::Platform));
}
