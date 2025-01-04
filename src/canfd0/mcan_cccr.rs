#[doc = "Register `MCAN_CCCR` reader"]
pub type R = crate::R<McanCccrSpec>;
#[doc = "Register `MCAN_CCCR` writer"]
pub type W = crate::W<McanCccrSpec>;
#[doc = "Field `MCAN_CCCR_INIT` reader - Initialization 0 Normal Operation 1 Initialization is started Note: Due to the synchronization mechanism between the two clock domains, there may be a delay until the value written to INIT can be read back. Therefore the programmer has to assure that the previous value written to INIT has been accepted by reading INIT before setting INIT to a new value."]
pub type McanCccrInitR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_INIT` writer - Initialization 0 Normal Operation 1 Initialization is started Note: Due to the synchronization mechanism between the two clock domains, there may be a delay until the value written to INIT can be read back. Therefore the programmer has to assure that the previous value written to INIT has been accepted by reading INIT before setting INIT to a new value."]
pub type McanCccrInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_CCE` reader - Configuration Change Enable 0 The CPU has no write access to the protected configuration registers 1 The CPU has write access to the protected configuration registers (while CCCR.INIT = '1') Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrCceR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_CCE` writer - Configuration Change Enable 0 The CPU has no write access to the protected configuration registers 1 The CPU has write access to the protected configuration registers (while CCCR.INIT = '1') Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrCceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_ASM` reader - Restricted Operation Mode. Bit ASM can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Normal CAN operation 1 Restricted Operation Mode active Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrAsmR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_ASM` writer - Restricted Operation Mode. Bit ASM can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Normal CAN operation 1 Restricted Operation Mode active Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrAsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_CSA` reader - Clock Stop Acknowledge 0 No clock stop acknowledged 1 MCAN may be set in power down by stopping the Host and CAN clocks"]
pub type McanCccrCsaR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_CSR` reader - Clock Stop Request 0 No clock stop is requested 1 Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
pub type McanCccrCsrR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_CSR` writer - Clock Stop Request 0 No clock stop is requested 1 Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
pub type McanCccrCsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_MON` reader - Bus Monitoring Mode. Bit MON can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Bus Monitoring Mode is disabled 1 Bus Monitoring Mode is enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrMonR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_MON` writer - Bus Monitoring Mode. Bit MON can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Bus Monitoring Mode is disabled 1 Bus Monitoring Mode is enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrMonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_DAR` reader - Disable Automatic Retransmission 0 Automatic retransmission of messages not transmitted successfully enabled 1 Automatic retransmission disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrDarR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_DAR` writer - Disable Automatic Retransmission 0 Automatic retransmission of messages not transmitted successfully enabled 1 Automatic retransmission disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrDarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_TEST` reader - Test Mode Enable 0 Normal operation, register TEST holds reset values 1 Test Mode, write access to register TEST enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrTestR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_TEST` writer - Test Mode Enable 0 Normal operation, register TEST holds reset values 1 Test Mode, write access to register TEST enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_FDOE` reader - Flexible Datarate Operation Enable 0 FD operation disabled 1 FD operation enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrFdoeR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_FDOE` writer - Flexible Datarate Operation Enable 0 FD operation disabled 1 FD operation enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrFdoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_BRSE` reader - Bit Rate Switch Enable 0 Bit rate switching for transmissions disabled 1 Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = '0', BRSE is not evaluated. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrBrseR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_BRSE` writer - Bit Rate Switch Enable 0 Bit rate switching for transmissions disabled 1 Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = '0', BRSE is not evaluated. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrBrseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_PXHD` reader - Protocol Exception Handling Disable 0 Protocol exception handling enabled 1 Protocol exception handling disabled Note: When protocol exception handling is disabled, the MCAN will transmit an error frame when it detects a protocol exception condition. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrPxhdR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_PXHD` writer - Protocol Exception Handling Disable 0 Protocol exception handling enabled 1 Protocol exception handling disabled Note: When protocol exception handling is disabled, the MCAN will transmit an error frame when it detects a protocol exception condition. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrPxhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_EFBI` reader - Edge Filtering during Bus Integration 0 Edge filtering disabled 1 Two consecutive dominant tq required to detect an edge for hard synchronization Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrEfbiR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_EFBI` writer - Edge Filtering during Bus Integration 0 Edge filtering disabled 1 Two consecutive dominant tq required to detect an edge for hard synchronization Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrEfbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_TXP` reader - Transmit Pause. If this bit is set, the MCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame. 0 Transmit pause disabled 1 Transmit pause enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrTxpR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_TXP` writer - Transmit Pause. If this bit is set, the MCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame. 0 Transmit pause disabled 1 Transmit pause enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanCccrTxpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_CCCR_NISO` reader - Non ISO Operation. If this bit is set, the MCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0 CAN FD frame format according to ISO 11898-1:2015 1 CAN FD frame format according to Bosch CAN FD Specification V1.0"]
pub type McanCccrNisoR = crate::BitReader;
#[doc = "Field `MCAN_CCCR_NISO` writer - Non ISO Operation. If this bit is set, the MCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0 CAN FD frame format according to ISO 11898-1:2015 1 CAN FD frame format according to Bosch CAN FD Specification V1.0"]
pub type McanCccrNisoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialization 0 Normal Operation 1 Initialization is started Note: Due to the synchronization mechanism between the two clock domains, there may be a delay until the value written to INIT can be read back. Therefore the programmer has to assure that the previous value written to INIT has been accepted by reading INIT before setting INIT to a new value."]
    #[inline(always)]
    pub fn mcan_cccr_init(&self) -> McanCccrInitR {
        McanCccrInitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable 0 The CPU has no write access to the protected configuration registers 1 The CPU has write access to the protected configuration registers (while CCCR.INIT = '1') Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_cce(&self) -> McanCccrCceR {
        McanCccrCceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restricted Operation Mode. Bit ASM can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Normal CAN operation 1 Restricted Operation Mode active Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_asm(&self) -> McanCccrAsmR {
        McanCccrAsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge 0 No clock stop acknowledged 1 MCAN may be set in power down by stopping the Host and CAN clocks"]
    #[inline(always)]
    pub fn mcan_cccr_csa(&self) -> McanCccrCsaR {
        McanCccrCsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request 0 No clock stop is requested 1 Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn mcan_cccr_csr(&self) -> McanCccrCsrR {
        McanCccrCsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode. Bit MON can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Bus Monitoring Mode is disabled 1 Bus Monitoring Mode is enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_mon(&self) -> McanCccrMonR {
        McanCccrMonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission 0 Automatic retransmission of messages not transmitted successfully enabled 1 Automatic retransmission disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_dar(&self) -> McanCccrDarR {
        McanCccrDarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable 0 Normal operation, register TEST holds reset values 1 Test Mode, write access to register TEST enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_test(&self) -> McanCccrTestR {
        McanCccrTestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flexible Datarate Operation Enable 0 FD operation disabled 1 FD operation enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_fdoe(&self) -> McanCccrFdoeR {
        McanCccrFdoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable 0 Bit rate switching for transmissions disabled 1 Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = '0', BRSE is not evaluated. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_brse(&self) -> McanCccrBrseR {
        McanCccrBrseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable 0 Protocol exception handling enabled 1 Protocol exception handling disabled Note: When protocol exception handling is disabled, the MCAN will transmit an error frame when it detects a protocol exception condition. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_pxhd(&self) -> McanCccrPxhdR {
        McanCccrPxhdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration 0 Edge filtering disabled 1 Two consecutive dominant tq required to detect an edge for hard synchronization Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_efbi(&self) -> McanCccrEfbiR {
        McanCccrEfbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause. If this bit is set, the MCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame. 0 Transmit pause disabled 1 Transmit pause enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_txp(&self) -> McanCccrTxpR {
        McanCccrTxpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO Operation. If this bit is set, the MCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0 CAN FD frame format according to ISO 11898-1:2015 1 CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn mcan_cccr_niso(&self) -> McanCccrNisoR {
        McanCccrNisoR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization 0 Normal Operation 1 Initialization is started Note: Due to the synchronization mechanism between the two clock domains, there may be a delay until the value written to INIT can be read back. Therefore the programmer has to assure that the previous value written to INIT has been accepted by reading INIT before setting INIT to a new value."]
    #[inline(always)]
    pub fn mcan_cccr_init(&mut self) -> McanCccrInitW<McanCccrSpec> {
        McanCccrInitW::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable 0 The CPU has no write access to the protected configuration registers 1 The CPU has write access to the protected configuration registers (while CCCR.INIT = '1') Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_cce(&mut self) -> McanCccrCceW<McanCccrSpec> {
        McanCccrCceW::new(self, 1)
    }
    #[doc = "Bit 2 - Restricted Operation Mode. Bit ASM can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Normal CAN operation 1 Restricted Operation Mode active Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_asm(&mut self) -> McanCccrAsmW<McanCccrSpec> {
        McanCccrAsmW::new(self, 2)
    }
    #[doc = "Bit 4 - Clock Stop Request 0 No clock stop is requested 1 Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn mcan_cccr_csr(&mut self) -> McanCccrCsrW<McanCccrSpec> {
        McanCccrCsrW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode. Bit MON can only be set by SW when both CCE and INIT are set to '1'. The bit can be reset by SW at any time. 0 Bus Monitoring Mode is disabled 1 Bus Monitoring Mode is enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_mon(&mut self) -> McanCccrMonW<McanCccrSpec> {
        McanCccrMonW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission 0 Automatic retransmission of messages not transmitted successfully enabled 1 Automatic retransmission disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_dar(&mut self) -> McanCccrDarW<McanCccrSpec> {
        McanCccrDarW::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Enable 0 Normal operation, register TEST holds reset values 1 Test Mode, write access to register TEST enabled Qualified Write 1 to Set is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_test(&mut self) -> McanCccrTestW<McanCccrSpec> {
        McanCccrTestW::new(self, 7)
    }
    #[doc = "Bit 8 - Flexible Datarate Operation Enable 0 FD operation disabled 1 FD operation enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_fdoe(&mut self) -> McanCccrFdoeW<McanCccrSpec> {
        McanCccrFdoeW::new(self, 8)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable 0 Bit rate switching for transmissions disabled 1 Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = '0', BRSE is not evaluated. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_brse(&mut self) -> McanCccrBrseW<McanCccrSpec> {
        McanCccrBrseW::new(self, 9)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable 0 Protocol exception handling enabled 1 Protocol exception handling disabled Note: When protocol exception handling is disabled, the MCAN will transmit an error frame when it detects a protocol exception condition. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_pxhd(&mut self) -> McanCccrPxhdW<McanCccrSpec> {
        McanCccrPxhdW::new(self, 12)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration 0 Edge filtering disabled 1 Two consecutive dominant tq required to detect an edge for hard synchronization Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_efbi(&mut self) -> McanCccrEfbiW<McanCccrSpec> {
        McanCccrEfbiW::new(self, 13)
    }
    #[doc = "Bit 14 - Transmit Pause. If this bit is set, the MCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame. 0 Transmit pause disabled 1 Transmit pause enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_cccr_txp(&mut self) -> McanCccrTxpW<McanCccrSpec> {
        McanCccrTxpW::new(self, 14)
    }
    #[doc = "Bit 15 - Non ISO Operation. If this bit is set, the MCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0 CAN FD frame format according to ISO 11898-1:2015 1 CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn mcan_cccr_niso(&mut self) -> McanCccrNisoW<McanCccrSpec> {
        McanCccrNisoW::new(self, 15)
    }
}
#[doc = "MCAN CC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanCccrSpec;
impl crate::RegisterSpec for McanCccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_cccr::R`](R) reader structure"]
impl crate::Readable for McanCccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_cccr::W`](W) writer structure"]
impl crate::Writable for McanCccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_CCCR to value 0x01"]
impl crate::Resettable for McanCccrSpec {
    const RESET_VALUE: u32 = 0x01;
}
