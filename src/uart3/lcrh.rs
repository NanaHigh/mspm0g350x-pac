#[doc = "Register `LCRH` reader"]
pub type R = crate::R<LcrhSpec>;
#[doc = "Register `LCRH` writer"]
pub type W = crate::W<LcrhSpec>;
#[doc = "UART Send Break (for LIN Protocol)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcrhBrk {
    #[doc = "0: DISABLE"]
    LcrhBrkDisable = 0,
    #[doc = "1: ENABLE"]
    LcrhBrkEnable = 1,
}
impl From<LcrhBrk> for bool {
    #[inline(always)]
    fn from(variant: LcrhBrk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCRH_BRK` reader - UART Send Break (for LIN Protocol)"]
pub type LcrhBrkR = crate::BitReader<LcrhBrk>;
impl LcrhBrkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcrhBrk {
        match self.bits {
            false => LcrhBrk::LcrhBrkDisable,
            true => LcrhBrk::LcrhBrkEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_lcrh_brk_disable(&self) -> bool {
        *self == LcrhBrk::LcrhBrkDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_lcrh_brk_enable(&self) -> bool {
        *self == LcrhBrk::LcrhBrkEnable
    }
}
#[doc = "Field `LCRH_BRK` writer - UART Send Break (for LIN Protocol)"]
pub type LcrhBrkW<'a, REG> = crate::BitWriter<'a, REG, LcrhBrk>;
impl<'a, REG> LcrhBrkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn lcrh_brk_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhBrk::LcrhBrkDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn lcrh_brk_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhBrk::LcrhBrkEnable)
    }
}
#[doc = "UART Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcrhPen {
    #[doc = "0: DISABLE"]
    LcrhPenDisable = 0,
    #[doc = "1: ENABLE"]
    LcrhPenEnable = 1,
}
impl From<LcrhPen> for bool {
    #[inline(always)]
    fn from(variant: LcrhPen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCRH_PEN` reader - UART Parity Enable"]
pub type LcrhPenR = crate::BitReader<LcrhPen>;
impl LcrhPenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcrhPen {
        match self.bits {
            false => LcrhPen::LcrhPenDisable,
            true => LcrhPen::LcrhPenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_lcrh_pen_disable(&self) -> bool {
        *self == LcrhPen::LcrhPenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_lcrh_pen_enable(&self) -> bool {
        *self == LcrhPen::LcrhPenEnable
    }
}
#[doc = "Field `LCRH_PEN` writer - UART Parity Enable"]
pub type LcrhPenW<'a, REG> = crate::BitWriter<'a, REG, LcrhPen>;
impl<'a, REG> LcrhPenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn lcrh_pen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhPen::LcrhPenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn lcrh_pen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhPen::LcrhPenEnable)
    }
}
#[doc = "UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcrhEps {
    #[doc = "0: ODD"]
    LcrhEpsOdd = 0,
    #[doc = "1: EVEN"]
    LcrhEpsEven = 1,
}
impl From<LcrhEps> for bool {
    #[inline(always)]
    fn from(variant: LcrhEps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCRH_EPS` reader - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
pub type LcrhEpsR = crate::BitReader<LcrhEps>;
impl LcrhEpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcrhEps {
        match self.bits {
            false => LcrhEps::LcrhEpsOdd,
            true => LcrhEps::LcrhEpsEven,
        }
    }
    #[doc = "ODD"]
    #[inline(always)]
    pub fn is_lcrh_eps_odd(&self) -> bool {
        *self == LcrhEps::LcrhEpsOdd
    }
    #[doc = "EVEN"]
    #[inline(always)]
    pub fn is_lcrh_eps_even(&self) -> bool {
        *self == LcrhEps::LcrhEpsEven
    }
}
#[doc = "Field `LCRH_EPS` writer - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
pub type LcrhEpsW<'a, REG> = crate::BitWriter<'a, REG, LcrhEps>;
impl<'a, REG> LcrhEpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ODD"]
    #[inline(always)]
    pub fn lcrh_eps_odd(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhEps::LcrhEpsOdd)
    }
    #[doc = "EVEN"]
    #[inline(always)]
    pub fn lcrh_eps_even(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhEps::LcrhEpsEven)
    }
}
#[doc = "UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcrhStp2 {
    #[doc = "0: DISABLE"]
    LcrhStp2Disable = 0,
    #[doc = "1: ENABLE"]
    LcrhStp2Enable = 1,
}
impl From<LcrhStp2> for bool {
    #[inline(always)]
    fn from(variant: LcrhStp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCRH_STP2` reader - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
pub type LcrhStp2R = crate::BitReader<LcrhStp2>;
impl LcrhStp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcrhStp2 {
        match self.bits {
            false => LcrhStp2::LcrhStp2Disable,
            true => LcrhStp2::LcrhStp2Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_lcrh_stp2_disable(&self) -> bool {
        *self == LcrhStp2::LcrhStp2Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_lcrh_stp2_enable(&self) -> bool {
        *self == LcrhStp2::LcrhStp2Enable
    }
}
#[doc = "Field `LCRH_STP2` writer - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
pub type LcrhStp2W<'a, REG> = crate::BitWriter<'a, REG, LcrhStp2>;
impl<'a, REG> LcrhStp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn lcrh_stp2_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhStp2::LcrhStp2Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn lcrh_stp2_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhStp2::LcrhStp2Enable)
    }
}
#[doc = "UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LcrhWlen {
    #[doc = "0: DATABIT5"]
    LcrhWlenDatabit5 = 0,
    #[doc = "1: DATABIT6"]
    LcrhWlenDatabit6 = 1,
    #[doc = "2: DATABIT7"]
    LcrhWlenDatabit7 = 2,
    #[doc = "3: DATABIT8"]
    LcrhWlenDatabit8 = 3,
}
impl From<LcrhWlen> for u8 {
    #[inline(always)]
    fn from(variant: LcrhWlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LcrhWlen {
    type Ux = u8;
}
impl crate::IsEnum for LcrhWlen {}
#[doc = "Field `LCRH_WLEN` reader - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
pub type LcrhWlenR = crate::FieldReader<LcrhWlen>;
impl LcrhWlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcrhWlen {
        match self.bits {
            0 => LcrhWlen::LcrhWlenDatabit5,
            1 => LcrhWlen::LcrhWlenDatabit6,
            2 => LcrhWlen::LcrhWlenDatabit7,
            3 => LcrhWlen::LcrhWlenDatabit8,
            _ => unreachable!(),
        }
    }
    #[doc = "DATABIT5"]
    #[inline(always)]
    pub fn is_lcrh_wlen_databit5(&self) -> bool {
        *self == LcrhWlen::LcrhWlenDatabit5
    }
    #[doc = "DATABIT6"]
    #[inline(always)]
    pub fn is_lcrh_wlen_databit6(&self) -> bool {
        *self == LcrhWlen::LcrhWlenDatabit6
    }
    #[doc = "DATABIT7"]
    #[inline(always)]
    pub fn is_lcrh_wlen_databit7(&self) -> bool {
        *self == LcrhWlen::LcrhWlenDatabit7
    }
    #[doc = "DATABIT8"]
    #[inline(always)]
    pub fn is_lcrh_wlen_databit8(&self) -> bool {
        *self == LcrhWlen::LcrhWlenDatabit8
    }
}
#[doc = "Field `LCRH_WLEN` writer - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
pub type LcrhWlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, LcrhWlen, crate::Safe>;
impl<'a, REG> LcrhWlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATABIT5"]
    #[inline(always)]
    pub fn lcrh_wlen_databit5(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhWlen::LcrhWlenDatabit5)
    }
    #[doc = "DATABIT6"]
    #[inline(always)]
    pub fn lcrh_wlen_databit6(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhWlen::LcrhWlenDatabit6)
    }
    #[doc = "DATABIT7"]
    #[inline(always)]
    pub fn lcrh_wlen_databit7(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhWlen::LcrhWlenDatabit7)
    }
    #[doc = "DATABIT8"]
    #[inline(always)]
    pub fn lcrh_wlen_databit8(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhWlen::LcrhWlenDatabit8)
    }
}
#[doc = "UART Stick Parity Select The Stick Parity Select (SPS) bit is used to set either a permanent '1' or a permanent '0' as parity when transmitting or receiving data. Its purpose is to typically indicate the first byte of a package or to mark an address byte, for example in a multi-drop RS-485 network. When bits PEN, EPS, and SPS of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits PEN and SPS are set and EPS is cleared, the parity bit is transmitted and checked as a 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcrhSps {
    #[doc = "0: DISABLE"]
    LcrhSpsDisable = 0,
    #[doc = "1: ENABLE"]
    LcrhSpsEnable = 1,
}
impl From<LcrhSps> for bool {
    #[inline(always)]
    fn from(variant: LcrhSps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCRH_SPS` reader - UART Stick Parity Select The Stick Parity Select (SPS) bit is used to set either a permanent '1' or a permanent '0' as parity when transmitting or receiving data. Its purpose is to typically indicate the first byte of a package or to mark an address byte, for example in a multi-drop RS-485 network. When bits PEN, EPS, and SPS of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits PEN and SPS are set and EPS is cleared, the parity bit is transmitted and checked as a 1."]
pub type LcrhSpsR = crate::BitReader<LcrhSps>;
impl LcrhSpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcrhSps {
        match self.bits {
            false => LcrhSps::LcrhSpsDisable,
            true => LcrhSps::LcrhSpsEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_lcrh_sps_disable(&self) -> bool {
        *self == LcrhSps::LcrhSpsDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_lcrh_sps_enable(&self) -> bool {
        *self == LcrhSps::LcrhSpsEnable
    }
}
#[doc = "Field `LCRH_SPS` writer - UART Stick Parity Select The Stick Parity Select (SPS) bit is used to set either a permanent '1' or a permanent '0' as parity when transmitting or receiving data. Its purpose is to typically indicate the first byte of a package or to mark an address byte, for example in a multi-drop RS-485 network. When bits PEN, EPS, and SPS of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits PEN and SPS are set and EPS is cleared, the parity bit is transmitted and checked as a 1."]
pub type LcrhSpsW<'a, REG> = crate::BitWriter<'a, REG, LcrhSps>;
impl<'a, REG> LcrhSpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn lcrh_sps_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhSps::LcrhSpsDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn lcrh_sps_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhSps::LcrhSpsEnable)
    }
}
#[doc = "UART send IDLE pattern. When this bit is set an SENDIDLE period of 11 bit times will be sent on the TX line. The bit is cleared by hardware afterwards.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcrhSendidle {
    #[doc = "0: DISABLE"]
    LcrhSendidleDisable = 0,
    #[doc = "1: ENABLE"]
    LcrhSendidleEnable = 1,
}
impl From<LcrhSendidle> for bool {
    #[inline(always)]
    fn from(variant: LcrhSendidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCRH_SENDIDLE` reader - UART send IDLE pattern. When this bit is set an SENDIDLE period of 11 bit times will be sent on the TX line. The bit is cleared by hardware afterwards."]
pub type LcrhSendidleR = crate::BitReader<LcrhSendidle>;
impl LcrhSendidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcrhSendidle {
        match self.bits {
            false => LcrhSendidle::LcrhSendidleDisable,
            true => LcrhSendidle::LcrhSendidleEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_lcrh_sendidle_disable(&self) -> bool {
        *self == LcrhSendidle::LcrhSendidleDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_lcrh_sendidle_enable(&self) -> bool {
        *self == LcrhSendidle::LcrhSendidleEnable
    }
}
#[doc = "Field `LCRH_SENDIDLE` writer - UART send IDLE pattern. When this bit is set an SENDIDLE period of 11 bit times will be sent on the TX line. The bit is cleared by hardware afterwards."]
pub type LcrhSendidleW<'a, REG> = crate::BitWriter<'a, REG, LcrhSendidle>;
impl<'a, REG> LcrhSendidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn lcrh_sendidle_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhSendidle::LcrhSendidleDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn lcrh_sendidle_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LcrhSendidle::LcrhSendidleEnable)
    }
}
#[doc = "Field `LCRH_EXTDIR_SETUP` reader - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
pub type LcrhExtdirSetupR = crate::FieldReader;
#[doc = "Field `LCRH_EXTDIR_SETUP` writer - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
pub type LcrhExtdirSetupW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LCRH_EXTDIR_HOLD` reader - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
pub type LcrhExtdirHoldR = crate::FieldReader;
#[doc = "Field `LCRH_EXTDIR_HOLD` writer - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
pub type LcrhExtdirHoldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - UART Send Break (for LIN Protocol)"]
    #[inline(always)]
    pub fn lcrh_brk(&self) -> LcrhBrkR {
        LcrhBrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn lcrh_pen(&self) -> LcrhPenR {
        LcrhPenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
    #[inline(always)]
    pub fn lcrh_eps(&self) -> LcrhEpsR {
        LcrhEpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
    #[inline(always)]
    pub fn lcrh_stp2(&self) -> LcrhStp2R {
        LcrhStp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
    #[inline(always)]
    pub fn lcrh_wlen(&self) -> LcrhWlenR {
        LcrhWlenR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - UART Stick Parity Select The Stick Parity Select (SPS) bit is used to set either a permanent '1' or a permanent '0' as parity when transmitting or receiving data. Its purpose is to typically indicate the first byte of a package or to mark an address byte, for example in a multi-drop RS-485 network. When bits PEN, EPS, and SPS of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits PEN and SPS are set and EPS is cleared, the parity bit is transmitted and checked as a 1."]
    #[inline(always)]
    pub fn lcrh_sps(&self) -> LcrhSpsR {
        LcrhSpsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART send IDLE pattern. When this bit is set an SENDIDLE period of 11 bit times will be sent on the TX line. The bit is cleared by hardware afterwards."]
    #[inline(always)]
    pub fn lcrh_sendidle(&self) -> LcrhSendidleR {
        LcrhSendidleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
    #[inline(always)]
    pub fn lcrh_extdir_setup(&self) -> LcrhExtdirSetupR {
        LcrhExtdirSetupR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
    #[inline(always)]
    pub fn lcrh_extdir_hold(&self) -> LcrhExtdirHoldR {
        LcrhExtdirHoldR::new(((self.bits >> 21) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART Send Break (for LIN Protocol)"]
    #[inline(always)]
    pub fn lcrh_brk(&mut self) -> LcrhBrkW<LcrhSpec> {
        LcrhBrkW::new(self, 0)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn lcrh_pen(&mut self) -> LcrhPenW<LcrhSpec> {
        LcrhPenW::new(self, 1)
    }
    #[doc = "Bit 2 - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
    #[inline(always)]
    pub fn lcrh_eps(&mut self) -> LcrhEpsW<LcrhSpec> {
        LcrhEpsW::new(self, 2)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
    #[inline(always)]
    pub fn lcrh_stp2(&mut self) -> LcrhStp2W<LcrhSpec> {
        LcrhStp2W::new(self, 3)
    }
    #[doc = "Bits 4:5 - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
    #[inline(always)]
    pub fn lcrh_wlen(&mut self) -> LcrhWlenW<LcrhSpec> {
        LcrhWlenW::new(self, 4)
    }
    #[doc = "Bit 6 - UART Stick Parity Select The Stick Parity Select (SPS) bit is used to set either a permanent '1' or a permanent '0' as parity when transmitting or receiving data. Its purpose is to typically indicate the first byte of a package or to mark an address byte, for example in a multi-drop RS-485 network. When bits PEN, EPS, and SPS of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits PEN and SPS are set and EPS is cleared, the parity bit is transmitted and checked as a 1."]
    #[inline(always)]
    pub fn lcrh_sps(&mut self) -> LcrhSpsW<LcrhSpec> {
        LcrhSpsW::new(self, 6)
    }
    #[doc = "Bit 7 - UART send IDLE pattern. When this bit is set an SENDIDLE period of 11 bit times will be sent on the TX line. The bit is cleared by hardware afterwards."]
    #[inline(always)]
    pub fn lcrh_sendidle(&mut self) -> LcrhSendidleW<LcrhSpec> {
        LcrhSendidleW::new(self, 7)
    }
    #[doc = "Bits 16:20 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
    #[inline(always)]
    pub fn lcrh_extdir_setup(&mut self) -> LcrhExtdirSetupW<LcrhSpec> {
        LcrhExtdirSetupW::new(self, 16)
    }
    #[doc = "Bits 21:25 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
    #[inline(always)]
    pub fn lcrh_extdir_hold(&mut self) -> LcrhExtdirHoldW<LcrhSpec> {
        LcrhExtdirHoldW::new(self, 21)
    }
}
#[doc = "UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrhSpec;
impl crate::RegisterSpec for LcrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrh::R`](R) reader structure"]
impl crate::Readable for LcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`lcrh::W`](W) writer structure"]
impl crate::Writable for LcrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LcrhSpec {
    const RESET_VALUE: u32 = 0;
}
