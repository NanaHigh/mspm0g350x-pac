#[doc = "Register `INT_EVENT2_ISET` writer"]
pub type W = crate::W<IntEvent2IsetSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMrxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMrxfifotrgSet = 1,
}
impl From<IntEvent2IsetMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent2IsetMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMrxfifotrg>;
impl<'a, REG> IntEvent2IsetMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_mrxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMrxfifotrg::IntEvent2IsetMrxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_mrxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMrxfifotrg::IntEvent2IsetMrxfifotrgSet)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMtxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMtxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMtxfifotrgSet = 1,
}
impl From<IntEvent2IsetMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent2IsetMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMtxfifotrg>;
impl<'a, REG> IntEvent2IsetMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_mtxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMtxfifotrg::IntEvent2IsetMtxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_mtxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMtxfifotrg::IntEvent2IsetMtxfifotrgSet)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetSrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetSrxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetSrxfifotrgSet = 1,
}
impl From<IntEvent2IsetSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent2IsetSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetSrxfifotrg>;
impl<'a, REG> IntEvent2IsetSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_srxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetSrxfifotrg::IntEvent2IsetSrxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_srxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetSrxfifotrg::IntEvent2IsetSrxfifotrgSet)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetStxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetStxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetStxfifotrgSet = 1,
}
impl From<IntEvent2IsetStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent2IsetStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetStxfifotrg>;
impl<'a, REG> IntEvent2IsetStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_stxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetStxfifotrg::IntEvent2IsetStxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_stxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetStxfifotrg::IntEvent2IsetStxfifotrgSet)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_iset_mrxfifotrg(&mut self) -> IntEvent2IsetMrxfifotrgW<IntEvent2IsetSpec> {
        IntEvent2IsetMrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_iset_mtxfifotrg(&mut self) -> IntEvent2IsetMtxfifotrgW<IntEvent2IsetSpec> {
        IntEvent2IsetMtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_iset_srxfifotrg(&mut self) -> IntEvent2IsetSrxfifotrgW<IntEvent2IsetSpec> {
        IntEvent2IsetSrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_iset_stxfifotrg(&mut self) -> IntEvent2IsetStxfifotrgW<IntEvent2IsetSpec> {
        IntEvent2IsetStxfifotrgW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IsetSpec;
impl crate::RegisterSpec for IntEvent2IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent2IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ISET to value 0"]
impl crate::Resettable for IntEvent2IsetSpec {
    const RESET_VALUE: u32 = 0;
}
