#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmach0 {
    #[doc = "0: CLR"]
    ImaskDmach0Clr = 0,
    #[doc = "1: SET"]
    ImaskDmach0Set = 1,
}
impl From<ImaskDmach0> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmach0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMACH0` reader - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach0R = crate::BitReader<ImaskDmach0>;
impl ImaskDmach0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmach0 {
        match self.bits {
            false => ImaskDmach0::ImaskDmach0Clr,
            true => ImaskDmach0::ImaskDmach0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmach0_clr(&self) -> bool {
        *self == ImaskDmach0::ImaskDmach0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmach0_set(&self) -> bool {
        *self == ImaskDmach0::ImaskDmach0Set
    }
}
#[doc = "Field `IMASK_DMACH0` writer - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach0W<'a, REG> = crate::BitWriter<'a, REG, ImaskDmach0>;
impl<'a, REG> ImaskDmach0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmach0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach0::ImaskDmach0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmach0_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach0::ImaskDmach0Set)
    }
}
#[doc = "DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmach1 {
    #[doc = "0: CLR"]
    ImaskDmach1Clr = 0,
    #[doc = "1: SET"]
    ImaskDmach1Set = 1,
}
impl From<ImaskDmach1> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmach1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMACH1` reader - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach1R = crate::BitReader<ImaskDmach1>;
impl ImaskDmach1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmach1 {
        match self.bits {
            false => ImaskDmach1::ImaskDmach1Clr,
            true => ImaskDmach1::ImaskDmach1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmach1_clr(&self) -> bool {
        *self == ImaskDmach1::ImaskDmach1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmach1_set(&self) -> bool {
        *self == ImaskDmach1::ImaskDmach1Set
    }
}
#[doc = "Field `IMASK_DMACH1` writer - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach1W<'a, REG> = crate::BitWriter<'a, REG, ImaskDmach1>;
impl<'a, REG> ImaskDmach1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmach1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach1::ImaskDmach1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmach1_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach1::ImaskDmach1Set)
    }
}
#[doc = "DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmach2 {
    #[doc = "0: CLR"]
    ImaskDmach2Clr = 0,
    #[doc = "1: SET"]
    ImaskDmach2Set = 1,
}
impl From<ImaskDmach2> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmach2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMACH2` reader - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach2R = crate::BitReader<ImaskDmach2>;
impl ImaskDmach2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmach2 {
        match self.bits {
            false => ImaskDmach2::ImaskDmach2Clr,
            true => ImaskDmach2::ImaskDmach2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmach2_clr(&self) -> bool {
        *self == ImaskDmach2::ImaskDmach2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmach2_set(&self) -> bool {
        *self == ImaskDmach2::ImaskDmach2Set
    }
}
#[doc = "Field `IMASK_DMACH2` writer - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach2W<'a, REG> = crate::BitWriter<'a, REG, ImaskDmach2>;
impl<'a, REG> ImaskDmach2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmach2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach2::ImaskDmach2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmach2_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach2::ImaskDmach2Set)
    }
}
#[doc = "DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmach3 {
    #[doc = "0: CLR"]
    ImaskDmach3Clr = 0,
    #[doc = "1: SET"]
    ImaskDmach3Set = 1,
}
impl From<ImaskDmach3> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmach3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMACH3` reader - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach3R = crate::BitReader<ImaskDmach3>;
impl ImaskDmach3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmach3 {
        match self.bits {
            false => ImaskDmach3::ImaskDmach3Clr,
            true => ImaskDmach3::ImaskDmach3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmach3_clr(&self) -> bool {
        *self == ImaskDmach3::ImaskDmach3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmach3_set(&self) -> bool {
        *self == ImaskDmach3::ImaskDmach3Set
    }
}
#[doc = "Field `IMASK_DMACH3` writer - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach3W<'a, REG> = crate::BitWriter<'a, REG, ImaskDmach3>;
impl<'a, REG> ImaskDmach3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmach3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach3::ImaskDmach3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmach3_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach3::ImaskDmach3Set)
    }
}
#[doc = "DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmach4 {
    #[doc = "0: CLR"]
    ImaskDmach4Clr = 0,
    #[doc = "1: SET"]
    ImaskDmach4Set = 1,
}
impl From<ImaskDmach4> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmach4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMACH4` reader - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach4R = crate::BitReader<ImaskDmach4>;
impl ImaskDmach4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmach4 {
        match self.bits {
            false => ImaskDmach4::ImaskDmach4Clr,
            true => ImaskDmach4::ImaskDmach4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmach4_clr(&self) -> bool {
        *self == ImaskDmach4::ImaskDmach4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmach4_set(&self) -> bool {
        *self == ImaskDmach4::ImaskDmach4Set
    }
}
#[doc = "Field `IMASK_DMACH4` writer - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach4W<'a, REG> = crate::BitWriter<'a, REG, ImaskDmach4>;
impl<'a, REG> ImaskDmach4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmach4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach4::ImaskDmach4Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmach4_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach4::ImaskDmach4Set)
    }
}
#[doc = "DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmach5 {
    #[doc = "0: CLR"]
    ImaskDmach5Clr = 0,
    #[doc = "1: SET"]
    ImaskDmach5Set = 1,
}
impl From<ImaskDmach5> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmach5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMACH5` reader - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach5R = crate::BitReader<ImaskDmach5>;
impl ImaskDmach5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmach5 {
        match self.bits {
            false => ImaskDmach5::ImaskDmach5Clr,
            true => ImaskDmach5::ImaskDmach5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmach5_clr(&self) -> bool {
        *self == ImaskDmach5::ImaskDmach5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmach5_set(&self) -> bool {
        *self == ImaskDmach5::ImaskDmach5Set
    }
}
#[doc = "Field `IMASK_DMACH5` writer - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach5W<'a, REG> = crate::BitWriter<'a, REG, ImaskDmach5>;
impl<'a, REG> ImaskDmach5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmach5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach5::ImaskDmach5Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmach5_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach5::ImaskDmach5Set)
    }
}
#[doc = "DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmach6 {
    #[doc = "0: CLR"]
    ImaskDmach6Clr = 0,
    #[doc = "1: SET"]
    ImaskDmach6Set = 1,
}
impl From<ImaskDmach6> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmach6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMACH6` reader - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach6R = crate::BitReader<ImaskDmach6>;
impl ImaskDmach6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmach6 {
        match self.bits {
            false => ImaskDmach6::ImaskDmach6Clr,
            true => ImaskDmach6::ImaskDmach6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmach6_clr(&self) -> bool {
        *self == ImaskDmach6::ImaskDmach6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmach6_set(&self) -> bool {
        *self == ImaskDmach6::ImaskDmach6Set
    }
}
#[doc = "Field `IMASK_DMACH6` writer - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type ImaskDmach6W<'a, REG> = crate::BitWriter<'a, REG, ImaskDmach6>;
impl<'a, REG> ImaskDmach6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmach6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach6::ImaskDmach6Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmach6_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmach6::ImaskDmach6Set)
    }
}
#[doc = "Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskPreirqch0 {
    #[doc = "0: CLR"]
    ImaskPreirqch0Clr = 0,
    #[doc = "1: SET"]
    ImaskPreirqch0Set = 1,
}
impl From<ImaskPreirqch0> for bool {
    #[inline(always)]
    fn from(variant: ImaskPreirqch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_PREIRQCH0` reader - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type ImaskPreirqch0R = crate::BitReader<ImaskPreirqch0>;
impl ImaskPreirqch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskPreirqch0 {
        match self.bits {
            false => ImaskPreirqch0::ImaskPreirqch0Clr,
            true => ImaskPreirqch0::ImaskPreirqch0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_preirqch0_clr(&self) -> bool {
        *self == ImaskPreirqch0::ImaskPreirqch0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_preirqch0_set(&self) -> bool {
        *self == ImaskPreirqch0::ImaskPreirqch0Set
    }
}
#[doc = "Field `IMASK_PREIRQCH0` writer - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type ImaskPreirqch0W<'a, REG> = crate::BitWriter<'a, REG, ImaskPreirqch0>;
impl<'a, REG> ImaskPreirqch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_preirqch0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPreirqch0::ImaskPreirqch0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_preirqch0_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPreirqch0::ImaskPreirqch0Set)
    }
}
#[doc = "Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskPreirqch1 {
    #[doc = "0: CLR"]
    ImaskPreirqch1Clr = 0,
    #[doc = "1: SET"]
    ImaskPreirqch1Set = 1,
}
impl From<ImaskPreirqch1> for bool {
    #[inline(always)]
    fn from(variant: ImaskPreirqch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_PREIRQCH1` reader - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type ImaskPreirqch1R = crate::BitReader<ImaskPreirqch1>;
impl ImaskPreirqch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskPreirqch1 {
        match self.bits {
            false => ImaskPreirqch1::ImaskPreirqch1Clr,
            true => ImaskPreirqch1::ImaskPreirqch1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_preirqch1_clr(&self) -> bool {
        *self == ImaskPreirqch1::ImaskPreirqch1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_preirqch1_set(&self) -> bool {
        *self == ImaskPreirqch1::ImaskPreirqch1Set
    }
}
#[doc = "Field `IMASK_PREIRQCH1` writer - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type ImaskPreirqch1W<'a, REG> = crate::BitWriter<'a, REG, ImaskPreirqch1>;
impl<'a, REG> ImaskPreirqch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_preirqch1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPreirqch1::ImaskPreirqch1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_preirqch1_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPreirqch1::ImaskPreirqch1Set)
    }
}
#[doc = "Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskPreirqch2 {
    #[doc = "0: CLR"]
    ImaskPreirqch2Clr = 0,
    #[doc = "1: SET"]
    ImaskPreirqch2Set = 1,
}
impl From<ImaskPreirqch2> for bool {
    #[inline(always)]
    fn from(variant: ImaskPreirqch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_PREIRQCH2` reader - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type ImaskPreirqch2R = crate::BitReader<ImaskPreirqch2>;
impl ImaskPreirqch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskPreirqch2 {
        match self.bits {
            false => ImaskPreirqch2::ImaskPreirqch2Clr,
            true => ImaskPreirqch2::ImaskPreirqch2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_preirqch2_clr(&self) -> bool {
        *self == ImaskPreirqch2::ImaskPreirqch2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_preirqch2_set(&self) -> bool {
        *self == ImaskPreirqch2::ImaskPreirqch2Set
    }
}
#[doc = "Field `IMASK_PREIRQCH2` writer - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type ImaskPreirqch2W<'a, REG> = crate::BitWriter<'a, REG, ImaskPreirqch2>;
impl<'a, REG> ImaskPreirqch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_preirqch2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPreirqch2::ImaskPreirqch2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_preirqch2_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPreirqch2::ImaskPreirqch2Set)
    }
}
#[doc = "DMA address error, SRC address not reachable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskAddrerr {
    #[doc = "0: CLR"]
    ImaskAddrerrClr = 0,
    #[doc = "1: SET"]
    ImaskAddrerrSet = 1,
}
impl From<ImaskAddrerr> for bool {
    #[inline(always)]
    fn from(variant: ImaskAddrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_ADDRERR` reader - DMA address error, SRC address not reachable."]
pub type ImaskAddrerrR = crate::BitReader<ImaskAddrerr>;
impl ImaskAddrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskAddrerr {
        match self.bits {
            false => ImaskAddrerr::ImaskAddrerrClr,
            true => ImaskAddrerr::ImaskAddrerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_addrerr_clr(&self) -> bool {
        *self == ImaskAddrerr::ImaskAddrerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_addrerr_set(&self) -> bool {
        *self == ImaskAddrerr::ImaskAddrerrSet
    }
}
#[doc = "Field `IMASK_ADDRERR` writer - DMA address error, SRC address not reachable."]
pub type ImaskAddrerrW<'a, REG> = crate::BitWriter<'a, REG, ImaskAddrerr>;
impl<'a, REG> ImaskAddrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_addrerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskAddrerr::ImaskAddrerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_addrerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskAddrerr::ImaskAddrerrSet)
    }
}
#[doc = "DMA data error, SRC data might be corrupted (PAR or ECC error).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDataerr {
    #[doc = "0: CLR"]
    ImaskDataerrClr = 0,
    #[doc = "1: SET"]
    ImaskDataerrSet = 1,
}
impl From<ImaskDataerr> for bool {
    #[inline(always)]
    fn from(variant: ImaskDataerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DATAERR` reader - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type ImaskDataerrR = crate::BitReader<ImaskDataerr>;
impl ImaskDataerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDataerr {
        match self.bits {
            false => ImaskDataerr::ImaskDataerrClr,
            true => ImaskDataerr::ImaskDataerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dataerr_clr(&self) -> bool {
        *self == ImaskDataerr::ImaskDataerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dataerr_set(&self) -> bool {
        *self == ImaskDataerr::ImaskDataerrSet
    }
}
#[doc = "Field `IMASK_DATAERR` writer - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type ImaskDataerrW<'a, REG> = crate::BitWriter<'a, REG, ImaskDataerr>;
impl<'a, REG> ImaskDataerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dataerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDataerr::ImaskDataerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dataerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDataerr::ImaskDataerrSet)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach0(&self) -> ImaskDmach0R {
        ImaskDmach0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach1(&self) -> ImaskDmach1R {
        ImaskDmach1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach2(&self) -> ImaskDmach2R {
        ImaskDmach2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach3(&self) -> ImaskDmach3R {
        ImaskDmach3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach4(&self) -> ImaskDmach4R {
        ImaskDmach4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach5(&self) -> ImaskDmach5R {
        ImaskDmach5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach6(&self) -> ImaskDmach6R {
        ImaskDmach6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn imask_preirqch0(&self) -> ImaskPreirqch0R {
        ImaskPreirqch0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn imask_preirqch1(&self) -> ImaskPreirqch1R {
        ImaskPreirqch1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn imask_preirqch2(&self) -> ImaskPreirqch2R {
        ImaskPreirqch2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn imask_addrerr(&self) -> ImaskAddrerrR {
        ImaskAddrerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn imask_dataerr(&self) -> ImaskDataerrR {
        ImaskDataerrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach0(&mut self) -> ImaskDmach0W<ImaskSpec> {
        ImaskDmach0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach1(&mut self) -> ImaskDmach1W<ImaskSpec> {
        ImaskDmach1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach2(&mut self) -> ImaskDmach2W<ImaskSpec> {
        ImaskDmach2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach3(&mut self) -> ImaskDmach3W<ImaskSpec> {
        ImaskDmach3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach4(&mut self) -> ImaskDmach4W<ImaskSpec> {
        ImaskDmach4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach5(&mut self) -> ImaskDmach5W<ImaskSpec> {
        ImaskDmach5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn imask_dmach6(&mut self) -> ImaskDmach6W<ImaskSpec> {
        ImaskDmach6W::new(self, 6)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn imask_preirqch0(&mut self) -> ImaskPreirqch0W<ImaskSpec> {
        ImaskPreirqch0W::new(self, 16)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn imask_preirqch1(&mut self) -> ImaskPreirqch1W<ImaskSpec> {
        ImaskPreirqch1W::new(self, 17)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn imask_preirqch2(&mut self) -> ImaskPreirqch2W<ImaskSpec> {
        ImaskPreirqch2W::new(self, 18)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn imask_addrerr(&mut self) -> ImaskAddrerrW<ImaskSpec> {
        ImaskAddrerrW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn imask_dataerr(&mut self) -> ImaskDataerrW<ImaskSpec> {
        ImaskDataerrW::new(self, 25)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
