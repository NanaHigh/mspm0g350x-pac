#[doc = "Register `INT_EVENT0_ICLR` writer"]
pub type W = crate::W<IntEvent0IclrSpec>;
#[doc = "Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrOvifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrOvifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrOvifgClr = 1,
}
impl From<IntEvent0IclrOvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrOvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_OVIFG` writer - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IclrOvifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrOvifg>;
impl<'a, REG> IntEvent0IclrOvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_ovifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrOvifg::IntEvent0IclrOvifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_ovifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrOvifg::IntEvent0IclrOvifgClr)
    }
}
#[doc = "Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrTovifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrTovifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrTovifgClr = 1,
}
impl From<IntEvent0IclrTovifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrTovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_TOVIFG` writer - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IclrTovifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrTovifg>;
impl<'a, REG> IntEvent0IclrTovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_tovifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTovifg::IntEvent0IclrTovifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_tovifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTovifg::IntEvent0IclrTovifgClr)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrHighifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrHighifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrHighifgClr = 1,
}
impl From<IntEvent0IclrHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IclrHighifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrHighifg>;
impl<'a, REG> IntEvent0IclrHighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_highifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrHighifg::IntEvent0IclrHighifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_highifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrHighifg::IntEvent0IclrHighifgClr)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrLowifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrLowifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrLowifgClr = 1,
}
impl From<IntEvent0IclrLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IclrLowifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrLowifg>;
impl<'a, REG> IntEvent0IclrLowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_lowifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrLowifg::IntEvent0IclrLowifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_lowifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrLowifg::IntEvent0IclrLowifgClr)
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrInifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrInifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrInifgClr = 1,
}
impl From<IntEvent0IclrInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_INIFG` writer - Mask INIFG in MIS_EX register."]
pub type IntEvent0IclrInifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrInifg>;
impl<'a, REG> IntEvent0IclrInifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_inifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrInifg::IntEvent0IclrInifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_inifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrInifg::IntEvent0IclrInifgClr)
    }
}
#[doc = "Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDmadone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDmadoneNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDmadoneClr = 1,
}
impl From<IntEvent0IclrDmadone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDmadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DMADONE` writer - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IclrDmadoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDmadone>;
impl<'a, REG> IntEvent0IclrDmadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dmadone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDmadone::IntEvent0IclrDmadoneNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dmadone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDmadone::IntEvent0IclrDmadoneClr)
    }
}
#[doc = "Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrUvifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrUvifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrUvifgClr = 1,
}
impl From<IntEvent0IclrUvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrUvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_UVIFG` writer - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IclrUvifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrUvifg>;
impl<'a, REG> IntEvent0IclrUvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_uvifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrUvifg::IntEvent0IclrUvifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_uvifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrUvifg::IntEvent0IclrUvifgClr)
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg0NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg0Clr = 1,
}
impl From<IntEvent0IclrMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg0>;
impl<'a, REG> IntEvent0IclrMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg0::IntEvent0IclrMemresifg0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg0::IntEvent0IclrMemresifg0Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg1NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg1Clr = 1,
}
impl From<IntEvent0IclrMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg1>;
impl<'a, REG> IntEvent0IclrMemresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg1::IntEvent0IclrMemresifg1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg1::IntEvent0IclrMemresifg1Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg2Clr = 1,
}
impl From<IntEvent0IclrMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg2>;
impl<'a, REG> IntEvent0IclrMemresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg2::IntEvent0IclrMemresifg2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg2::IntEvent0IclrMemresifg2Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg3NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg3Clr = 1,
}
impl From<IntEvent0IclrMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg3>;
impl<'a, REG> IntEvent0IclrMemresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg3::IntEvent0IclrMemresifg3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg3::IntEvent0IclrMemresifg3Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg4NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg4Clr = 1,
}
impl From<IntEvent0IclrMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG4` writer - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg4>;
impl<'a, REG> IntEvent0IclrMemresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg4::IntEvent0IclrMemresifg4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg4::IntEvent0IclrMemresifg4Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg5NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg5Clr = 1,
}
impl From<IntEvent0IclrMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG5` writer - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg5>;
impl<'a, REG> IntEvent0IclrMemresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg5::IntEvent0IclrMemresifg5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg5::IntEvent0IclrMemresifg5Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg6NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg6Clr = 1,
}
impl From<IntEvent0IclrMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG6` writer - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg6>;
impl<'a, REG> IntEvent0IclrMemresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg6::IntEvent0IclrMemresifg6NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg6::IntEvent0IclrMemresifg6Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg7NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg7Clr = 1,
}
impl From<IntEvent0IclrMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG7` writer - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg7>;
impl<'a, REG> IntEvent0IclrMemresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg7::IntEvent0IclrMemresifg7NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg7::IntEvent0IclrMemresifg7Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg8NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg8Clr = 1,
}
impl From<IntEvent0IclrMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG8` writer - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg8>;
impl<'a, REG> IntEvent0IclrMemresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg8::IntEvent0IclrMemresifg8NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg8::IntEvent0IclrMemresifg8Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg9NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg9Clr = 1,
}
impl From<IntEvent0IclrMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG9` writer - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg9>;
impl<'a, REG> IntEvent0IclrMemresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg9::IntEvent0IclrMemresifg9NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg9::IntEvent0IclrMemresifg9Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg10NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg10Clr = 1,
}
impl From<IntEvent0IclrMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG10` writer - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg10>;
impl<'a, REG> IntEvent0IclrMemresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg10::IntEvent0IclrMemresifg10NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg10::IntEvent0IclrMemresifg10Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMemresifg11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMemresifg11NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMemresifg11Clr = 1,
}
impl From<IntEvent0IclrMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MEMRESIFG11` writer - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IclrMemresifg11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMemresifg11>;
impl<'a, REG> IntEvent0IclrMemresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg11::IntEvent0IclrMemresifg11NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMemresifg11::IntEvent0IclrMemresifg11Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iclr_ovifg(&mut self) -> IntEvent0IclrOvifgW<IntEvent0IclrSpec> {
        IntEvent0IclrOvifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iclr_tovifg(&mut self) -> IntEvent0IclrTovifgW<IntEvent0IclrSpec> {
        IntEvent0IclrTovifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iclr_highifg(&mut self) -> IntEvent0IclrHighifgW<IntEvent0IclrSpec> {
        IntEvent0IclrHighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iclr_lowifg(&mut self) -> IntEvent0IclrLowifgW<IntEvent0IclrSpec> {
        IntEvent0IclrLowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event0_iclr_inifg(&mut self) -> IntEvent0IclrInifgW<IntEvent0IclrSpec> {
        IntEvent0IclrInifgW::new(self, 4)
    }
    #[doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iclr_dmadone(&mut self) -> IntEvent0IclrDmadoneW<IntEvent0IclrSpec> {
        IntEvent0IclrDmadoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iclr_uvifg(&mut self) -> IntEvent0IclrUvifgW<IntEvent0IclrSpec> {
        IntEvent0IclrUvifgW::new(self, 6)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg0(&mut self) -> IntEvent0IclrMemresifg0W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg1(&mut self) -> IntEvent0IclrMemresifg1W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg2(&mut self) -> IntEvent0IclrMemresifg2W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg3(&mut self) -> IntEvent0IclrMemresifg3W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg4(&mut self) -> IntEvent0IclrMemresifg4W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg5(&mut self) -> IntEvent0IclrMemresifg5W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg6(&mut self) -> IntEvent0IclrMemresifg6W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg7(&mut self) -> IntEvent0IclrMemresifg7W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg8(&mut self) -> IntEvent0IclrMemresifg8W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg9(&mut self) -> IntEvent0IclrMemresifg9W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg10(&mut self) -> IntEvent0IclrMemresifg10W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iclr_memresifg11(&mut self) -> IntEvent0IclrMemresifg11W<IntEvent0IclrSpec> {
        IntEvent0IclrMemresifg11W::new(self, 19)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IclrSpec;
impl crate::RegisterSpec for IntEvent0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent0IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ICLR to value 0"]
impl crate::Resettable for IntEvent0IclrSpec {
    const RESET_VALUE: u32 = 0;
}
