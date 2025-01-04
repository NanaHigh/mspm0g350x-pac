#[doc = "Register `MCANSS_EOI` reader"]
pub type R = crate::R<McanssEoiSpec>;
#[doc = "Register `MCANSS_EOI` writer"]
pub type W = crate::W<McanssEoiSpec>;
#[doc = "Field `MCANSS_EOI_EOI` reader - End of Interrupt. A write to this register will clear the associated interrupt. If the unserviced interrupt counter is &gt; 1, another interrupt is generated. 0x00 External TS Interrupt is cleared 0x01 MCAN(0) interrupt is cleared 0x02 MCAN(1) interrupt is cleared Other writes are ignored."]
pub type McanssEoiEoiR = crate::FieldReader;
#[doc = "Field `MCANSS_EOI_EOI` writer - End of Interrupt. A write to this register will clear the associated interrupt. If the unserviced interrupt counter is &gt; 1, another interrupt is generated. 0x00 External TS Interrupt is cleared 0x01 MCAN(0) interrupt is cleared 0x02 MCAN(1) interrupt is cleared Other writes are ignored."]
pub type McanssEoiEoiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - End of Interrupt. A write to this register will clear the associated interrupt. If the unserviced interrupt counter is &gt; 1, another interrupt is generated. 0x00 External TS Interrupt is cleared 0x01 MCAN(0) interrupt is cleared 0x02 MCAN(1) interrupt is cleared Other writes are ignored."]
    #[inline(always)]
    pub fn mcanss_eoi_eoi(&self) -> McanssEoiEoiR {
        McanssEoiEoiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End of Interrupt. A write to this register will clear the associated interrupt. If the unserviced interrupt counter is &gt; 1, another interrupt is generated. 0x00 External TS Interrupt is cleared 0x01 MCAN(0) interrupt is cleared 0x02 MCAN(1) interrupt is cleared Other writes are ignored."]
    #[inline(always)]
    pub fn mcanss_eoi_eoi(&mut self) -> McanssEoiEoiW<McanssEoiSpec> {
        McanssEoiEoiW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem End of Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssEoiSpec;
impl crate::RegisterSpec for McanssEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_eoi::R`](R) reader structure"]
impl crate::Readable for McanssEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_eoi::W`](W) writer structure"]
impl crate::Writable for McanssEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_EOI to value 0"]
impl crate::Resettable for McanssEoiSpec {
    const RESET_VALUE: u32 = 0;
}
