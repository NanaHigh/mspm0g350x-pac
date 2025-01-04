#[doc = "Register `INT_EVENT2_ICLR` writer"]
pub type W = crate::W<IntEvent2IclrSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMrxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMrxfifotrgClr = 1,
}
impl From<IntEvent2IclrMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent2IclrMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMrxfifotrg>;
impl<'a, REG> IntEvent2IclrMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_mrxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMrxfifotrg::IntEvent2IclrMrxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_mrxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMrxfifotrg::IntEvent2IclrMrxfifotrgClr)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMtxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMtxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMtxfifotrgClr = 1,
}
impl From<IntEvent2IclrMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent2IclrMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMtxfifotrg>;
impl<'a, REG> IntEvent2IclrMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_mtxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMtxfifotrg::IntEvent2IclrMtxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_mtxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMtxfifotrg::IntEvent2IclrMtxfifotrgClr)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrSrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrSrxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrSrxfifotrgClr = 1,
}
impl From<IntEvent2IclrSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent2IclrSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrSrxfifotrg>;
impl<'a, REG> IntEvent2IclrSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_srxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrSrxfifotrg::IntEvent2IclrSrxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_srxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrSrxfifotrg::IntEvent2IclrSrxfifotrgClr)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrStxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrStxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrStxfifotrgClr = 1,
}
impl From<IntEvent2IclrStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent2IclrStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrStxfifotrg>;
impl<'a, REG> IntEvent2IclrStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_stxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrStxfifotrg::IntEvent2IclrStxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_stxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrStxfifotrg::IntEvent2IclrStxfifotrgClr)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_iclr_mrxfifotrg(&mut self) -> IntEvent2IclrMrxfifotrgW<IntEvent2IclrSpec> {
        IntEvent2IclrMrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_iclr_mtxfifotrg(&mut self) -> IntEvent2IclrMtxfifotrgW<IntEvent2IclrSpec> {
        IntEvent2IclrMtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_iclr_srxfifotrg(&mut self) -> IntEvent2IclrSrxfifotrgW<IntEvent2IclrSpec> {
        IntEvent2IclrSrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_iclr_stxfifotrg(&mut self) -> IntEvent2IclrStxfifotrgW<IntEvent2IclrSpec> {
        IntEvent2IclrStxfifotrgW::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IclrSpec;
impl crate::RegisterSpec for IntEvent2IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent2IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ICLR to value 0"]
impl crate::Resettable for IntEvent2IclrSpec {
    const RESET_VALUE: u32 = 0;
}
