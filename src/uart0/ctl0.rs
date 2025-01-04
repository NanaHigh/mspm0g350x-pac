#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Enable {
    #[doc = "0: DISABLE"]
    Ctl0EnableDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0EnableEnable = 1,
}
impl From<Ctl0Enable> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_ENABLE` reader - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
pub type Ctl0EnableR = crate::BitReader<Ctl0Enable>;
impl Ctl0EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Enable {
        match self.bits {
            false => Ctl0Enable::Ctl0EnableDisable,
            true => Ctl0Enable::Ctl0EnableEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_enable_disable(&self) -> bool {
        *self == Ctl0Enable::Ctl0EnableDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_enable_enable(&self) -> bool {
        *self == Ctl0Enable::Ctl0EnableEnable
    }
}
#[doc = "Field `CTL0_ENABLE` writer - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
pub type Ctl0EnableW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Enable>;
impl<'a, REG> Ctl0EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_enable_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enable::Ctl0EnableDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_enable_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enable::Ctl0EnableEnable)
    }
}
#[doc = "UART Loop Back Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Lbe {
    #[doc = "0: DISABLE"]
    Ctl0LbeDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0LbeEnable = 1,
}
impl From<Ctl0Lbe> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Lbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_LBE` reader - UART Loop Back Enable"]
pub type Ctl0LbeR = crate::BitReader<Ctl0Lbe>;
impl Ctl0LbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Lbe {
        match self.bits {
            false => Ctl0Lbe::Ctl0LbeDisable,
            true => Ctl0Lbe::Ctl0LbeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_lbe_disable(&self) -> bool {
        *self == Ctl0Lbe::Ctl0LbeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_lbe_enable(&self) -> bool {
        *self == Ctl0Lbe::Ctl0LbeEnable
    }
}
#[doc = "Field `CTL0_LBE` writer - UART Loop Back Enable"]
pub type Ctl0LbeW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Lbe>;
impl<'a, REG> Ctl0LbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_lbe_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Lbe::Ctl0LbeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_lbe_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Lbe::Ctl0LbeEnable)
    }
}
#[doc = "UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Rxe {
    #[doc = "0: DISABLE"]
    Ctl0RxeDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0RxeEnable = 1,
}
impl From<Ctl0Rxe> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Rxe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_RXE` reader - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
pub type Ctl0RxeR = crate::BitReader<Ctl0Rxe>;
impl Ctl0RxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Rxe {
        match self.bits {
            false => Ctl0Rxe::Ctl0RxeDisable,
            true => Ctl0Rxe::Ctl0RxeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_rxe_disable(&self) -> bool {
        *self == Ctl0Rxe::Ctl0RxeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_rxe_enable(&self) -> bool {
        *self == Ctl0Rxe::Ctl0RxeEnable
    }
}
#[doc = "Field `CTL0_RXE` writer - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
pub type Ctl0RxeW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Rxe>;
impl<'a, REG> Ctl0RxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_rxe_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Rxe::Ctl0RxeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_rxe_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Rxe::Ctl0RxeEnable)
    }
}
#[doc = "UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Txe {
    #[doc = "0: DISABLE"]
    Ctl0TxeDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0TxeEnable = 1,
}
impl From<Ctl0Txe> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Txe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_TXE` reader - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
pub type Ctl0TxeR = crate::BitReader<Ctl0Txe>;
impl Ctl0TxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Txe {
        match self.bits {
            false => Ctl0Txe::Ctl0TxeDisable,
            true => Ctl0Txe::Ctl0TxeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_txe_disable(&self) -> bool {
        *self == Ctl0Txe::Ctl0TxeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_txe_enable(&self) -> bool {
        *self == Ctl0Txe::Ctl0TxeEnable
    }
}
#[doc = "Field `CTL0_TXE` writer - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
pub type Ctl0TxeW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Txe>;
impl<'a, REG> Ctl0TxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_txe_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Txe::Ctl0TxeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_txe_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Txe::Ctl0TxeEnable)
    }
}
#[doc = "TXD Pin Control Enable. When the transmit section of the UART is disabled (TXE = 0), the TXD pin can be controlled by the TXD_OUT bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0TxdOutEn {
    #[doc = "0: DISABLE"]
    Ctl0TxdOutEnDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0TxdOutEnEnable = 1,
}
impl From<Ctl0TxdOutEn> for bool {
    #[inline(always)]
    fn from(variant: Ctl0TxdOutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_TXD_OUT_EN` reader - TXD Pin Control Enable. When the transmit section of the UART is disabled (TXE = 0), the TXD pin can be controlled by the TXD_OUT bit."]
pub type Ctl0TxdOutEnR = crate::BitReader<Ctl0TxdOutEn>;
impl Ctl0TxdOutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0TxdOutEn {
        match self.bits {
            false => Ctl0TxdOutEn::Ctl0TxdOutEnDisable,
            true => Ctl0TxdOutEn::Ctl0TxdOutEnEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_txd_out_en_disable(&self) -> bool {
        *self == Ctl0TxdOutEn::Ctl0TxdOutEnDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_txd_out_en_enable(&self) -> bool {
        *self == Ctl0TxdOutEn::Ctl0TxdOutEnEnable
    }
}
#[doc = "Field `CTL0_TXD_OUT_EN` writer - TXD Pin Control Enable. When the transmit section of the UART is disabled (TXE = 0), the TXD pin can be controlled by the TXD_OUT bit."]
pub type Ctl0TxdOutEnW<'a, REG> = crate::BitWriter<'a, REG, Ctl0TxdOutEn>;
impl<'a, REG> Ctl0TxdOutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_txd_out_en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0TxdOutEn::Ctl0TxdOutEnDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_txd_out_en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0TxdOutEn::Ctl0TxdOutEnEnable)
    }
}
#[doc = "TXD Pin Control Controls the TXD pin when TXD_OUT_EN = 1 and TXE = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0TxdOut {
    #[doc = "0: LOW"]
    Ctl0TxdOutLow = 0,
    #[doc = "1: HIGH"]
    Ctl0TxdOutHigh = 1,
}
impl From<Ctl0TxdOut> for bool {
    #[inline(always)]
    fn from(variant: Ctl0TxdOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_TXD_OUT` reader - TXD Pin Control Controls the TXD pin when TXD_OUT_EN = 1 and TXE = 0."]
pub type Ctl0TxdOutR = crate::BitReader<Ctl0TxdOut>;
impl Ctl0TxdOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0TxdOut {
        match self.bits {
            false => Ctl0TxdOut::Ctl0TxdOutLow,
            true => Ctl0TxdOut::Ctl0TxdOutHigh,
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn is_ctl0_txd_out_low(&self) -> bool {
        *self == Ctl0TxdOut::Ctl0TxdOutLow
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn is_ctl0_txd_out_high(&self) -> bool {
        *self == Ctl0TxdOut::Ctl0TxdOutHigh
    }
}
#[doc = "Field `CTL0_TXD_OUT` writer - TXD Pin Control Controls the TXD pin when TXD_OUT_EN = 1 and TXE = 0."]
pub type Ctl0TxdOutW<'a, REG> = crate::BitWriter<'a, REG, Ctl0TxdOut>;
impl<'a, REG> Ctl0TxdOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOW"]
    #[inline(always)]
    pub fn ctl0_txd_out_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0TxdOut::Ctl0TxdOutLow)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn ctl0_txd_out_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0TxdOut::Ctl0TxdOutHigh)
    }
}
#[doc = "Manchester Encode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Menc {
    #[doc = "0: DISABLE"]
    Ctl0MencDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0MencEnable = 1,
}
impl From<Ctl0Menc> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Menc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_MENC` reader - Manchester Encode enable"]
pub type Ctl0MencR = crate::BitReader<Ctl0Menc>;
impl Ctl0MencR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Menc {
        match self.bits {
            false => Ctl0Menc::Ctl0MencDisable,
            true => Ctl0Menc::Ctl0MencEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_menc_disable(&self) -> bool {
        *self == Ctl0Menc::Ctl0MencDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_menc_enable(&self) -> bool {
        *self == Ctl0Menc::Ctl0MencEnable
    }
}
#[doc = "Field `CTL0_MENC` writer - Manchester Encode enable"]
pub type Ctl0MencW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Menc>;
impl<'a, REG> Ctl0MencW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_menc_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Menc::Ctl0MencDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_menc_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Menc::Ctl0MencEnable)
    }
}
#[doc = "Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Mode {
    #[doc = "0: UART"]
    Ctl0ModeUart = 0,
    #[doc = "1: RS485"]
    Ctl0ModeRs485 = 1,
    #[doc = "2: IDLELINE"]
    Ctl0ModeIdleline = 2,
    #[doc = "3: ADDR9BIT"]
    Ctl0ModeAddr9bit = 3,
    #[doc = "4: SMART"]
    Ctl0ModeSmart = 4,
    #[doc = "5: DALI"]
    Ctl0ModeDali = 5,
}
impl From<Ctl0Mode> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Mode {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Mode {}
#[doc = "Field `CTL0_MODE` reader - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
pub type Ctl0ModeR = crate::FieldReader<Ctl0Mode>;
impl Ctl0ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl0Mode> {
        match self.bits {
            0 => Some(Ctl0Mode::Ctl0ModeUart),
            1 => Some(Ctl0Mode::Ctl0ModeRs485),
            2 => Some(Ctl0Mode::Ctl0ModeIdleline),
            3 => Some(Ctl0Mode::Ctl0ModeAddr9bit),
            4 => Some(Ctl0Mode::Ctl0ModeSmart),
            5 => Some(Ctl0Mode::Ctl0ModeDali),
            _ => None,
        }
    }
    #[doc = "UART"]
    #[inline(always)]
    pub fn is_ctl0_mode_uart(&self) -> bool {
        *self == Ctl0Mode::Ctl0ModeUart
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn is_ctl0_mode_rs485(&self) -> bool {
        *self == Ctl0Mode::Ctl0ModeRs485
    }
    #[doc = "IDLELINE"]
    #[inline(always)]
    pub fn is_ctl0_mode_idleline(&self) -> bool {
        *self == Ctl0Mode::Ctl0ModeIdleline
    }
    #[doc = "ADDR9BIT"]
    #[inline(always)]
    pub fn is_ctl0_mode_addr9bit(&self) -> bool {
        *self == Ctl0Mode::Ctl0ModeAddr9bit
    }
    #[doc = "SMART"]
    #[inline(always)]
    pub fn is_ctl0_mode_smart(&self) -> bool {
        *self == Ctl0Mode::Ctl0ModeSmart
    }
    #[doc = "DALI"]
    #[inline(always)]
    pub fn is_ctl0_mode_dali(&self) -> bool {
        *self == Ctl0Mode::Ctl0ModeDali
    }
}
#[doc = "Field `CTL0_MODE` writer - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
pub type Ctl0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctl0Mode>;
impl<'a, REG> Ctl0ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART"]
    #[inline(always)]
    pub fn ctl0_mode_uart(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Mode::Ctl0ModeUart)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn ctl0_mode_rs485(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Mode::Ctl0ModeRs485)
    }
    #[doc = "IDLELINE"]
    #[inline(always)]
    pub fn ctl0_mode_idleline(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Mode::Ctl0ModeIdleline)
    }
    #[doc = "ADDR9BIT"]
    #[inline(always)]
    pub fn ctl0_mode_addr9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Mode::Ctl0ModeAddr9bit)
    }
    #[doc = "SMART"]
    #[inline(always)]
    pub fn ctl0_mode_smart(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Mode::Ctl0ModeSmart)
    }
    #[doc = "DALI"]
    #[inline(always)]
    pub fn ctl0_mode_dali(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Mode::Ctl0ModeDali)
    }
}
#[doc = "Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or TXDATA buffer. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Rts {
    #[doc = "0: CLR"]
    Ctl0RtsClr = 0,
    #[doc = "1: SET"]
    Ctl0RtsSet = 1,
}
impl From<Ctl0Rts> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Rts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_RTS` reader - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or TXDATA buffer. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
pub type Ctl0RtsR = crate::BitReader<Ctl0Rts>;
impl Ctl0RtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Rts {
        match self.bits {
            false => Ctl0Rts::Ctl0RtsClr,
            true => Ctl0Rts::Ctl0RtsSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ctl0_rts_clr(&self) -> bool {
        *self == Ctl0Rts::Ctl0RtsClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ctl0_rts_set(&self) -> bool {
        *self == Ctl0Rts::Ctl0RtsSet
    }
}
#[doc = "Field `CTL0_RTS` writer - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or TXDATA buffer. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
pub type Ctl0RtsW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Rts>;
impl<'a, REG> Ctl0RtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn ctl0_rts_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Rts::Ctl0RtsClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn ctl0_rts_set(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Rts::Ctl0RtsSet)
    }
}
#[doc = "Enable hardware controlled Request to Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Rtsen {
    #[doc = "0: DISABLE"]
    Ctl0RtsenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0RtsenEnable = 1,
}
impl From<Ctl0Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Rtsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_RTSEN` reader - Enable hardware controlled Request to Send"]
pub type Ctl0RtsenR = crate::BitReader<Ctl0Rtsen>;
impl Ctl0RtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Rtsen {
        match self.bits {
            false => Ctl0Rtsen::Ctl0RtsenDisable,
            true => Ctl0Rtsen::Ctl0RtsenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_rtsen_disable(&self) -> bool {
        *self == Ctl0Rtsen::Ctl0RtsenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_rtsen_enable(&self) -> bool {
        *self == Ctl0Rtsen::Ctl0RtsenEnable
    }
}
#[doc = "Field `CTL0_RTSEN` writer - Enable hardware controlled Request to Send"]
pub type Ctl0RtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Rtsen>;
impl<'a, REG> Ctl0RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_rtsen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Rtsen::Ctl0RtsenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_rtsen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Rtsen::Ctl0RtsenEnable)
    }
}
#[doc = "Enable Clear To Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Ctsen {
    #[doc = "0: DISABLE"]
    Ctl0CtsenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0CtsenEnable = 1,
}
impl From<Ctl0Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_CTSEN` reader - Enable Clear To Send"]
pub type Ctl0CtsenR = crate::BitReader<Ctl0Ctsen>;
impl Ctl0CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Ctsen {
        match self.bits {
            false => Ctl0Ctsen::Ctl0CtsenDisable,
            true => Ctl0Ctsen::Ctl0CtsenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_ctsen_disable(&self) -> bool {
        *self == Ctl0Ctsen::Ctl0CtsenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_ctsen_enable(&self) -> bool {
        *self == Ctl0Ctsen::Ctl0CtsenEnable
    }
}
#[doc = "Field `CTL0_CTSEN` writer - Enable Clear To Send"]
pub type Ctl0CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Ctsen>;
impl<'a, REG> Ctl0CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_ctsen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ctsen::Ctl0CtsenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_ctsen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ctsen::Ctl0CtsenEnable)
    }
}
#[doc = "High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Hse {
    #[doc = "0: OVS16"]
    Ctl0HseOvs16 = 0,
    #[doc = "1: OVS8"]
    Ctl0HseOvs8 = 1,
    #[doc = "2: OVS3"]
    Ctl0HseOvs3 = 2,
}
impl From<Ctl0Hse> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Hse) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Hse {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Hse {}
#[doc = "Field `CTL0_HSE` reader - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
pub type Ctl0HseR = crate::FieldReader<Ctl0Hse>;
impl Ctl0HseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl0Hse> {
        match self.bits {
            0 => Some(Ctl0Hse::Ctl0HseOvs16),
            1 => Some(Ctl0Hse::Ctl0HseOvs8),
            2 => Some(Ctl0Hse::Ctl0HseOvs3),
            _ => None,
        }
    }
    #[doc = "OVS16"]
    #[inline(always)]
    pub fn is_ctl0_hse_ovs16(&self) -> bool {
        *self == Ctl0Hse::Ctl0HseOvs16
    }
    #[doc = "OVS8"]
    #[inline(always)]
    pub fn is_ctl0_hse_ovs8(&self) -> bool {
        *self == Ctl0Hse::Ctl0HseOvs8
    }
    #[doc = "OVS3"]
    #[inline(always)]
    pub fn is_ctl0_hse_ovs3(&self) -> bool {
        *self == Ctl0Hse::Ctl0HseOvs3
    }
}
#[doc = "Field `CTL0_HSE` writer - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
pub type Ctl0HseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl0Hse>;
impl<'a, REG> Ctl0HseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OVS16"]
    #[inline(always)]
    pub fn ctl0_hse_ovs16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Hse::Ctl0HseOvs16)
    }
    #[doc = "OVS8"]
    #[inline(always)]
    pub fn ctl0_hse_ovs8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Hse::Ctl0HseOvs8)
    }
    #[doc = "OVS3"]
    #[inline(always)]
    pub fn ctl0_hse_ovs3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Hse::Ctl0HseOvs3)
    }
}
#[doc = "UART Enable FIFOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Fen {
    #[doc = "0: DISABLE"]
    Ctl0FenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0FenEnable = 1,
}
impl From<Ctl0Fen> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Fen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_FEN` reader - UART Enable FIFOs"]
pub type Ctl0FenR = crate::BitReader<Ctl0Fen>;
impl Ctl0FenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Fen {
        match self.bits {
            false => Ctl0Fen::Ctl0FenDisable,
            true => Ctl0Fen::Ctl0FenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_fen_disable(&self) -> bool {
        *self == Ctl0Fen::Ctl0FenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_fen_enable(&self) -> bool {
        *self == Ctl0Fen::Ctl0FenEnable
    }
}
#[doc = "Field `CTL0_FEN` writer - UART Enable FIFOs"]
pub type Ctl0FenW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Fen>;
impl<'a, REG> Ctl0FenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_fen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Fen::Ctl0FenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_fen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Fen::Ctl0FenEnable)
    }
}
#[doc = "When enabled with oversmapling of 16, samples samples 7, 8, and 9 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values do not match, RIS.NERR bit is set along with RDR.NERR When enabled with oversmapling of 8, samples samples 3, 4, and 5 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values donot match, RIS.NERR bit is set along with RDR.NERR When disabled, only a single sample of received bit is taken.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Majvote {
    #[doc = "0: DISABLE"]
    Ctl0MajvoteDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0MajvoteEnable = 1,
}
impl From<Ctl0Majvote> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Majvote) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_MAJVOTE` reader - When enabled with oversmapling of 16, samples samples 7, 8, and 9 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values do not match, RIS.NERR bit is set along with RDR.NERR When enabled with oversmapling of 8, samples samples 3, 4, and 5 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values donot match, RIS.NERR bit is set along with RDR.NERR When disabled, only a single sample of received bit is taken."]
pub type Ctl0MajvoteR = crate::BitReader<Ctl0Majvote>;
impl Ctl0MajvoteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Majvote {
        match self.bits {
            false => Ctl0Majvote::Ctl0MajvoteDisable,
            true => Ctl0Majvote::Ctl0MajvoteEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_majvote_disable(&self) -> bool {
        *self == Ctl0Majvote::Ctl0MajvoteDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_majvote_enable(&self) -> bool {
        *self == Ctl0Majvote::Ctl0MajvoteEnable
    }
}
#[doc = "Field `CTL0_MAJVOTE` writer - When enabled with oversmapling of 16, samples samples 7, 8, and 9 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values do not match, RIS.NERR bit is set along with RDR.NERR When enabled with oversmapling of 8, samples samples 3, 4, and 5 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values donot match, RIS.NERR bit is set along with RDR.NERR When disabled, only a single sample of received bit is taken."]
pub type Ctl0MajvoteW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Majvote>;
impl<'a, REG> Ctl0MajvoteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_majvote_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Majvote::Ctl0MajvoteDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_majvote_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Majvote::Ctl0MajvoteEnable)
    }
}
#[doc = "Most Significant Bit First This bit has effect both on the way protocol byte is transmitted and received. Notes: User needs to match the protocol to the correct value of this bit to send MSb or LSb first. The hardware engine will send the byte entirely based on this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Msbfirst {
    #[doc = "0: DISABLE"]
    Ctl0MsbfirstDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0MsbfirstEnable = 1,
}
impl From<Ctl0Msbfirst> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Msbfirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_MSBFIRST` reader - Most Significant Bit First This bit has effect both on the way protocol byte is transmitted and received. Notes: User needs to match the protocol to the correct value of this bit to send MSb or LSb first. The hardware engine will send the byte entirely based on this bit."]
pub type Ctl0MsbfirstR = crate::BitReader<Ctl0Msbfirst>;
impl Ctl0MsbfirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Msbfirst {
        match self.bits {
            false => Ctl0Msbfirst::Ctl0MsbfirstDisable,
            true => Ctl0Msbfirst::Ctl0MsbfirstEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_msbfirst_disable(&self) -> bool {
        *self == Ctl0Msbfirst::Ctl0MsbfirstDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_msbfirst_enable(&self) -> bool {
        *self == Ctl0Msbfirst::Ctl0MsbfirstEnable
    }
}
#[doc = "Field `CTL0_MSBFIRST` writer - Most Significant Bit First This bit has effect both on the way protocol byte is transmitted and received. Notes: User needs to match the protocol to the correct value of this bit to send MSb or LSb first. The hardware engine will send the byte entirely based on this bit."]
pub type Ctl0MsbfirstW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Msbfirst>;
impl<'a, REG> Ctl0MsbfirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_msbfirst_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Msbfirst::Ctl0MsbfirstDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_msbfirst_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Msbfirst::Ctl0MsbfirstEnable)
    }
}
impl R {
    #[doc = "Bit 0 - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
    #[inline(always)]
    pub fn ctl0_enable(&self) -> Ctl0EnableR {
        Ctl0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn ctl0_lbe(&self) -> Ctl0LbeR {
        Ctl0LbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn ctl0_rxe(&self) -> Ctl0RxeR {
        Ctl0RxeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn ctl0_txe(&self) -> Ctl0TxeR {
        Ctl0TxeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXD Pin Control Enable. When the transmit section of the UART is disabled (TXE = 0), the TXD pin can be controlled by the TXD_OUT bit."]
    #[inline(always)]
    pub fn ctl0_txd_out_en(&self) -> Ctl0TxdOutEnR {
        Ctl0TxdOutEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXD Pin Control Controls the TXD pin when TXD_OUT_EN = 1 and TXE = 0."]
    #[inline(always)]
    pub fn ctl0_txd_out(&self) -> Ctl0TxdOutR {
        Ctl0TxdOutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Manchester Encode enable"]
    #[inline(always)]
    pub fn ctl0_menc(&self) -> Ctl0MencR {
        Ctl0MencR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
    #[inline(always)]
    pub fn ctl0_mode(&self) -> Ctl0ModeR {
        Ctl0ModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or TXDATA buffer. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
    #[inline(always)]
    pub fn ctl0_rts(&self) -> Ctl0RtsR {
        Ctl0RtsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable hardware controlled Request to Send"]
    #[inline(always)]
    pub fn ctl0_rtsen(&self) -> Ctl0RtsenR {
        Ctl0RtsenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Clear To Send"]
    #[inline(always)]
    pub fn ctl0_ctsen(&self) -> Ctl0CtsenR {
        Ctl0CtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
    #[inline(always)]
    pub fn ctl0_hse(&self) -> Ctl0HseR {
        Ctl0HseR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn ctl0_fen(&self) -> Ctl0FenR {
        Ctl0FenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When enabled with oversmapling of 16, samples samples 7, 8, and 9 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values do not match, RIS.NERR bit is set along with RDR.NERR When enabled with oversmapling of 8, samples samples 3, 4, and 5 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values donot match, RIS.NERR bit is set along with RDR.NERR When disabled, only a single sample of received bit is taken."]
    #[inline(always)]
    pub fn ctl0_majvote(&self) -> Ctl0MajvoteR {
        Ctl0MajvoteR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most Significant Bit First This bit has effect both on the way protocol byte is transmitted and received. Notes: User needs to match the protocol to the correct value of this bit to send MSb or LSb first. The hardware engine will send the byte entirely based on this bit."]
    #[inline(always)]
    pub fn ctl0_msbfirst(&self) -> Ctl0MsbfirstR {
        Ctl0MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
    #[inline(always)]
    pub fn ctl0_enable(&mut self) -> Ctl0EnableW<Ctl0Spec> {
        Ctl0EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn ctl0_lbe(&mut self) -> Ctl0LbeW<Ctl0Spec> {
        Ctl0LbeW::new(self, 2)
    }
    #[doc = "Bit 3 - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn ctl0_rxe(&mut self) -> Ctl0RxeW<Ctl0Spec> {
        Ctl0RxeW::new(self, 3)
    }
    #[doc = "Bit 4 - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn ctl0_txe(&mut self) -> Ctl0TxeW<Ctl0Spec> {
        Ctl0TxeW::new(self, 4)
    }
    #[doc = "Bit 5 - TXD Pin Control Enable. When the transmit section of the UART is disabled (TXE = 0), the TXD pin can be controlled by the TXD_OUT bit."]
    #[inline(always)]
    pub fn ctl0_txd_out_en(&mut self) -> Ctl0TxdOutEnW<Ctl0Spec> {
        Ctl0TxdOutEnW::new(self, 5)
    }
    #[doc = "Bit 6 - TXD Pin Control Controls the TXD pin when TXD_OUT_EN = 1 and TXE = 0."]
    #[inline(always)]
    pub fn ctl0_txd_out(&mut self) -> Ctl0TxdOutW<Ctl0Spec> {
        Ctl0TxdOutW::new(self, 6)
    }
    #[doc = "Bit 7 - Manchester Encode enable"]
    #[inline(always)]
    pub fn ctl0_menc(&mut self) -> Ctl0MencW<Ctl0Spec> {
        Ctl0MencW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
    #[inline(always)]
    pub fn ctl0_mode(&mut self) -> Ctl0ModeW<Ctl0Spec> {
        Ctl0ModeW::new(self, 8)
    }
    #[doc = "Bit 12 - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or TXDATA buffer. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
    #[inline(always)]
    pub fn ctl0_rts(&mut self) -> Ctl0RtsW<Ctl0Spec> {
        Ctl0RtsW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable hardware controlled Request to Send"]
    #[inline(always)]
    pub fn ctl0_rtsen(&mut self) -> Ctl0RtsenW<Ctl0Spec> {
        Ctl0RtsenW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Clear To Send"]
    #[inline(always)]
    pub fn ctl0_ctsen(&mut self) -> Ctl0CtsenW<Ctl0Spec> {
        Ctl0CtsenW::new(self, 14)
    }
    #[doc = "Bits 15:16 - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
    #[inline(always)]
    pub fn ctl0_hse(&mut self) -> Ctl0HseW<Ctl0Spec> {
        Ctl0HseW::new(self, 15)
    }
    #[doc = "Bit 17 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn ctl0_fen(&mut self) -> Ctl0FenW<Ctl0Spec> {
        Ctl0FenW::new(self, 17)
    }
    #[doc = "Bit 18 - When enabled with oversmapling of 16, samples samples 7, 8, and 9 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values do not match, RIS.NERR bit is set along with RDR.NERR When enabled with oversmapling of 8, samples samples 3, 4, and 5 are majority voted to decide the sampled bit value. The value correspond to al least 2 of the 3 samples is considered to be the received value. In case the 3 values donot match, RIS.NERR bit is set along with RDR.NERR When disabled, only a single sample of received bit is taken."]
    #[inline(always)]
    pub fn ctl0_majvote(&mut self) -> Ctl0MajvoteW<Ctl0Spec> {
        Ctl0MajvoteW::new(self, 18)
    }
    #[doc = "Bit 19 - Most Significant Bit First This bit has effect both on the way protocol byte is transmitted and received. Notes: User needs to match the protocol to the correct value of this bit to send MSb or LSb first. The hardware engine will send the byte entirely based on this bit."]
    #[inline(always)]
    pub fn ctl0_msbfirst(&mut self) -> Ctl0MsbfirstW<Ctl0Spec> {
        Ctl0MsbfirstW::new(self, 19)
    }
}
#[doc = "UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x38"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x38;
}
