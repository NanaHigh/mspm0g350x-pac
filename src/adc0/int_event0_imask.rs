#[doc = "Register `INT_EVENT0_IMASK` reader"]
pub type R = crate::R<IntEvent0ImaskSpec>;
#[doc = "Register `INT_EVENT0_IMASK` writer"]
pub type W = crate::W<IntEvent0ImaskSpec>;
#[doc = "Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskOvifg {
    #[doc = "0: CLR"]
    IntEvent0ImaskOvifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskOvifgSet = 1,
}
impl From<IntEvent0ImaskOvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskOvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_OVIFG` reader - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskOvifgR = crate::BitReader<IntEvent0ImaskOvifg>;
impl IntEvent0ImaskOvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskOvifg {
        match self.bits {
            false => IntEvent0ImaskOvifg::IntEvent0ImaskOvifgClr,
            true => IntEvent0ImaskOvifg::IntEvent0ImaskOvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_ovifg_clr(&self) -> bool {
        *self == IntEvent0ImaskOvifg::IntEvent0ImaskOvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_ovifg_set(&self) -> bool {
        *self == IntEvent0ImaskOvifg::IntEvent0ImaskOvifgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_OVIFG` writer - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskOvifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskOvifg>;
impl<'a, REG> IntEvent0ImaskOvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_ovifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskOvifg::IntEvent0ImaskOvifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_ovifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskOvifg::IntEvent0ImaskOvifgSet)
    }
}
#[doc = "Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskTovifg {
    #[doc = "0: CLR"]
    IntEvent0ImaskTovifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskTovifgSet = 1,
}
impl From<IntEvent0ImaskTovifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskTovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TOVIFG` reader - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskTovifgR = crate::BitReader<IntEvent0ImaskTovifg>;
impl IntEvent0ImaskTovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskTovifg {
        match self.bits {
            false => IntEvent0ImaskTovifg::IntEvent0ImaskTovifgClr,
            true => IntEvent0ImaskTovifg::IntEvent0ImaskTovifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_tovifg_clr(&self) -> bool {
        *self == IntEvent0ImaskTovifg::IntEvent0ImaskTovifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_tovifg_set(&self) -> bool {
        *self == IntEvent0ImaskTovifg::IntEvent0ImaskTovifgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TOVIFG` writer - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskTovifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskTovifg>;
impl<'a, REG> IntEvent0ImaskTovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_tovifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTovifg::IntEvent0ImaskTovifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_tovifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTovifg::IntEvent0ImaskTovifgSet)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskHighifg {
    #[doc = "0: CLR"]
    IntEvent0ImaskHighifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskHighifgSet = 1,
}
impl From<IntEvent0ImaskHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskHighifgR = crate::BitReader<IntEvent0ImaskHighifg>;
impl IntEvent0ImaskHighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskHighifg {
        match self.bits {
            false => IntEvent0ImaskHighifg::IntEvent0ImaskHighifgClr,
            true => IntEvent0ImaskHighifg::IntEvent0ImaskHighifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_highifg_clr(&self) -> bool {
        *self == IntEvent0ImaskHighifg::IntEvent0ImaskHighifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_highifg_set(&self) -> bool {
        *self == IntEvent0ImaskHighifg::IntEvent0ImaskHighifgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskHighifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskHighifg>;
impl<'a, REG> IntEvent0ImaskHighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_highifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskHighifg::IntEvent0ImaskHighifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_highifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskHighifg::IntEvent0ImaskHighifgSet)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskLowifg {
    #[doc = "0: CLR"]
    IntEvent0ImaskLowifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskLowifgSet = 1,
}
impl From<IntEvent0ImaskLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskLowifgR = crate::BitReader<IntEvent0ImaskLowifg>;
impl IntEvent0ImaskLowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskLowifg {
        match self.bits {
            false => IntEvent0ImaskLowifg::IntEvent0ImaskLowifgClr,
            true => IntEvent0ImaskLowifg::IntEvent0ImaskLowifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_lowifg_clr(&self) -> bool {
        *self == IntEvent0ImaskLowifg::IntEvent0ImaskLowifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_lowifg_set(&self) -> bool {
        *self == IntEvent0ImaskLowifg::IntEvent0ImaskLowifgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskLowifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskLowifg>;
impl<'a, REG> IntEvent0ImaskLowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_lowifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskLowifg::IntEvent0ImaskLowifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_lowifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskLowifg::IntEvent0ImaskLowifgSet)
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskInifg {
    #[doc = "0: CLR"]
    IntEvent0ImaskInifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskInifgSet = 1,
}
impl From<IntEvent0ImaskInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_INIFG` reader - Mask INIFG in MIS_EX register."]
pub type IntEvent0ImaskInifgR = crate::BitReader<IntEvent0ImaskInifg>;
impl IntEvent0ImaskInifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskInifg {
        match self.bits {
            false => IntEvent0ImaskInifg::IntEvent0ImaskInifgClr,
            true => IntEvent0ImaskInifg::IntEvent0ImaskInifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_inifg_clr(&self) -> bool {
        *self == IntEvent0ImaskInifg::IntEvent0ImaskInifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_inifg_set(&self) -> bool {
        *self == IntEvent0ImaskInifg::IntEvent0ImaskInifgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_INIFG` writer - Mask INIFG in MIS_EX register."]
pub type IntEvent0ImaskInifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskInifg>;
impl<'a, REG> IntEvent0ImaskInifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_inifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskInifg::IntEvent0ImaskInifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_inifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskInifg::IntEvent0ImaskInifgSet)
    }
}
#[doc = "Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDmadone {
    #[doc = "0: CLR"]
    IntEvent0ImaskDmadoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDmadoneSet = 1,
}
impl From<IntEvent0ImaskDmadone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDmadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMADONE` reader - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskDmadoneR = crate::BitReader<IntEvent0ImaskDmadone>;
impl IntEvent0ImaskDmadoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDmadone {
        match self.bits {
            false => IntEvent0ImaskDmadone::IntEvent0ImaskDmadoneClr,
            true => IntEvent0ImaskDmadone::IntEvent0ImaskDmadoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dmadone_clr(&self) -> bool {
        *self == IntEvent0ImaskDmadone::IntEvent0ImaskDmadoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dmadone_set(&self) -> bool {
        *self == IntEvent0ImaskDmadone::IntEvent0ImaskDmadoneSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMADONE` writer - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0ImaskDmadoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDmadone>;
impl<'a, REG> IntEvent0ImaskDmadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dmadone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmadone::IntEvent0ImaskDmadoneClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dmadone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmadone::IntEvent0ImaskDmadoneSet)
    }
}
#[doc = "Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskUvifg {
    #[doc = "0: CLR"]
    IntEvent0ImaskUvifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskUvifgSet = 1,
}
impl From<IntEvent0ImaskUvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskUvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_UVIFG` reader - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
pub type IntEvent0ImaskUvifgR = crate::BitReader<IntEvent0ImaskUvifg>;
impl IntEvent0ImaskUvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskUvifg {
        match self.bits {
            false => IntEvent0ImaskUvifg::IntEvent0ImaskUvifgClr,
            true => IntEvent0ImaskUvifg::IntEvent0ImaskUvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_uvifg_clr(&self) -> bool {
        *self == IntEvent0ImaskUvifg::IntEvent0ImaskUvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_uvifg_set(&self) -> bool {
        *self == IntEvent0ImaskUvifg::IntEvent0ImaskUvifgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_UVIFG` writer - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
pub type IntEvent0ImaskUvifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskUvifg>;
impl<'a, REG> IntEvent0ImaskUvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_uvifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskUvifg::IntEvent0ImaskUvifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_uvifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskUvifg::IntEvent0ImaskUvifgSet)
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg0Set = 1,
}
impl From<IntEvent0ImaskMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg0R = crate::BitReader<IntEvent0ImaskMemresifg0>;
impl IntEvent0ImaskMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg0 {
        match self.bits {
            false => IntEvent0ImaskMemresifg0::IntEvent0ImaskMemresifg0Clr,
            true => IntEvent0ImaskMemresifg0::IntEvent0ImaskMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg0_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg0::IntEvent0ImaskMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg0_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg0::IntEvent0ImaskMemresifg0Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg0>;
impl<'a, REG> IntEvent0ImaskMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg0::IntEvent0ImaskMemresifg0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg0::IntEvent0ImaskMemresifg0Set)
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg1 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg1Set = 1,
}
impl From<IntEvent0ImaskMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg1R = crate::BitReader<IntEvent0ImaskMemresifg1>;
impl IntEvent0ImaskMemresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg1 {
        match self.bits {
            false => IntEvent0ImaskMemresifg1::IntEvent0ImaskMemresifg1Clr,
            true => IntEvent0ImaskMemresifg1::IntEvent0ImaskMemresifg1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg1_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg1::IntEvent0ImaskMemresifg1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg1_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg1::IntEvent0ImaskMemresifg1Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg1>;
impl<'a, REG> IntEvent0ImaskMemresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg1::IntEvent0ImaskMemresifg1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg1::IntEvent0ImaskMemresifg1Set)
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg2 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg2Set = 1,
}
impl From<IntEvent0ImaskMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg2R = crate::BitReader<IntEvent0ImaskMemresifg2>;
impl IntEvent0ImaskMemresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg2 {
        match self.bits {
            false => IntEvent0ImaskMemresifg2::IntEvent0ImaskMemresifg2Clr,
            true => IntEvent0ImaskMemresifg2::IntEvent0ImaskMemresifg2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg2_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg2::IntEvent0ImaskMemresifg2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg2_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg2::IntEvent0ImaskMemresifg2Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg2>;
impl<'a, REG> IntEvent0ImaskMemresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg2::IntEvent0ImaskMemresifg2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg2::IntEvent0ImaskMemresifg2Set)
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg3 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg3Set = 1,
}
impl From<IntEvent0ImaskMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg3R = crate::BitReader<IntEvent0ImaskMemresifg3>;
impl IntEvent0ImaskMemresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg3 {
        match self.bits {
            false => IntEvent0ImaskMemresifg3::IntEvent0ImaskMemresifg3Clr,
            true => IntEvent0ImaskMemresifg3::IntEvent0ImaskMemresifg3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg3_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg3::IntEvent0ImaskMemresifg3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg3_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg3::IntEvent0ImaskMemresifg3Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg3>;
impl<'a, REG> IntEvent0ImaskMemresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg3::IntEvent0ImaskMemresifg3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg3::IntEvent0ImaskMemresifg3Set)
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg4 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg4Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg4Set = 1,
}
impl From<IntEvent0ImaskMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG4` reader - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg4R = crate::BitReader<IntEvent0ImaskMemresifg4>;
impl IntEvent0ImaskMemresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg4 {
        match self.bits {
            false => IntEvent0ImaskMemresifg4::IntEvent0ImaskMemresifg4Clr,
            true => IntEvent0ImaskMemresifg4::IntEvent0ImaskMemresifg4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg4_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg4::IntEvent0ImaskMemresifg4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg4_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg4::IntEvent0ImaskMemresifg4Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG4` writer - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg4>;
impl<'a, REG> IntEvent0ImaskMemresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg4::IntEvent0ImaskMemresifg4Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg4::IntEvent0ImaskMemresifg4Set)
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg5 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg5Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg5Set = 1,
}
impl From<IntEvent0ImaskMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG5` reader - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg5R = crate::BitReader<IntEvent0ImaskMemresifg5>;
impl IntEvent0ImaskMemresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg5 {
        match self.bits {
            false => IntEvent0ImaskMemresifg5::IntEvent0ImaskMemresifg5Clr,
            true => IntEvent0ImaskMemresifg5::IntEvent0ImaskMemresifg5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg5_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg5::IntEvent0ImaskMemresifg5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg5_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg5::IntEvent0ImaskMemresifg5Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG5` writer - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg5>;
impl<'a, REG> IntEvent0ImaskMemresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg5::IntEvent0ImaskMemresifg5Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg5::IntEvent0ImaskMemresifg5Set)
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg6 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg6Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg6Set = 1,
}
impl From<IntEvent0ImaskMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG6` reader - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg6R = crate::BitReader<IntEvent0ImaskMemresifg6>;
impl IntEvent0ImaskMemresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg6 {
        match self.bits {
            false => IntEvent0ImaskMemresifg6::IntEvent0ImaskMemresifg6Clr,
            true => IntEvent0ImaskMemresifg6::IntEvent0ImaskMemresifg6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg6_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg6::IntEvent0ImaskMemresifg6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg6_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg6::IntEvent0ImaskMemresifg6Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG6` writer - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg6>;
impl<'a, REG> IntEvent0ImaskMemresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg6::IntEvent0ImaskMemresifg6Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg6::IntEvent0ImaskMemresifg6Set)
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg7 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg7Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg7Set = 1,
}
impl From<IntEvent0ImaskMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG7` reader - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg7R = crate::BitReader<IntEvent0ImaskMemresifg7>;
impl IntEvent0ImaskMemresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg7 {
        match self.bits {
            false => IntEvent0ImaskMemresifg7::IntEvent0ImaskMemresifg7Clr,
            true => IntEvent0ImaskMemresifg7::IntEvent0ImaskMemresifg7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg7_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg7::IntEvent0ImaskMemresifg7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg7_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg7::IntEvent0ImaskMemresifg7Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG7` writer - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg7>;
impl<'a, REG> IntEvent0ImaskMemresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg7::IntEvent0ImaskMemresifg7Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg7::IntEvent0ImaskMemresifg7Set)
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg8 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg8Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg8Set = 1,
}
impl From<IntEvent0ImaskMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG8` reader - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg8R = crate::BitReader<IntEvent0ImaskMemresifg8>;
impl IntEvent0ImaskMemresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg8 {
        match self.bits {
            false => IntEvent0ImaskMemresifg8::IntEvent0ImaskMemresifg8Clr,
            true => IntEvent0ImaskMemresifg8::IntEvent0ImaskMemresifg8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg8_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg8::IntEvent0ImaskMemresifg8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg8_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg8::IntEvent0ImaskMemresifg8Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG8` writer - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg8>;
impl<'a, REG> IntEvent0ImaskMemresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg8::IntEvent0ImaskMemresifg8Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg8::IntEvent0ImaskMemresifg8Set)
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg9 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg9Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg9Set = 1,
}
impl From<IntEvent0ImaskMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG9` reader - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg9R = crate::BitReader<IntEvent0ImaskMemresifg9>;
impl IntEvent0ImaskMemresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg9 {
        match self.bits {
            false => IntEvent0ImaskMemresifg9::IntEvent0ImaskMemresifg9Clr,
            true => IntEvent0ImaskMemresifg9::IntEvent0ImaskMemresifg9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg9_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg9::IntEvent0ImaskMemresifg9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg9_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg9::IntEvent0ImaskMemresifg9Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG9` writer - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg9>;
impl<'a, REG> IntEvent0ImaskMemresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg9::IntEvent0ImaskMemresifg9Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg9::IntEvent0ImaskMemresifg9Set)
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg10 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg10Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg10Set = 1,
}
impl From<IntEvent0ImaskMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG10` reader - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg10R = crate::BitReader<IntEvent0ImaskMemresifg10>;
impl IntEvent0ImaskMemresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg10 {
        match self.bits {
            false => IntEvent0ImaskMemresifg10::IntEvent0ImaskMemresifg10Clr,
            true => IntEvent0ImaskMemresifg10::IntEvent0ImaskMemresifg10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg10_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg10::IntEvent0ImaskMemresifg10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg10_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg10::IntEvent0ImaskMemresifg10Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG10` writer - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg10>;
impl<'a, REG> IntEvent0ImaskMemresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg10::IntEvent0ImaskMemresifg10Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg10::IntEvent0ImaskMemresifg10Set)
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMemresifg11 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMemresifg11Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMemresifg11Set = 1,
}
impl From<IntEvent0ImaskMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG11` reader - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg11R = crate::BitReader<IntEvent0ImaskMemresifg11>;
impl IntEvent0ImaskMemresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMemresifg11 {
        match self.bits {
            false => IntEvent0ImaskMemresifg11::IntEvent0ImaskMemresifg11Clr,
            true => IntEvent0ImaskMemresifg11::IntEvent0ImaskMemresifg11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg11_clr(&self) -> bool {
        *self == IntEvent0ImaskMemresifg11::IntEvent0ImaskMemresifg11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_memresifg11_set(&self) -> bool {
        *self == IntEvent0ImaskMemresifg11::IntEvent0ImaskMemresifg11Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MEMRESIFG11` writer - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0ImaskMemresifg11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMemresifg11>;
impl<'a, REG> IntEvent0ImaskMemresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg11::IntEvent0ImaskMemresifg11Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMemresifg11::IntEvent0ImaskMemresifg11Set)
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_ovifg(&self) -> IntEvent0ImaskOvifgR {
        IntEvent0ImaskOvifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_tovifg(&self) -> IntEvent0ImaskTovifgR {
        IntEvent0ImaskTovifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_highifg(&self) -> IntEvent0ImaskHighifgR {
        IntEvent0ImaskHighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_lowifg(&self) -> IntEvent0ImaskLowifgR {
        IntEvent0ImaskLowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event0_imask_inifg(&self) -> IntEvent0ImaskInifgR {
        IntEvent0ImaskInifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_dmadone(&self) -> IntEvent0ImaskDmadoneR {
        IntEvent0ImaskDmadoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_uvifg(&self) -> IntEvent0ImaskUvifgR {
        IntEvent0ImaskUvifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg0(&self) -> IntEvent0ImaskMemresifg0R {
        IntEvent0ImaskMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg1(&self) -> IntEvent0ImaskMemresifg1R {
        IntEvent0ImaskMemresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg2(&self) -> IntEvent0ImaskMemresifg2R {
        IntEvent0ImaskMemresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg3(&self) -> IntEvent0ImaskMemresifg3R {
        IntEvent0ImaskMemresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg4(&self) -> IntEvent0ImaskMemresifg4R {
        IntEvent0ImaskMemresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg5(&self) -> IntEvent0ImaskMemresifg5R {
        IntEvent0ImaskMemresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg6(&self) -> IntEvent0ImaskMemresifg6R {
        IntEvent0ImaskMemresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg7(&self) -> IntEvent0ImaskMemresifg7R {
        IntEvent0ImaskMemresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg8(&self) -> IntEvent0ImaskMemresifg8R {
        IntEvent0ImaskMemresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg9(&self) -> IntEvent0ImaskMemresifg9R {
        IntEvent0ImaskMemresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg10(&self) -> IntEvent0ImaskMemresifg10R {
        IntEvent0ImaskMemresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg11(&self) -> IntEvent0ImaskMemresifg11R {
        IntEvent0ImaskMemresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_ovifg(&mut self) -> IntEvent0ImaskOvifgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskOvifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_tovifg(&mut self) -> IntEvent0ImaskTovifgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskTovifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_highifg(&mut self) -> IntEvent0ImaskHighifgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskHighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_lowifg(&mut self) -> IntEvent0ImaskLowifgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskLowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event0_imask_inifg(&mut self) -> IntEvent0ImaskInifgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskInifgW::new(self, 4)
    }
    #[doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_dmadone(&mut self) -> IntEvent0ImaskDmadoneW<IntEvent0ImaskSpec> {
        IntEvent0ImaskDmadoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
    #[inline(always)]
    pub fn int_event0_imask_uvifg(&mut self) -> IntEvent0ImaskUvifgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskUvifgW::new(self, 6)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg0(&mut self) -> IntEvent0ImaskMemresifg0W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg1(&mut self) -> IntEvent0ImaskMemresifg1W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg2(&mut self) -> IntEvent0ImaskMemresifg2W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg3(&mut self) -> IntEvent0ImaskMemresifg3W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg4(&mut self) -> IntEvent0ImaskMemresifg4W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg5(&mut self) -> IntEvent0ImaskMemresifg5W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg6(&mut self) -> IntEvent0ImaskMemresifg6W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg7(&mut self) -> IntEvent0ImaskMemresifg7W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg8(&mut self) -> IntEvent0ImaskMemresifg8W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg9(&mut self) -> IntEvent0ImaskMemresifg9W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg10(
        &mut self,
    ) -> IntEvent0ImaskMemresifg10W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_imask_memresifg11(
        &mut self,
    ) -> IntEvent0ImaskMemresifg11W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMemresifg11W::new(self, 19)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0ImaskSpec;
impl crate::RegisterSpec for IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for IntEvent0ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
