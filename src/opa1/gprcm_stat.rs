#[doc = "Register `GPRCM_STAT` reader"]
pub type R = crate::R<GprcmStatSpec>;
#[doc = "This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GprcmStatResetstky {
    #[doc = "0: NORES"]
    GprcmStatResetstkyNores = 0,
    #[doc = "1: RESET"]
    GprcmStatResetstkyReset = 1,
}
impl From<GprcmStatResetstky> for bool {
    #[inline(always)]
    fn from(variant: GprcmStatResetstky) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPRCM_STAT_RESETSTKY` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type GprcmStatResetstkyR = crate::BitReader<GprcmStatResetstky>;
impl GprcmStatResetstkyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GprcmStatResetstky {
        match self.bits {
            false => GprcmStatResetstky::GprcmStatResetstkyNores,
            true => GprcmStatResetstky::GprcmStatResetstkyReset,
        }
    }
    #[doc = "NORES"]
    #[inline(always)]
    pub fn is_gprcm_stat_resetstky_nores(&self) -> bool {
        *self == GprcmStatResetstky::GprcmStatResetstkyNores
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn is_gprcm_stat_resetstky_reset(&self) -> bool {
        *self == GprcmStatResetstky::GprcmStatResetstkyReset
    }
}
impl R {
    #[doc = "Bit 16 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn gprcm_stat_resetstky(&self) -> GprcmStatResetstkyR {
        GprcmStatResetstkyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gprcm_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GprcmStatSpec;
impl crate::RegisterSpec for GprcmStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gprcm_stat::R`](R) reader structure"]
impl crate::Readable for GprcmStatSpec {}
#[doc = "`reset()` method sets GPRCM_STAT to value 0"]
impl crate::Resettable for GprcmStatSpec {
    const RESET_VALUE: u32 = 0;
}
