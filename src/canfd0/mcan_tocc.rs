#[doc = "Register `MCAN_TOCC` reader"]
pub type R = crate::R<McanToccSpec>;
#[doc = "Register `MCAN_TOCC` writer"]
pub type W = crate::W<McanToccSpec>;
#[doc = "Field `MCAN_TOCC_ETOC` reader - Enable Timeout Counter 0 Timeout Counter disabled 1 Timeout Counter enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanToccEtocR = crate::BitReader;
#[doc = "Field `MCAN_TOCC_ETOC` writer - Enable Timeout Counter 0 Timeout Counter disabled 1 Timeout Counter enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanToccEtocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TOCC_TOS` reader - Timeout Select. When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00 Continuous operation 01 Timeout controlled by Tx Event FIFO 10 Timeout controlled by Rx FIFO 0 11 Timeout controlled by Rx FIFO 1 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanToccTosR = crate::FieldReader;
#[doc = "Field `MCAN_TOCC_TOS` writer - Timeout Select. When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00 Continuous operation 01 Timeout controlled by Tx Event FIFO 10 Timeout controlled by Rx FIFO 0 11 Timeout controlled by Rx FIFO 1 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanToccTosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCAN_TOCC_TOP` reader - Timeout Period. Start value of the Timeout Counter (down-counter). Configures the Timeout Period. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanToccTopR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_TOCC_TOP` writer - Timeout Period. Start value of the Timeout Counter (down-counter). Configures the Timeout Period. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanToccTopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter 0 Timeout Counter disabled 1 Timeout Counter enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tocc_etoc(&self) -> McanToccEtocR {
        McanToccEtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select. When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00 Continuous operation 01 Timeout controlled by Tx Event FIFO 10 Timeout controlled by Rx FIFO 0 11 Timeout controlled by Rx FIFO 1 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tocc_tos(&self) -> McanToccTosR {
        McanToccTosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period. Start value of the Timeout Counter (down-counter). Configures the Timeout Period. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tocc_top(&self) -> McanToccTopR {
        McanToccTopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter 0 Timeout Counter disabled 1 Timeout Counter enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tocc_etoc(&mut self) -> McanToccEtocW<McanToccSpec> {
        McanToccEtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout Select. When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00 Continuous operation 01 Timeout controlled by Tx Event FIFO 10 Timeout controlled by Rx FIFO 0 11 Timeout controlled by Rx FIFO 1 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tocc_tos(&mut self) -> McanToccTosW<McanToccSpec> {
        McanToccTosW::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout Period. Start value of the Timeout Counter (down-counter). Configures the Timeout Period. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_tocc_top(&mut self) -> McanToccTopW<McanToccSpec> {
        McanToccTopW::new(self, 16)
    }
}
#[doc = "MCAN Timeout Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanToccSpec;
impl crate::RegisterSpec for McanToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_tocc::R`](R) reader structure"]
impl crate::Readable for McanToccSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_tocc::W`](W) writer structure"]
impl crate::Writable for McanToccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TOCC to value 0xffff_0000"]
impl crate::Resettable for McanToccSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
