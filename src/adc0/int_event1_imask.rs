#[doc = "Register `INT_EVENT1_IMASK` reader"]
pub type R = crate::R<IntEvent1ImaskSpec>;
#[doc = "Register `INT_EVENT1_IMASK` writer"]
pub type W = crate::W<IntEvent1ImaskSpec>;
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskHighifg {
    #[doc = "0: CLR"]
    IntEvent1ImaskHighifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskHighifgSet = 1,
}
impl From<IntEvent1ImaskHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1ImaskHighifgR = crate::BitReader<IntEvent1ImaskHighifg>;
impl IntEvent1ImaskHighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskHighifg {
        match self.bits {
            false => IntEvent1ImaskHighifg::IntEvent1ImaskHighifgClr,
            true => IntEvent1ImaskHighifg::IntEvent1ImaskHighifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_highifg_clr(&self) -> bool {
        *self == IntEvent1ImaskHighifg::IntEvent1ImaskHighifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_highifg_set(&self) -> bool {
        *self == IntEvent1ImaskHighifg::IntEvent1ImaskHighifgSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1ImaskHighifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskHighifg>;
impl<'a, REG> IntEvent1ImaskHighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_highifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskHighifg::IntEvent1ImaskHighifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_highifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskHighifg::IntEvent1ImaskHighifgSet)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskLowifg {
    #[doc = "0: CLR"]
    IntEvent1ImaskLowifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskLowifgSet = 1,
}
impl From<IntEvent1ImaskLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1ImaskLowifgR = crate::BitReader<IntEvent1ImaskLowifg>;
impl IntEvent1ImaskLowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskLowifg {
        match self.bits {
            false => IntEvent1ImaskLowifg::IntEvent1ImaskLowifgClr,
            true => IntEvent1ImaskLowifg::IntEvent1ImaskLowifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_lowifg_clr(&self) -> bool {
        *self == IntEvent1ImaskLowifg::IntEvent1ImaskLowifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_lowifg_set(&self) -> bool {
        *self == IntEvent1ImaskLowifg::IntEvent1ImaskLowifgSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1ImaskLowifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskLowifg>;
impl<'a, REG> IntEvent1ImaskLowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_lowifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskLowifg::IntEvent1ImaskLowifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_lowifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskLowifg::IntEvent1ImaskLowifgSet)
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskInifg {
    #[doc = "0: CLR"]
    IntEvent1ImaskInifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskInifgSet = 1,
}
impl From<IntEvent1ImaskInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_INIFG` reader - Mask INIFG in MIS_EX register."]
pub type IntEvent1ImaskInifgR = crate::BitReader<IntEvent1ImaskInifg>;
impl IntEvent1ImaskInifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskInifg {
        match self.bits {
            false => IntEvent1ImaskInifg::IntEvent1ImaskInifgClr,
            true => IntEvent1ImaskInifg::IntEvent1ImaskInifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_inifg_clr(&self) -> bool {
        *self == IntEvent1ImaskInifg::IntEvent1ImaskInifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_inifg_set(&self) -> bool {
        *self == IntEvent1ImaskInifg::IntEvent1ImaskInifgSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_INIFG` writer - Mask INIFG in MIS_EX register."]
pub type IntEvent1ImaskInifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskInifg>;
impl<'a, REG> IntEvent1ImaskInifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_inifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskInifg::IntEvent1ImaskInifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_inifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskInifg::IntEvent1ImaskInifgSet)
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent1ImaskMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskMemresifg0Set = 1,
}
impl From<IntEvent1ImaskMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent1ImaskMemresifg0R = crate::BitReader<IntEvent1ImaskMemresifg0>;
impl IntEvent1ImaskMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskMemresifg0 {
        match self.bits {
            false => IntEvent1ImaskMemresifg0::IntEvent1ImaskMemresifg0Clr,
            true => IntEvent1ImaskMemresifg0::IntEvent1ImaskMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_memresifg0_clr(&self) -> bool {
        *self == IntEvent1ImaskMemresifg0::IntEvent1ImaskMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_memresifg0_set(&self) -> bool {
        *self == IntEvent1ImaskMemresifg0::IntEvent1ImaskMemresifg0Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent1ImaskMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskMemresifg0>;
impl<'a, REG> IntEvent1ImaskMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_memresifg0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskMemresifg0::IntEvent1ImaskMemresifg0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_memresifg0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskMemresifg0::IntEvent1ImaskMemresifg0Set)
    }
}
impl R {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_imask_highifg(&self) -> IntEvent1ImaskHighifgR {
        IntEvent1ImaskHighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_imask_lowifg(&self) -> IntEvent1ImaskLowifgR {
        IntEvent1ImaskLowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event1_imask_inifg(&self) -> IntEvent1ImaskInifgR {
        IntEvent1ImaskInifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event1_imask_memresifg0(&self) -> IntEvent1ImaskMemresifg0R {
        IntEvent1ImaskMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_imask_highifg(&mut self) -> IntEvent1ImaskHighifgW<IntEvent1ImaskSpec> {
        IntEvent1ImaskHighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_imask_lowifg(&mut self) -> IntEvent1ImaskLowifgW<IntEvent1ImaskSpec> {
        IntEvent1ImaskLowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event1_imask_inifg(&mut self) -> IntEvent1ImaskInifgW<IntEvent1ImaskSpec> {
        IntEvent1ImaskInifgW::new(self, 4)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event1_imask_memresifg0(&mut self) -> IntEvent1ImaskMemresifg0W<IntEvent1ImaskSpec> {
        IntEvent1ImaskMemresifg0W::new(self, 8)
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
