#[doc = "Register `MCANERR_ERR_STAT1` reader"]
pub type R = crate::R<McanerrErrStat1Spec>;
#[doc = "Register `MCANERR_ERR_STAT1` writer"]
pub type W = crate::W<McanerrErrStat1Spec>;
#[doc = "Field `MCANERR_ERR_STAT1_ECC_SEC` reader - Single Bit Error Corrected Status. A 2-bit saturating counter of the number of SEC errors that have occurred since last cleared. 0 No single-bit error detected 1 One single-bit error was detected and corrected 2 Two single-bit errors were detected and corrected 3 Three single-bit errors were detected and corrected A write of a non-zero value to this bit field increments it by the value provided."]
pub type McanerrErrStat1EccSecR = crate::FieldReader;
#[doc = "Field `MCANERR_ERR_STAT1_ECC_SEC` writer - Single Bit Error Corrected Status. A 2-bit saturating counter of the number of SEC errors that have occurred since last cleared. 0 No single-bit error detected 1 One single-bit error was detected and corrected 2 Two single-bit errors were detected and corrected 3 Three single-bit errors were detected and corrected A write of a non-zero value to this bit field increments it by the value provided."]
pub type McanerrErrStat1EccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCANERR_ERR_STAT1_ECC_DED` reader - Double Bit Error Detected Status. A 2-bit saturating counter of the number of DED errors that have occurred since last cleared. 0 No double-bit error detected 1 One double-bit error was detected 2 Two double-bit errors were detected 3 Three double-bit errors were detected A write of a non-zero value to this bit field increments it by the value provided."]
pub type McanerrErrStat1EccDedR = crate::FieldReader;
#[doc = "Field `MCANERR_ERR_STAT1_ECC_DED` writer - Double Bit Error Detected Status. A 2-bit saturating counter of the number of DED errors that have occurred since last cleared. 0 No double-bit error detected 1 One double-bit error was detected 2 Two double-bit errors were detected 3 Three double-bit errors were detected A write of a non-zero value to this bit field increments it by the value provided."]
pub type McanerrErrStat1EccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCANERR_ERR_STAT1_ECC_OTHER` reader - SEC While Writeback Error Status 0 No SEC error while writeback pending 1 Indicates that successive single-bit errors have occurred while a writeback is still pending"]
pub type McanerrErrStat1EccOtherR = crate::BitReader;
#[doc = "Field `MCANERR_ERR_STAT1_ECC_OTHER` writer - SEC While Writeback Error Status 0 No SEC error while writeback pending 1 Indicates that successive single-bit errors have occurred while a writeback is still pending"]
pub type McanerrErrStat1EccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_ERR_STAT1_CTRL_REG_ERROR` reader - Control Register Error. A bit field in the control register is in an ambiguous state. This means that the redundancy registers have detected a state where not all values are the same and has defaulted to the reset state. S/W needs to re-write these registers to a known state. A write of 1 will set this interrupt flag."]
pub type McanerrErrStat1CtrlRegErrorR = crate::BitReader;
#[doc = "Field `MCANERR_ERR_STAT1_CTRL_REG_ERROR` writer - Control Register Error. A bit field in the control register is in an ambiguous state. This means that the redundancy registers have detected a state where not all values are the same and has defaulted to the reset state. S/W needs to re-write these registers to a known state. A write of 1 will set this interrupt flag."]
pub type McanerrErrStat1CtrlRegErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_ECC_SEC` reader - Clear ECC_SEC. A write of a non-zero value to this bit field decrements the ECC_SEC bit field by the value provided."]
pub type McanerrErrStat1ClrEccSecR = crate::FieldReader;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_ECC_SEC` writer - Clear ECC_SEC. A write of a non-zero value to this bit field decrements the ECC_SEC bit field by the value provided."]
pub type McanerrErrStat1ClrEccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_ECC_DED` reader - Clear ECC_DED. A write of a non-zero value to this bit field decrements the ECC_DED bit field by the value provided."]
pub type McanerrErrStat1ClrEccDedR = crate::FieldReader;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_ECC_DED` writer - Clear ECC_DED. A write of a non-zero value to this bit field decrements the ECC_DED bit field by the value provided."]
pub type McanerrErrStat1ClrEccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_ECC_OTHER` reader - Writing a '1' clears the ECC_OTHER bit."]
pub type McanerrErrStat1ClrEccOtherR = crate::BitReader;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_ECC_OTHER` writer - Writing a '1' clears the ECC_OTHER bit."]
pub type McanerrErrStat1ClrEccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_CTRL_REG_ERROR` reader - Writing a '1' clears the CTRL_REG_ERROR bit"]
pub type McanerrErrStat1ClrCtrlRegErrorR = crate::BitReader;
#[doc = "Field `MCANERR_ERR_STAT1_CLR_CTRL_REG_ERROR` writer - Writing a '1' clears the CTRL_REG_ERROR bit"]
pub type McanerrErrStat1ClrCtrlRegErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_ERR_STAT1_ECC_BIT1` reader - ECC Error Bit Position. Indicates the bit position in the RAM data that is in error on an SEC error. Only valid on an SEC error. 0 Bit 0 is in error 1 Bit 1 is in error 2 Bit 2 is in error 3 Bit 3 is in error ... 31 Bit 31 is in error &gt;32 Invalid"]
pub type McanerrErrStat1EccBit1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Single Bit Error Corrected Status. A 2-bit saturating counter of the number of SEC errors that have occurred since last cleared. 0 No single-bit error detected 1 One single-bit error was detected and corrected 2 Two single-bit errors were detected and corrected 3 Three single-bit errors were detected and corrected A write of a non-zero value to this bit field increments it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ecc_sec(&self) -> McanerrErrStat1EccSecR {
        McanerrErrStat1EccSecR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Double Bit Error Detected Status. A 2-bit saturating counter of the number of DED errors that have occurred since last cleared. 0 No double-bit error detected 1 One double-bit error was detected 2 Two double-bit errors were detected 3 Three double-bit errors were detected A write of a non-zero value to this bit field increments it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ecc_ded(&self) -> McanerrErrStat1EccDedR {
        McanerrErrStat1EccDedR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SEC While Writeback Error Status 0 No SEC error while writeback pending 1 Indicates that successive single-bit errors have occurred while a writeback is still pending"]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ecc_other(&self) -> McanerrErrStat1EccOtherR {
        McanerrErrStat1EccOtherR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Control Register Error. A bit field in the control register is in an ambiguous state. This means that the redundancy registers have detected a state where not all values are the same and has defaulted to the reset state. S/W needs to re-write these registers to a known state. A write of 1 will set this interrupt flag."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ctrl_reg_error(&self) -> McanerrErrStat1CtrlRegErrorR {
        McanerrErrStat1CtrlRegErrorR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clear ECC_SEC. A write of a non-zero value to this bit field decrements the ECC_SEC bit field by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ecc_sec(&self) -> McanerrErrStat1ClrEccSecR {
        McanerrErrStat1ClrEccSecR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Clear ECC_DED. A write of a non-zero value to this bit field decrements the ECC_DED bit field by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ecc_ded(&self) -> McanerrErrStat1ClrEccDedR {
        McanerrErrStat1ClrEccDedR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Writing a '1' clears the ECC_OTHER bit."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ecc_other(&self) -> McanerrErrStat1ClrEccOtherR {
        McanerrErrStat1ClrEccOtherR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Writing a '1' clears the CTRL_REG_ERROR bit"]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ctrl_reg_error(&self) -> McanerrErrStat1ClrCtrlRegErrorR {
        McanerrErrStat1ClrCtrlRegErrorR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - ECC Error Bit Position. Indicates the bit position in the RAM data that is in error on an SEC error. Only valid on an SEC error. 0 Bit 0 is in error 1 Bit 1 is in error 2 Bit 2 is in error 3 Bit 3 is in error ... 31 Bit 31 is in error &gt;32 Invalid"]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ecc_bit1(&self) -> McanerrErrStat1EccBit1R {
        McanerrErrStat1EccBit1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Single Bit Error Corrected Status. A 2-bit saturating counter of the number of SEC errors that have occurred since last cleared. 0 No single-bit error detected 1 One single-bit error was detected and corrected 2 Two single-bit errors were detected and corrected 3 Three single-bit errors were detected and corrected A write of a non-zero value to this bit field increments it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ecc_sec(&mut self) -> McanerrErrStat1EccSecW<McanerrErrStat1Spec> {
        McanerrErrStat1EccSecW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Double Bit Error Detected Status. A 2-bit saturating counter of the number of DED errors that have occurred since last cleared. 0 No double-bit error detected 1 One double-bit error was detected 2 Two double-bit errors were detected 3 Three double-bit errors were detected A write of a non-zero value to this bit field increments it by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ecc_ded(&mut self) -> McanerrErrStat1EccDedW<McanerrErrStat1Spec> {
        McanerrErrStat1EccDedW::new(self, 2)
    }
    #[doc = "Bit 4 - SEC While Writeback Error Status 0 No SEC error while writeback pending 1 Indicates that successive single-bit errors have occurred while a writeback is still pending"]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ecc_other(&mut self) -> McanerrErrStat1EccOtherW<McanerrErrStat1Spec> {
        McanerrErrStat1EccOtherW::new(self, 4)
    }
    #[doc = "Bit 7 - Control Register Error. A bit field in the control register is in an ambiguous state. This means that the redundancy registers have detected a state where not all values are the same and has defaulted to the reset state. S/W needs to re-write these registers to a known state. A write of 1 will set this interrupt flag."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_ctrl_reg_error(
        &mut self,
    ) -> McanerrErrStat1CtrlRegErrorW<McanerrErrStat1Spec> {
        McanerrErrStat1CtrlRegErrorW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clear ECC_SEC. A write of a non-zero value to this bit field decrements the ECC_SEC bit field by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ecc_sec(
        &mut self,
    ) -> McanerrErrStat1ClrEccSecW<McanerrErrStat1Spec> {
        McanerrErrStat1ClrEccSecW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Clear ECC_DED. A write of a non-zero value to this bit field decrements the ECC_DED bit field by the value provided."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ecc_ded(
        &mut self,
    ) -> McanerrErrStat1ClrEccDedW<McanerrErrStat1Spec> {
        McanerrErrStat1ClrEccDedW::new(self, 10)
    }
    #[doc = "Bit 12 - Writing a '1' clears the ECC_OTHER bit."]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ecc_other(
        &mut self,
    ) -> McanerrErrStat1ClrEccOtherW<McanerrErrStat1Spec> {
        McanerrErrStat1ClrEccOtherW::new(self, 12)
    }
    #[doc = "Bit 15 - Writing a '1' clears the CTRL_REG_ERROR bit"]
    #[inline(always)]
    pub fn mcanerr_err_stat1_clr_ctrl_reg_error(
        &mut self,
    ) -> McanerrErrStat1ClrCtrlRegErrorW<McanerrErrStat1Spec> {
        McanerrErrStat1ClrCtrlRegErrorW::new(self, 15)
    }
}
#[doc = "MCAN ECC Error Status 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_stat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_stat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrErrStat1Spec;
impl crate::RegisterSpec for McanerrErrStat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_err_stat1::R`](R) reader structure"]
impl crate::Readable for McanerrErrStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_err_stat1::W`](W) writer structure"]
impl crate::Writable for McanerrErrStat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_ERR_STAT1 to value 0"]
impl crate::Resettable for McanerrErrStat1Spec {
    const RESET_VALUE: u32 = 0;
}
