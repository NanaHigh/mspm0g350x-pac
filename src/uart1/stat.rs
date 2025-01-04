#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "UART Busy This bit is set as soon as the transmit FIFO or TXDATA register becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatBusy {
    #[doc = "0: CLEARED"]
    StatBusyCleared = 0,
    #[doc = "1: SET"]
    StatBusySet = 1,
}
impl From<StatBusy> for bool {
    #[inline(always)]
    fn from(variant: StatBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_BUSY` reader - UART Busy This bit is set as soon as the transmit FIFO or TXDATA register becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation."]
pub type StatBusyR = crate::BitReader<StatBusy>;
impl StatBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatBusy {
        match self.bits {
            false => StatBusy::StatBusyCleared,
            true => StatBusy::StatBusySet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_stat_busy_cleared(&self) -> bool {
        *self == StatBusy::StatBusyCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_stat_busy_set(&self) -> bool {
        *self == StatBusy::StatBusySet
    }
}
#[doc = "UART Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatRxfe {
    #[doc = "0: CLEARED"]
    StatRxfeCleared = 0,
    #[doc = "1: SET"]
    StatRxfeSet = 1,
}
impl From<StatRxfe> for bool {
    #[inline(always)]
    fn from(variant: StatRxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_RXFE` reader - UART Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type StatRxfeR = crate::BitReader<StatRxfe>;
impl StatRxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatRxfe {
        match self.bits {
            false => StatRxfe::StatRxfeCleared,
            true => StatRxfe::StatRxfeSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_stat_rxfe_cleared(&self) -> bool {
        *self == StatRxfe::StatRxfeCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_stat_rxfe_set(&self) -> bool {
        *self == StatRxfe::StatRxfeSet
    }
}
#[doc = "UART Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatRxff {
    #[doc = "0: CLEARED"]
    StatRxffCleared = 0,
    #[doc = "1: SET"]
    StatRxffSet = 1,
}
impl From<StatRxff> for bool {
    #[inline(always)]
    fn from(variant: StatRxff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_RXFF` reader - UART Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type StatRxffR = crate::BitReader<StatRxff>;
impl StatRxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatRxff {
        match self.bits {
            false => StatRxff::StatRxffCleared,
            true => StatRxff::StatRxffSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_stat_rxff_cleared(&self) -> bool {
        *self == StatRxff::StatRxffCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_stat_rxff_set(&self) -> bool {
        *self == StatRxff::StatRxffSet
    }
}
#[doc = "UART Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatTxfe {
    #[doc = "0: CLEARED"]
    StatTxfeCleared = 0,
    #[doc = "1: SET"]
    StatTxfeSet = 1,
}
impl From<StatTxfe> for bool {
    #[inline(always)]
    fn from(variant: StatTxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_TXFE` reader - UART Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type StatTxfeR = crate::BitReader<StatTxfe>;
impl StatTxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatTxfe {
        match self.bits {
            false => StatTxfe::StatTxfeCleared,
            true => StatTxfe::StatTxfeSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_stat_txfe_cleared(&self) -> bool {
        *self == StatTxfe::StatTxfeCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_stat_txfe_set(&self) -> bool {
        *self == StatTxfe::StatTxfeSet
    }
}
#[doc = "UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatTxff {
    #[doc = "0: CLEARED"]
    StatTxffCleared = 0,
    #[doc = "1: SET"]
    StatTxffSet = 1,
}
impl From<StatTxff> for bool {
    #[inline(always)]
    fn from(variant: StatTxff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_TXFF` reader - UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type StatTxffR = crate::BitReader<StatTxff>;
impl StatTxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatTxff {
        match self.bits {
            false => StatTxff::StatTxffCleared,
            true => StatTxff::StatTxffSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_stat_txff_cleared(&self) -> bool {
        *self == StatTxff::StatTxffCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_stat_txff_set(&self) -> bool {
        *self == StatTxff::StatTxffSet
    }
}
#[doc = "Clear To Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatCts {
    #[doc = "0: CLEARED"]
    StatCtsCleared = 0,
    #[doc = "1: SET"]
    StatCtsSet = 1,
}
impl From<StatCts> for bool {
    #[inline(always)]
    fn from(variant: StatCts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_CTS` reader - Clear To Send"]
pub type StatCtsR = crate::BitReader<StatCts>;
impl StatCtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatCts {
        match self.bits {
            false => StatCts::StatCtsCleared,
            true => StatCts::StatCtsSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_stat_cts_cleared(&self) -> bool {
        *self == StatCts::StatCtsCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_stat_cts_set(&self) -> bool {
        *self == StatCts::StatCtsSet
    }
}
#[doc = "IDLE mode has been detected in Idleline-Mulitprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatIdle {
    #[doc = "0: CLEARED"]
    StatIdleCleared = 0,
    #[doc = "1: SET"]
    StatIdleSet = 1,
}
impl From<StatIdle> for bool {
    #[inline(always)]
    fn from(variant: StatIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_IDLE` reader - IDLE mode has been detected in Idleline-Mulitprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address."]
pub type StatIdleR = crate::BitReader<StatIdle>;
impl StatIdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatIdle {
        match self.bits {
            false => StatIdle::StatIdleCleared,
            true => StatIdle::StatIdleSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_stat_idle_cleared(&self) -> bool {
        *self == StatIdle::StatIdleCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_stat_idle_set(&self) -> bool {
        *self == StatIdle::StatIdleSet
    }
}
impl R {
    #[doc = "Bit 0 - UART Busy This bit is set as soon as the transmit FIFO or TXDATA register becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation."]
    #[inline(always)]
    pub fn stat_busy(&self) -> StatBusyR {
        StatBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - UART Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn stat_rxfe(&self) -> StatRxfeR {
        StatRxfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn stat_rxff(&self) -> StatRxffR {
        StatRxffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn stat_txfe(&self) -> StatTxfeR {
        StatTxfeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn stat_txff(&self) -> StatTxffR {
        StatTxffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear To Send"]
    #[inline(always)]
    pub fn stat_cts(&self) -> StatCtsR {
        StatCtsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDLE mode has been detected in Idleline-Mulitprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address."]
    #[inline(always)]
    pub fn stat_idle(&self) -> StatIdleR {
        StatIdleR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
