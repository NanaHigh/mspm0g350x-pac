#[doc = "Register `MCANERR_ERR_STAT3` reader"]
pub type R = crate::R<McanerrErrStat3Spec>;
#[doc = "Register `MCANERR_ERR_STAT3` writer"]
pub type W = crate::W<McanerrErrStat3Spec>;
#[doc = "Field `MCANERR_ERR_STAT3_WB_PEND` reader - Delayed Write Back Pending Status 0 No write back pending 1 An ECC data correction write back is pending"]
pub type McanerrErrStat3WbPendR = crate::BitReader;
#[doc = "Field `MCANERR_ERR_STAT3_SVBUS_TIMEOUT` reader - Serial VBUS Timeout Flag. Write 1 to set."]
pub type McanerrErrStat3SvbusTimeoutR = crate::BitReader;
#[doc = "Field `MCANERR_ERR_STAT3_SVBUS_TIMEOUT` writer - Serial VBUS Timeout Flag. Write 1 to set."]
pub type McanerrErrStat3SvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_ERR_STAT3_CLR_SVBUS_TIMEOUT` reader - Write 1 to clear the Serial VBUS Timeout Flag"]
pub type McanerrErrStat3ClrSvbusTimeoutR = crate::BitReader;
#[doc = "Field `MCANERR_ERR_STAT3_CLR_SVBUS_TIMEOUT` writer - Write 1 to clear the Serial VBUS Timeout Flag"]
pub type McanerrErrStat3ClrSvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Delayed Write Back Pending Status 0 No write back pending 1 An ECC data correction write back is pending"]
    #[inline(always)]
    pub fn mcanerr_err_stat3_wb_pend(&self) -> McanerrErrStat3WbPendR {
        McanerrErrStat3WbPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial VBUS Timeout Flag. Write 1 to set."]
    #[inline(always)]
    pub fn mcanerr_err_stat3_svbus_timeout(&self) -> McanerrErrStat3SvbusTimeoutR {
        McanerrErrStat3SvbusTimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to clear the Serial VBUS Timeout Flag"]
    #[inline(always)]
    pub fn mcanerr_err_stat3_clr_svbus_timeout(&self) -> McanerrErrStat3ClrSvbusTimeoutR {
        McanerrErrStat3ClrSvbusTimeoutR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Serial VBUS Timeout Flag. Write 1 to set."]
    #[inline(always)]
    pub fn mcanerr_err_stat3_svbus_timeout(
        &mut self,
    ) -> McanerrErrStat3SvbusTimeoutW<McanerrErrStat3Spec> {
        McanerrErrStat3SvbusTimeoutW::new(self, 1)
    }
    #[doc = "Bit 9 - Write 1 to clear the Serial VBUS Timeout Flag"]
    #[inline(always)]
    pub fn mcanerr_err_stat3_clr_svbus_timeout(
        &mut self,
    ) -> McanerrErrStat3ClrSvbusTimeoutW<McanerrErrStat3Spec> {
        McanerrErrStat3ClrSvbusTimeoutW::new(self, 9)
    }
}
#[doc = "MCAN ECC Error Status 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_stat3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_stat3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrErrStat3Spec;
impl crate::RegisterSpec for McanerrErrStat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_err_stat3::R`](R) reader structure"]
impl crate::Readable for McanerrErrStat3Spec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_err_stat3::W`](W) writer structure"]
impl crate::Writable for McanerrErrStat3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_ERR_STAT3 to value 0"]
impl crate::Resettable for McanerrErrStat3Spec {
    const RESET_VALUE: u32 = 0;
}
