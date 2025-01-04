#[doc = "Register `FSCTL` reader"]
pub type R = crate::R<FsctlSpec>;
#[doc = "Register `FSCTL` writer"]
pub type W = crate::W<FsctlSpec>;
#[doc = "This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsctlFcen {
    #[doc = "0: DISABLE"]
    FsctlFcenDisable = 0,
    #[doc = "1: ENABLE"]
    FsctlFcenEnable = 1,
}
impl From<FsctlFcen> for bool {
    #[inline(always)]
    fn from(variant: FsctlFcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSCTL_FCEN` reader - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFcenR = crate::BitReader<FsctlFcen>;
impl FsctlFcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsctlFcen {
        match self.bits {
            false => FsctlFcen::FsctlFcenDisable,
            true => FsctlFcen::FsctlFcenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fsctl_fcen_disable(&self) -> bool {
        *self == FsctlFcen::FsctlFcenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fsctl_fcen_enable(&self) -> bool {
        *self == FsctlFcen::FsctlFcenEnable
    }
}
#[doc = "Field `FSCTL_FCEN` writer - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFcenW<'a, REG> = crate::BitWriter<'a, REG, FsctlFcen>;
impl<'a, REG> FsctlFcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fsctl_fcen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFcen::FsctlFcenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fsctl_fcen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFcen::FsctlFcenEnable)
    }
}
#[doc = "This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsctlFac0en {
    #[doc = "0: DISABLE"]
    FsctlFac0enDisable = 0,
    #[doc = "1: ENABLE"]
    FsctlFac0enEnable = 1,
}
impl From<FsctlFac0en> for bool {
    #[inline(always)]
    fn from(variant: FsctlFac0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSCTL_FAC0EN` reader - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
pub type FsctlFac0enR = crate::BitReader<FsctlFac0en>;
impl FsctlFac0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsctlFac0en {
        match self.bits {
            false => FsctlFac0en::FsctlFac0enDisable,
            true => FsctlFac0en::FsctlFac0enEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fsctl_fac0en_disable(&self) -> bool {
        *self == FsctlFac0en::FsctlFac0enDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fsctl_fac0en_enable(&self) -> bool {
        *self == FsctlFac0en::FsctlFac0enEnable
    }
}
#[doc = "Field `FSCTL_FAC0EN` writer - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
pub type FsctlFac0enW<'a, REG> = crate::BitWriter<'a, REG, FsctlFac0en>;
impl<'a, REG> FsctlFac0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fsctl_fac0en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFac0en::FsctlFac0enDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac0en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFac0en::FsctlFac0enEnable)
    }
}
#[doc = "This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsctlFac1en {
    #[doc = "0: DISABLE"]
    FsctlFac1enDisable = 0,
    #[doc = "1: ENABLE"]
    FsctlFac1enEnable = 1,
}
impl From<FsctlFac1en> for bool {
    #[inline(always)]
    fn from(variant: FsctlFac1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSCTL_FAC1EN` reader - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
pub type FsctlFac1enR = crate::BitReader<FsctlFac1en>;
impl FsctlFac1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsctlFac1en {
        match self.bits {
            false => FsctlFac1en::FsctlFac1enDisable,
            true => FsctlFac1en::FsctlFac1enEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fsctl_fac1en_disable(&self) -> bool {
        *self == FsctlFac1en::FsctlFac1enDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fsctl_fac1en_enable(&self) -> bool {
        *self == FsctlFac1en::FsctlFac1enEnable
    }
}
#[doc = "Field `FSCTL_FAC1EN` writer - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
pub type FsctlFac1enW<'a, REG> = crate::BitWriter<'a, REG, FsctlFac1en>;
impl<'a, REG> FsctlFac1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fsctl_fac1en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFac1en::FsctlFac1enDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac1en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFac1en::FsctlFac1enEnable)
    }
}
#[doc = "This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsctlFac2en {
    #[doc = "0: DISABLE"]
    FsctlFac2enDisable = 0,
    #[doc = "1: ENABLE"]
    FsctlFac2enEnable = 1,
}
impl From<FsctlFac2en> for bool {
    #[inline(always)]
    fn from(variant: FsctlFac2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSCTL_FAC2EN` reader - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
pub type FsctlFac2enR = crate::BitReader<FsctlFac2en>;
impl FsctlFac2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsctlFac2en {
        match self.bits {
            false => FsctlFac2en::FsctlFac2enDisable,
            true => FsctlFac2en::FsctlFac2enEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fsctl_fac2en_disable(&self) -> bool {
        *self == FsctlFac2en::FsctlFac2enDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fsctl_fac2en_enable(&self) -> bool {
        *self == FsctlFac2en::FsctlFac2enEnable
    }
}
#[doc = "Field `FSCTL_FAC2EN` writer - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
pub type FsctlFac2enW<'a, REG> = crate::BitWriter<'a, REG, FsctlFac2en>;
impl<'a, REG> FsctlFac2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fsctl_fac2en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFac2en::FsctlFac2enDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac2en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFac2en::FsctlFac2enEnable)
    }
}
#[doc = "This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsctlFex0en {
    #[doc = "0: DISABLE"]
    FsctlFex0enDisable = 0,
    #[doc = "1: ENABLE"]
    FsctlFex0enEnable = 1,
}
impl From<FsctlFex0en> for bool {
    #[inline(always)]
    fn from(variant: FsctlFex0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSCTL_FEX0EN` reader - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFex0enR = crate::BitReader<FsctlFex0en>;
impl FsctlFex0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsctlFex0en {
        match self.bits {
            false => FsctlFex0en::FsctlFex0enDisable,
            true => FsctlFex0en::FsctlFex0enEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fsctl_fex0en_disable(&self) -> bool {
        *self == FsctlFex0en::FsctlFex0enDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fsctl_fex0en_enable(&self) -> bool {
        *self == FsctlFex0en::FsctlFex0enEnable
    }
}
#[doc = "Field `FSCTL_FEX0EN` writer - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFex0enW<'a, REG> = crate::BitWriter<'a, REG, FsctlFex0en>;
impl<'a, REG> FsctlFex0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fsctl_fex0en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFex0en::FsctlFex0enDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex0en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFex0en::FsctlFex0enEnable)
    }
}
#[doc = "This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsctlFex1en {
    #[doc = "0: DISABLE"]
    FsctlFex1enDisable = 0,
    #[doc = "1: ENABLE"]
    FsctlFex1enEnable = 1,
}
impl From<FsctlFex1en> for bool {
    #[inline(always)]
    fn from(variant: FsctlFex1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSCTL_FEX1EN` reader - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFex1enR = crate::BitReader<FsctlFex1en>;
impl FsctlFex1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsctlFex1en {
        match self.bits {
            false => FsctlFex1en::FsctlFex1enDisable,
            true => FsctlFex1en::FsctlFex1enEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fsctl_fex1en_disable(&self) -> bool {
        *self == FsctlFex1en::FsctlFex1enDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fsctl_fex1en_enable(&self) -> bool {
        *self == FsctlFex1en::FsctlFex1enEnable
    }
}
#[doc = "Field `FSCTL_FEX1EN` writer - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFex1enW<'a, REG> = crate::BitWriter<'a, REG, FsctlFex1en>;
impl<'a, REG> FsctlFex1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fsctl_fex1en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFex1en::FsctlFex1enDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex1en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFex1en::FsctlFex1enEnable)
    }
}
#[doc = "This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsctlFex2en {
    #[doc = "0: DISABLE"]
    FsctlFex2enDisable = 0,
    #[doc = "1: ENABLE"]
    FsctlFex2enEnable = 1,
}
impl From<FsctlFex2en> for bool {
    #[inline(always)]
    fn from(variant: FsctlFex2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSCTL_FEX2EN` reader - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFex2enR = crate::BitReader<FsctlFex2en>;
impl FsctlFex2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsctlFex2en {
        match self.bits {
            false => FsctlFex2en::FsctlFex2enDisable,
            true => FsctlFex2en::FsctlFex2enEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fsctl_fex2en_disable(&self) -> bool {
        *self == FsctlFex2en::FsctlFex2enDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fsctl_fex2en_enable(&self) -> bool {
        *self == FsctlFex2en::FsctlFex2enEnable
    }
}
#[doc = "Field `FSCTL_FEX2EN` writer - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
pub type FsctlFex2enW<'a, REG> = crate::BitWriter<'a, REG, FsctlFex2en>;
impl<'a, REG> FsctlFex2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fsctl_fex2en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFex2en::FsctlFex2enDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex2en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FsctlFex2en::FsctlFex2enEnable)
    }
}
impl R {
    #[doc = "Bit 0 - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fcen(&self) -> FsctlFcenR {
        FsctlFcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac0en(&self) -> FsctlFac0enR {
        FsctlFac0enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac1en(&self) -> FsctlFac1enR {
        FsctlFac1enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac2en(&self) -> FsctlFac2enR {
        FsctlFac2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex0en(&self) -> FsctlFex0enR {
        FsctlFex0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex1en(&self) -> FsctlFex1enR {
        FsctlFex1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex2en(&self) -> FsctlFex2enR {
        FsctlFex2enR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fcen(&mut self) -> FsctlFcenW<FsctlSpec> {
        FsctlFcenW::new(self, 0)
    }
    #[doc = "Bit 1 - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac0en(&mut self) -> FsctlFac0enW<FsctlSpec> {
        FsctlFac0enW::new(self, 1)
    }
    #[doc = "Bit 2 - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac1en(&mut self) -> FsctlFac1enW<FsctlSpec> {
        FsctlFac1enW::new(self, 2)
    }
    #[doc = "Bit 3 - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fac2en(&mut self) -> FsctlFac2enW<FsctlSpec> {
        FsctlFac2enW::new(self, 3)
    }
    #[doc = "Bit 4 - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex0en(&mut self) -> FsctlFex0enW<FsctlSpec> {
        FsctlFex0enW::new(self, 4)
    }
    #[doc = "Bit 5 - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex1en(&mut self) -> FsctlFex1enW<FsctlSpec> {
        FsctlFex1enW::new(self, 5)
    }
    #[doc = "Bit 6 - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fsctl_fex2en(&mut self) -> FsctlFex2enW<FsctlSpec> {
        FsctlFex2enW::new(self, 6)
    }
}
#[doc = "Fault Source Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsctlSpec;
impl crate::RegisterSpec for FsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsctl::R`](R) reader structure"]
impl crate::Readable for FsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fsctl::W`](W) writer structure"]
impl crate::Writable for FsctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCTL to value 0"]
impl crate::Resettable for FsctlSpec {
    const RESET_VALUE: u32 = 0;
}
