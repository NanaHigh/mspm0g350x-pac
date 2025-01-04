#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatResetstky {
    #[doc = "0: NORES"]
    StatResetstkyNores = 0,
    #[doc = "1: RESET"]
    StatResetstkyReset = 1,
}
impl From<StatResetstky> for bool {
    #[inline(always)]
    fn from(variant: StatResetstky) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_RESETSTKY` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type StatResetstkyR = crate::BitReader<StatResetstky>;
impl StatResetstkyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatResetstky {
        match self.bits {
            false => StatResetstky::StatResetstkyNores,
            true => StatResetstky::StatResetstkyReset,
        }
    }
    #[doc = "NORES"]
    #[inline(always)]
    pub fn is_stat_resetstky_nores(&self) -> bool {
        *self == StatResetstky::StatResetstkyNores
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn is_stat_resetstky_reset(&self) -> bool {
        *self == StatResetstky::StatResetstkyReset
    }
}
impl R {
    #[doc = "Bit 16 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn stat_resetstky(&self) -> StatResetstkyR {
        StatResetstkyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
