#[doc = "Register `MCAN_ECR` reader"]
pub type R = crate::R<McanEcrSpec>;
#[doc = "Field `MCAN_ECR_TEC` reader - Transmit Error Counter. Actual state of the Transmit Error Counter, values between 0 and 255. Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type McanEcrTecR = crate::FieldReader;
#[doc = "Field `MCAN_ECR_REC` reader - Receive Error Counter. Actual state of the Receive Error Counter, values between 0 and 127. Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type McanEcrRecR = crate::FieldReader;
#[doc = "Field `MCAN_ECR_RP` reader - Receive Error Passive 0 The Receive Error Counter is below the error passive level of 128 1 The Receive Error Counter has reached the error passive level of 128"]
pub type McanEcrRpR = crate::BitReader;
#[doc = "Field `MCAN_ECR_CEL` reader - CAN Error Logging. The counter is incremented each time when a CAN protocol error causes the Transmit Error Counter or the Receive Error Counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO. Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type McanEcrCelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter. Actual state of the Transmit Error Counter, values between 0 and 255. Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn mcan_ecr_tec(&self) -> McanEcrTecR {
        McanEcrTecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter. Actual state of the Receive Error Counter, values between 0 and 127. Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn mcan_ecr_rec(&self) -> McanEcrRecR {
        McanEcrRecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive 0 The Receive Error Counter is below the error passive level of 128 1 The Receive Error Counter has reached the error passive level of 128"]
    #[inline(always)]
    pub fn mcan_ecr_rp(&self) -> McanEcrRpR {
        McanEcrRpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN Error Logging. The counter is incremented each time when a CAN protocol error causes the Transmit Error Counter or the Receive Error Counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO. Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn mcan_ecr_cel(&self) -> McanEcrCelR {
        McanEcrCelR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "MCAN Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanEcrSpec;
impl crate::RegisterSpec for McanEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_ecr::R`](R) reader structure"]
impl crate::Readable for McanEcrSpec {}
#[doc = "`reset()` method sets MCAN_ECR to value 0"]
impl crate::Resettable for McanEcrSpec {
    const RESET_VALUE: u32 = 0;
}
