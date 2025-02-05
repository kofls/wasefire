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

use wasefire_applet_api::uart::{self as api, Api};
use wasefire_board_api::uart::{Api as _, Direction, Event};
use wasefire_board_api::{self as board, Api as Board, Id, Support};

use crate::applet::store::MemoryApi;
use crate::event::uart::Key;
use crate::event::Handler;
use crate::{DispatchSchedulerCall, SchedulerCall, Trap};

pub fn process<B: Board>(call: Api<DispatchSchedulerCall<B>>) {
    match call {
        Api::Count(call) => count(call),
        Api::Read(call) => read(call),
        Api::Write(call) => write(call),
        Api::Register(call) => register(call),
        Api::Unregister(call) => unregister(call),
    }
}

fn count<B: Board>(call: SchedulerCall<B, api::count::Sig>) {
    let api::count::Params {} = call.read();
    let count = board::Uart::<B>::SUPPORT as u32;
    call.reply(Ok(api::count::Results { cnt: count.into() }));
}

fn read<B: Board>(mut call: SchedulerCall<B, api::read::Sig>) {
    let api::read::Params { uart, ptr, len } = call.read();
    let scheduler = call.scheduler();
    let memory = scheduler.applet.memory();
    let results = try {
        let uart = Id::new(*uart as usize).ok_or(Trap)?;
        let output = memory.get_mut(*ptr, *len)?;
        let len = match board::Uart::<B>::read(uart, output) {
            Ok(len) => (len as u32).into(),
            Err(_) => u32::MAX.into(),
        };
        api::read::Results { len }
    };
    call.reply(results);
}

fn write<B: Board>(mut call: SchedulerCall<B, api::write::Sig>) {
    let api::write::Params { uart, ptr, len } = call.read();
    let scheduler = call.scheduler();
    let memory = scheduler.applet.memory();
    let results = try {
        let uart = Id::new(*uart as usize).ok_or(Trap)?;
        let input = memory.get(*ptr, *len)?;
        let len = match board::Uart::<B>::write(uart, input) {
            Ok(len) => (len as u32).into(),
            Err(_) => u32::MAX.into(),
        };
        api::write::Results { len }
    };
    call.reply(results);
}

fn register<B: Board>(mut call: SchedulerCall<B, api::register::Sig>) {
    let api::register::Params { uart, event, handler_func, handler_data } = call.read();
    let inst = call.inst();
    let scheduler = call.scheduler();
    let results = try {
        let uart = Id::new(*uart as usize).ok_or(Trap)?;
        let event = convert_event(uart, *event)?;
        scheduler.applet.enable(Handler {
            key: Key::from(&event).into(),
            inst,
            func: *handler_func,
            data: *handler_data,
        })?;
        board::Uart::<B>::enable(uart, event.direction).map_err(|_| Trap)?;
        api::register::Results {}
    };
    call.reply(results);
}

fn unregister<B: Board>(mut call: SchedulerCall<B, api::unregister::Sig>) {
    let api::unregister::Params { uart, event } = call.read();
    let scheduler = call.scheduler();
    let results = try {
        let uart = Id::new(*uart as usize).ok_or(Trap)?;
        let event = convert_event(uart, *event)?;
        board::Uart::<B>::disable(uart, event.direction).map_err(|_| Trap)?;
        scheduler.disable_event(Key::from(&event).into())?;
        api::unregister::Results {}
    };
    call.reply(results);
}

fn convert_event<B: Board>(uart: Id<board::Uart<B>>, event: u32) -> Result<Event<B>, Trap> {
    let direction = match api::Event::try_from(event)? {
        api::Event::Read => Direction::Read,
        api::Event::Write => Direction::Write,
    };
    Ok(Event { uart, direction })
}
