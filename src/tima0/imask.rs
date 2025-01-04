#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Zero Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskZ {
    #[doc = "0: CLR"]
    ImaskZClr = 0,
    #[doc = "1: SET"]
    ImaskZSet = 1,
}
impl From<ImaskZ> for bool {
    #[inline(always)]
    fn from(variant: ImaskZ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_Z` reader - Zero Event mask"]
pub type ImaskZR = crate::BitReader<ImaskZ>;
impl ImaskZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskZ {
        match self.bits {
            false => ImaskZ::ImaskZClr,
            true => ImaskZ::ImaskZSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_z_clr(&self) -> bool {
        *self == ImaskZ::ImaskZClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_z_set(&self) -> bool {
        *self == ImaskZ::ImaskZSet
    }
}
#[doc = "Field `IMASK_Z` writer - Zero Event mask"]
pub type ImaskZW<'a, REG> = crate::BitWriter<'a, REG, ImaskZ>;
impl<'a, REG> ImaskZW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_z_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskZ::ImaskZClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_z_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskZ::ImaskZSet)
    }
}
#[doc = "Load Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskL {
    #[doc = "0: CLR"]
    ImaskLClr = 0,
    #[doc = "1: SET"]
    ImaskLSet = 1,
}
impl From<ImaskL> for bool {
    #[inline(always)]
    fn from(variant: ImaskL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_L` reader - Load Event mask"]
pub type ImaskLR = crate::BitReader<ImaskL>;
impl ImaskLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskL {
        match self.bits {
            false => ImaskL::ImaskLClr,
            true => ImaskL::ImaskLSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_l_clr(&self) -> bool {
        *self == ImaskL::ImaskLClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_l_set(&self) -> bool {
        *self == ImaskL::ImaskLSet
    }
}
#[doc = "Field `IMASK_L` writer - Load Event mask"]
pub type ImaskLW<'a, REG> = crate::BitWriter<'a, REG, ImaskL>;
impl<'a, REG> ImaskLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_l_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskL::ImaskLClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_l_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskL::ImaskLSet)
    }
}
#[doc = "Capture or Compare DN event mask CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcd0 {
    #[doc = "0: CLR"]
    ImaskCcd0Clr = 0,
    #[doc = "1: SET"]
    ImaskCcd0Set = 1,
}
impl From<ImaskCcd0> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCD0` reader - Capture or Compare DN event mask CCP0"]
pub type ImaskCcd0R = crate::BitReader<ImaskCcd0>;
impl ImaskCcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcd0 {
        match self.bits {
            false => ImaskCcd0::ImaskCcd0Clr,
            true => ImaskCcd0::ImaskCcd0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccd0_clr(&self) -> bool {
        *self == ImaskCcd0::ImaskCcd0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccd0_set(&self) -> bool {
        *self == ImaskCcd0::ImaskCcd0Set
    }
}
#[doc = "Field `IMASK_CCD0` writer - Capture or Compare DN event mask CCP0"]
pub type ImaskCcd0W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcd0>;
impl<'a, REG> ImaskCcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccd0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd0::ImaskCcd0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccd0_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd0::ImaskCcd0Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcd1 {
    #[doc = "0: CLR"]
    ImaskCcd1Clr = 0,
    #[doc = "1: SET"]
    ImaskCcd1Set = 1,
}
impl From<ImaskCcd1> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCD1` reader - Capture or Compare DN event mask CCP1"]
pub type ImaskCcd1R = crate::BitReader<ImaskCcd1>;
impl ImaskCcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcd1 {
        match self.bits {
            false => ImaskCcd1::ImaskCcd1Clr,
            true => ImaskCcd1::ImaskCcd1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccd1_clr(&self) -> bool {
        *self == ImaskCcd1::ImaskCcd1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccd1_set(&self) -> bool {
        *self == ImaskCcd1::ImaskCcd1Set
    }
}
#[doc = "Field `IMASK_CCD1` writer - Capture or Compare DN event mask CCP1"]
pub type ImaskCcd1W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcd1>;
impl<'a, REG> ImaskCcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccd1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd1::ImaskCcd1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccd1_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd1::ImaskCcd1Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcd2 {
    #[doc = "0: CLR"]
    ImaskCcd2Clr = 0,
    #[doc = "1: SET"]
    ImaskCcd2Set = 1,
}
impl From<ImaskCcd2> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCD2` reader - Capture or Compare DN event mask CCP2"]
pub type ImaskCcd2R = crate::BitReader<ImaskCcd2>;
impl ImaskCcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcd2 {
        match self.bits {
            false => ImaskCcd2::ImaskCcd2Clr,
            true => ImaskCcd2::ImaskCcd2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccd2_clr(&self) -> bool {
        *self == ImaskCcd2::ImaskCcd2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccd2_set(&self) -> bool {
        *self == ImaskCcd2::ImaskCcd2Set
    }
}
#[doc = "Field `IMASK_CCD2` writer - Capture or Compare DN event mask CCP2"]
pub type ImaskCcd2W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcd2>;
impl<'a, REG> ImaskCcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccd2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd2::ImaskCcd2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccd2_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd2::ImaskCcd2Set)
    }
}
#[doc = "Capture or Compare DN event mask CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcd3 {
    #[doc = "0: CLR"]
    ImaskCcd3Clr = 0,
    #[doc = "1: SET"]
    ImaskCcd3Set = 1,
}
impl From<ImaskCcd3> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCD3` reader - Capture or Compare DN event mask CCP3"]
pub type ImaskCcd3R = crate::BitReader<ImaskCcd3>;
impl ImaskCcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcd3 {
        match self.bits {
            false => ImaskCcd3::ImaskCcd3Clr,
            true => ImaskCcd3::ImaskCcd3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccd3_clr(&self) -> bool {
        *self == ImaskCcd3::ImaskCcd3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccd3_set(&self) -> bool {
        *self == ImaskCcd3::ImaskCcd3Set
    }
}
#[doc = "Field `IMASK_CCD3` writer - Capture or Compare DN event mask CCP3"]
pub type ImaskCcd3W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcd3>;
impl<'a, REG> ImaskCcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccd3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd3::ImaskCcd3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccd3_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd3::ImaskCcd3Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcu0 {
    #[doc = "0: CLR"]
    ImaskCcu0Clr = 0,
    #[doc = "1: SET"]
    ImaskCcu0Set = 1,
}
impl From<ImaskCcu0> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCU0` reader - Capture or Compare UP event mask CCP0"]
pub type ImaskCcu0R = crate::BitReader<ImaskCcu0>;
impl ImaskCcu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcu0 {
        match self.bits {
            false => ImaskCcu0::ImaskCcu0Clr,
            true => ImaskCcu0::ImaskCcu0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccu0_clr(&self) -> bool {
        *self == ImaskCcu0::ImaskCcu0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccu0_set(&self) -> bool {
        *self == ImaskCcu0::ImaskCcu0Set
    }
}
#[doc = "Field `IMASK_CCU0` writer - Capture or Compare UP event mask CCP0"]
pub type ImaskCcu0W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcu0>;
impl<'a, REG> ImaskCcu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccu0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu0::ImaskCcu0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccu0_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu0::ImaskCcu0Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcu1 {
    #[doc = "0: CLR"]
    ImaskCcu1Clr = 0,
    #[doc = "1: SET"]
    ImaskCcu1Set = 1,
}
impl From<ImaskCcu1> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCU1` reader - Capture or Compare UP event mask CCP1"]
pub type ImaskCcu1R = crate::BitReader<ImaskCcu1>;
impl ImaskCcu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcu1 {
        match self.bits {
            false => ImaskCcu1::ImaskCcu1Clr,
            true => ImaskCcu1::ImaskCcu1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccu1_clr(&self) -> bool {
        *self == ImaskCcu1::ImaskCcu1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccu1_set(&self) -> bool {
        *self == ImaskCcu1::ImaskCcu1Set
    }
}
#[doc = "Field `IMASK_CCU1` writer - Capture or Compare UP event mask CCP1"]
pub type ImaskCcu1W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcu1>;
impl<'a, REG> ImaskCcu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccu1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu1::ImaskCcu1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccu1_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu1::ImaskCcu1Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcu2 {
    #[doc = "0: CLR"]
    ImaskCcu2Clr = 0,
    #[doc = "1: SET"]
    ImaskCcu2Set = 1,
}
impl From<ImaskCcu2> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCU2` reader - Capture or Compare UP event mask CCP2"]
pub type ImaskCcu2R = crate::BitReader<ImaskCcu2>;
impl ImaskCcu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcu2 {
        match self.bits {
            false => ImaskCcu2::ImaskCcu2Clr,
            true => ImaskCcu2::ImaskCcu2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccu2_clr(&self) -> bool {
        *self == ImaskCcu2::ImaskCcu2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccu2_set(&self) -> bool {
        *self == ImaskCcu2::ImaskCcu2Set
    }
}
#[doc = "Field `IMASK_CCU2` writer - Capture or Compare UP event mask CCP2"]
pub type ImaskCcu2W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcu2>;
impl<'a, REG> ImaskCcu2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccu2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu2::ImaskCcu2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccu2_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu2::ImaskCcu2Set)
    }
}
#[doc = "Capture or Compare UP event mask CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcu3 {
    #[doc = "0: CLR"]
    ImaskCcu3Clr = 0,
    #[doc = "1: SET"]
    ImaskCcu3Set = 1,
}
impl From<ImaskCcu3> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcu3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCU3` reader - Capture or Compare UP event mask CCP3"]
pub type ImaskCcu3R = crate::BitReader<ImaskCcu3>;
impl ImaskCcu3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcu3 {
        match self.bits {
            false => ImaskCcu3::ImaskCcu3Clr,
            true => ImaskCcu3::ImaskCcu3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccu3_clr(&self) -> bool {
        *self == ImaskCcu3::ImaskCcu3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccu3_set(&self) -> bool {
        *self == ImaskCcu3::ImaskCcu3Set
    }
}
#[doc = "Field `IMASK_CCU3` writer - Capture or Compare UP event mask CCP3"]
pub type ImaskCcu3W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcu3>;
impl<'a, REG> ImaskCcu3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccu3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu3::ImaskCcu3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccu3_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu3::ImaskCcu3Set)
    }
}
#[doc = "Compare DN event mask CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcd4 {
    #[doc = "0: CLR"]
    ImaskCcd4Clr = 0,
    #[doc = "1: SET"]
    ImaskCcd4Set = 1,
}
impl From<ImaskCcd4> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCD4` reader - Compare DN event mask CCP4"]
pub type ImaskCcd4R = crate::BitReader<ImaskCcd4>;
impl ImaskCcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcd4 {
        match self.bits {
            false => ImaskCcd4::ImaskCcd4Clr,
            true => ImaskCcd4::ImaskCcd4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccd4_clr(&self) -> bool {
        *self == ImaskCcd4::ImaskCcd4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccd4_set(&self) -> bool {
        *self == ImaskCcd4::ImaskCcd4Set
    }
}
#[doc = "Field `IMASK_CCD4` writer - Compare DN event mask CCP4"]
pub type ImaskCcd4W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcd4>;
impl<'a, REG> ImaskCcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccd4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd4::ImaskCcd4Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccd4_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd4::ImaskCcd4Set)
    }
}
#[doc = "Compare DN event mask CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcd5 {
    #[doc = "0: CLR"]
    ImaskCcd5Clr = 0,
    #[doc = "1: SET"]
    ImaskCcd5Set = 1,
}
impl From<ImaskCcd5> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCD5` reader - Compare DN event mask CCP5"]
pub type ImaskCcd5R = crate::BitReader<ImaskCcd5>;
impl ImaskCcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcd5 {
        match self.bits {
            false => ImaskCcd5::ImaskCcd5Clr,
            true => ImaskCcd5::ImaskCcd5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccd5_clr(&self) -> bool {
        *self == ImaskCcd5::ImaskCcd5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccd5_set(&self) -> bool {
        *self == ImaskCcd5::ImaskCcd5Set
    }
}
#[doc = "Field `IMASK_CCD5` writer - Compare DN event mask CCP5"]
pub type ImaskCcd5W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcd5>;
impl<'a, REG> ImaskCcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccd5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd5::ImaskCcd5Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccd5_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcd5::ImaskCcd5Set)
    }
}
#[doc = "Compare UP event mask CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcu4 {
    #[doc = "0: CLR"]
    ImaskCcu4Clr = 0,
    #[doc = "1: SET"]
    ImaskCcu4Set = 1,
}
impl From<ImaskCcu4> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCU4` reader - Compare UP event mask CCP4"]
pub type ImaskCcu4R = crate::BitReader<ImaskCcu4>;
impl ImaskCcu4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcu4 {
        match self.bits {
            false => ImaskCcu4::ImaskCcu4Clr,
            true => ImaskCcu4::ImaskCcu4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccu4_clr(&self) -> bool {
        *self == ImaskCcu4::ImaskCcu4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccu4_set(&self) -> bool {
        *self == ImaskCcu4::ImaskCcu4Set
    }
}
#[doc = "Field `IMASK_CCU4` writer - Compare UP event mask CCP4"]
pub type ImaskCcu4W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcu4>;
impl<'a, REG> ImaskCcu4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccu4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu4::ImaskCcu4Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccu4_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu4::ImaskCcu4Set)
    }
}
#[doc = "Compare UP event mask CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCcu5 {
    #[doc = "0: CLR"]
    ImaskCcu5Clr = 0,
    #[doc = "1: SET"]
    ImaskCcu5Set = 1,
}
impl From<ImaskCcu5> for bool {
    #[inline(always)]
    fn from(variant: ImaskCcu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_CCU5` reader - Compare UP event mask CCP5"]
pub type ImaskCcu5R = crate::BitReader<ImaskCcu5>;
impl ImaskCcu5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCcu5 {
        match self.bits {
            false => ImaskCcu5::ImaskCcu5Clr,
            true => ImaskCcu5::ImaskCcu5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ccu5_clr(&self) -> bool {
        *self == ImaskCcu5::ImaskCcu5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ccu5_set(&self) -> bool {
        *self == ImaskCcu5::ImaskCcu5Set
    }
}
#[doc = "Field `IMASK_CCU5` writer - Compare UP event mask CCP5"]
pub type ImaskCcu5W<'a, REG> = crate::BitWriter<'a, REG, ImaskCcu5>;
impl<'a, REG> ImaskCcu5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ccu5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu5::ImaskCcu5Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ccu5_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCcu5::ImaskCcu5Set)
    }
}
#[doc = "Fault Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskF {
    #[doc = "0: CLR"]
    ImaskFClr = 0,
    #[doc = "1: SET"]
    ImaskFSet = 1,
}
impl From<ImaskF> for bool {
    #[inline(always)]
    fn from(variant: ImaskF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_F` reader - Fault Event mask"]
pub type ImaskFR = crate::BitReader<ImaskF>;
impl ImaskFR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskF {
        match self.bits {
            false => ImaskF::ImaskFClr,
            true => ImaskF::ImaskFSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_f_clr(&self) -> bool {
        *self == ImaskF::ImaskFClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_f_set(&self) -> bool {
        *self == ImaskF::ImaskFSet
    }
}
#[doc = "Field `IMASK_F` writer - Fault Event mask"]
pub type ImaskFW<'a, REG> = crate::BitWriter<'a, REG, ImaskF>;
impl<'a, REG> ImaskFW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_f_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskF::ImaskFClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_f_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskF::ImaskFSet)
    }
}
#[doc = "Trigger Overflow Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskTov {
    #[doc = "0: CLR"]
    ImaskTovClr = 0,
    #[doc = "1: SET"]
    ImaskTovSet = 1,
}
impl From<ImaskTov> for bool {
    #[inline(always)]
    fn from(variant: ImaskTov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_TOV` reader - Trigger Overflow Event mask"]
pub type ImaskTovR = crate::BitReader<ImaskTov>;
impl ImaskTovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskTov {
        match self.bits {
            false => ImaskTov::ImaskTovClr,
            true => ImaskTov::ImaskTovSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_tov_clr(&self) -> bool {
        *self == ImaskTov::ImaskTovClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_tov_set(&self) -> bool {
        *self == ImaskTov::ImaskTovSet
    }
}
#[doc = "Field `IMASK_TOV` writer - Trigger Overflow Event mask"]
pub type ImaskTovW<'a, REG> = crate::BitWriter<'a, REG, ImaskTov>;
impl<'a, REG> ImaskTovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_tov_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskTov::ImaskTovClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_tov_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskTov::ImaskTovSet)
    }
}
#[doc = "Repeat Counter Zero Event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskRepc {
    #[doc = "0: CLR"]
    ImaskRepcClr = 0,
    #[doc = "1: SET"]
    ImaskRepcSet = 1,
}
impl From<ImaskRepc> for bool {
    #[inline(always)]
    fn from(variant: ImaskRepc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_REPC` reader - Repeat Counter Zero Event mask"]
pub type ImaskRepcR = crate::BitReader<ImaskRepc>;
impl ImaskRepcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskRepc {
        match self.bits {
            false => ImaskRepc::ImaskRepcClr,
            true => ImaskRepc::ImaskRepcSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_repc_clr(&self) -> bool {
        *self == ImaskRepc::ImaskRepcClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_repc_set(&self) -> bool {
        *self == ImaskRepc::ImaskRepcSet
    }
}
#[doc = "Field `IMASK_REPC` writer - Repeat Counter Zero Event mask"]
pub type ImaskRepcW<'a, REG> = crate::BitWriter<'a, REG, ImaskRepc>;
impl<'a, REG> ImaskRepcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_repc_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskRepc::ImaskRepcClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_repc_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskRepc::ImaskRepcSet)
    }
}
impl R {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn imask_z(&self) -> ImaskZR {
        ImaskZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn imask_l(&self) -> ImaskLR {
        ImaskLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or Compare DN event mask CCP0"]
    #[inline(always)]
    pub fn imask_ccd0(&self) -> ImaskCcd0R {
        ImaskCcd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or Compare DN event mask CCP1"]
    #[inline(always)]
    pub fn imask_ccd1(&self) -> ImaskCcd1R {
        ImaskCcd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture or Compare DN event mask CCP2"]
    #[inline(always)]
    pub fn imask_ccd2(&self) -> ImaskCcd2R {
        ImaskCcd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture or Compare DN event mask CCP3"]
    #[inline(always)]
    pub fn imask_ccd3(&self) -> ImaskCcd3R {
        ImaskCcd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or Compare UP event mask CCP0"]
    #[inline(always)]
    pub fn imask_ccu0(&self) -> ImaskCcu0R {
        ImaskCcu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or Compare UP event mask CCP1"]
    #[inline(always)]
    pub fn imask_ccu1(&self) -> ImaskCcu1R {
        ImaskCcu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture or Compare UP event mask CCP2"]
    #[inline(always)]
    pub fn imask_ccu2(&self) -> ImaskCcu2R {
        ImaskCcu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture or Compare UP event mask CCP3"]
    #[inline(always)]
    pub fn imask_ccu3(&self) -> ImaskCcu3R {
        ImaskCcu3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare DN event mask CCP4"]
    #[inline(always)]
    pub fn imask_ccd4(&self) -> ImaskCcd4R {
        ImaskCcd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare DN event mask CCP5"]
    #[inline(always)]
    pub fn imask_ccd5(&self) -> ImaskCcd5R {
        ImaskCcd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare UP event mask CCP4"]
    #[inline(always)]
    pub fn imask_ccu4(&self) -> ImaskCcu4R {
        ImaskCcu4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare UP event mask CCP5"]
    #[inline(always)]
    pub fn imask_ccu5(&self) -> ImaskCcu5R {
        ImaskCcu5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault Event mask"]
    #[inline(always)]
    pub fn imask_f(&self) -> ImaskFR {
        ImaskFR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn imask_tov(&self) -> ImaskTovR {
        ImaskTovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Repeat Counter Zero Event mask"]
    #[inline(always)]
    pub fn imask_repc(&self) -> ImaskRepcR {
        ImaskRepcR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn imask_z(&mut self) -> ImaskZW<ImaskSpec> {
        ImaskZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn imask_l(&mut self) -> ImaskLW<ImaskSpec> {
        ImaskLW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or Compare DN event mask CCP0"]
    #[inline(always)]
    pub fn imask_ccd0(&mut self) -> ImaskCcd0W<ImaskSpec> {
        ImaskCcd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or Compare DN event mask CCP1"]
    #[inline(always)]
    pub fn imask_ccd1(&mut self) -> ImaskCcd1W<ImaskSpec> {
        ImaskCcd1W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture or Compare DN event mask CCP2"]
    #[inline(always)]
    pub fn imask_ccd2(&mut self) -> ImaskCcd2W<ImaskSpec> {
        ImaskCcd2W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture or Compare DN event mask CCP3"]
    #[inline(always)]
    pub fn imask_ccd3(&mut self) -> ImaskCcd3W<ImaskSpec> {
        ImaskCcd3W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture or Compare UP event mask CCP0"]
    #[inline(always)]
    pub fn imask_ccu0(&mut self) -> ImaskCcu0W<ImaskSpec> {
        ImaskCcu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or Compare UP event mask CCP1"]
    #[inline(always)]
    pub fn imask_ccu1(&mut self) -> ImaskCcu1W<ImaskSpec> {
        ImaskCcu1W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture or Compare UP event mask CCP2"]
    #[inline(always)]
    pub fn imask_ccu2(&mut self) -> ImaskCcu2W<ImaskSpec> {
        ImaskCcu2W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture or Compare UP event mask CCP3"]
    #[inline(always)]
    pub fn imask_ccu3(&mut self) -> ImaskCcu3W<ImaskSpec> {
        ImaskCcu3W::new(self, 11)
    }
    #[doc = "Bit 12 - Compare DN event mask CCP4"]
    #[inline(always)]
    pub fn imask_ccd4(&mut self) -> ImaskCcd4W<ImaskSpec> {
        ImaskCcd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare DN event mask CCP5"]
    #[inline(always)]
    pub fn imask_ccd5(&mut self) -> ImaskCcd5W<ImaskSpec> {
        ImaskCcd5W::new(self, 13)
    }
    #[doc = "Bit 14 - Compare UP event mask CCP4"]
    #[inline(always)]
    pub fn imask_ccu4(&mut self) -> ImaskCcu4W<ImaskSpec> {
        ImaskCcu4W::new(self, 14)
    }
    #[doc = "Bit 15 - Compare UP event mask CCP5"]
    #[inline(always)]
    pub fn imask_ccu5(&mut self) -> ImaskCcu5W<ImaskSpec> {
        ImaskCcu5W::new(self, 15)
    }
    #[doc = "Bit 24 - Fault Event mask"]
    #[inline(always)]
    pub fn imask_f(&mut self) -> ImaskFW<ImaskSpec> {
        ImaskFW::new(self, 24)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn imask_tov(&mut self) -> ImaskTovW<ImaskSpec> {
        ImaskTovW::new(self, 25)
    }
    #[doc = "Bit 26 - Repeat Counter Zero Event mask"]
    #[inline(always)]
    pub fn imask_repc(&mut self) -> ImaskRepcW<ImaskSpec> {
        ImaskRepcW::new(self, 26)
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
