#[doc = "Register `MCAN_TSCC` reader"]
pub type R = crate::R<McanTsccSpec>;
#[doc = "Register `MCAN_TSCC` writer"]
pub type W = crate::W<McanTsccSpec>;
#[doc = "Field `MCAN_TSCC_TSS` reader - Timestamp Select 00 Timestamp counter value always 0x0000 01 Timestamp counter value incremented according to TCP 10 External timestamp counter value used 11 Same as &amp;quot;00&amp;quot; Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTsccTssR = crate::FieldReader;
#[doc = "Field `MCAN_TSCC_TSS` writer - Timestamp Select 00 Timestamp counter value always 0x0000 01 Timestamp counter value incremented according to TCP 10 External timestamp counter value used 11 Same as &amp;quot;00&amp;quot; Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTsccTssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCAN_TSCC_TCP` reader - Timestamp Counter Prescaler. Configures the timestamp and timeout counters time unit in multiples of CAN bit times. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Note: With CAN FD an external counter is required for timestamp generation (TSS = &amp;quot;10&amp;quot;). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTsccTcpR = crate::FieldReader;
#[doc = "Field `MCAN_TSCC_TCP` writer - Timestamp Counter Prescaler. Configures the timestamp and timeout counters time unit in multiples of CAN bit times. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Note: With CAN FD an external counter is required for timestamp generation (TSS = &amp;quot;10&amp;quot;). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTsccTcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timestamp Select 00 Timestamp counter value always 0x0000 01 Timestamp counter value incremented according to TCP 10 External timestamp counter value used 11 Same as &amp;quot;00&amp;quot; Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tscc_tss(&self) -> McanTsccTssR {
        McanTsccTssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler. Configures the timestamp and timeout counters time unit in multiples of CAN bit times. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Note: With CAN FD an external counter is required for timestamp generation (TSS = &amp;quot;10&amp;quot;). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tscc_tcp(&self) -> McanTsccTcpR {
        McanTsccTcpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select 00 Timestamp counter value always 0x0000 01 Timestamp counter value incremented according to TCP 10 External timestamp counter value used 11 Same as &amp;quot;00&amp;quot; Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tscc_tss(&mut self) -> McanTsccTssW<McanTsccSpec> {
        McanTsccTssW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler. Configures the timestamp and timeout counters time unit in multiples of CAN bit times. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Note: With CAN FD an external counter is required for timestamp generation (TSS = &amp;quot;10&amp;quot;). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tscc_tcp(&mut self) -> McanTsccTcpW<McanTsccSpec> {
        McanTsccTcpW::new(self, 16)
    }
}
#[doc = "MCAN Timestamp Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTsccSpec;
impl crate::RegisterSpec for McanTsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_tscc::R`](R) reader structure"]
impl crate::Readable for McanTsccSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_tscc::W`](W) writer structure"]
impl crate::Writable for McanTsccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TSCC to value 0"]
impl crate::Resettable for McanTsccSpec {
    const RESET_VALUE: u32 = 0;
}
