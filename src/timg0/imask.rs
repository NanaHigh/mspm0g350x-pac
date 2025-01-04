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
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn imask_tov(&self) -> ImaskTovR {
        ImaskTovR::new(((self.bits >> 25) & 1) != 0)
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
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn imask_tov(&mut self) -> ImaskTovW<ImaskSpec> {
        ImaskTovW::new(self, 25)
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
