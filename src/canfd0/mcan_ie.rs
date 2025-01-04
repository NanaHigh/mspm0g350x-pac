#[doc = "Register `MCAN_IE` reader"]
pub type R = crate::R<McanIeSpec>;
#[doc = "Register `MCAN_IE` writer"]
pub type W = crate::W<McanIeSpec>;
#[doc = "Field `MCAN_IE_RF0NE` reader - Rx FIFO 0 New Message Enable"]
pub type McanIeRf0neR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF0NE` writer - Rx FIFO 0 New Message Enable"]
pub type McanIeRf0neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_RF0WE` reader - Rx FIFO 0 Watermark Reached Enable"]
pub type McanIeRf0weR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF0WE` writer - Rx FIFO 0 Watermark Reached Enable"]
pub type McanIeRf0weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_RF0FE` reader - Rx FIFO 0 Full Enable"]
pub type McanIeRf0feR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF0FE` writer - Rx FIFO 0 Full Enable"]
pub type McanIeRf0feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_RF0LE` reader - Rx FIFO 0 Message Lost Enable"]
pub type McanIeRf0leR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF0LE` writer - Rx FIFO 0 Message Lost Enable"]
pub type McanIeRf0leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_RF1NE` reader - Rx FIFO 1 New Message Enable"]
pub type McanIeRf1neR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF1NE` writer - Rx FIFO 1 New Message Enable"]
pub type McanIeRf1neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_RF1WE` reader - Rx FIFO 1 Watermark Reached Enable"]
pub type McanIeRf1weR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF1WE` writer - Rx FIFO 1 Watermark Reached Enable"]
pub type McanIeRf1weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_RF1FE` reader - Rx FIFO 1 Full Enable"]
pub type McanIeRf1feR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF1FE` writer - Rx FIFO 1 Full Enable"]
pub type McanIeRf1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_RF1LE` reader - Rx FIFO 1 Message Lost Enable"]
pub type McanIeRf1leR = crate::BitReader;
#[doc = "Field `MCAN_IE_RF1LE` writer - Rx FIFO 1 Message Lost Enable"]
pub type McanIeRf1leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_HPME` reader - High Priority Message Enable"]
pub type McanIeHpmeR = crate::BitReader;
#[doc = "Field `MCAN_IE_HPME` writer - High Priority Message Enable"]
pub type McanIeHpmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TCE` reader - Transmission Completed Enable"]
pub type McanIeTceR = crate::BitReader;
#[doc = "Field `MCAN_IE_TCE` writer - Transmission Completed Enable"]
pub type McanIeTceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TCFE` reader - Transmission Cancellation Finished Enable"]
pub type McanIeTcfeR = crate::BitReader;
#[doc = "Field `MCAN_IE_TCFE` writer - Transmission Cancellation Finished Enable"]
pub type McanIeTcfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TFEE` reader - Tx FIFO Empty Enable"]
pub type McanIeTfeeR = crate::BitReader;
#[doc = "Field `MCAN_IE_TFEE` writer - Tx FIFO Empty Enable"]
pub type McanIeTfeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TEFNE` reader - Tx Event FIFO New Entry Enable"]
pub type McanIeTefneR = crate::BitReader;
#[doc = "Field `MCAN_IE_TEFNE` writer - Tx Event FIFO New Entry Enable"]
pub type McanIeTefneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TEFWE` reader - Tx Event FIFO Watermark Reached Enable"]
pub type McanIeTefweR = crate::BitReader;
#[doc = "Field `MCAN_IE_TEFWE` writer - Tx Event FIFO Watermark Reached Enable"]
pub type McanIeTefweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TEFFE` reader - Tx Event FIFO Full Enable"]
pub type McanIeTeffeR = crate::BitReader;
#[doc = "Field `MCAN_IE_TEFFE` writer - Tx Event FIFO Full Enable"]
pub type McanIeTeffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TEFLE` reader - Tx Event FIFO Element Lost Enable"]
pub type McanIeTefleR = crate::BitReader;
#[doc = "Field `MCAN_IE_TEFLE` writer - Tx Event FIFO Element Lost Enable"]
pub type McanIeTefleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TSWE` reader - Timestamp Wraparound Enable"]
pub type McanIeTsweR = crate::BitReader;
#[doc = "Field `MCAN_IE_TSWE` writer - Timestamp Wraparound Enable"]
pub type McanIeTsweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_MRAFE` reader - Message RAM Access Failure Enable"]
pub type McanIeMrafeR = crate::BitReader;
#[doc = "Field `MCAN_IE_MRAFE` writer - Message RAM Access Failure Enable"]
pub type McanIeMrafeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_TOOE` reader - Timeout Occurred Enable"]
pub type McanIeTooeR = crate::BitReader;
#[doc = "Field `MCAN_IE_TOOE` writer - Timeout Occurred Enable"]
pub type McanIeTooeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_DRXE` reader - Message Stored to Dedicated Rx Buffer Enable"]
pub type McanIeDrxeR = crate::BitReader;
#[doc = "Field `MCAN_IE_DRXE` writer - Message Stored to Dedicated Rx Buffer Enable"]
pub type McanIeDrxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_BECE` reader - Bit Error Corrected Enable A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave this bit cleared to '0'."]
pub type McanIeBeceR = crate::BitReader;
#[doc = "Field `MCAN_IE_BECE` writer - Bit Error Corrected Enable A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave this bit cleared to '0'."]
pub type McanIeBeceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_BEUE` reader - Bit Error Uncorrected Enable"]
pub type McanIeBeueR = crate::BitReader;
#[doc = "Field `MCAN_IE_BEUE` writer - Bit Error Uncorrected Enable"]
pub type McanIeBeueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_ELOE` reader - Error Logging Overflow Enable"]
pub type McanIeEloeR = crate::BitReader;
#[doc = "Field `MCAN_IE_ELOE` writer - Error Logging Overflow Enable"]
pub type McanIeEloeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_EPE` reader - Error Passive Enable"]
pub type McanIeEpeR = crate::BitReader;
#[doc = "Field `MCAN_IE_EPE` writer - Error Passive Enable"]
pub type McanIeEpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_EWE` reader - Warning Status Enable"]
pub type McanIeEweR = crate::BitReader;
#[doc = "Field `MCAN_IE_EWE` writer - Warning Status Enable"]
pub type McanIeEweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_BOE` reader - Bus_Off Status Enable"]
pub type McanIeBoeR = crate::BitReader;
#[doc = "Field `MCAN_IE_BOE` writer - Bus_Off Status Enable"]
pub type McanIeBoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_WDIE` reader - Watchdog Interrupt Enable"]
pub type McanIeWdieR = crate::BitReader;
#[doc = "Field `MCAN_IE_WDIE` writer - Watchdog Interrupt Enable"]
pub type McanIeWdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_PEAE` reader - Protocol Error in Arbitration Phase Enable"]
pub type McanIePeaeR = crate::BitReader;
#[doc = "Field `MCAN_IE_PEAE` writer - Protocol Error in Arbitration Phase Enable"]
pub type McanIePeaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_PEDE` reader - Protocol Error in Data Phase Enable"]
pub type McanIePedeR = crate::BitReader;
#[doc = "Field `MCAN_IE_PEDE` writer - Protocol Error in Data Phase Enable"]
pub type McanIePedeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IE_ARAE` reader - Access to Reserved Address Enable"]
pub type McanIeAraeR = crate::BitReader;
#[doc = "Field `MCAN_IE_ARAE` writer - Access to Reserved Address Enable"]
pub type McanIeAraeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0ne(&self) -> McanIeRf0neR {
        McanIeRf0neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0we(&self) -> McanIeRf0weR {
        McanIeRf0weR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0fe(&self) -> McanIeRf0feR {
        McanIeRf0feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0le(&self) -> McanIeRf0leR {
        McanIeRf0leR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1ne(&self) -> McanIeRf1neR {
        McanIeRf1neR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1we(&self) -> McanIeRf1weR {
        McanIeRf1weR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1fe(&self) -> McanIeRf1feR {
        McanIeRf1feR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1le(&self) -> McanIeRf1leR {
        McanIeRf1leR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Enable"]
    #[inline(always)]
    pub fn mcan_ie_hpme(&self) -> McanIeHpmeR {
        McanIeHpmeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Enable"]
    #[inline(always)]
    pub fn mcan_ie_tce(&self) -> McanIeTceR {
        McanIeTceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub fn mcan_ie_tcfe(&self) -> McanIeTcfeR {
        McanIeTcfeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Enable"]
    #[inline(always)]
    pub fn mcan_ie_tfee(&self) -> McanIeTfeeR {
        McanIeTfeeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub fn mcan_ie_tefne(&self) -> McanIeTefneR {
        McanIeTefneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub fn mcan_ie_tefwe(&self) -> McanIeTefweR {
        McanIeTefweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub fn mcan_ie_teffe(&self) -> McanIeTeffeR {
        McanIeTeffeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub fn mcan_ie_tefle(&self) -> McanIeTefleR {
        McanIeTefleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Enable"]
    #[inline(always)]
    pub fn mcan_ie_tswe(&self) -> McanIeTsweR {
        McanIeTsweR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Enable"]
    #[inline(always)]
    pub fn mcan_ie_mrafe(&self) -> McanIeMrafeR {
        McanIeMrafeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Enable"]
    #[inline(always)]
    pub fn mcan_ie_tooe(&self) -> McanIeTooeR {
        McanIeTooeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub fn mcan_ie_drxe(&self) -> McanIeDrxeR {
        McanIeDrxeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Enable A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave this bit cleared to '0'."]
    #[inline(always)]
    pub fn mcan_ie_bece(&self) -> McanIeBeceR {
        McanIeBeceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Enable"]
    #[inline(always)]
    pub fn mcan_ie_beue(&self) -> McanIeBeueR {
        McanIeBeueR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Enable"]
    #[inline(always)]
    pub fn mcan_ie_eloe(&self) -> McanIeEloeR {
        McanIeEloeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Enable"]
    #[inline(always)]
    pub fn mcan_ie_epe(&self) -> McanIeEpeR {
        McanIeEpeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Enable"]
    #[inline(always)]
    pub fn mcan_ie_ewe(&self) -> McanIeEweR {
        McanIeEweR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Enable"]
    #[inline(always)]
    pub fn mcan_ie_boe(&self) -> McanIeBoeR {
        McanIeBoeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn mcan_ie_wdie(&self) -> McanIeWdieR {
        McanIeWdieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn mcan_ie_peae(&self) -> McanIePeaeR {
        McanIePeaeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn mcan_ie_pede(&self) -> McanIePedeR {
        McanIePedeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn mcan_ie_arae(&self) -> McanIeAraeR {
        McanIeAraeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0ne(&mut self) -> McanIeRf0neW<McanIeSpec> {
        McanIeRf0neW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0we(&mut self) -> McanIeRf0weW<McanIeSpec> {
        McanIeRf0weW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0fe(&mut self) -> McanIeRf0feW<McanIeSpec> {
        McanIeRf0feW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf0le(&mut self) -> McanIeRf0leW<McanIeSpec> {
        McanIeRf0leW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1ne(&mut self) -> McanIeRf1neW<McanIeSpec> {
        McanIeRf1neW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1we(&mut self) -> McanIeRf1weW<McanIeSpec> {
        McanIeRf1weW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1fe(&mut self) -> McanIeRf1feW<McanIeSpec> {
        McanIeRf1feW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub fn mcan_ie_rf1le(&mut self) -> McanIeRf1leW<McanIeSpec> {
        McanIeRf1leW::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message Enable"]
    #[inline(always)]
    pub fn mcan_ie_hpme(&mut self) -> McanIeHpmeW<McanIeSpec> {
        McanIeHpmeW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Completed Enable"]
    #[inline(always)]
    pub fn mcan_ie_tce(&mut self) -> McanIeTceW<McanIeSpec> {
        McanIeTceW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub fn mcan_ie_tcfe(&mut self) -> McanIeTcfeW<McanIeSpec> {
        McanIeTcfeW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Enable"]
    #[inline(always)]
    pub fn mcan_ie_tfee(&mut self) -> McanIeTfeeW<McanIeSpec> {
        McanIeTfeeW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub fn mcan_ie_tefne(&mut self) -> McanIeTefneW<McanIeSpec> {
        McanIeTefneW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub fn mcan_ie_tefwe(&mut self) -> McanIeTefweW<McanIeSpec> {
        McanIeTefweW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub fn mcan_ie_teffe(&mut self) -> McanIeTeffeW<McanIeSpec> {
        McanIeTeffeW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub fn mcan_ie_tefle(&mut self) -> McanIeTefleW<McanIeSpec> {
        McanIeTefleW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Enable"]
    #[inline(always)]
    pub fn mcan_ie_tswe(&mut self) -> McanIeTsweW<McanIeSpec> {
        McanIeTsweW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Enable"]
    #[inline(always)]
    pub fn mcan_ie_mrafe(&mut self) -> McanIeMrafeW<McanIeSpec> {
        McanIeMrafeW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred Enable"]
    #[inline(always)]
    pub fn mcan_ie_tooe(&mut self) -> McanIeTooeW<McanIeSpec> {
        McanIeTooeW::new(self, 18)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub fn mcan_ie_drxe(&mut self) -> McanIeDrxeW<McanIeSpec> {
        McanIeDrxeW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit Error Corrected Enable A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave this bit cleared to '0'."]
    #[inline(always)]
    pub fn mcan_ie_bece(&mut self) -> McanIeBeceW<McanIeSpec> {
        McanIeBeceW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Enable"]
    #[inline(always)]
    pub fn mcan_ie_beue(&mut self) -> McanIeBeueW<McanIeSpec> {
        McanIeBeueW::new(self, 21)
    }
    #[doc = "Bit 22 - Error Logging Overflow Enable"]
    #[inline(always)]
    pub fn mcan_ie_eloe(&mut self) -> McanIeEloeW<McanIeSpec> {
        McanIeEloeW::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive Enable"]
    #[inline(always)]
    pub fn mcan_ie_epe(&mut self) -> McanIeEpeW<McanIeSpec> {
        McanIeEpeW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status Enable"]
    #[inline(always)]
    pub fn mcan_ie_ewe(&mut self) -> McanIeEweW<McanIeSpec> {
        McanIeEweW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status Enable"]
    #[inline(always)]
    pub fn mcan_ie_boe(&mut self) -> McanIeBoeW<McanIeSpec> {
        McanIeBoeW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn mcan_ie_wdie(&mut self) -> McanIeWdieW<McanIeSpec> {
        McanIeWdieW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn mcan_ie_peae(&mut self) -> McanIePeaeW<McanIeSpec> {
        McanIePeaeW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn mcan_ie_pede(&mut self) -> McanIePedeW<McanIeSpec> {
        McanIePedeW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn mcan_ie_arae(&mut self) -> McanIeAraeW<McanIeSpec> {
        McanIeAraeW::new(self, 29)
    }
}
#[doc = "MCAN Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanIeSpec;
impl crate::RegisterSpec for McanIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_ie::R`](R) reader structure"]
impl crate::Readable for McanIeSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_ie::W`](W) writer structure"]
impl crate::Writable for McanIeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_IE to value 0"]
impl crate::Resettable for McanIeSpec {
    const RESET_VALUE: u32 = 0;
}
