#[doc = "Register `INT_EVENT2_IMASK` reader"]
pub type R = crate::R<IntEvent2ImaskSpec>;
#[doc = "Register `INT_EVENT2_IMASK` writer"]
pub type W = crate::W<IntEvent2ImaskSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2ImaskMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMrxfifotrgSet = 1,
}
impl From<IntEvent2ImaskMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent2ImaskMrxfifotrgR = crate::BitReader<IntEvent2ImaskMrxfifotrg>;
impl IntEvent2ImaskMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMrxfifotrg {
        match self.bits {
            false => IntEvent2ImaskMrxfifotrg::IntEvent2ImaskMrxfifotrgClr,
            true => IntEvent2ImaskMrxfifotrg::IntEvent2ImaskMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent2ImaskMrxfifotrg::IntEvent2ImaskMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent2ImaskMrxfifotrg::IntEvent2ImaskMrxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent2ImaskMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMrxfifotrg>;
impl<'a, REG> IntEvent2ImaskMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_mrxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMrxfifotrg::IntEvent2ImaskMrxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_mrxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMrxfifotrg::IntEvent2ImaskMrxfifotrgSet)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2ImaskMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMtxfifotrgSet = 1,
}
impl From<IntEvent2ImaskMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent2ImaskMtxfifotrgR = crate::BitReader<IntEvent2ImaskMtxfifotrg>;
impl IntEvent2ImaskMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMtxfifotrg {
        match self.bits {
            false => IntEvent2ImaskMtxfifotrg::IntEvent2ImaskMtxfifotrgClr,
            true => IntEvent2ImaskMtxfifotrg::IntEvent2ImaskMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent2ImaskMtxfifotrg::IntEvent2ImaskMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent2ImaskMtxfifotrg::IntEvent2ImaskMtxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent2ImaskMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMtxfifotrg>;
impl<'a, REG> IntEvent2ImaskMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_mtxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMtxfifotrg::IntEvent2ImaskMtxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_mtxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMtxfifotrg::IntEvent2ImaskMtxfifotrgSet)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2ImaskSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskSrxfifotrgSet = 1,
}
impl From<IntEvent2ImaskSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent2ImaskSrxfifotrgR = crate::BitReader<IntEvent2ImaskSrxfifotrg>;
impl IntEvent2ImaskSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskSrxfifotrg {
        match self.bits {
            false => IntEvent2ImaskSrxfifotrg::IntEvent2ImaskSrxfifotrgClr,
            true => IntEvent2ImaskSrxfifotrg::IntEvent2ImaskSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent2ImaskSrxfifotrg::IntEvent2ImaskSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_srxfifotrg_set(&self) -> bool {
        *self == IntEvent2ImaskSrxfifotrg::IntEvent2ImaskSrxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT2_IMASK_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent2ImaskSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskSrxfifotrg>;
impl<'a, REG> IntEvent2ImaskSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_srxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskSrxfifotrg::IntEvent2ImaskSrxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_srxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskSrxfifotrg::IntEvent2ImaskSrxfifotrgSet)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2ImaskStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskStxfifotrgSet = 1,
}
impl From<IntEvent2ImaskStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent2ImaskStxfifotrgR = crate::BitReader<IntEvent2ImaskStxfifotrg>;
impl IntEvent2ImaskStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskStxfifotrg {
        match self.bits {
            false => IntEvent2ImaskStxfifotrg::IntEvent2ImaskStxfifotrgClr,
            true => IntEvent2ImaskStxfifotrg::IntEvent2ImaskStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent2ImaskStxfifotrg::IntEvent2ImaskStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_stxfifotrg_set(&self) -> bool {
        *self == IntEvent2ImaskStxfifotrg::IntEvent2ImaskStxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT2_IMASK_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent2ImaskStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskStxfifotrg>;
impl<'a, REG> IntEvent2ImaskStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_stxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskStxfifotrg::IntEvent2ImaskStxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_stxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskStxfifotrg::IntEvent2ImaskStxfifotrgSet)
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_imask_mrxfifotrg(&self) -> IntEvent2ImaskMrxfifotrgR {
        IntEvent2ImaskMrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_imask_mtxfifotrg(&self) -> IntEvent2ImaskMtxfifotrgR {
        IntEvent2ImaskMtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_imask_srxfifotrg(&self) -> IntEvent2ImaskSrxfifotrgR {
        IntEvent2ImaskSrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_imask_stxfifotrg(&self) -> IntEvent2ImaskStxfifotrgR {
        IntEvent2ImaskStxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_imask_mrxfifotrg(&mut self) -> IntEvent2ImaskMrxfifotrgW<IntEvent2ImaskSpec> {
        IntEvent2ImaskMrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_imask_mtxfifotrg(&mut self) -> IntEvent2ImaskMtxfifotrgW<IntEvent2ImaskSpec> {
        IntEvent2ImaskMtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_imask_srxfifotrg(&mut self) -> IntEvent2ImaskSrxfifotrgW<IntEvent2ImaskSpec> {
        IntEvent2ImaskSrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_imask_stxfifotrg(&mut self) -> IntEvent2ImaskStxfifotrgW<IntEvent2ImaskSpec> {
        IntEvent2ImaskStxfifotrgW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2ImaskSpec;
impl crate::RegisterSpec for IntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for IntEvent2ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
