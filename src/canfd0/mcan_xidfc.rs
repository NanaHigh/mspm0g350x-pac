#[doc = "Register `MCAN_XIDFC` reader"]
pub type R = crate::R<McanXidfcSpec>;
#[doc = "Register `MCAN_XIDFC` writer"]
pub type W = crate::W<McanXidfcSpec>;
#[doc = "Field `MCAN_XIDFC_FLESA` reader - List Size Extended 0 No extended Message ID filter 1-64 Number of extended Message ID filter elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanXidfcFlesaR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_XIDFC_FLESA` writer - List Size Extended 0 No extended Message ID filter 1-64 Number of extended Message ID filter elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanXidfcFlesaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MCAN_XIDFC_LSE` reader - Filter List Extended Start Address. Start address of extended Message ID filter list (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanXidfcLseR = crate::FieldReader;
#[doc = "Field `MCAN_XIDFC_LSE` writer - Filter List Extended Start Address. Start address of extended Message ID filter list (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanXidfcLseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 2:15 - List Size Extended 0 No extended Message ID filter 1-64 Number of extended Message ID filter elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_xidfc_flesa(&self) -> McanXidfcFlesaR {
        McanXidfcFlesaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Filter List Extended Start Address. Start address of extended Message ID filter list (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_xidfc_lse(&self) -> McanXidfcLseR {
        McanXidfcLseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - List Size Extended 0 No extended Message ID filter 1-64 Number of extended Message ID filter elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_xidfc_flesa(&mut self) -> McanXidfcFlesaW<McanXidfcSpec> {
        McanXidfcFlesaW::new(self, 2)
    }
    #[doc = "Bits 16:22 - Filter List Extended Start Address. Start address of extended Message ID filter list (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_xidfc_lse(&mut self) -> McanXidfcLseW<McanXidfcSpec> {
        McanXidfcLseW::new(self, 16)
    }
}
#[doc = "MCAN Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_xidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_xidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanXidfcSpec;
impl crate::RegisterSpec for McanXidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_xidfc::R`](R) reader structure"]
impl crate::Readable for McanXidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_xidfc::W`](W) writer structure"]
impl crate::Writable for McanXidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_XIDFC to value 0"]
impl crate::Resettable for McanXidfcSpec {
    const RESET_VALUE: u32 = 0;
}
