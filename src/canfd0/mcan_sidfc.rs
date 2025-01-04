#[doc = "Register `MCAN_SIDFC` reader"]
pub type R = crate::R<McanSidfcSpec>;
#[doc = "Register `MCAN_SIDFC` writer"]
pub type W = crate::W<McanSidfcSpec>;
#[doc = "Field `MCAN_SIDFC_FLSSA` reader - Filter List Standard Start Address. Start address of standard Message ID filter list (32-bit word address)."]
pub type McanSidfcFlssaR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_SIDFC_FLSSA` writer - Filter List Standard Start Address. Start address of standard Message ID filter list (32-bit word address)."]
pub type McanSidfcFlssaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MCAN_SIDFC_LSS` reader - List Size Standard 0 No standard Message ID filter 1-128 Number of standard Message ID filter elements &gt;128 Values greater than 128 are interpreted as 128"]
pub type McanSidfcLssR = crate::FieldReader;
#[doc = "Field `MCAN_SIDFC_LSS` writer - List Size Standard 0 No standard Message ID filter 1-128 Number of standard Message ID filter elements &gt;128 Values greater than 128 are interpreted as 128"]
pub type McanSidfcLssW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - Filter List Standard Start Address. Start address of standard Message ID filter list (32-bit word address)."]
    #[inline(always)]
    pub fn mcan_sidfc_flssa(&self) -> McanSidfcFlssaR {
        McanSidfcFlssaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Standard 0 No standard Message ID filter 1-128 Number of standard Message ID filter elements &gt;128 Values greater than 128 are interpreted as 128"]
    #[inline(always)]
    pub fn mcan_sidfc_lss(&self) -> McanSidfcLssR {
        McanSidfcLssR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Standard Start Address. Start address of standard Message ID filter list (32-bit word address)."]
    #[inline(always)]
    pub fn mcan_sidfc_flssa(&mut self) -> McanSidfcFlssaW<McanSidfcSpec> {
        McanSidfcFlssaW::new(self, 2)
    }
    #[doc = "Bits 16:23 - List Size Standard 0 No standard Message ID filter 1-128 Number of standard Message ID filter elements &gt;128 Values greater than 128 are interpreted as 128"]
    #[inline(always)]
    pub fn mcan_sidfc_lss(&mut self) -> McanSidfcLssW<McanSidfcSpec> {
        McanSidfcLssW::new(self, 16)
    }
}
#[doc = "MCAN Standard ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_sidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_sidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanSidfcSpec;
impl crate::RegisterSpec for McanSidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_sidfc::R`](R) reader structure"]
impl crate::Readable for McanSidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_sidfc::W`](W) writer structure"]
impl crate::Writable for McanSidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_SIDFC to value 0"]
impl crate::Resettable for McanSidfcSpec {
    const RESET_VALUE: u32 = 0;
}
