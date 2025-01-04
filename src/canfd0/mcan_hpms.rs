#[doc = "Register `MCAN_HPMS` reader"]
pub type R = crate::R<McanHpmsSpec>;
#[doc = "Field `MCAN_HPMS_BIDX` reader - Buffer Index. Index of Rx FIFO element to which the message was stored. Only valid when MSI(1) = '1'."]
pub type McanHpmsBidxR = crate::FieldReader;
#[doc = "Field `MCAN_HPMS_MSI` reader - Message Storage Indicator 00 No FIFO selected 01 FIFO message lost 10 Message stored in FIFO 0 11 Message stored in FIFO 1"]
pub type McanHpmsMsiR = crate::FieldReader;
#[doc = "Field `MCAN_HPMS_FIDX` reader - Filter Index. Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
pub type McanHpmsFidxR = crate::FieldReader;
#[doc = "Field `MCAN_HPMS_FLST` reader - Filter List. Indicates the filter list of the matching filter element. 0 Standard Filter List 1 Extended Filter List"]
pub type McanHpmsFlstR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Buffer Index. Index of Rx FIFO element to which the message was stored. Only valid when MSI(1) = '1'."]
    #[inline(always)]
    pub fn mcan_hpms_bidx(&self) -> McanHpmsBidxR {
        McanHpmsBidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator 00 No FIFO selected 01 FIFO message lost 10 Message stored in FIFO 0 11 Message stored in FIFO 1"]
    #[inline(always)]
    pub fn mcan_hpms_msi(&self) -> McanHpmsMsiR {
        McanHpmsMsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index. Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
    #[inline(always)]
    pub fn mcan_hpms_fidx(&self) -> McanHpmsFidxR {
        McanHpmsFidxR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List. Indicates the filter list of the matching filter element. 0 Standard Filter List 1 Extended Filter List"]
    #[inline(always)]
    pub fn mcan_hpms_flst(&self) -> McanHpmsFlstR {
        McanHpmsFlstR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "MCAN High Priority Message Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanHpmsSpec;
impl crate::RegisterSpec for McanHpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_hpms::R`](R) reader structure"]
impl crate::Readable for McanHpmsSpec {}
#[doc = "`reset()` method sets MCAN_HPMS to value 0"]
impl crate::Resettable for McanHpmsSpec {
    const RESET_VALUE: u32 = 0;
}
