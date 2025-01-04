#[doc = "Register `MCANERR_CTRL` reader"]
pub type R = crate::R<McanerrCtrlSpec>;
#[doc = "Register `MCANERR_CTRL` writer"]
pub type W = crate::W<McanerrCtrlSpec>;
#[doc = "Field `MCANERR_CTRL_ECC_ENABLE` reader - Enable ECC Generation"]
pub type McanerrCtrlEccEnableR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_ECC_ENABLE` writer - Enable ECC Generation"]
pub type McanerrCtrlEccEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_CTRL_ECC_CHECK` reader - Enable ECC Check. ECC is completely bypassed if both ECC_ENABLE and ECC_CHECK are '0'."]
pub type McanerrCtrlEccCheckR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_ECC_CHECK` writer - Enable ECC Check. ECC is completely bypassed if both ECC_ENABLE and ECC_CHECK are '0'."]
pub type McanerrCtrlEccCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_CTRL_ENABLE_RMW` reader - Enable read-modify-write on partial word writes"]
pub type McanerrCtrlEnableRmwR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_ENABLE_RMW` writer - Enable read-modify-write on partial word writes"]
pub type McanerrCtrlEnableRmwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_CTRL_FORCE_SEC` reader - Force single-bit error. Cleared on a writeback or the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
pub type McanerrCtrlForceSecR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_FORCE_SEC` writer - Force single-bit error. Cleared on a writeback or the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
pub type McanerrCtrlForceSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_CTRL_FORCE_DED` reader - Force double-bit error. Cleared the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
pub type McanerrCtrlForceDedR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_FORCE_DED` writer - Force double-bit error. Cleared the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
pub type McanerrCtrlForceDedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_CTRL_FORCE_N_ROW` reader - Enable single/double-bit error on the next RAM read, regardless of the MCANERR_ERR_CTRL1.ECC_ROW setting. For write through mode, this applies to writes as well as reads."]
pub type McanerrCtrlForceNRowR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_FORCE_N_ROW` writer - Enable single/double-bit error on the next RAM read, regardless of the MCANERR_ERR_CTRL1.ECC_ROW setting. For write through mode, this applies to writes as well as reads."]
pub type McanerrCtrlForceNRowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_CTRL_ERROR_ONCE` reader - If this bit is set, the FORCE_SEC/FORCE_DED will inject an error to the specified row only once. The FORCE_SEC bit will be cleared once a writeback happens. If writeback is not enabled, this error will be cleared the cycle following the read when the data is corrected. For double-bit errors, the FORCE_DED bit will be cleared the cycle following the double-bit error. Any subsequent reads will not force an error."]
pub type McanerrCtrlErrorOnceR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_ERROR_ONCE` writer - If this bit is set, the FORCE_SEC/FORCE_DED will inject an error to the specified row only once. The FORCE_SEC bit will be cleared once a writeback happens. If writeback is not enabled, this error will be cleared the cycle following the read when the data is corrected. For double-bit errors, the FORCE_DED bit will be cleared the cycle following the double-bit error. Any subsequent reads will not force an error."]
pub type McanerrCtrlErrorOnceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_CTRL_CHECK_SVBUS_TIMEOUT` reader - Enables Serial VBUS timeout mechanism"]
pub type McanerrCtrlCheckSvbusTimeoutR = crate::BitReader;
#[doc = "Field `MCANERR_CTRL_CHECK_SVBUS_TIMEOUT` writer - Enables Serial VBUS timeout mechanism"]
pub type McanerrCtrlCheckSvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC Generation"]
    #[inline(always)]
    pub fn mcanerr_ctrl_ecc_enable(&self) -> McanerrCtrlEccEnableR {
        McanerrCtrlEccEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ECC Check. ECC is completely bypassed if both ECC_ENABLE and ECC_CHECK are '0'."]
    #[inline(always)]
    pub fn mcanerr_ctrl_ecc_check(&self) -> McanerrCtrlEccCheckR {
        McanerrCtrlEccCheckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable read-modify-write on partial word writes"]
    #[inline(always)]
    pub fn mcanerr_ctrl_enable_rmw(&self) -> McanerrCtrlEnableRmwR {
        McanerrCtrlEnableRmwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force single-bit error. Cleared on a writeback or the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
    #[inline(always)]
    pub fn mcanerr_ctrl_force_sec(&self) -> McanerrCtrlForceSecR {
        McanerrCtrlForceSecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force double-bit error. Cleared the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
    #[inline(always)]
    pub fn mcanerr_ctrl_force_ded(&self) -> McanerrCtrlForceDedR {
        McanerrCtrlForceDedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable single/double-bit error on the next RAM read, regardless of the MCANERR_ERR_CTRL1.ECC_ROW setting. For write through mode, this applies to writes as well as reads."]
    #[inline(always)]
    pub fn mcanerr_ctrl_force_n_row(&self) -> McanerrCtrlForceNRowR {
        McanerrCtrlForceNRowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is set, the FORCE_SEC/FORCE_DED will inject an error to the specified row only once. The FORCE_SEC bit will be cleared once a writeback happens. If writeback is not enabled, this error will be cleared the cycle following the read when the data is corrected. For double-bit errors, the FORCE_DED bit will be cleared the cycle following the double-bit error. Any subsequent reads will not force an error."]
    #[inline(always)]
    pub fn mcanerr_ctrl_error_once(&self) -> McanerrCtrlErrorOnceR {
        McanerrCtrlErrorOnceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables Serial VBUS timeout mechanism"]
    #[inline(always)]
    pub fn mcanerr_ctrl_check_svbus_timeout(&self) -> McanerrCtrlCheckSvbusTimeoutR {
        McanerrCtrlCheckSvbusTimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC Generation"]
    #[inline(always)]
    pub fn mcanerr_ctrl_ecc_enable(&mut self) -> McanerrCtrlEccEnableW<McanerrCtrlSpec> {
        McanerrCtrlEccEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ECC Check. ECC is completely bypassed if both ECC_ENABLE and ECC_CHECK are '0'."]
    #[inline(always)]
    pub fn mcanerr_ctrl_ecc_check(&mut self) -> McanerrCtrlEccCheckW<McanerrCtrlSpec> {
        McanerrCtrlEccCheckW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable read-modify-write on partial word writes"]
    #[inline(always)]
    pub fn mcanerr_ctrl_enable_rmw(&mut self) -> McanerrCtrlEnableRmwW<McanerrCtrlSpec> {
        McanerrCtrlEnableRmwW::new(self, 2)
    }
    #[doc = "Bit 3 - Force single-bit error. Cleared on a writeback or the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
    #[inline(always)]
    pub fn mcanerr_ctrl_force_sec(&mut self) -> McanerrCtrlForceSecW<McanerrCtrlSpec> {
        McanerrCtrlForceSecW::new(self, 3)
    }
    #[doc = "Bit 4 - Force double-bit error. Cleared the cycle following the error if ERROR_ONCE is asserted. For write through mode, this applies to writes as well as reads. MCANERR_ERR_CTRL1 and MCANERR_ERR_CTRL2 should be configured prior to setting this bit."]
    #[inline(always)]
    pub fn mcanerr_ctrl_force_ded(&mut self) -> McanerrCtrlForceDedW<McanerrCtrlSpec> {
        McanerrCtrlForceDedW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable single/double-bit error on the next RAM read, regardless of the MCANERR_ERR_CTRL1.ECC_ROW setting. For write through mode, this applies to writes as well as reads."]
    #[inline(always)]
    pub fn mcanerr_ctrl_force_n_row(&mut self) -> McanerrCtrlForceNRowW<McanerrCtrlSpec> {
        McanerrCtrlForceNRowW::new(self, 5)
    }
    #[doc = "Bit 6 - If this bit is set, the FORCE_SEC/FORCE_DED will inject an error to the specified row only once. The FORCE_SEC bit will be cleared once a writeback happens. If writeback is not enabled, this error will be cleared the cycle following the read when the data is corrected. For double-bit errors, the FORCE_DED bit will be cleared the cycle following the double-bit error. Any subsequent reads will not force an error."]
    #[inline(always)]
    pub fn mcanerr_ctrl_error_once(&mut self) -> McanerrCtrlErrorOnceW<McanerrCtrlSpec> {
        McanerrCtrlErrorOnceW::new(self, 6)
    }
    #[doc = "Bit 8 - Enables Serial VBUS timeout mechanism"]
    #[inline(always)]
    pub fn mcanerr_ctrl_check_svbus_timeout(
        &mut self,
    ) -> McanerrCtrlCheckSvbusTimeoutW<McanerrCtrlSpec> {
        McanerrCtrlCheckSvbusTimeoutW::new(self, 8)
    }
}
#[doc = "MCAN ECC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrCtrlSpec;
impl crate::RegisterSpec for McanerrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_ctrl::R`](R) reader structure"]
impl crate::Readable for McanerrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_ctrl::W`](W) writer structure"]
impl crate::Writable for McanerrCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_CTRL to value 0x0187"]
impl crate::Resettable for McanerrCtrlSpec {
    const RESET_VALUE: u32 = 0x0187;
}
