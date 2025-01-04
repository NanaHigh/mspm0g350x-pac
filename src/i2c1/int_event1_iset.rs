#[doc = "Register `INT_EVENT1_ISET` writer"]
pub type W = crate::W<IntEvent1IsetSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetMrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetMrxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetMrxfifotrgSet = 1,
}
impl From<IntEvent1IsetMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent1IsetMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetMrxfifotrg>;
impl<'a, REG> IntEvent1IsetMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_mrxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetMrxfifotrg::IntEvent1IsetMrxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_mrxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetMrxfifotrg::IntEvent1IsetMrxfifotrgSet)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetMtxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetMtxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetMtxfifotrgSet = 1,
}
impl From<IntEvent1IsetMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent1IsetMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetMtxfifotrg>;
impl<'a, REG> IntEvent1IsetMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_mtxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetMtxfifotrg::IntEvent1IsetMtxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_mtxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetMtxfifotrg::IntEvent1IsetMtxfifotrgSet)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetSrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetSrxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetSrxfifotrgSet = 1,
}
impl From<IntEvent1IsetSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent1IsetSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetSrxfifotrg>;
impl<'a, REG> IntEvent1IsetSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_srxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetSrxfifotrg::IntEvent1IsetSrxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_srxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetSrxfifotrg::IntEvent1IsetSrxfifotrgSet)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetStxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetStxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetStxfifotrgSet = 1,
}
impl From<IntEvent1IsetStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent1IsetStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetStxfifotrg>;
impl<'a, REG> IntEvent1IsetStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_stxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetStxfifotrg::IntEvent1IsetStxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_stxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetStxfifotrg::IntEvent1IsetStxfifotrgSet)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_iset_mrxfifotrg(&mut self) -> IntEvent1IsetMrxfifotrgW<IntEvent1IsetSpec> {
        IntEvent1IsetMrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_iset_mtxfifotrg(&mut self) -> IntEvent1IsetMtxfifotrgW<IntEvent1IsetSpec> {
        IntEvent1IsetMtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_iset_srxfifotrg(&mut self) -> IntEvent1IsetSrxfifotrgW<IntEvent1IsetSpec> {
        IntEvent1IsetSrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_iset_stxfifotrg(&mut self) -> IntEvent1IsetStxfifotrgW<IntEvent1IsetSpec> {
        IntEvent1IsetStxfifotrgW::new(self, 3)
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
