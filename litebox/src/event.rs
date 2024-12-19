//! Event related functionality

// TODO(jayb): Is this `poller`/`pollee` interface the right interface we want?

use thiserror::Error;

/// A `Pollee` maintains a set of active [`PollEvents`], which can be polled with [`Poller`]s or be
/// monitored with observers.
pub struct Pollee {
    events: PollEvents,
}

impl Default for Pollee {
    fn default() -> Self {
        Self::new()
    }
}

impl Pollee {
    /// Creates a new `Pollee`.
    pub fn new() -> Self {
        Pollee {
            events: PollEvents::empty(),
        }
    }

    /// Add `events` to the active set
    pub fn add_events(&mut self, events: PollEvents) -> &mut Self {
        self.events.insert(events);
        self
    }

    /// Remove `events` from the active set
    pub fn remove_events(&mut self, events: PollEvents) -> &mut Self {
        self.events.remove(events);
        self
    }

    /// Reset the active set to the default
    pub fn reset_events(&mut self) -> &mut Self {
        self.events = PollEvents::empty();
        self
    }

    /// Get the current active set of events
    pub fn events(&self) -> PollEvents {
        self.events
    }
}

/// A `Poller` gets notified when its associated [`Pollee`] has interesting events.
pub struct Poller {
    __todo: (),
}

impl Default for Poller {
    fn default() -> Self {
        Self::new()
    }
}

impl Poller {
    /// Creates a new `Poller`
    pub fn new() -> Self {
        Poller { __todo: () }
    }

    // Wait for the poller to be notified.
    //
    /// Note that this function may get spurious wake-ups.
    pub fn wait(&self) -> Result<(), WaitError> {
        todo!()
    }

    /// Wait for the poller to be notified, timing out after a specified duration.
    pub fn wait_timeout(&self, timeout: core::time::Duration) -> Result<(), WaitError> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum WaitError {
    #[error("Operation timed out")]
    TimeOut,
    #[error("Operation was interrupted")]
    Interrupted,
}

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct PollEvents: u32 {
        /// `POLLIN`: There is data to be read.
        const IN    = 0x0001;
        /// `POLLPRI`: There is some exceptional condition on the file descriptor.
        const PRI   = 0x0002;
        /// `POLLOUT`: Writing is now possible, though a write larger than the available space in a socket or pipe will still block.
        const OUT   = 0x0004;
        /// `POLLERR`: Error condition (only returned in `revents`; ignored in `events`).
        const ERR   = 0x0008;
        /// `POLLHUP`: Hang up (only returned in revents; ignored in events).
        const HUP   = 0x0010;
        /// `POLLNVAL`: Invalid request: fd not open (only returned in revents; ignored in events).
        const NVAL  = 0x0020;
        /// `POLLRDHUP`: Stream socket peer closed connection, or shut down writing half of connection.
        const RDHUP = 0x2000;

        /// Events that can be returned even if they are not specified
        const ALWAYS_POLLED = Self::ERR.bits() | Self::HUP.bits() | Self::NVAL.bits();

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}
