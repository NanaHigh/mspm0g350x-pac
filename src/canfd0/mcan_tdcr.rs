#[doc = "Register `MCAN_TDCR` reader"]
pub type R = crate::R<McanTdcrSpec>;
#[doc = "Register `MCAN_TDCR` writer"]
pub type W = crate::W<McanTdcrSpec>;
#[doc = "Field `MCAN_TDCR_TDCF` reader - Transmitter Delay Compensation Filter Window Length. Defines the minimum value for the SSP position, dominant edges on the internal CAN RX signal that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTdcrTdcfR = crate::FieldReader;
#[doc = "Field `MCAN_TDCR_TDCF` writer - Transmitter Delay Compensation Filter Window Length. Defines the minimum value for the SSP position, dominant edges on the internal CAN RX signal that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTdcrTdcfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MCAN_TDCR_TDCO` reader - Transmitter Delay Compensation Offset. Offset value defining the distance between the measured delay from the internal CAN TX signal to the internal CAN RX signal and the secondary sample point. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTdcrTdcoR = crate::FieldReader;
#[doc = "Field `MCAN_TDCR_TDCO` writer - Transmitter Delay Compensation Offset. Offset value defining the distance between the measured delay from the internal CAN TX signal to the internal CAN RX signal and the secondary sample point. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTdcrTdcoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length. Defines the minimum value for the SSP position, dominant edges on the internal CAN RX signal that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tdcr_tdcf(&self) -> McanTdcrTdcfR {
        McanTdcrTdcfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset. Offset value defining the distance between the measured delay from the internal CAN TX signal to the internal CAN RX signal and the secondary sample point. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tdcr_tdco(&self) -> McanTdcrTdcoR {
        McanTdcrTdcoR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length. Defines the minimum value for the SSP position, dominant edges on the internal CAN RX signal that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tdcr_tdcf(&mut self) -> McanTdcrTdcfW<McanTdcrSpec> {
        McanTdcrTdcfW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset. Offset value defining the distance between the measured delay from the internal CAN TX signal to the internal CAN RX signal and the secondary sample point. Valid values are 0 to 127 mtq. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tdcr_tdco(&mut self) -> McanTdcrTdcoW<McanTdcrSpec> {
        McanTdcrTdcoW::new(self, 8)
    }
}
#[doc = "MCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTdcrSpec;
impl crate::RegisterSpec for McanTdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_tdcr::R`](R) reader structure"]
impl crate::Readable for McanTdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_tdcr::W`](W) writer structure"]
impl crate::Writable for McanTdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TDCR to value 0"]
impl crate::Resettable for McanTdcrSpec {
    const RESET_VALUE: u32 = 0;
}
