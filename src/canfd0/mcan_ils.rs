#[doc = "Register `MCAN_ILS` reader"]
pub type R = crate::R<McanIlsSpec>;
#[doc = "Register `MCAN_ILS` writer"]
pub type W = crate::W<McanIlsSpec>;
#[doc = "Field `MCAN_ILS_RF0NL` reader - Rx FIFO 0 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0nlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF0NL` writer - Rx FIFO 0 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0nlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_RF0WL` reader - Rx FIFO 0 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0wlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF0WL` writer - Rx FIFO 0 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0wlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_RF0FL` reader - Rx FIFO 0 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0flR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF0FL` writer - Rx FIFO 0 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0flW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_RF0LL` reader - Rx FIFO 0 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0llR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF0LL` writer - Rx FIFO 0 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf0llW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_RF1NL` reader - Rx FIFO 1 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1nlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF1NL` writer - Rx FIFO 1 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1nlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_RF1WL` reader - Rx FIFO 1 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1wlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF1WL` writer - Rx FIFO 1 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1wlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_RF1FL` reader - Rx FIFO 1 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1flR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF1FL` writer - Rx FIFO 1 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1flW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_RF1LL` reader - Rx FIFO 1 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1llR = crate::BitReader;
#[doc = "Field `MCAN_ILS_RF1LL` writer - Rx FIFO 1 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsRf1llW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_HPML` reader - High Priority Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsHpmlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_HPML` writer - High Priority Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsHpmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TCL` reader - Transmission Completed Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTclR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TCL` writer - Transmission Completed Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TCFL` reader - Transmission Cancellation Finished Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTcflR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TCFL` writer - Transmission Cancellation Finished Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTcflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TFEL` reader - Tx FIFO Empty Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTfelR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TFEL` writer - Tx FIFO Empty Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTfelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TEFNL` reader - Tx Event FIFO New Entry Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefnlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TEFNL` writer - Tx Event FIFO New Entry Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefnlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TEFWL` reader - Tx Event FIFO Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefwlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TEFWL` writer - Tx Event FIFO Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TEFFL` reader - Tx Event FIFO Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefflR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TEFFL` writer - Tx Event FIFO Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TEFLL` reader - Tx Event FIFO Element Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefllR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TEFLL` writer - Tx Event FIFO Element Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTefllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TSWL` reader - Timestamp Wraparound Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTswlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TSWL` writer - Timestamp Wraparound Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsTswlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_MRAFL` reader - Message RAM Access Failure Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsMraflR = crate::BitReader;
#[doc = "Field `MCAN_ILS_MRAFL` writer - Message RAM Access Failure Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsMraflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_TOOL` reader - Timeout Occurred Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsToolR = crate::BitReader;
#[doc = "Field `MCAN_ILS_TOOL` writer - Timeout Occurred Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsToolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_DRXL` reader - Message Stored to Dedicated Rx Buffer Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsDrxlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_DRXL` writer - Message Stored to Dedicated Rx Buffer Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsDrxlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_BECL` reader - Bit Error Corrected Line A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave the MCAN_IE.BECE bit cleared to '0' (disabled), thereby relegating this bit to not applicable."]
pub type McanIlsBeclR = crate::BitReader;
#[doc = "Field `MCAN_ILS_BECL` writer - Bit Error Corrected Line A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave the MCAN_IE.BECE bit cleared to '0' (disabled), thereby relegating this bit to not applicable."]
pub type McanIlsBeclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_BEUL` reader - Bit Error Uncorrected Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsBeulR = crate::BitReader;
#[doc = "Field `MCAN_ILS_BEUL` writer - Bit Error Uncorrected Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsBeulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_ELOL` reader - Error Logging Overflow Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsElolR = crate::BitReader;
#[doc = "Field `MCAN_ILS_ELOL` writer - Error Logging Overflow Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsElolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_EPL` reader - Error Passive Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsEplR = crate::BitReader;
#[doc = "Field `MCAN_ILS_EPL` writer - Error Passive Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsEplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_EWL` reader - Warning Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsEwlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_EWL` writer - Warning Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsEwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_BOL` reader - Bus_Off Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsBolR = crate::BitReader;
#[doc = "Field `MCAN_ILS_BOL` writer - Bus_Off Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsBolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_WDIL` reader - Watchdog Interrupt Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsWdilR = crate::BitReader;
#[doc = "Field `MCAN_ILS_WDIL` writer - Watchdog Interrupt Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsWdilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_PEAL` reader - Protocol Error in Arbitration Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsPealR = crate::BitReader;
#[doc = "Field `MCAN_ILS_PEAL` writer - Protocol Error in Arbitration Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsPealW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_PEDL` reader - Protocol Error in Data Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsPedlR = crate::BitReader;
#[doc = "Field `MCAN_ILS_PEDL` writer - Protocol Error in Data Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsPedlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILS_ARAL` reader - Access to Reserved Address Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsAralR = crate::BitReader;
#[doc = "Field `MCAN_ILS_ARAL` writer - Access to Reserved Address Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
pub type McanIlsAralW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0nl(&self) -> McanIlsRf0nlR {
        McanIlsRf0nlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0wl(&self) -> McanIlsRf0wlR {
        McanIlsRf0wlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0fl(&self) -> McanIlsRf0flR {
        McanIlsRf0flR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0ll(&self) -> McanIlsRf0llR {
        McanIlsRf0llR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1nl(&self) -> McanIlsRf1nlR {
        McanIlsRf1nlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1wl(&self) -> McanIlsRf1wlR {
        McanIlsRf1wlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1fl(&self) -> McanIlsRf1flR {
        McanIlsRf1flR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1ll(&self) -> McanIlsRf1llR {
        McanIlsRf1llR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_hpml(&self) -> McanIlsHpmlR {
        McanIlsHpmlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tcl(&self) -> McanIlsTclR {
        McanIlsTclR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tcfl(&self) -> McanIlsTcflR {
        McanIlsTcflR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tfel(&self) -> McanIlsTfelR {
        McanIlsTfelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tefnl(&self) -> McanIlsTefnlR {
        McanIlsTefnlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tefwl(&self) -> McanIlsTefwlR {
        McanIlsTefwlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_teffl(&self) -> McanIlsTefflR {
        McanIlsTefflR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tefll(&self) -> McanIlsTefllR {
        McanIlsTefllR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tswl(&self) -> McanIlsTswlR {
        McanIlsTswlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_mrafl(&self) -> McanIlsMraflR {
        McanIlsMraflR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tool(&self) -> McanIlsToolR {
        McanIlsToolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_drxl(&self) -> McanIlsDrxlR {
        McanIlsDrxlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Line A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave the MCAN_IE.BECE bit cleared to '0' (disabled), thereby relegating this bit to not applicable."]
    #[inline(always)]
    pub fn mcan_ils_becl(&self) -> McanIlsBeclR {
        McanIlsBeclR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_beul(&self) -> McanIlsBeulR {
        McanIlsBeulR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_elol(&self) -> McanIlsElolR {
        McanIlsElolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_epl(&self) -> McanIlsEplR {
        McanIlsEplR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_ewl(&self) -> McanIlsEwlR {
        McanIlsEwlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_bol(&self) -> McanIlsBolR {
        McanIlsBolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_wdil(&self) -> McanIlsWdilR {
        McanIlsWdilR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_peal(&self) -> McanIlsPealR {
        McanIlsPealR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_pedl(&self) -> McanIlsPedlR {
        McanIlsPedlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_aral(&self) -> McanIlsAralR {
        McanIlsAralR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0nl(&mut self) -> McanIlsRf0nlW<McanIlsSpec> {
        McanIlsRf0nlW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0wl(&mut self) -> McanIlsRf0wlW<McanIlsSpec> {
        McanIlsRf0wlW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0fl(&mut self) -> McanIlsRf0flW<McanIlsSpec> {
        McanIlsRf0flW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf0ll(&mut self) -> McanIlsRf0llW<McanIlsSpec> {
        McanIlsRf0llW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1nl(&mut self) -> McanIlsRf1nlW<McanIlsSpec> {
        McanIlsRf1nlW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1wl(&mut self) -> McanIlsRf1wlW<McanIlsSpec> {
        McanIlsRf1wlW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1fl(&mut self) -> McanIlsRf1flW<McanIlsSpec> {
        McanIlsRf1flW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_rf1ll(&mut self) -> McanIlsRf1llW<McanIlsSpec> {
        McanIlsRf1llW::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_hpml(&mut self) -> McanIlsHpmlW<McanIlsSpec> {
        McanIlsHpmlW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Completed Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tcl(&mut self) -> McanIlsTclW<McanIlsSpec> {
        McanIlsTclW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tcfl(&mut self) -> McanIlsTcflW<McanIlsSpec> {
        McanIlsTcflW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tfel(&mut self) -> McanIlsTfelW<McanIlsSpec> {
        McanIlsTfelW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tefnl(&mut self) -> McanIlsTefnlW<McanIlsSpec> {
        McanIlsTefnlW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tefwl(&mut self) -> McanIlsTefwlW<McanIlsSpec> {
        McanIlsTefwlW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_teffl(&mut self) -> McanIlsTefflW<McanIlsSpec> {
        McanIlsTefflW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tefll(&mut self) -> McanIlsTefllW<McanIlsSpec> {
        McanIlsTefllW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tswl(&mut self) -> McanIlsTswlW<McanIlsSpec> {
        McanIlsTswlW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_mrafl(&mut self) -> McanIlsMraflW<McanIlsSpec> {
        McanIlsMraflW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_tool(&mut self) -> McanIlsToolW<McanIlsSpec> {
        McanIlsToolW::new(self, 18)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_drxl(&mut self) -> McanIlsDrxlW<McanIlsSpec> {
        McanIlsDrxlW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit Error Corrected Line A separate interrupt line reserved for corrected bit errors is provided via the MCAN_ERROR_REGS. It advised for the user to use these registers and leave the MCAN_IE.BECE bit cleared to '0' (disabled), thereby relegating this bit to not applicable."]
    #[inline(always)]
    pub fn mcan_ils_becl(&mut self) -> McanIlsBeclW<McanIlsSpec> {
        McanIlsBeclW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_beul(&mut self) -> McanIlsBeulW<McanIlsSpec> {
        McanIlsBeulW::new(self, 21)
    }
    #[doc = "Bit 22 - Error Logging Overflow Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_elol(&mut self) -> McanIlsElolW<McanIlsSpec> {
        McanIlsElolW::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_epl(&mut self) -> McanIlsEplW<McanIlsSpec> {
        McanIlsEplW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_ewl(&mut self) -> McanIlsEwlW<McanIlsSpec> {
        McanIlsEwlW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_bol(&mut self) -> McanIlsBolW<McanIlsSpec> {
        McanIlsBolW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_wdil(&mut self) -> McanIlsWdilW<McanIlsSpec> {
        McanIlsWdilW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_peal(&mut self) -> McanIlsPealW<McanIlsSpec> {
        McanIlsPealW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_pedl(&mut self) -> McanIlsPedlW<McanIlsSpec> {
        McanIlsPedlW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line 0 Interrupt source is assigned to Interrupt Line 0 1 Interrupt source is assigned to Interrupt Line 1"]
    #[inline(always)]
    pub fn mcan_ils_aral(&mut self) -> McanIlsAralW<McanIlsSpec> {
        McanIlsAralW::new(self, 29)
    }
}
#[doc = "MCAN Interrupt Line Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanIlsSpec;
impl crate::RegisterSpec for McanIlsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_ils::R`](R) reader structure"]
impl crate::Readable for McanIlsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_ils::W`](W) writer structure"]
impl crate::Writable for McanIlsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_ILS to value 0"]
impl crate::Resettable for McanIlsSpec {
    const RESET_VALUE: u32 = 0;
}
