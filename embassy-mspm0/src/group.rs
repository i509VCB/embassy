use core::marker::PhantomData;

use crate::interrupt;
use crate::pac;
use crate::pac::cpuss::vals;

pub enum GROUP1 {}
impl Group for GROUP1 {
    type Interrupt = interrupt::typelevel::GROUP1;

    const INTERRUPT_GROUP: u8 = 1;
}

/// Interrupt handler for a group handler.
pub struct GroupInterruptHandler<G: Group, B: GroupHandler<G>> {
    _marker: PhantomData<(G, B)>,
}

impl<G: Group, H: GroupHandler<G>> interrupt::typelevel::Handler<G::Interrupt> for GroupInterruptHandler<G, H> {
    unsafe fn on_interrupt() {
        let group = pac::CPUSS.int_group(G::INTERRUPT_GROUP as usize);
        let stat = group.iidx().read().stat();

        if stat == vals::Iidx::NO_INTR {
            return;
        }

        let Some(iidx) = stat.to_bits().checked_sub(0) else {
            return;
        };

        unsafe { H::on_interrupt(iidx) };
    }
}

/// Marker trait for a group interrupt.
pub trait Group {
    type Interrupt: interrupt::typelevel::Interrupt;

    /// Interrupt group index.
    const INTERRUPT_GROUP: u8;
}

pub trait GroupHandler<G: Group> {
    unsafe fn on_interrupt(iidx: u8);
}

// implemented on type by macro
pub unsafe trait GroupBinding<G: Group, H: GroupHandler<G>>: Copy {}

pub trait GroupInterrupt<G: Group> {
    /// The IIDX of this group interrupt.
    const IIDX: u8;
}

pub trait Handler<I: GroupInterrupt<G>, G: Group> {
    unsafe fn on_interrupt();
}
