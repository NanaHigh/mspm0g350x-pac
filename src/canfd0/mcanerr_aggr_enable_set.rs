#[doc = "Register `MCANERR_AGGR_ENABLE_SET` reader"]
pub type R = crate::R<McanerrAggrEnableSetSpec>;
#[doc = "Register `MCANERR_AGGR_ENABLE_SET` writer"]
pub type W = crate::W<McanerrAggrEnableSetSpec>;
#[doc = "Field `MCANERR_AGGR_ENABLE_SET_ENABLE_PARITY_SET` reader - Write 1 to enable parity errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableSetEnableParitySetR = crate::BitReader;
#[doc = "Field `MCANERR_AGGR_ENABLE_SET_ENABLE_PARITY_SET` writer - Write 1 to enable parity errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableSetEnableParitySetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_AGGR_ENABLE_SET_ENABLE_TIMEOUT_SET` reader - Write 1 to enable timeout errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableSetEnableTimeoutSetR = crate::BitReader;
#[doc = "Field `MCANERR_AGGR_ENABLE_SET_ENABLE_TIMEOUT_SET` writer - Write 1 to enable timeout errors. Reads return the corresponding enable bit's current value."]
pub type McanerrAggrEnableSetEnableTimeoutSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable parity errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_set_enable_parity_set(
        &self,
    ) -> McanerrAggrEnableSetEnableParitySetR {
        McanerrAggrEnableSetEnableParitySetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable timeout errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_set_enable_timeout_set(
        &self,
    ) -> McanerrAggrEnableSetEnableTimeoutSetR {
        McanerrAggrEnableSetEnableTimeoutSetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable parity errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_set_enable_parity_set(
        &mut self,
    ) -> McanerrAggrEnableSetEnableParitySetW<McanerrAggrEnableSetSpec> {
        McanerrAggrEnableSetEnableParitySetW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable timeout errors. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_aggr_enable_set_enable_timeout_set(
        &mut self,
    ) -> McanerrAggrEnableSetEnableTimeoutSetW<McanerrAggrEnableSetSpec> {
        McanerrAggrEnableSetEnableTimeoutSetW::new(self, 1)
    }
}
#[doc = "MCAN Error Aggregator Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_aggr_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_aggr_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrAggrEnableSetSpec;
impl crate::RegisterSpec for McanerrAggrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_aggr_enable_set::R`](R) reader structure"]
impl crate::Readable for McanerrAggrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_aggr_enable_set::W`](W) writer structure"]
impl crate::Writable for McanerrAggrEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_AGGR_ENABLE_SET to value 0"]
impl crate::Resettable for McanerrAggrEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
