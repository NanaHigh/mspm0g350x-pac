#[doc = "Register `INT_EVENT1_IMASK` reader"]
pub type R = crate::R<IntEvent1ImaskSpec>;
#[doc = "Register `INT_EVENT1_IMASK` writer"]
pub type W = crate::W<IntEvent1ImaskSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1ImaskMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskMrxfifotrgSet = 1,
}
impl From<IntEvent1ImaskMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent1ImaskMrxfifotrgR = crate::BitReader<IntEvent1ImaskMrxfifotrg>;
impl IntEvent1ImaskMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskMrxfifotrg {
        match self.bits {
            false => IntEvent1ImaskMrxfifotrg::IntEvent1ImaskMrxfifotrgClr,
            true => IntEvent1ImaskMrxfifotrg::IntEvent1ImaskMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent1ImaskMrxfifotrg::IntEvent1ImaskMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent1ImaskMrxfifotrg::IntEvent1ImaskMrxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent1ImaskMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskMrxfifotrg>;
impl<'a, REG> IntEvent1ImaskMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_mrxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskMrxfifotrg::IntEvent1ImaskMrxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_mrxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskMrxfifotrg::IntEvent1ImaskMrxfifotrgSet)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1ImaskMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskMtxfifotrgSet = 1,
}
impl From<IntEvent1ImaskMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent1ImaskMtxfifotrgR = crate::BitReader<IntEvent1ImaskMtxfifotrg>;
impl IntEvent1ImaskMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskMtxfifotrg {
        match self.bits {
            false => IntEvent1ImaskMtxfifotrg::IntEvent1ImaskMtxfifotrgClr,
            true => IntEvent1ImaskMtxfifotrg::IntEvent1ImaskMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent1ImaskMtxfifotrg::IntEvent1ImaskMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent1ImaskMtxfifotrg::IntEvent1ImaskMtxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent1ImaskMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskMtxfifotrg>;
impl<'a, REG> IntEvent1ImaskMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_mtxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskMtxfifotrg::IntEvent1ImaskMtxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_mtxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskMtxfifotrg::IntEvent1ImaskMtxfifotrgSet)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1ImaskSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskSrxfifotrgSet = 1,
}
impl From<IntEvent1ImaskSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent1ImaskSrxfifotrgR = crate::BitReader<IntEvent1ImaskSrxfifotrg>;
impl IntEvent1ImaskSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskSrxfifotrg {
        match self.bits {
            false => IntEvent1ImaskSrxfifotrg::IntEvent1ImaskSrxfifotrgClr,
            true => IntEvent1ImaskSrxfifotrg::IntEvent1ImaskSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent1ImaskSrxfifotrg::IntEvent1ImaskSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_srxfifotrg_set(&self) -> bool {
        *self == IntEvent1ImaskSrxfifotrg::IntEvent1ImaskSrxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent1ImaskSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskSrxfifotrg>;
impl<'a, REG> IntEvent1ImaskSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_srxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskSrxfifotrg::IntEvent1ImaskSrxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_srxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskSrxfifotrg::IntEvent1ImaskSrxfifotrgSet)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1ImaskStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskStxfifotrgSet = 1,
}
impl From<IntEvent1ImaskStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent1ImaskStxfifotrgR = crate::BitReader<IntEvent1ImaskStxfifotrg>;
impl IntEvent1ImaskStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskStxfifotrg {
        match self.bits {
            false => IntEvent1ImaskStxfifotrg::IntEvent1ImaskStxfifotrgClr,
            true => IntEvent1ImaskStxfifotrg::IntEvent1ImaskStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent1ImaskStxfifotrg::IntEvent1ImaskStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_stxfifotrg_set(&self) -> bool {
        *self == IntEvent1ImaskStxfifotrg::IntEvent1ImaskStxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent1ImaskStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskStxfifotrg>;
impl<'a, REG> IntEvent1ImaskStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_stxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskStxfifotrg::IntEvent1ImaskStxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_stxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskStxfifotrg::IntEvent1ImaskStxfifotrgSet)
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_imask_mrxfifotrg(&self) -> IntEvent1ImaskMrxfifotrgR {
        IntEvent1ImaskMrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_imask_mtxfifotrg(&self) -> IntEvent1ImaskMtxfifotrgR {
        IntEvent1ImaskMtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_imask_srxfifotrg(&self) -> IntEvent1ImaskSrxfifotrgR {
        IntEvent1ImaskSrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_imask_stxfifotrg(&self) -> IntEvent1ImaskStxfifotrgR {
        IntEvent1ImaskStxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_imask_mrxfifotrg(&mut self) -> IntEvent1ImaskMrxfifotrgW<IntEvent1ImaskSpec> {
        IntEvent1ImaskMrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_imask_mtxfifotrg(&mut self) -> IntEvent1ImaskMtxfifotrgW<IntEvent1ImaskSpec> {
        IntEvent1ImaskMtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_imask_srxfifotrg(&mut self) -> IntEvent1ImaskSrxfifotrgW<IntEvent1ImaskSpec> {
        IntEvent1ImaskSrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_imask_stxfifotrg(&mut self) -> IntEvent1ImaskStxfifotrgW<IntEvent1ImaskSpec> {
        IntEvent1ImaskStxfifotrgW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1ImaskSpec;
impl crate::RegisterSpec for IntEvent1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent1ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event1_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent1ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_IMASK to value 0"]
impl crate::Resettable for IntEvent1ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
