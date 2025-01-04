#[doc = "Register `INT_EVENT0_ISET` writer"]
pub type W = crate::W<IntEvent0IsetSpec>;
#[doc = "Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetOvifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetOvifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetOvifgSet = 1,
}
impl From<IntEvent0IsetOvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetOvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_OVIFG` writer - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IsetOvifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetOvifg>;
impl<'a, REG> IntEvent0IsetOvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_ovifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetOvifg::IntEvent0IsetOvifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_ovifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetOvifg::IntEvent0IsetOvifgSet)
    }
}
#[doc = "Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetTovifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetTovifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetTovifgSet = 1,
}
impl From<IntEvent0IsetTovifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetTovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_TOVIFG` writer - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IsetTovifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetTovifg>;
impl<'a, REG> IntEvent0IsetTovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_tovifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTovifg::IntEvent0IsetTovifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_tovifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTovifg::IntEvent0IsetTovifgSet)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetHighifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetHighifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetHighifgSet = 1,
}
impl From<IntEvent0IsetHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IsetHighifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetHighifg>;
impl<'a, REG> IntEvent0IsetHighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_highifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetHighifg::IntEvent0IsetHighifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_highifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetHighifg::IntEvent0IsetHighifgSet)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetLowifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetLowifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetLowifgSet = 1,
}
impl From<IntEvent0IsetLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IsetLowifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetLowifg>;
impl<'a, REG> IntEvent0IsetLowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_lowifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetLowifg::IntEvent0IsetLowifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_lowifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetLowifg::IntEvent0IsetLowifgSet)
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetInifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetInifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetInifgSet = 1,
}
impl From<IntEvent0IsetInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_INIFG` writer - Mask INIFG in MIS_EX register."]
pub type IntEvent0IsetInifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetInifg>;
impl<'a, REG> IntEvent0IsetInifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_inifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetInifg::IntEvent0IsetInifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_inifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetInifg::IntEvent0IsetInifgSet)
    }
}
#[doc = "Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDmadone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDmadoneNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDmadoneSet = 1,
}
impl From<IntEvent0IsetDmadone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDmadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DMADONE` writer - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IsetDmadoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDmadone>;
impl<'a, REG> IntEvent0IsetDmadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dmadone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDmadone::IntEvent0IsetDmadoneNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dmadone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDmadone::IntEvent0IsetDmadoneSet)
    }
}
#[doc = "Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetUvifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetUvifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetUvifgSet = 1,
}
impl From<IntEvent0IsetUvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetUvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_UVIFG` writer - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0IsetUvifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetUvifg>;
impl<'a, REG> IntEvent0IsetUvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_uvifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetUvifg::IntEvent0IsetUvifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_uvifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetUvifg::IntEvent0IsetUvifgSet)
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg0NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg0Set = 1,
}
impl From<IntEvent0IsetMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg0>;
impl<'a, REG> IntEvent0IsetMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg0::IntEvent0IsetMemresifg0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg0::IntEvent0IsetMemresifg0Set)
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg1NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg1Set = 1,
}
impl From<IntEvent0IsetMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg1>;
impl<'a, REG> IntEvent0IsetMemresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg1::IntEvent0IsetMemresifg1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg1::IntEvent0IsetMemresifg1Set)
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg2Set = 1,
}
impl From<IntEvent0IsetMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg2>;
impl<'a, REG> IntEvent0IsetMemresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg2::IntEvent0IsetMemresifg2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg2::IntEvent0IsetMemresifg2Set)
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg3NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg3Set = 1,
}
impl From<IntEvent0IsetMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg3>;
impl<'a, REG> IntEvent0IsetMemresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg3::IntEvent0IsetMemresifg3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg3::IntEvent0IsetMemresifg3Set)
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg4NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg4Set = 1,
}
impl From<IntEvent0IsetMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG4` writer - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg4>;
impl<'a, REG> IntEvent0IsetMemresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg4::IntEvent0IsetMemresifg4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg4::IntEvent0IsetMemresifg4Set)
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg5NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg5Set = 1,
}
impl From<IntEvent0IsetMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG5` writer - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg5>;
impl<'a, REG> IntEvent0IsetMemresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg5::IntEvent0IsetMemresifg5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg5::IntEvent0IsetMemresifg5Set)
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg6NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg6Set = 1,
}
impl From<IntEvent0IsetMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG6` writer - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg6>;
impl<'a, REG> IntEvent0IsetMemresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg6::IntEvent0IsetMemresifg6NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg6::IntEvent0IsetMemresifg6Set)
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg7NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg7Set = 1,
}
impl From<IntEvent0IsetMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG7` writer - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg7>;
impl<'a, REG> IntEvent0IsetMemresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg7::IntEvent0IsetMemresifg7NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg7::IntEvent0IsetMemresifg7Set)
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg8NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg8Set = 1,
}
impl From<IntEvent0IsetMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG8` writer - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg8>;
impl<'a, REG> IntEvent0IsetMemresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg8::IntEvent0IsetMemresifg8NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg8::IntEvent0IsetMemresifg8Set)
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg9NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg9Set = 1,
}
impl From<IntEvent0IsetMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG9` writer - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg9>;
impl<'a, REG> IntEvent0IsetMemresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg9::IntEvent0IsetMemresifg9NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg9::IntEvent0IsetMemresifg9Set)
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg10NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg10Set = 1,
}
impl From<IntEvent0IsetMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG10` writer - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg10>;
impl<'a, REG> IntEvent0IsetMemresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg10::IntEvent0IsetMemresifg10NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg10::IntEvent0IsetMemresifg10Set)
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMemresifg11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMemresifg11NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMemresifg11Set = 1,
}
impl From<IntEvent0IsetMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MEMRESIFG11` writer - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0IsetMemresifg11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMemresifg11>;
impl<'a, REG> IntEvent0IsetMemresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg11::IntEvent0IsetMemresifg11NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMemresifg11::IntEvent0IsetMemresifg11Set)
    }
}
impl W {
    #[doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iset_ovifg(&mut self) -> IntEvent0IsetOvifgW<IntEvent0IsetSpec> {
        IntEvent0IsetOvifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iset_tovifg(&mut self) -> IntEvent0IsetTovifgW<IntEvent0IsetSpec> {
        IntEvent0IsetTovifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iset_highifg(&mut self) -> IntEvent0IsetHighifgW<IntEvent0IsetSpec> {
        IntEvent0IsetHighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iset_lowifg(&mut self) -> IntEvent0IsetLowifgW<IntEvent0IsetSpec> {
        IntEvent0IsetLowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event0_iset_inifg(&mut self) -> IntEvent0IsetInifgW<IntEvent0IsetSpec> {
        IntEvent0IsetInifgW::new(self, 4)
    }
    #[doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iset_dmadone(&mut self) -> IntEvent0IsetDmadoneW<IntEvent0IsetSpec> {
        IntEvent0IsetDmadoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_iset_uvifg(&mut self) -> IntEvent0IsetUvifgW<IntEvent0IsetSpec> {
        IntEvent0IsetUvifgW::new(self, 6)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg0(&mut self) -> IntEvent0IsetMemresifg0W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg1(&mut self) -> IntEvent0IsetMemresifg1W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg2(&mut self) -> IntEvent0IsetMemresifg2W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg3(&mut self) -> IntEvent0IsetMemresifg3W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg4(&mut self) -> IntEvent0IsetMemresifg4W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg5(&mut self) -> IntEvent0IsetMemresifg5W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg6(&mut self) -> IntEvent0IsetMemresifg6W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg7(&mut self) -> IntEvent0IsetMemresifg7W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg8(&mut self) -> IntEvent0IsetMemresifg8W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg9(&mut self) -> IntEvent0IsetMemresifg9W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg10(&mut self) -> IntEvent0IsetMemresifg10W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_iset_memresifg11(&mut self) -> IntEvent0IsetMemresifg11W<IntEvent0IsetSpec> {
        IntEvent0IsetMemresifg11W::new(self, 19)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IsetSpec;
impl crate::RegisterSpec for IntEvent0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent0IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ISET to value 0"]
impl crate::Resettable for IntEvent0IsetSpec {
    const RESET_VALUE: u32 = 0;
}
