use std::{
    any::Any,
    cell::{Cell, RefCell},
    fmt::{self, Debug, Formatter},
    future::Future,
    num::NonZeroU64,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;

thread_local! {
    // Always have at least one effect in flight. That way signals don't need to
    // check if getting the last value is valid.
    static EFFECTS: RefCell<Vec<Effect>> = const { RefCell::new(vec![Effect::new()]) };
}

#[derive(Debug, Clone, Default)]
struct Effect {
    signals: Vec<SignalId>,
    waker: Option<Waker>,
    is_triggered: bool,
}

impl Effect {
    pub const fn new() -> Self {
        Self {
            signals: vec![],
            waker: None,
            is_triggered: false,
        }
    }

    pub fn trigger(&mut self, signal: SignalId) -> SignalResult {
        // To avoid storing owned backlinks to Signal from Effect, signals will
        // only remove their reference to an Effect after a failed attempt to
        // trigger that Effect.
        if !self.signals.contains(&signal) {
            return SignalResult::NotListening;
        }

        self.is_triggered = true;
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
        SignalResult::Listening
    }
}

enum SignalResult {
    Listening,
    NotListening,
}

impl Future for Effect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_triggered {
            self.is_triggered = false;
            Poll::Ready(())
        } else {
            self.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub fn create_effect(callback: impl FnMut()) {
    future_to_promise(async move {
        // TODO: Reuse effect memory
        loop {
            EFFECTS.with(move |effects| effects.borrow_mut().push(Effect::new()));
            callback();
            let effect =
                EFFECTS.with(|effects| unsafe { effects.borrow_mut().pop().unwrap_unchecked() });
            if effect.signals.is_empty() {
                break Ok(JsValue::null());
            }
            effect.await;
        }
    });
}

struct SlotMap {
    slots: Vec<Slot>,
    head: u32,
}

impl SlotMap {
    pub const fn new() -> Self {
        Self {
            slots: vec![],
            head: 0,
        }
    }
}

struct SlotKey {
    index: u32,
    generation: u32,
}

struct Slot {
    generation: u32,
    contents: ContentsUnion,
}

impl Slot {
    const IS_FILLED_BIT: u32 = !(u32::MAX >> 1);

    pub fn new_empty(generation: u32, next: u32) -> Self {
        assert!(generation & Self::IS_FILLED_BIT == 0);
        Self {
            generation,
            contents: ContentsUnion { empty: next },
        }
    }

    pub fn new_filled(generation: u32, mut contents: Box<dyn Any>) -> Self {
        assert!(generation & Self::IS_FILLED_BIT == 0);
        Self {
            generation: generation | Self::IS_FILLED_BIT,
            contents: ContentsUnion {
                filled: contents.as_mut() as *mut dyn Any,
            },
        }
    }

    pub fn contents(&self) -> Contents {
        if self.generation & Self::IS_FILLED_BIT == 0 {
            Contents::Empty(self.contents.empty)
        } else {
            Contents::Filled(self.contents.filled)
        }
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        if self.generation & Self::IS_FILLED_BIT > 0 {
            let filling = unsafe { Box::from_raw(self.contents.filled) };
        }
    }
}

enum Contents {
    Empty(u32),
    Filled(*mut dyn Any),
}

union ContentsUnion {
    empty: u32,
    filled: *mut dyn Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Signal(SignalId);

type SignalId = u32;

pub fn create_signal<T>(value: T) -> (impl Fn() -> T, impl Fn(fn(T) -> T))
where
    T: Copy,
{
    let mut effects = vec![];
    let value = Rc::new(Cell::new(value));
    let get = {
        let value = value.clone();
        move || value.get()
    };
    let set = move |map: fn(T) -> T| {
        value.set(map(value.get()));
    };
    (get, set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
