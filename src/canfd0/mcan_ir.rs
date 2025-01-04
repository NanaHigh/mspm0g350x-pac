#[doc = "Register `MCAN_IR` reader"]
pub type R = crate::R<McanIrSpec>;
#[doc = "Register `MCAN_IR` writer"]
pub type W = crate::W<McanIrSpec>;
#[doc = "Field `MCAN_IR_RF0N` reader - Rx FIFO 0 New Message 0 No new message written to Rx FIFO 0 1 New message written to Rx FIFO 0"]
pub type McanIrRf0nR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF0N` writer - Rx FIFO 0 New Message 0 No new message written to Rx FIFO 0 1 New message written to Rx FIFO 0"]
pub type McanIrRf0nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_RF0W` reader - Rx FIFO 0 Watermark Reached 0 Rx FIFO 0 fill level below watermark 1 Rx FIFO 0 fill level reached watermark"]
pub type McanIrRf0wR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF0W` writer - Rx FIFO 0 Watermark Reached 0 Rx FIFO 0 fill level below watermark 1 Rx FIFO 0 fill level reached watermark"]
pub type McanIrRf0wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_RF0F` reader - Rx FIFO 0 Full 0 Rx FIFO 0 not full 1 Rx FIFO 0 full"]
pub type McanIrRf0fR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF0F` writer - Rx FIFO 0 Full 0 Rx FIFO 0 not full 1 Rx FIFO 0 full"]
pub type McanIrRf0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_RF0L` reader - Rx FIFO 0 Message Lost 0 No Rx FIFO 0 message lost 1 Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
pub type McanIrRf0lR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF0L` writer - Rx FIFO 0 Message Lost 0 No Rx FIFO 0 message lost 1 Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
pub type McanIrRf0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_RF1N` reader - Rx FIFO 1 New Message 0 No new message written to Rx FIFO 1 1 New message written to Rx FIFO 1"]
pub type McanIrRf1nR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF1N` writer - Rx FIFO 1 New Message 0 No new message written to Rx FIFO 1 1 New message written to Rx FIFO 1"]
pub type McanIrRf1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_RF1W` reader - Rx FIFO 1 Watermark Reached 0 Rx FIFO 1 fill level below watermark 1 Rx FIFO 1 fill level reached watermark"]
pub type McanIrRf1wR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF1W` writer - Rx FIFO 1 Watermark Reached 0 Rx FIFO 1 fill level below watermark 1 Rx FIFO 1 fill level reached watermark"]
pub type McanIrRf1wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_RF1F` reader - Rx FIFO 1 Full 0 Rx FIFO 1 not full 1 Rx FIFO 1 full"]
pub type McanIrRf1fR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF1F` writer - Rx FIFO 1 Full 0 Rx FIFO 1 not full 1 Rx FIFO 1 full"]
pub type McanIrRf1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_RF1L` reader - Rx FIFO 1 Message Lost 0 No Rx FIFO 1 message lost 1 Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
pub type McanIrRf1lR = crate::BitReader;
#[doc = "Field `MCAN_IR_RF1L` writer - Rx FIFO 1 Message Lost 0 No Rx FIFO 1 message lost 1 Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
pub type McanIrRf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_HPM` reader - High Priority Message 0 No high priority message received 1 High priority message received"]
pub type McanIrHpmR = crate::BitReader;
#[doc = "Field `MCAN_IR_HPM` writer - High Priority Message 0 No high priority message received 1 High priority message received"]
pub type McanIrHpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TC` reader - Transmission Completed 0 No transmission completed 1 Transmission completed"]
pub type McanIrTcR = crate::BitReader;
#[doc = "Field `MCAN_IR_TC` writer - Transmission Completed 0 No transmission completed 1 Transmission completed"]
pub type McanIrTcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TCF` reader - Transmission Cancellation Finished 0 No transmission cancellation finished 1 Transmission cancellation finished"]
pub type McanIrTcfR = crate::BitReader;
#[doc = "Field `MCAN_IR_TCF` writer - Transmission Cancellation Finished 0 No transmission cancellation finished 1 Transmission cancellation finished"]
pub type McanIrTcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TFE` reader - Tx FIFO Empty 0 Tx FIFO non-empty 1 Tx FIFO empty"]
pub type McanIrTfeR = crate::BitReader;
#[doc = "Field `MCAN_IR_TFE` writer - Tx FIFO Empty 0 Tx FIFO non-empty 1 Tx FIFO empty"]
pub type McanIrTfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TEFN` reader - Tx Event FIFO New Entry 0 Tx Event FIFO unchanged 1 Tx Handler wrote Tx Event FIFO element"]
pub type McanIrTefnR = crate::BitReader;
#[doc = "Field `MCAN_IR_TEFN` writer - Tx Event FIFO New Entry 0 Tx Event FIFO unchanged 1 Tx Handler wrote Tx Event FIFO element"]
pub type McanIrTefnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TEFW` reader - Tx Event FIFO Watermark Reached 0 Tx Event FIFO fill level below watermark 1 Tx Event FIFO fill level reached watermark"]
pub type McanIrTefwR = crate::BitReader;
#[doc = "Field `MCAN_IR_TEFW` writer - Tx Event FIFO Watermark Reached 0 Tx Event FIFO fill level below watermark 1 Tx Event FIFO fill level reached watermark"]
pub type McanIrTefwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TEFF` reader - Tx Event FIFO Full 0 Tx Event FIFO not full 1 Tx Event FIFO full"]
pub type McanIrTeffR = crate::BitReader;
#[doc = "Field `MCAN_IR_TEFF` writer - Tx Event FIFO Full 0 Tx Event FIFO not full 1 Tx Event FIFO full"]
pub type McanIrTeffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TEFL` reader - Tx Event FIFO Element Lost 0 No Tx Event FIFO element lost 1 Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
pub type McanIrTeflR = crate::BitReader;
#[doc = "Field `MCAN_IR_TEFL` writer - Tx Event FIFO Element Lost 0 No Tx Event FIFO element lost 1 Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
pub type McanIrTeflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TSW` reader - Timestamp Wraparound 0 No timestamp counter wrap-around 1 Timestamp counter wrapped around"]
pub type McanIrTswR = crate::BitReader;
#[doc = "Field `MCAN_IR_TSW` writer - Timestamp Wraparound 0 No timestamp counter wrap-around 1 Timestamp counter wrapped around"]
pub type McanIrTswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_MRAF` reader - Message RAM Access Failure. The flag is set, when the Rx Handler: - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the MCAN is switched into Restricted Operation Mode. To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0 No Message RAM access failure occurred 1 Message RAM access failure occurred"]
pub type McanIrMrafR = crate::BitReader;
#[doc = "Field `MCAN_IR_MRAF` writer - Message RAM Access Failure. The flag is set, when the Rx Handler: - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the MCAN is switched into Restricted Operation Mode. To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0 No Message RAM access failure occurred 1 Message RAM access failure occurred"]
pub type McanIrMrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_TOO` reader - Timeout Occurred 0 No timeout 1 Timeout reached"]
pub type McanIrTooR = crate::BitReader;
#[doc = "Field `MCAN_IR_TOO` writer - Timeout Occurred 0 No timeout 1 Timeout reached"]
pub type McanIrTooW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_DRX` reader - Message Stored to Dedicated Rx Buffer. The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0 No Rx Buffer updated 1 At least one received message stored into an Rx Buffer"]
pub type McanIrDrxR = crate::BitReader;
#[doc = "Field `MCAN_IR_DRX` writer - Message Stored to Dedicated Rx Buffer. The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0 No Rx Buffer updated 1 At least one received message stored into an Rx Buffer"]
pub type McanIrDrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_BEU` reader - Bit Error Uncorrected. Message RAM bit error detected, uncorrected. This bit is set when a double bit error is detected by the ECC aggregator attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0 No bit error detected when reading from Message RAM 1 Bit error detected, uncorrected (e.g. parity logic)"]
pub type McanIrBeuR = crate::BitReader;
#[doc = "Field `MCAN_IR_BEU` writer - Bit Error Uncorrected. Message RAM bit error detected, uncorrected. This bit is set when a double bit error is detected by the ECC aggregator attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0 No bit error detected when reading from Message RAM 1 Bit error detected, uncorrected (e.g. parity logic)"]
pub type McanIrBeuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_ELO` reader - Error Logging Overflow 0 CAN Error Logging Counter did not overflow 1 Overflow of CAN Error Logging Counter occurred"]
pub type McanIrEloR = crate::BitReader;
#[doc = "Field `MCAN_IR_ELO` writer - Error Logging Overflow 0 CAN Error Logging Counter did not overflow 1 Overflow of CAN Error Logging Counter occurred"]
pub type McanIrEloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_EP` reader - Error Passive 0 Error_Passive status unchanged 1 Error_Passive status changed"]
pub type McanIrEpR = crate::BitReader;
#[doc = "Field `MCAN_IR_EP` writer - Error Passive 0 Error_Passive status unchanged 1 Error_Passive status changed"]
pub type McanIrEpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_EW` reader - Warning Status 0 Error_Warning status unchanged 1 Error_Warning status changed"]
pub type McanIrEwR = crate::BitReader;
#[doc = "Field `MCAN_IR_EW` writer - Warning Status 0 Error_Warning status unchanged 1 Error_Warning status changed"]
pub type McanIrEwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_BO` reader - Bus_Off Status 0 Bus_Off status unchanged 1 Bus_Off status changed"]
pub type McanIrBoR = crate::BitReader;
#[doc = "Field `MCAN_IR_BO` writer - Bus_Off Status 0 Bus_Off status unchanged 1 Bus_Off status changed"]
pub type McanIrBoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_WDI` reader - Watchdog Interrupt 0 No Message RAM Watchdog event occurred 1 Message RAM Watchdog event due to missing READY"]
pub type McanIrWdiR = crate::BitReader;
#[doc = "Field `MCAN_IR_WDI` writer - Watchdog Interrupt 0 No Message RAM Watchdog event occurred 1 Message RAM Watchdog event due to missing READY"]
pub type McanIrWdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0 No protocol error in arbitration phase 1 Protocol error in arbitration phase detected (PSR.LEC ? 0,7)"]
pub type McanIrPeaR = crate::BitReader;
#[doc = "Field `MCAN_IR_PEA` writer - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0 No protocol error in arbitration phase 1 Protocol error in arbitration phase detected (PSR.LEC ? 0,7)"]
pub type McanIrPeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_PED` reader - Protocol Error in Data Phase (Data Bit Time is used) 0 No protocol error in data phase 1 Protocol error in data phase detected (PSR.DLEC ? 0,7)"]
pub type McanIrPedR = crate::BitReader;
#[doc = "Field `MCAN_IR_PED` writer - Protocol Error in Data Phase (Data Bit Time is used) 0 No protocol error in data phase 1 Protocol error in data phase detected (PSR.DLEC ? 0,7)"]
pub type McanIrPedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_IR_ARA` reader - Access to Reserved Address 0 No access to reserved address occurred 1 Access to reserved address occurred"]
pub type McanIrAraR = crate::BitReader;
#[doc = "Field `MCAN_IR_ARA` writer - Access to Reserved Address 0 No access to reserved address occurred 1 Access to reserved address occurred"]
pub type McanIrAraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message 0 No new message written to Rx FIFO 0 1 New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn mcan_ir_rf0n(&self) -> McanIrRf0nR {
        McanIrRf0nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached 0 Rx FIFO 0 fill level below watermark 1 Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub fn mcan_ir_rf0w(&self) -> McanIrRf0wR {
        McanIrRf0wR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full 0 Rx FIFO 0 not full 1 Rx FIFO 0 full"]
    #[inline(always)]
    pub fn mcan_ir_rf0f(&self) -> McanIrRf0fR {
        McanIrRf0fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost 0 No Rx FIFO 0 message lost 1 Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn mcan_ir_rf0l(&self) -> McanIrRf0lR {
        McanIrRf0lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message 0 No new message written to Rx FIFO 1 1 New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn mcan_ir_rf1n(&self) -> McanIrRf1nR {
        McanIrRf1nR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached 0 Rx FIFO 1 fill level below watermark 1 Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub fn mcan_ir_rf1w(&self) -> McanIrRf1wR {
        McanIrRf1wR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full 0 Rx FIFO 1 not full 1 Rx FIFO 1 full"]
    #[inline(always)]
    pub fn mcan_ir_rf1f(&self) -> McanIrRf1fR {
        McanIrRf1fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost 0 No Rx FIFO 1 message lost 1 Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn mcan_ir_rf1l(&self) -> McanIrRf1lR {
        McanIrRf1lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message 0 No high priority message received 1 High priority message received"]
    #[inline(always)]
    pub fn mcan_ir_hpm(&self) -> McanIrHpmR {
        McanIrHpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed 0 No transmission completed 1 Transmission completed"]
    #[inline(always)]
    pub fn mcan_ir_tc(&self) -> McanIrTcR {
        McanIrTcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished 0 No transmission cancellation finished 1 Transmission cancellation finished"]
    #[inline(always)]
    pub fn mcan_ir_tcf(&self) -> McanIrTcfR {
        McanIrTcfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty 0 Tx FIFO non-empty 1 Tx FIFO empty"]
    #[inline(always)]
    pub fn mcan_ir_tfe(&self) -> McanIrTfeR {
        McanIrTfeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry 0 Tx Event FIFO unchanged 1 Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub fn mcan_ir_tefn(&self) -> McanIrTefnR {
        McanIrTefnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached 0 Tx Event FIFO fill level below watermark 1 Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub fn mcan_ir_tefw(&self) -> McanIrTefwR {
        McanIrTefwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full 0 Tx Event FIFO not full 1 Tx Event FIFO full"]
    #[inline(always)]
    pub fn mcan_ir_teff(&self) -> McanIrTeffR {
        McanIrTeffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost 0 No Tx Event FIFO element lost 1 Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub fn mcan_ir_tefl(&self) -> McanIrTeflR {
        McanIrTeflR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound 0 No timestamp counter wrap-around 1 Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn mcan_ir_tsw(&self) -> McanIrTswR {
        McanIrTswR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure. The flag is set, when the Rx Handler: - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the MCAN is switched into Restricted Operation Mode. To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0 No Message RAM access failure occurred 1 Message RAM access failure occurred"]
    #[inline(always)]
    pub fn mcan_ir_mraf(&self) -> McanIrMrafR {
        McanIrMrafR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred 0 No timeout 1 Timeout reached"]
    #[inline(always)]
    pub fn mcan_ir_too(&self) -> McanIrTooR {
        McanIrTooR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer. The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0 No Rx Buffer updated 1 At least one received message stored into an Rx Buffer"]
    #[inline(always)]
    pub fn mcan_ir_drx(&self) -> McanIrDrxR {
        McanIrDrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected. Message RAM bit error detected, uncorrected. This bit is set when a double bit error is detected by the ECC aggregator attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0 No bit error detected when reading from Message RAM 1 Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    pub fn mcan_ir_beu(&self) -> McanIrBeuR {
        McanIrBeuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow 0 CAN Error Logging Counter did not overflow 1 Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub fn mcan_ir_elo(&self) -> McanIrEloR {
        McanIrEloR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive 0 Error_Passive status unchanged 1 Error_Passive status changed"]
    #[inline(always)]
    pub fn mcan_ir_ep(&self) -> McanIrEpR {
        McanIrEpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status 0 Error_Warning status unchanged 1 Error_Warning status changed"]
    #[inline(always)]
    pub fn mcan_ir_ew(&self) -> McanIrEwR {
        McanIrEwR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status 0 Bus_Off status unchanged 1 Bus_Off status changed"]
    #[inline(always)]
    pub fn mcan_ir_bo(&self) -> McanIrBoR {
        McanIrBoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt 0 No Message RAM Watchdog event occurred 1 Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub fn mcan_ir_wdi(&self) -> McanIrWdiR {
        McanIrWdiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0 No protocol error in arbitration phase 1 Protocol error in arbitration phase detected (PSR.LEC ? 0,7)"]
    #[inline(always)]
    pub fn mcan_ir_pea(&self) -> McanIrPeaR {
        McanIrPeaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used) 0 No protocol error in data phase 1 Protocol error in data phase detected (PSR.DLEC ? 0,7)"]
    #[inline(always)]
    pub fn mcan_ir_ped(&self) -> McanIrPedR {
        McanIrPedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address 0 No access to reserved address occurred 1 Access to reserved address occurred"]
    #[inline(always)]
    pub fn mcan_ir_ara(&self) -> McanIrAraR {
        McanIrAraR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message 0 No new message written to Rx FIFO 0 1 New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn mcan_ir_rf0n(&mut self) -> McanIrRf0nW<McanIrSpec> {
        McanIrRf0nW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached 0 Rx FIFO 0 fill level below watermark 1 Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub fn mcan_ir_rf0w(&mut self) -> McanIrRf0wW<McanIrSpec> {
        McanIrRf0wW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full 0 Rx FIFO 0 not full 1 Rx FIFO 0 full"]
    #[inline(always)]
    pub fn mcan_ir_rf0f(&mut self) -> McanIrRf0fW<McanIrSpec> {
        McanIrRf0fW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost 0 No Rx FIFO 0 message lost 1 Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn mcan_ir_rf0l(&mut self) -> McanIrRf0lW<McanIrSpec> {
        McanIrRf0lW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message 0 No new message written to Rx FIFO 1 1 New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn mcan_ir_rf1n(&mut self) -> McanIrRf1nW<McanIrSpec> {
        McanIrRf1nW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached 0 Rx FIFO 1 fill level below watermark 1 Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub fn mcan_ir_rf1w(&mut self) -> McanIrRf1wW<McanIrSpec> {
        McanIrRf1wW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full 0 Rx FIFO 1 not full 1 Rx FIFO 1 full"]
    #[inline(always)]
    pub fn mcan_ir_rf1f(&mut self) -> McanIrRf1fW<McanIrSpec> {
        McanIrRf1fW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost 0 No Rx FIFO 1 message lost 1 Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn mcan_ir_rf1l(&mut self) -> McanIrRf1lW<McanIrSpec> {
        McanIrRf1lW::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message 0 No high priority message received 1 High priority message received"]
    #[inline(always)]
    pub fn mcan_ir_hpm(&mut self) -> McanIrHpmW<McanIrSpec> {
        McanIrHpmW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Completed 0 No transmission completed 1 Transmission completed"]
    #[inline(always)]
    pub fn mcan_ir_tc(&mut self) -> McanIrTcW<McanIrSpec> {
        McanIrTcW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished 0 No transmission cancellation finished 1 Transmission cancellation finished"]
    #[inline(always)]
    pub fn mcan_ir_tcf(&mut self) -> McanIrTcfW<McanIrSpec> {
        McanIrTcfW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty 0 Tx FIFO non-empty 1 Tx FIFO empty"]
    #[inline(always)]
    pub fn mcan_ir_tfe(&mut self) -> McanIrTfeW<McanIrSpec> {
        McanIrTfeW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry 0 Tx Event FIFO unchanged 1 Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub fn mcan_ir_tefn(&mut self) -> McanIrTefnW<McanIrSpec> {
        McanIrTefnW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached 0 Tx Event FIFO fill level below watermark 1 Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub fn mcan_ir_tefw(&mut self) -> McanIrTefwW<McanIrSpec> {
        McanIrTefwW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full 0 Tx Event FIFO not full 1 Tx Event FIFO full"]
    #[inline(always)]
    pub fn mcan_ir_teff(&mut self) -> McanIrTeffW<McanIrSpec> {
        McanIrTeffW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost 0 No Tx Event FIFO element lost 1 Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub fn mcan_ir_tefl(&mut self) -> McanIrTeflW<McanIrSpec> {
        McanIrTeflW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound 0 No timestamp counter wrap-around 1 Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn mcan_ir_tsw(&mut self) -> McanIrTswW<McanIrSpec> {
        McanIrTswW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure. The flag is set, when the Rx Handler: - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the MCAN is switched into Restricted Operation Mode. To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0 No Message RAM access failure occurred 1 Message RAM access failure occurred"]
    #[inline(always)]
    pub fn mcan_ir_mraf(&mut self) -> McanIrMrafW<McanIrSpec> {
        McanIrMrafW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred 0 No timeout 1 Timeout reached"]
    #[inline(always)]
    pub fn mcan_ir_too(&mut self) -> McanIrTooW<McanIrSpec> {
        McanIrTooW::new(self, 18)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer. The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0 No Rx Buffer updated 1 At least one received message stored into an Rx Buffer"]
    #[inline(always)]
    pub fn mcan_ir_drx(&mut self) -> McanIrDrxW<McanIrSpec> {
        McanIrDrxW::new(self, 19)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected. Message RAM bit error detected, uncorrected. This bit is set when a double bit error is detected by the ECC aggregator attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0 No bit error detected when reading from Message RAM 1 Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    pub fn mcan_ir_beu(&mut self) -> McanIrBeuW<McanIrSpec> {
        McanIrBeuW::new(self, 21)
    }
    #[doc = "Bit 22 - Error Logging Overflow 0 CAN Error Logging Counter did not overflow 1 Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub fn mcan_ir_elo(&mut self) -> McanIrEloW<McanIrSpec> {
        McanIrEloW::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive 0 Error_Passive status unchanged 1 Error_Passive status changed"]
    #[inline(always)]
    pub fn mcan_ir_ep(&mut self) -> McanIrEpW<McanIrSpec> {
        McanIrEpW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status 0 Error_Warning status unchanged 1 Error_Warning status changed"]
    #[inline(always)]
    pub fn mcan_ir_ew(&mut self) -> McanIrEwW<McanIrSpec> {
        McanIrEwW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status 0 Bus_Off status unchanged 1 Bus_Off status changed"]
    #[inline(always)]
    pub fn mcan_ir_bo(&mut self) -> McanIrBoW<McanIrSpec> {
        McanIrBoW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt 0 No Message RAM Watchdog event occurred 1 Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub fn mcan_ir_wdi(&mut self) -> McanIrWdiW<McanIrSpec> {
        McanIrWdiW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0 No protocol error in arbitration phase 1 Protocol error in arbitration phase detected (PSR.LEC ? 0,7)"]
    #[inline(always)]
    pub fn mcan_ir_pea(&mut self) -> McanIrPeaW<McanIrSpec> {
        McanIrPeaW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used) 0 No protocol error in data phase 1 Protocol error in data phase detected (PSR.DLEC ? 0,7)"]
    #[inline(always)]
    pub fn mcan_ir_ped(&mut self) -> McanIrPedW<McanIrSpec> {
        McanIrPedW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address 0 No access to reserved address occurred 1 Access to reserved address occurred"]
    #[inline(always)]
    pub fn mcan_ir_ara(&mut self) -> McanIrAraW<McanIrSpec> {
        McanIrAraW::new(self, 29)
    }
}
#[doc = "MCAN Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanIrSpec;
impl crate::RegisterSpec for McanIrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_ir::R`](R) reader structure"]
impl crate::Readable for McanIrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_ir::W`](W) writer structure"]
impl crate::Writable for McanIrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_IR to value 0x8000_0000"]
impl crate::Resettable for McanIrSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
