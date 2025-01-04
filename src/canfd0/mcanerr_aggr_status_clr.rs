#[doc = "Register `MCANERR_AGGR_STATUS_CLR` reader"]
pub type R = crate::R<McanerrAggrStatusClrSpec>;
#[doc = "Register `MCANERR_AGGR_STATUS_CLR` writer"]
pub type W = crate::W<McanerrAggrStatusClrSpec>;
#[doc = "Field `MCANERR_AGGR_STATUS_CLR_AGGR_PARITY_ERR` reader - Aggregator Parity Error Status 2-bit saturating counter of the number of parity errors that have occurred since last cleared. 0 No parity errors have occurred 1 One parity error has occurred 2 Two parity errors have occurred 3 Three parity errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
pub type McanerrAggrStatusClrAggrParityErrR = crate::FieldReader;
#[doc = "Field `MCANERR_AGGR_STATUS_CLR_AGGR_PARITY_ERR` writer - Aggregator Parity Error Status 2-bit saturating counter of the number of parity errors that have occurred since last cleared. 0 No parity errors have occurred 1 One parity error has occurred 2 Two parity errors have occurred 3 Three parity errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
pub type McanerrAggrStatusClrAggrParityErrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCANERR_AGGR_STATUS_CLR_SVBUS_TIMEOUT` reader - Aggregator Serial VBUS Timeout Error Status 2-bit saturating counter of the number of SVBUS timeout errors that have occurred since last cleared. 0 No timeout errors have occurred 1 One timeout error has occurred 2 Two timeout errors have occurred 3 Three timeout errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
pub type McanerrAggrStatusClrSvbusTimeoutR = crate::FieldReader;
#[doc = "Field `MCANERR_AGGR_STATUS_CLR_SVBUS_TIMEOUT` writer - Aggregator Serial VBUS Timeout Error Status 2-bit saturating counter of the number of SVBUS timeout errors that have occurred since last cleared. 0 No timeout errors have occurred 1 One timeout error has occurred 2 Two timeout errors have occurred 3 Three timeout errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
pub type McanerrAggrStatusClrSvbusTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Aggregator Parity Error Status 2-bit saturating counter of the number of parity errors that have occurred since last cleared. 0 No parity errors have occurred 1 One parity error has occurred 2 Two parity errors have occurred 3 Three parity errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_aggr_status_clr_aggr_parity_err(&self) -> McanerrAggrStatusClrAggrParityErrR {
        McanerrAggrStatusClrAggrParityErrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Aggregator Serial VBUS Timeout Error Status 2-bit saturating counter of the number of SVBUS timeout errors that have occurred since last cleared. 0 No timeout errors have occurred 1 One timeout error has occurred 2 Two timeout errors have occurred 3 Three timeout errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_aggr_status_clr_svbus_timeout(&self) -> McanerrAggrStatusClrSvbusTimeoutR {
        McanerrAggrStatusClrSvbusTimeoutR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Aggregator Parity Error Status 2-bit saturating counter of the number of parity errors that have occurred since last cleared. 0 No parity errors have occurred 1 One parity error has occurred 2 Two parity errors have occurred 3 Three parity errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_aggr_status_clr_aggr_parity_err(
        &mut self,
    ) -> McanerrAggrStatusClrAggrParityErrW<McanerrAggrStatusClrSpec> {
        McanerrAggrStatusClrAggrParityErrW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Aggregator Serial VBUS Timeout Error Status 2-bit saturating counter of the number of SVBUS timeout errors that have occurred since last cleared. 0 No timeout errors have occurred 1 One timeout error has occurred 2 Two timeout errors have occurred 3 Three timeout errors have occurred A write of a non-zero value to this bit field decrements it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_aggr_status_clr_svbus_timeout(
        &mut self,
    ) -> McanerrAggrStatusClrSvbusTimeoutW<McanerrAggrStatusClrSpec> {
        McanerrAggrStatusClrSvbusTimeoutW::new(self, 2)
    }
}
#[doc = "MCAN Error Aggregator Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_aggr_status_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_aggr_status_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrAggrStatusClrSpec;
impl crate::RegisterSpec for McanerrAggrStatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_aggr_status_clr::R`](R) reader structure"]
impl crate::Readable for McanerrAggrStatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_aggr_status_clr::W`](W) writer structure"]
impl crate::Writable for McanerrAggrStatusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_AGGR_STATUS_CLR to value 0"]
impl crate::Resettable for McanerrAggrStatusClrSpec {
    const RESET_VALUE: u32 = 0;
}
