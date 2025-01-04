#[doc = "Register `INT_EVENT1_ICLR` writer"]
pub type W = crate::W<IntEvent1IclrSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrMrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrMrxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrMrxfifotrgClr = 1,
}
impl From<IntEvent1IclrMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent1IclrMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrMrxfifotrg>;
impl<'a, REG> IntEvent1IclrMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_mrxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrMrxfifotrg::IntEvent1IclrMrxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_mrxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrMrxfifotrg::IntEvent1IclrMrxfifotrgClr)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrMtxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrMtxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrMtxfifotrgClr = 1,
}
impl From<IntEvent1IclrMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent1IclrMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrMtxfifotrg>;
impl<'a, REG> IntEvent1IclrMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_mtxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrMtxfifotrg::IntEvent1IclrMtxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_mtxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrMtxfifotrg::IntEvent1IclrMtxfifotrgClr)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrSrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrSrxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrSrxfifotrgClr = 1,
}
impl From<IntEvent1IclrSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent1IclrSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrSrxfifotrg>;
impl<'a, REG> IntEvent1IclrSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_srxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrSrxfifotrg::IntEvent1IclrSrxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_srxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrSrxfifotrg::IntEvent1IclrSrxfifotrgClr)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrStxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrStxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrStxfifotrgClr = 1,
}
impl From<IntEvent1IclrStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent1IclrStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrStxfifotrg>;
impl<'a, REG> IntEvent1IclrStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_stxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrStxfifotrg::IntEvent1IclrStxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_stxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrStxfifotrg::IntEvent1IclrStxfifotrgClr)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_iclr_mrxfifotrg(&mut self) -> IntEvent1IclrMrxfifotrgW<IntEvent1IclrSpec> {
        IntEvent1IclrMrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_iclr_mtxfifotrg(&mut self) -> IntEvent1IclrMtxfifotrgW<IntEvent1IclrSpec> {
        IntEvent1IclrMtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_iclr_srxfifotrg(&mut self) -> IntEvent1IclrSrxfifotrgW<IntEvent1IclrSpec> {
        IntEvent1IclrSrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_iclr_stxfifotrg(&mut self) -> IntEvent1IclrStxfifotrgW<IntEvent1IclrSpec> {
        IntEvent1IclrStxfifotrgW::new(self, 3)
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
