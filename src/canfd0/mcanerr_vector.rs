#[doc = "Register `MCANERR_VECTOR` reader"]
pub type R = crate::R<McanerrVectorSpec>;
#[doc = "Register `MCANERR_VECTOR` writer"]
pub type W = crate::W<McanerrVectorSpec>;
#[doc = "Field `MCANERR_VECTOR_ECC_VECTOR` reader - ECC RAM ID. Each error detection and correction (EDC) controller has a bank of error registers (offsets 0x10 - 0x3B) associated with it. These registers are accessed via an internal serial bus (SVBUS). To access them through the ECC aggregator the controller ID desired must be written to the ECC_VECTOR field, together with the RD_SVBUS trigger and RD_SVBUS_ADDRESS bit field. This initiates the serial read which consummates by setting the RD_SVBUS_DONE bit. At this point the addressed register may be read by a normal CPU read of the appropriate offset address. 0x000 Message RAM ECC controller is selected Others Reserved (do not use) Subsequent writes through the SVBUS (offsets 0x10 - 0x3B) have a delayed completion. To avoid conflicts, perform a read back of a register within this range after writing."]
pub type McanerrVectorEccVectorR = crate::FieldReader<u16>;
#[doc = "Field `MCANERR_VECTOR_ECC_VECTOR` writer - ECC RAM ID. Each error detection and correction (EDC) controller has a bank of error registers (offsets 0x10 - 0x3B) associated with it. These registers are accessed via an internal serial bus (SVBUS). To access them through the ECC aggregator the controller ID desired must be written to the ECC_VECTOR field, together with the RD_SVBUS trigger and RD_SVBUS_ADDRESS bit field. This initiates the serial read which consummates by setting the RD_SVBUS_DONE bit. At this point the addressed register may be read by a normal CPU read of the appropriate offset address. 0x000 Message RAM ECC controller is selected Others Reserved (do not use) Subsequent writes through the SVBUS (offsets 0x10 - 0x3B) have a delayed completion. To avoid conflicts, perform a read back of a register within this range after writing."]
pub type McanerrVectorEccVectorW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MCANERR_VECTOR_RD_SVBUS` reader - Read Trigger"]
pub type McanerrVectorRdSvbusR = crate::BitReader;
#[doc = "Field `MCANERR_VECTOR_RD_SVBUS` writer - Read Trigger"]
pub type McanerrVectorRdSvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANERR_VECTOR_RD_SVBUS_ADDRESS` reader - Read Address Offset"]
pub type McanerrVectorRdSvbusAddressR = crate::FieldReader;
#[doc = "Field `MCANERR_VECTOR_RD_SVBUS_ADDRESS` writer - Read Address Offset"]
pub type McanerrVectorRdSvbusAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MCANERR_VECTOR_RD_SVBUS_DONE` reader - Read Completion Flag"]
pub type McanerrVectorRdSvbusDoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - ECC RAM ID. Each error detection and correction (EDC) controller has a bank of error registers (offsets 0x10 - 0x3B) associated with it. These registers are accessed via an internal serial bus (SVBUS). To access them through the ECC aggregator the controller ID desired must be written to the ECC_VECTOR field, together with the RD_SVBUS trigger and RD_SVBUS_ADDRESS bit field. This initiates the serial read which consummates by setting the RD_SVBUS_DONE bit. At this point the addressed register may be read by a normal CPU read of the appropriate offset address. 0x000 Message RAM ECC controller is selected Others Reserved (do not use) Subsequent writes through the SVBUS (offsets 0x10 - 0x3B) have a delayed completion. To avoid conflicts, perform a read back of a register within this range after writing."]
    #[inline(always)]
    pub fn mcanerr_vector_ecc_vector(&self) -> McanerrVectorEccVectorR {
        McanerrVectorEccVectorR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Read Trigger"]
    #[inline(always)]
    pub fn mcanerr_vector_rd_svbus(&self) -> McanerrVectorRdSvbusR {
        McanerrVectorRdSvbusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Read Address Offset"]
    #[inline(always)]
    pub fn mcanerr_vector_rd_svbus_address(&self) -> McanerrVectorRdSvbusAddressR {
        McanerrVectorRdSvbusAddressR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Read Completion Flag"]
    #[inline(always)]
    pub fn mcanerr_vector_rd_svbus_done(&self) -> McanerrVectorRdSvbusDoneR {
        McanerrVectorRdSvbusDoneR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - ECC RAM ID. Each error detection and correction (EDC) controller has a bank of error registers (offsets 0x10 - 0x3B) associated with it. These registers are accessed via an internal serial bus (SVBUS). To access them through the ECC aggregator the controller ID desired must be written to the ECC_VECTOR field, together with the RD_SVBUS trigger and RD_SVBUS_ADDRESS bit field. This initiates the serial read which consummates by setting the RD_SVBUS_DONE bit. At this point the addressed register may be read by a normal CPU read of the appropriate offset address. 0x000 Message RAM ECC controller is selected Others Reserved (do not use) Subsequent writes through the SVBUS (offsets 0x10 - 0x3B) have a delayed completion. To avoid conflicts, perform a read back of a register within this range after writing."]
    #[inline(always)]
    pub fn mcanerr_vector_ecc_vector(&mut self) -> McanerrVectorEccVectorW<McanerrVectorSpec> {
        McanerrVectorEccVectorW::new(self, 0)
    }
    #[doc = "Bit 15 - Read Trigger"]
    #[inline(always)]
    pub fn mcanerr_vector_rd_svbus(&mut self) -> McanerrVectorRdSvbusW<McanerrVectorSpec> {
        McanerrVectorRdSvbusW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Read Address Offset"]
    #[inline(always)]
    pub fn mcanerr_vector_rd_svbus_address(
        &mut self,
    ) -> McanerrVectorRdSvbusAddressW<McanerrVectorSpec> {
        McanerrVectorRdSvbusAddressW::new(self, 16)
    }
}
#[doc = "MCAN ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_vector::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_vector::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrVectorSpec;
impl crate::RegisterSpec for McanerrVectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_vector::R`](R) reader structure"]
impl crate::Readable for McanerrVectorSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_vector::W`](W) writer structure"]
impl crate::Writable for McanerrVectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_VECTOR to value 0"]
impl crate::Resettable for McanerrVectorSpec {
    const RESET_VALUE: u32 = 0;
}
