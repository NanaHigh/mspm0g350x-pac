#[doc = "Register `INT_EVENT1_ISET` writer"]
pub type W = crate::W<IntEvent1IsetSpec>;
#[doc = "Set UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRtout {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetRtoutNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRtoutSet = 1,
}
impl From<IntEvent1IsetRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RTOUT` writer - Set UARTOUT Receive Time-Out Interrupt."]
pub type IntEvent1IsetRtoutW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRtout>;
impl<'a, REG> IntEvent1IsetRtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_rtout_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtout::IntEvent1IsetRtoutNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rtout_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtout::IntEvent1IsetRtoutSet)
    }
}
#[doc = "Set UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetRxintNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRxintSet = 1,
}
impl From<IntEvent1IsetRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RXINT` writer - Set UART Receive Interrupt."]
pub type IntEvent1IsetRxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRxint>;
impl<'a, REG> IntEvent1IsetRxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_rxint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRxint::IntEvent1IsetRxintNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rxint_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRxint::IntEvent1IsetRxintSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event1_iset_rtout(&mut self) -> IntEvent1IsetRtoutW<IntEvent1IsetSpec> {
        IntEvent1IsetRtoutW::new(self, 0)
    }
    #[doc = "Bit 10 - Set UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event1_iset_rxint(&mut self) -> IntEvent1IsetRxintW<IntEvent1IsetSpec> {
        IntEvent1IsetRxintW::new(self, 10)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IsetSpec;
impl crate::RegisterSpec for IntEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent1IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ISET to value 0"]
impl crate::Resettable for IntEvent1IsetSpec {
    const RESET_VALUE: u32 = 0;
}
