#[doc = "Register `INT_EVENT1_ICLR` writer"]
pub type W = crate::W<IntEvent1IclrSpec>;
#[doc = "Clear UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRtout {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrRtoutNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrRtoutClr = 1,
}
impl From<IntEvent1IclrRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RTOUT` writer - Clear UARTOUT Receive Time-Out Interrupt."]
pub type IntEvent1IclrRtoutW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRtout>;
impl<'a, REG> IntEvent1IclrRtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_rtout_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtout::IntEvent1IclrRtoutNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rtout_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtout::IntEvent1IclrRtoutClr)
    }
}
#[doc = "Clear UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrRxintNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrRxintClr = 1,
}
impl From<IntEvent1IclrRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RXINT` writer - Clear UART Receive Interrupt."]
pub type IntEvent1IclrRxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRxint>;
impl<'a, REG> IntEvent1IclrRxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_rxint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRxint::IntEvent1IclrRxintNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rxint_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRxint::IntEvent1IclrRxintClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event1_iclr_rtout(&mut self) -> IntEvent1IclrRtoutW<IntEvent1IclrSpec> {
        IntEvent1IclrRtoutW::new(self, 0)
    }
    #[doc = "Bit 10 - Clear UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event1_iclr_rxint(&mut self) -> IntEvent1IclrRxintW<IntEvent1IclrSpec> {
        IntEvent1IclrRxintW::new(self, 10)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IclrSpec;
impl crate::RegisterSpec for IntEvent1IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent1IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ICLR to value 0"]
impl crate::Resettable for IntEvent1IclrSpec {
    const RESET_VALUE: u32 = 0;
}
