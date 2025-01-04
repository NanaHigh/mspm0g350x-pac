#[doc = "Register `MCANERR_AGGR_ENABLE_CLR` reader"]
pub type R = crate::R<McanerrAggrEnableClrSpec>;
#[doc = "Register `MCANERR_AGGR_ENABLE_CLR` writer"]
pub type W = crate::W<McanerrAggrEnableClrSpec>;
#[doc = "Field `MCANERR_AGGR_ENABLE_CLR_ENABLE_PARITY_CLR` reader - Write 1 to disable parity errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableClrEnableParityClrR = crate::BitReader;
#[doc = "Field `MCANERR_AGGR_ENABLE_CLR_ENABLE_PARITY_CLR` writer - Write 1 to disable parity errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableClrEnableParityClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_AGGR_ENABLE_CLR_ENABLE_TIMEOUT_CLR` reader - Write 1 to disable timeout errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableClrEnableTimeoutClrR = crate::BitReader;
#[doc = "Field `MCANERR_AGGR_ENABLE_CLR_ENABLE_TIMEOUT_CLR` writer - Write 1 to disable timeout errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableClrEnableTimeoutClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to disable parity errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_clr_enable_parity_clr(
        &self,
    ) -> McanerrAggrEnableClrEnableParityClrR {
        McanerrAggrEnableClrEnableParityClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to disable timeout errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_clr_enable_timeout_clr(
        &self,
    ) -> McanerrAggrEnableClrEnableTimeoutClrR {
        McanerrAggrEnableClrEnableTimeoutClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to disable parity errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_clr_enable_parity_clr(
        &mut self,
    ) -> McanerrAggrEnableClrEnableParityClrW<McanerrAggrEnableClrSpec> {
        McanerrAggrEnableClrEnableParityClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to disable timeout errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_clr_enable_timeout_clr(
        &mut self,
    ) -> McanerrAggrEnableClrEnableTimeoutClrW<McanerrAggrEnableClrSpec> {
        McanerrAggrEnableClrEnableTimeoutClrW::new(self, 1)
    }
}
#[doc = "MCAN Error Aggregator Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_aggr_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_aggr_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrAggrEnableClrSpec;
impl crate::RegisterSpec for McanerrAggrEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_aggr_enable_clr::R`](R) reader structure"]
impl crate::Readable for McanerrAggrEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_aggr_enable_clr::W`](W) writer structure"]
impl crate::Writable for McanerrAggrEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_AGGR_ENABLE_CLR to value 0"]
impl crate::Resettable for McanerrAggrEnableClrSpec {
    const RESET_VALUE: u32 = 0;
}
