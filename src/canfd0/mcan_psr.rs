#[doc = "Register `MCAN_PSR` reader"]
pub type R = crate::R<McanPsrSpec>;
#[doc = "Field `MCAN_PSR_LEC` reader - Last Error Code. The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. 0 No Error: No error occurred since LEC has been reset by successful reception or transmission. 1 Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2 Form Error: A fixed format part of a received frame has the wrong format. 3 AckError: The message transmitted by the MCAN was not acknowledged by another node. 4 Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant. 5 Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value '0'), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6 CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7 NoChange: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows the value '7', no CAN bus event was detected since the last CPU read access to the Protocol Status Register. Note: When a frame in CAN FD format has reached the data phase with BRS flag set, the next CAN event (error or valid frame) will be shown in DLEC instead of LEC. An error in a fixed stuff bit of a CAN FD CRC sequence will be shown as a Form Error, not Stuff Error. Note: The Bus_Off recovery sequence (see ISO 11898-1:2015) cannot be shortened by setting or resetting CCCR.INIT. If the device goes Bus_Off, it will set CCCR.INIT of its own accord, stopping all bus activities. Once CCCR.INIT has been cleared by the CPU, the device will then wait for 129 occurrences of Bus Idle (129 * 11 consecutive recessive bits) before resuming normal operation. At the end of the Bus_Off recovery sequence, the Error Management Counters will be reset. During the waiting time after the resetting of CCCR.INIT, each time a sequence of 11 recessive bits has been monitored, a Bit0Error code is written to PSR.LEC, enabling the CPU to readily check up whether the CAN bus is stuck at dominant or continuously disturbed and to monitor the Bus_Off recovery sequence. ECR.REC is used to count these sequences."]
pub type McanPsrLecR = crate::FieldReader;
#[doc = "Field `MCAN_PSR_ACT` reader - Node Activity. Monitors the module's CAN communication state. 00 Synchronizing - node is synchronizing on CAN communication 01 Idle - node is neither receiver nor transmitter 10 Receiver - node is operating as receiver 11 Transmitter - node is operating as transmitter Note: ACT is set to &amp;quot;00&amp;quot; by a Protocol Exception Event."]
pub type McanPsrActR = crate::FieldReader;
#[doc = "Field `MCAN_PSR_EP` reader - Error Passive 0 The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1 The M_CAN is in the Error_Passive state"]
pub type McanPsrEpR = crate::BitReader;
#[doc = "Field `MCAN_PSR_EW` reader - Warning Status 0 Both error counters are below the Error_Warning limit of 96 1 At least one of error counter has reached the Error_Warning limit of 96"]
pub type McanPsrEwR = crate::BitReader;
#[doc = "Field `MCAN_PSR_BO` reader - Bus_Off Status 0 The M_CAN is not Bus_Off 1 The M_CAN is in Bus_Off state"]
pub type McanPsrBoR = crate::BitReader;
#[doc = "Field `MCAN_PSR_DLEC` reader - Data Phase Last Error Code. Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set. Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error."]
pub type McanPsrDlecR = crate::FieldReader;
#[doc = "Field `MCAN_PSR_RESI` reader - ESI Flag of Last Received CAN FD Message. This bit is set together with RFDF, independent of acceptance filtering. 0 Last received CAN FD message did not have its ESI flag set 1 Last received CAN FD message had its ESI flag set"]
pub type McanPsrResiR = crate::BitReader;
#[doc = "Field `MCAN_PSR_RBRS` reader - BRS Flag of Last Received CAN FD Message. This bit is set together with RFDF, independent of acceptance filtering. 0 Last received CAN FD message did not have its BRS flag set 1 Last received CAN FD message had its BRS flag set"]
pub type McanPsrRbrsR = crate::BitReader;
#[doc = "Field `MCAN_PSR_RFDF` reader - Received a CAN FD Message. This bit is set independent of acceptance filtering. 0 Since this bit was reset by the CPU, no CAN FD message has been received 1 Message in CAN FD format with FDF flag set has been received"]
pub type McanPsrRfdfR = crate::BitReader;
#[doc = "Field `MCAN_PSR_PXE` reader - Protocol Exception Event 0 No protocol exception event occurred since last read access 1 Protocol exception event occurred"]
pub type McanPsrPxeR = crate::BitReader;
#[doc = "Field `MCAN_PSR_TDCV` reader - Transmitter Delay Compensation Value. Position of the secondary sample point, defined by the sum of the measured delay from the internal CAN TX signal to the internal CAN RX signal and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
pub type McanPsrTdcvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code. The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. 0 No Error: No error occurred since LEC has been reset by successful reception or transmission. 1 Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2 Form Error: A fixed format part of a received frame has the wrong format. 3 AckError: The message transmitted by the MCAN was not acknowledged by another node. 4 Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant. 5 Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value '0'), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6 CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7 NoChange: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows the value '7', no CAN bus event was detected since the last CPU read access to the Protocol Status Register. Note: When a frame in CAN FD format has reached the data phase with BRS flag set, the next CAN event (error or valid frame) will be shown in DLEC instead of LEC. An error in a fixed stuff bit of a CAN FD CRC sequence will be shown as a Form Error, not Stuff Error. Note: The Bus_Off recovery sequence (see ISO 11898-1:2015) cannot be shortened by setting or resetting CCCR.INIT. If the device goes Bus_Off, it will set CCCR.INIT of its own accord, stopping all bus activities. Once CCCR.INIT has been cleared by the CPU, the device will then wait for 129 occurrences of Bus Idle (129 * 11 consecutive recessive bits) before resuming normal operation. At the end of the Bus_Off recovery sequence, the Error Management Counters will be reset. During the waiting time after the resetting of CCCR.INIT, each time a sequence of 11 recessive bits has been monitored, a Bit0Error code is written to PSR.LEC, enabling the CPU to readily check up whether the CAN bus is stuck at dominant or continuously disturbed and to monitor the Bus_Off recovery sequence. ECR.REC is used to count these sequences."]
    #[inline(always)]
    pub fn mcan_psr_lec(&self) -> McanPsrLecR {
        McanPsrLecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Node Activity. Monitors the module's CAN communication state. 00 Synchronizing - node is synchronizing on CAN communication 01 Idle - node is neither receiver nor transmitter 10 Receiver - node is operating as receiver 11 Transmitter - node is operating as transmitter Note: ACT is set to &amp;quot;00&amp;quot; by a Protocol Exception Event."]
    #[inline(always)]
    pub fn mcan_psr_act(&self) -> McanPsrActR {
        McanPsrActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive 0 The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1 The M_CAN is in the Error_Passive state"]
    #[inline(always)]
    pub fn mcan_psr_ep(&self) -> McanPsrEpR {
        McanPsrEpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status 0 Both error counters are below the Error_Warning limit of 96 1 At least one of error counter has reached the Error_Warning limit of 96"]
    #[inline(always)]
    pub fn mcan_psr_ew(&self) -> McanPsrEwR {
        McanPsrEwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status 0 The M_CAN is not Bus_Off 1 The M_CAN is in Bus_Off state"]
    #[inline(always)]
    pub fn mcan_psr_bo(&self) -> McanPsrBoR {
        McanPsrBoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Phase Last Error Code. Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set. Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error."]
    #[inline(always)]
    pub fn mcan_psr_dlec(&self) -> McanPsrDlecR {
        McanPsrDlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI Flag of Last Received CAN FD Message. This bit is set together with RFDF, independent of acceptance filtering. 0 Last received CAN FD message did not have its ESI flag set 1 Last received CAN FD message had its ESI flag set"]
    #[inline(always)]
    pub fn mcan_psr_resi(&self) -> McanPsrResiR {
        McanPsrResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS Flag of Last Received CAN FD Message. This bit is set together with RFDF, independent of acceptance filtering. 0 Last received CAN FD message did not have its BRS flag set 1 Last received CAN FD message had its BRS flag set"]
    #[inline(always)]
    pub fn mcan_psr_rbrs(&self) -> McanPsrRbrsR {
        McanPsrRbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message. This bit is set independent of acceptance filtering. 0 Since this bit was reset by the CPU, no CAN FD message has been received 1 Message in CAN FD format with FDF flag set has been received"]
    #[inline(always)]
    pub fn mcan_psr_rfdf(&self) -> McanPsrRfdfR {
        McanPsrRfdfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event 0 No protocol exception event occurred since last read access 1 Protocol exception event occurred"]
    #[inline(always)]
    pub fn mcan_psr_pxe(&self) -> McanPsrPxeR {
        McanPsrPxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value. Position of the secondary sample point, defined by the sum of the measured delay from the internal CAN TX signal to the internal CAN RX signal and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn mcan_psr_tdcv(&self) -> McanPsrTdcvR {
        McanPsrTdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "MCAN Protocol Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanPsrSpec;
impl crate::RegisterSpec for McanPsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_psr::R`](R) reader structure"]
impl crate::Readable for McanPsrSpec {}
#[doc = "`reset()` method sets MCAN_PSR to value 0x0707"]
impl crate::Resettable for McanPsrSpec {
    const RESET_VALUE: u32 = 0x0707;
}
