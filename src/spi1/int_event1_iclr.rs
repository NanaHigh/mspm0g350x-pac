#[doc = "Register `INT_EVENT1_ICLR` writer"]
pub type W = crate::W<IntEvent1IclrSpec>;
#[doc = "Clear SPI Receive Time-Out event.\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT1_ICLR_RTOUT` writer - Clear SPI Receive Time-Out event."]
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
#[doc = "Clear Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRx {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrRxNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrRxClr = 1,
}
impl From<IntEvent1IclrRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RX` writer - Clear Receive FIFO event."]
pub type IntEvent1IclrRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRx>;
impl<'a, REG> IntEvent1IclrRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_rx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRx::IntEvent1IclrRxNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRx::IntEvent1IclrRxClr)
    }
}
impl W {
    #[doc = "Bit 2 - Clear SPI Receive Time-Out event."]
    #[inline(always)]
    pub fn int_event1_iclr_rtout(&mut self) -> IntEvent1IclrRtoutW<IntEvent1IclrSpec> {
        IntEvent1IclrRtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Receive FIFO event."]
    #[inline(always)]
    pub fn int_event1_iclr_rx(&mut self) -> IntEvent1IclrRxW<IntEvent1IclrSpec> {
        IntEvent1IclrRxW::new(self, 3)
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
