#[doc = "Register `MCAN_XIDAM` reader"]
pub type R = crate::R<McanXidamSpec>;
#[doc = "Register `MCAN_XIDAM` writer"]
pub type W = crate::W<McanXidamSpec>;
#[doc = "Field `MCAN_XIDAM_EIDM` reader - Extended ID Mask. For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanXidamEidmR = crate::FieldReader<u32>;
#[doc = "Field `MCAN_XIDAM_EIDM` writer - Extended ID Mask. For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanXidamEidmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask. For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_xidam_eidm(&self) -> McanXidamEidmR {
        McanXidamEidmR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask. For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_xidam_eidm(&mut self) -> McanXidamEidmW<McanXidamSpec> {
        McanXidamEidmW::new(self, 0)
    }
}
#[doc = "MCAN Extended ID and Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanXidamSpec;
impl crate::RegisterSpec for McanXidamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_xidam::R`](R) reader structure"]
impl crate::Readable for McanXidamSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_xidam::W`](W) writer structure"]
impl crate::Writable for McanXidamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for McanXidamSpec {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
