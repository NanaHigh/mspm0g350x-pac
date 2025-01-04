#[doc = "Register `MCAN_TOCV` reader"]
pub type R = crate::R<McanTocvSpec>;
#[doc = "Register `MCAN_TOCV` writer"]
pub type W = crate::W<McanTocvSpec>;
#[doc = "Field `MCAN_TOCV_TOC` reader - Timeout Counter. The Timeout Counter is decremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
pub type McanTocvTocR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_TOCV_TOC` writer - Timeout Counter. The Timeout Counter is decremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
pub type McanTocvTocW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter. The Timeout Counter is decremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
    #[inline(always)]
    pub fn mcan_tocv_toc(&self) -> McanTocvTocR {
        McanTocvTocR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter. The Timeout Counter is decremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
    #[inline(always)]
    pub fn mcan_tocv_toc(&mut self) -> McanTocvTocW<McanTocvSpec> {
        McanTocvTocW::new(self, 0)
    }
}
#[doc = "MCAN Timeout Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tocv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tocv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTocvSpec;
impl crate::RegisterSpec for McanTocvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_tocv::R`](R) reader structure"]
impl crate::Readable for McanTocvSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_tocv::W`](W) writer structure"]
impl crate::Writable for McanTocvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TOCV to value 0xffff"]
impl crate::Resettable for McanTocvSpec {
    const RESET_VALUE: u32 = 0xffff;
}
