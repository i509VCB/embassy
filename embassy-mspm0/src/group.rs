use crate::{pac, sealed::Sealed};

/// Interrupt handler trait.
///
/// Drivers that need to handle interrupts implement this trait.
/// The user must ensure `on_group_interrupt()` is called every time the interrupt fires.
/// Drivers must use use [`Binding`] to assert at compile time that the user has done so.
pub trait Handler<I: GroupInterrupt<G>, G: InterruptGroup> {
    /// Interrupt group handler function.
    ///
    /// Must be called every time the `I` interrupt fires, synchronously from
    /// the interrupt handler context.
    ///
    /// # Safety
    ///
    /// This function must ONLY be called from the interrupt handler for `I`.
    unsafe fn on_group_interrupt();
}

/// Compile-time assertion that an interrupt has been bound to a group handler.
///
/// For the vast majority of cases, you should use the `bind_groups!`
/// macro instead of writing `unsafe impl`s of this trait.
///
/// # Safety
///
/// By implementing this trait, you are asserting that you have arranged for `H::on_interrupt()`
/// to be called every time the `I` interrupt group fires.
///
/// This allows drivers to check bindings at compile-time.
pub unsafe trait Binding<I: GroupInterrupt<G>, G: InterruptGroup, H: Handler<I, G>> {}

/// An interrupt group.
#[allow(private_bounds)]
pub unsafe trait InterruptGroup: SealedInterruptGroup {
    /// The typelevel interrupt for this group.
    type Interrupt: crate::interrupt::typelevel::Interrupt;

    /// Get the index of the highest priority pending interrupt in this group.
    ///
    /// Returns [`None`] if no interrupt is pending.
    #[doc(hidden)]
    fn stat() -> Option<u8> {
        let stat = pac::CPUSS.int_group(Self::IIDX)
            .iidx()
            .read()
            // FIXME: 
            .stat()
            .to_bits();

        // 0 is NO_INTR
        if stat != 0 {
            return Some(stat);
        }

        None
    }
}

// Make the group enums accessible.
pub use crate::_generated::groups::*;

/// An interrupt belonging to an interrupt group.
#[allow(private_bounds)]
pub trait GroupInterrupt<Group: InterruptGroup>: Sealed {
    /// The index for this interrupt.
    const INDEX: u8;
}

pub(crate) unsafe trait SealedInterruptGroup {
    /// The IIDX register index for this interrupt group.
    const IIDX: usize;
}
