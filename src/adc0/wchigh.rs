#[doc = "Register `WCHIGH` reader"]
pub type R = crate::R<WchighSpec>;
#[doc = "Register `WCHIGH` writer"]
pub type W = crate::W<WchighSpec>;
#[doc = "Field `WCHIGH_DATA` reader - If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
pub type WchighDataR = crate::FieldReader<u16>;
#[doc = "Field `WCHIGH_DATA` writer - If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
pub type WchighDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
    #[inline(always)]
    pub fn wchigh_data(&self) -> WchighDataR {
        WchighDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If DF = 0, unsigned binary format has to be used. The threshold value has to be right aligned, with the MSB on the left. For 10-bits and 8-bits resolution, unused bit have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bit have to be 0s."]
    #[inline(always)]
    pub fn wchigh_data(&mut self) -> WchighDataW<WchighSpec> {
        WchighDataW::new(self, 0)
    }
}
#[doc = "Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wchigh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wchigh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WchighSpec;
impl crate::RegisterSpec for WchighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wchigh::R`](R) reader structure"]
impl crate::Readable for WchighSpec {}
#[doc = "`write(|w| ..)` method takes [`wchigh::W`](W) writer structure"]
impl crate::Writable for WchighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WCHIGH to value 0"]
impl crate::Resettable for WchighSpec {
    const RESET_VALUE: u32 = 0;
}
