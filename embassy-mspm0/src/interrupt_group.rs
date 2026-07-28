//! Interrupt group definitions.

#![macro_use]

use crate::interrupt;

/// Marker trait for an interrupt which is an interrupt group.
#[allow(private_bounds)]
pub trait InterruptGroup: SealedInterruptGroup + interrupt::typelevel::Interrupt {
    /// The number of this interrupt group.
    const NUMBER: u8;
}

// Implementation details

pub(crate) trait SealedInterruptGroup {}

macro_rules! impl_interrupt_group {
    ($interrupt: ident, $number: expr) => {
        impl crate::interrupt_group::SealedInterruptGroup for crate::interrupt::typelevel::$interrupt {}
        impl crate::interrupt_group::InterruptGroup for crate::interrupt::typelevel::$interrupt {
            const NUMBER: u8 = $number;
        }
    };
}
