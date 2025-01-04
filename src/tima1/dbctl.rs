#[doc = "Register `DBCTL` reader"]
pub type R = crate::R<DbctlSpec>;
#[doc = "Register `DBCTL` writer"]
pub type W = crate::W<DbctlSpec>;
#[doc = "Field `DBCTL_RISEDELAY` reader - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
pub type DbctlRisedelayR = crate::FieldReader<u16>;
#[doc = "Field `DBCTL_RISEDELAY` writer - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
pub type DbctlRisedelayW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Dead Band Mode 1 Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbctlM1Enable {
    #[doc = "0: DISABLED"]
    DbctlM1EnableDisabled = 0,
    #[doc = "1: ENABLED"]
    DbctlM1EnableEnabled = 1,
}
impl From<DbctlM1Enable> for bool {
    #[inline(always)]
    fn from(variant: DbctlM1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBCTL_M1_ENABLE` reader - Dead Band Mode 1 Enable."]
pub type DbctlM1EnableR = crate::BitReader<DbctlM1Enable>;
impl DbctlM1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbctlM1Enable {
        match self.bits {
            false => DbctlM1Enable::DbctlM1EnableDisabled,
            true => DbctlM1Enable::DbctlM1EnableEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_dbctl_m1_enable_disabled(&self) -> bool {
        *self == DbctlM1Enable::DbctlM1EnableDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_dbctl_m1_enable_enabled(&self) -> bool {
        *self == DbctlM1Enable::DbctlM1EnableEnabled
    }
}
#[doc = "Field `DBCTL_M1_ENABLE` writer - Dead Band Mode 1 Enable."]
pub type DbctlM1EnableW<'a, REG> = crate::BitWriter<'a, REG, DbctlM1Enable>;
impl<'a, REG> DbctlM1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn dbctl_m1_enable_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DbctlM1Enable::DbctlM1EnableDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn dbctl_m1_enable_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DbctlM1Enable::DbctlM1EnableEnabled)
    }
}
#[doc = "Field `DBCTL_FALLDELAY` reader - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
pub type DbctlFalldelayR = crate::FieldReader<u16>;
#[doc = "Field `DBCTL_FALLDELAY` writer - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
pub type DbctlFalldelayW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
    #[inline(always)]
    pub fn dbctl_risedelay(&self) -> DbctlRisedelayR {
        DbctlRisedelayR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Dead Band Mode 1 Enable."]
    #[inline(always)]
    pub fn dbctl_m1_enable(&self) -> DbctlM1EnableR {
        DbctlM1EnableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
    #[inline(always)]
    pub fn dbctl_falldelay(&self) -> DbctlFalldelayR {
        DbctlFalldelayR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Rise Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPAo."]
    #[inline(always)]
    pub fn dbctl_risedelay(&mut self) -> DbctlRisedelayW<DbctlSpec> {
        DbctlRisedelayW::new(self, 0)
    }
    #[doc = "Bit 12 - Dead Band Mode 1 Enable."]
    #[inline(always)]
    pub fn dbctl_m1_enable(&mut self) -> DbctlM1EnableW<DbctlSpec> {
        DbctlM1EnableW::new(self, 12)
    }
    #[doc = "Bits 16:27 - Fall Delay The number of TIMCLK periods inserted between the fall of CCPi and the rise of CCPBo"]
    #[inline(always)]
    pub fn dbctl_falldelay(&mut self) -> DbctlFalldelayW<DbctlSpec> {
        DbctlFalldelayW::new(self, 16)
    }
}
#[doc = "Dead Band insertion control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbctlSpec;
impl crate::RegisterSpec for DbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbctl::R`](R) reader structure"]
impl crate::Readable for DbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbctl::W`](W) writer structure"]
impl crate::Writable for DbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBCTL to value 0"]
impl crate::Resettable for DbctlSpec {
    const RESET_VALUE: u32 = 0;
}
