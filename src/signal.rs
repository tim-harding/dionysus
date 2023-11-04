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

pub fn create_effect(mut callback: impl FnMut() + 'static) {
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

thread_local! {
    static SLOT_MAP: SlotMap = const { SlotMap::new() };
}

struct SlotMap {
    slots: Vec<SlotContents>,
    head: usize,
}

impl SlotMap {
    pub const fn new() -> Self {
        Self {
            slots: vec![],
            head: 0,
        }
    }

    pub fn insert<T>(&mut self, value: T) -> Slot {
        let value = Box::new(value);
        if self.head == self.slots.len() {
            self.slots.push(SlotContents::Occupied {
                references: 1,
                contents: value,
            });
        }
    }

    pub fn remove_reference(&mut self, index: u32) {
        let index = index as usize;
        let slot = self.slots.get_mut(index).unwrap();
        match slot {
            SlotContents::Empty { next_empty } => todo!(),
            SlotContents::Occupied {
                references,
                contents,
            } => todo!(),
        }
    }
}

struct Slot {
    index: u32,
}

impl Drop for Slot {
    fn drop(&mut self) {
        SLOT_MAP.with(|slot_map| slot_map.remove_reference(self.index));
    }
}

enum SlotContents {
    Empty {
        next_empty: usize,
    },
    Occupied {
        references: u64,
        contents: Box<dyn Any>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Signal(SignalId);

type SignalId = u32;
