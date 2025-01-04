#[doc = "Register `MCAN_TSCV` reader"]
pub type R = crate::R<McanTscvSpec>;
#[doc = "Register `MCAN_TSCV` writer"]
pub type W = crate::W<McanTscvSpec>;
#[doc = "Field `MCAN_TSCV_TSC` reader - Timestamp Counter. The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = &amp;quot;01&amp;quot;, the Timestamp Counter is incremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = &amp;quot;10&amp;quot;, TSC reflects the External Timestamp Counter value, and a write access has no impact. Note: A &amp;quot;wrap around&amp;quot; is a change of the Timestamp Counter value from non-zero to zero not caused by write access to MCAN_TSCV."]
pub type McanTscvTscR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_TSCV_TSC` writer - Timestamp Counter. The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = &amp;quot;01&amp;quot;, the Timestamp Counter is incremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = &amp;quot;10&amp;quot;, TSC reflects the External Timestamp Counter value, and a write access has no impact. Note: A &amp;quot;wrap around&amp;quot; is a change of the Timestamp Counter value from non-zero to zero not caused by write access to MCAN_TSCV."]
pub type McanTscvTscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter. The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = &amp;quot;01&amp;quot;, the Timestamp Counter is incremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = &amp;quot;10&amp;quot;, TSC reflects the External Timestamp Counter value, and a write access has no impact. Note: A &amp;quot;wrap around&amp;quot; is a change of the Timestamp Counter value from non-zero to zero not caused by write access to MCAN_TSCV."]
    #[inline(always)]
    pub fn mcan_tscv_tsc(&self) -> McanTscvTscR {
        McanTscvTscR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Counter. The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = &amp;quot;01&amp;quot;, the Timestamp Counter is incremented in multiples of CAN bit times, (1...16), depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = &amp;quot;10&amp;quot;, TSC reflects the External Timestamp Counter value, and a write access has no impact. Note: A &amp;quot;wrap around&amp;quot; is a change of the Timestamp Counter value from non-zero to zero not caused by write access to MCAN_TSCV."]
    #[inline(always)]
    pub fn mcan_tscv_tsc(&mut self) -> McanTscvTscW<McanTscvSpec> {
        McanTscvTscW::new(self, 0)
    }
}
#[doc = "MCAN Timestamp Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTscvSpec;
impl crate::RegisterSpec for McanTscvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_tscv::R`](R) reader structure"]
impl crate::Readable for McanTscvSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_tscv::W`](W) writer structure"]
impl crate::Writable for McanTscvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TSCV to value 0"]
impl crate::Resettable for McanTscvSpec {
    const RESET_VALUE: u32 = 0;
}
