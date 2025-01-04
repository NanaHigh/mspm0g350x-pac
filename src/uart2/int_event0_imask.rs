#[doc = "Register `INT_EVENT0_IMASK` reader"]
pub type R = crate::R<IntEvent0ImaskSpec>;
#[doc = "Register `INT_EVENT0_IMASK` writer"]
pub type W = crate::W<IntEvent0ImaskSpec>;
#[doc = "Enable UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRtout {
    #[doc = "0: CLR"]
    IntEvent0ImaskRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRtoutSet = 1,
}
impl From<IntEvent0ImaskRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTOUT` reader - Enable UARTOUT Receive Time-Out Interrupt."]
pub type IntEvent0ImaskRtoutR = crate::BitReader<IntEvent0ImaskRtout>;
impl IntEvent0ImaskRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRtout {
        match self.bits {
            false => IntEvent0ImaskRtout::IntEvent0ImaskRtoutClr,
            true => IntEvent0ImaskRtout::IntEvent0ImaskRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtout_clr(&self) -> bool {
        *self == IntEvent0ImaskRtout::IntEvent0ImaskRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtout_set(&self) -> bool {
        *self == IntEvent0ImaskRtout::IntEvent0ImaskRtoutSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTOUT` writer - Enable UARTOUT Receive Time-Out Interrupt."]
pub type IntEvent0ImaskRtoutW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRtout>;
impl<'a, REG> IntEvent0ImaskRtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rtout_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtout::IntEvent0ImaskRtoutClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rtout_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtout::IntEvent0ImaskRtoutSet)
    }
}
#[doc = "Enable UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskFrmerr {
    #[doc = "0: CLR"]
    IntEvent0ImaskFrmerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskFrmerrSet = 1,
}
impl From<IntEvent0ImaskFrmerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskFrmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_FRMERR` reader - Enable UART Framing Error Interrupt."]
pub type IntEvent0ImaskFrmerrR = crate::BitReader<IntEvent0ImaskFrmerr>;
impl IntEvent0ImaskFrmerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskFrmerr {
        match self.bits {
            false => IntEvent0ImaskFrmerr::IntEvent0ImaskFrmerrClr,
            true => IntEvent0ImaskFrmerr::IntEvent0ImaskFrmerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_frmerr_clr(&self) -> bool {
        *self == IntEvent0ImaskFrmerr::IntEvent0ImaskFrmerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_frmerr_set(&self) -> bool {
        *self == IntEvent0ImaskFrmerr::IntEvent0ImaskFrmerrSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_FRMERR` writer - Enable UART Framing Error Interrupt."]
pub type IntEvent0ImaskFrmerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskFrmerr>;
impl<'a, REG> IntEvent0ImaskFrmerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_frmerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskFrmerr::IntEvent0ImaskFrmerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_frmerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskFrmerr::IntEvent0ImaskFrmerrSet)
    }
}
#[doc = "Enable UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskParerr {
    #[doc = "0: CLR"]
    IntEvent0ImaskParerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskParerrSet = 1,
}
impl From<IntEvent0ImaskParerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskParerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_PARERR` reader - Enable UART Parity Error Interrupt."]
pub type IntEvent0ImaskParerrR = crate::BitReader<IntEvent0ImaskParerr>;
impl IntEvent0ImaskParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskParerr {
        match self.bits {
            false => IntEvent0ImaskParerr::IntEvent0ImaskParerrClr,
            true => IntEvent0ImaskParerr::IntEvent0ImaskParerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_parerr_clr(&self) -> bool {
        *self == IntEvent0ImaskParerr::IntEvent0ImaskParerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_parerr_set(&self) -> bool {
        *self == IntEvent0ImaskParerr::IntEvent0ImaskParerrSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_PARERR` writer - Enable UART Parity Error Interrupt."]
pub type IntEvent0ImaskParerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskParerr>;
impl<'a, REG> IntEvent0ImaskParerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_parerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskParerr::IntEvent0ImaskParerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_parerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskParerr::IntEvent0ImaskParerrSet)
    }
}
#[doc = "Enable UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskBrkerr {
    #[doc = "0: CLR"]
    IntEvent0ImaskBrkerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskBrkerrSet = 1,
}
impl From<IntEvent0ImaskBrkerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskBrkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_BRKERR` reader - Enable UART Break Error Interrupt."]
pub type IntEvent0ImaskBrkerrR = crate::BitReader<IntEvent0ImaskBrkerr>;
impl IntEvent0ImaskBrkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskBrkerr {
        match self.bits {
            false => IntEvent0ImaskBrkerr::IntEvent0ImaskBrkerrClr,
            true => IntEvent0ImaskBrkerr::IntEvent0ImaskBrkerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_brkerr_clr(&self) -> bool {
        *self == IntEvent0ImaskBrkerr::IntEvent0ImaskBrkerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_brkerr_set(&self) -> bool {
        *self == IntEvent0ImaskBrkerr::IntEvent0ImaskBrkerrSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_BRKERR` writer - Enable UART Break Error Interrupt."]
pub type IntEvent0ImaskBrkerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskBrkerr>;
impl<'a, REG> IntEvent0ImaskBrkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_brkerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskBrkerr::IntEvent0ImaskBrkerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_brkerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskBrkerr::IntEvent0ImaskBrkerrSet)
    }
}
#[doc = "Enable UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskOvrerr {
    #[doc = "0: CLR"]
    IntEvent0ImaskOvrerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskOvrerrSet = 1,
}
impl From<IntEvent0ImaskOvrerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskOvrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_OVRERR` reader - Enable UART Receive Overrun Error Interrupt."]
pub type IntEvent0ImaskOvrerrR = crate::BitReader<IntEvent0ImaskOvrerr>;
impl IntEvent0ImaskOvrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskOvrerr {
        match self.bits {
            false => IntEvent0ImaskOvrerr::IntEvent0ImaskOvrerrClr,
            true => IntEvent0ImaskOvrerr::IntEvent0ImaskOvrerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_ovrerr_clr(&self) -> bool {
        *self == IntEvent0ImaskOvrerr::IntEvent0ImaskOvrerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_ovrerr_set(&self) -> bool {
        *self == IntEvent0ImaskOvrerr::IntEvent0ImaskOvrerrSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_OVRERR` writer - Enable UART Receive Overrun Error Interrupt."]
pub type IntEvent0ImaskOvrerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskOvrerr>;
impl<'a, REG> IntEvent0ImaskOvrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_ovrerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskOvrerr::IntEvent0ImaskOvrerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_ovrerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskOvrerr::IntEvent0ImaskOvrerrSet)
    }
}
#[doc = "Enable Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRxne {
    #[doc = "0: CLR"]
    IntEvent0ImaskRxneClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRxneSet = 1,
}
impl From<IntEvent0ImaskRxne> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXNE` reader - Enable Negative Edge on UARTxRXD Interrupt."]
pub type IntEvent0ImaskRxneR = crate::BitReader<IntEvent0ImaskRxne>;
impl IntEvent0ImaskRxneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRxne {
        match self.bits {
            false => IntEvent0ImaskRxne::IntEvent0ImaskRxneClr,
            true => IntEvent0ImaskRxne::IntEvent0ImaskRxneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxne_clr(&self) -> bool {
        *self == IntEvent0ImaskRxne::IntEvent0ImaskRxneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxne_set(&self) -> bool {
        *self == IntEvent0ImaskRxne::IntEvent0ImaskRxneSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXNE` writer - Enable Negative Edge on UARTxRXD Interrupt."]
pub type IntEvent0ImaskRxneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRxne>;
impl<'a, REG> IntEvent0ImaskRxneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rxne_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxne::IntEvent0ImaskRxneClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rxne_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxne::IntEvent0ImaskRxneSet)
    }
}
#[doc = "Enable Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRxpe {
    #[doc = "0: CLR"]
    IntEvent0ImaskRxpeClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRxpeSet = 1,
}
impl From<IntEvent0ImaskRxpe> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXPE` reader - Enable Positive Edge on UARTxRXD Interrupt."]
pub type IntEvent0ImaskRxpeR = crate::BitReader<IntEvent0ImaskRxpe>;
impl IntEvent0ImaskRxpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRxpe {
        match self.bits {
            false => IntEvent0ImaskRxpe::IntEvent0ImaskRxpeClr,
            true => IntEvent0ImaskRxpe::IntEvent0ImaskRxpeSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxpe_clr(&self) -> bool {
        *self == IntEvent0ImaskRxpe::IntEvent0ImaskRxpeClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxpe_set(&self) -> bool {
        *self == IntEvent0ImaskRxpe::IntEvent0ImaskRxpeSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXPE` writer - Enable Positive Edge on UARTxRXD Interrupt."]
pub type IntEvent0ImaskRxpeW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRxpe>;
impl<'a, REG> IntEvent0ImaskRxpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rxpe_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxpe::IntEvent0ImaskRxpeClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rxpe_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxpe::IntEvent0ImaskRxpeSet)
    }
}
#[doc = "Enable UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRxint {
    #[doc = "0: CLR"]
    IntEvent0ImaskRxintClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRxintSet = 1,
}
impl From<IntEvent0ImaskRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXINT` reader - Enable UART Receive Interrupt."]
pub type IntEvent0ImaskRxintR = crate::BitReader<IntEvent0ImaskRxint>;
impl IntEvent0ImaskRxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRxint {
        match self.bits {
            false => IntEvent0ImaskRxint::IntEvent0ImaskRxintClr,
            true => IntEvent0ImaskRxint::IntEvent0ImaskRxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxint_clr(&self) -> bool {
        *self == IntEvent0ImaskRxint::IntEvent0ImaskRxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxint_set(&self) -> bool {
        *self == IntEvent0ImaskRxint::IntEvent0ImaskRxintSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXINT` writer - Enable UART Receive Interrupt."]
pub type IntEvent0ImaskRxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRxint>;
impl<'a, REG> IntEvent0ImaskRxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rxint_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxint::IntEvent0ImaskRxintClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rxint_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxint::IntEvent0ImaskRxintSet)
    }
}
#[doc = "Enable UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskTxint {
    #[doc = "0: CLR"]
    IntEvent0ImaskTxintClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskTxintSet = 1,
}
impl From<IntEvent0ImaskTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TXINT` reader - Enable UART Transmit Interrupt."]
pub type IntEvent0ImaskTxintR = crate::BitReader<IntEvent0ImaskTxint>;
impl IntEvent0ImaskTxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskTxint {
        match self.bits {
            false => IntEvent0ImaskTxint::IntEvent0ImaskTxintClr,
            true => IntEvent0ImaskTxint::IntEvent0ImaskTxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_txint_clr(&self) -> bool {
        *self == IntEvent0ImaskTxint::IntEvent0ImaskTxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_txint_set(&self) -> bool {
        *self == IntEvent0ImaskTxint::IntEvent0ImaskTxintSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TXINT` writer - Enable UART Transmit Interrupt."]
pub type IntEvent0ImaskTxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskTxint>;
impl<'a, REG> IntEvent0ImaskTxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_txint_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTxint::IntEvent0ImaskTxintClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_txint_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTxint::IntEvent0ImaskTxintSet)
    }
}
#[doc = "Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskEot {
    #[doc = "0: CLR"]
    IntEvent0ImaskEotClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskEotSet = 1,
}
impl From<IntEvent0ImaskEot> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskEot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_EOT` reader - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type IntEvent0ImaskEotR = crate::BitReader<IntEvent0ImaskEot>;
impl IntEvent0ImaskEotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskEot {
        match self.bits {
            false => IntEvent0ImaskEot::IntEvent0ImaskEotClr,
            true => IntEvent0ImaskEot::IntEvent0ImaskEotSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_eot_clr(&self) -> bool {
        *self == IntEvent0ImaskEot::IntEvent0ImaskEotClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_eot_set(&self) -> bool {
        *self == IntEvent0ImaskEot::IntEvent0ImaskEotSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_EOT` writer - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type IntEvent0ImaskEotW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskEot>;
impl<'a, REG> IntEvent0ImaskEotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_eot_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskEot::IntEvent0ImaskEotClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_eot_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskEot::IntEvent0ImaskEotSet)
    }
}
#[doc = "Enable Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskAddrMatch {
    #[doc = "0: CLR"]
    IntEvent0ImaskAddrMatchClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskAddrMatchSet = 1,
}
impl From<IntEvent0ImaskAddrMatch> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskAddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_ADDR_MATCH` reader - Enable Address Match Interrupt."]
pub type IntEvent0ImaskAddrMatchR = crate::BitReader<IntEvent0ImaskAddrMatch>;
impl IntEvent0ImaskAddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskAddrMatch {
        match self.bits {
            false => IntEvent0ImaskAddrMatch::IntEvent0ImaskAddrMatchClr,
            true => IntEvent0ImaskAddrMatch::IntEvent0ImaskAddrMatchSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_addr_match_clr(&self) -> bool {
        *self == IntEvent0ImaskAddrMatch::IntEvent0ImaskAddrMatchClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_addr_match_set(&self) -> bool {
        *self == IntEvent0ImaskAddrMatch::IntEvent0ImaskAddrMatchSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_ADDR_MATCH` writer - Enable Address Match Interrupt."]
pub type IntEvent0ImaskAddrMatchW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskAddrMatch>;
impl<'a, REG> IntEvent0ImaskAddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_addr_match_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskAddrMatch::IntEvent0ImaskAddrMatchClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_addr_match_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskAddrMatch::IntEvent0ImaskAddrMatchSet)
    }
}
#[doc = "Enable UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskCts {
    #[doc = "0: CLR"]
    IntEvent0ImaskCtsClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskCtsSet = 1,
}
impl From<IntEvent0ImaskCts> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskCts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_CTS` reader - Enable UART Clear to Send Modem Interrupt."]
pub type IntEvent0ImaskCtsR = crate::BitReader<IntEvent0ImaskCts>;
impl IntEvent0ImaskCtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskCts {
        match self.bits {
            false => IntEvent0ImaskCts::IntEvent0ImaskCtsClr,
            true => IntEvent0ImaskCts::IntEvent0ImaskCtsSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_cts_clr(&self) -> bool {
        *self == IntEvent0ImaskCts::IntEvent0ImaskCtsClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_cts_set(&self) -> bool {
        *self == IntEvent0ImaskCts::IntEvent0ImaskCtsSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_CTS` writer - Enable UART Clear to Send Modem Interrupt."]
pub type IntEvent0ImaskCtsW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskCts>;
impl<'a, REG> IntEvent0ImaskCtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_cts_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskCts::IntEvent0ImaskCtsClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_cts_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskCts::IntEvent0ImaskCtsSet)
    }
}
#[doc = "Enable DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDmaDoneRx {
    #[doc = "0: CLR"]
    IntEvent0ImaskDmaDoneRxClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDmaDoneRxSet = 1,
}
impl From<IntEvent0ImaskDmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_RX` reader - Enable DMA Done on RX Event Channel Interrupt"]
pub type IntEvent0ImaskDmaDoneRxR = crate::BitReader<IntEvent0ImaskDmaDoneRx>;
impl IntEvent0ImaskDmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDmaDoneRx {
        match self.bits {
            false => IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxClr,
            true => IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_rx_clr(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_rx_set(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_RX` writer - Enable DMA Done on RX Event Channel Interrupt"]
pub type IntEvent0ImaskDmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDmaDoneRx>;
impl<'a, REG> IntEvent0ImaskDmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxSet)
    }
}
#[doc = "Enable DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDmaDoneTx {
    #[doc = "0: CLR"]
    IntEvent0ImaskDmaDoneTxClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDmaDoneTxSet = 1,
}
impl From<IntEvent0ImaskDmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_TX` reader - Enable DMA Done on TX Event Channel Interrupt"]
pub type IntEvent0ImaskDmaDoneTxR = crate::BitReader<IntEvent0ImaskDmaDoneTx>;
impl IntEvent0ImaskDmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDmaDoneTx {
        match self.bits {
            false => IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxClr,
            true => IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_tx_clr(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_tx_set(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_TX` writer - Enable DMA Done on TX Event Channel Interrupt"]
pub type IntEvent0ImaskDmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDmaDoneTx>;
impl<'a, REG> IntEvent0ImaskDmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxSet)
    }
}
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskNerr {
    #[doc = "0: CLR"]
    IntEvent0ImaskNerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskNerrSet = 1,
}
impl From<IntEvent0ImaskNerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskNerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_NERR` reader - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type IntEvent0ImaskNerrR = crate::BitReader<IntEvent0ImaskNerr>;
impl IntEvent0ImaskNerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskNerr {
        match self.bits {
            false => IntEvent0ImaskNerr::IntEvent0ImaskNerrClr,
            true => IntEvent0ImaskNerr::IntEvent0ImaskNerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_nerr_clr(&self) -> bool {
        *self == IntEvent0ImaskNerr::IntEvent0ImaskNerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_nerr_set(&self) -> bool {
        *self == IntEvent0ImaskNerr::IntEvent0ImaskNerrSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type IntEvent0ImaskNerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskNerr>;
impl<'a, REG> IntEvent0ImaskNerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_nerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskNerr::IntEvent0ImaskNerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_nerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskNerr::IntEvent0ImaskNerrSet)
    }
}
impl R {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rtout(&self) -> IntEvent0ImaskRtoutR {
        IntEvent0ImaskRtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_frmerr(&self) -> IntEvent0ImaskFrmerrR {
        IntEvent0ImaskFrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_parerr(&self) -> IntEvent0ImaskParerrR {
        IntEvent0ImaskParerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable UART Break Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_brkerr(&self) -> IntEvent0ImaskBrkerrR {
        IntEvent0ImaskBrkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_ovrerr(&self) -> IntEvent0ImaskOvrerrR {
        IntEvent0ImaskOvrerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rxne(&self) -> IntEvent0ImaskRxneR {
        IntEvent0ImaskRxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rxpe(&self) -> IntEvent0ImaskRxpeR {
        IntEvent0ImaskRxpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rxint(&self) -> IntEvent0ImaskRxintR {
        IntEvent0ImaskRxintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_txint(&self) -> IntEvent0ImaskTxintR {
        IntEvent0ImaskTxintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn int_event0_imask_eot(&self) -> IntEvent0ImaskEotR {
        IntEvent0ImaskEotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Address Match Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_addr_match(&self) -> IntEvent0ImaskAddrMatchR {
        IntEvent0ImaskAddrMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_cts(&self) -> IntEvent0ImaskCtsR {
        IntEvent0ImaskCtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx(&self) -> IntEvent0ImaskDmaDoneRxR {
        IntEvent0ImaskDmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx(&self) -> IntEvent0ImaskDmaDoneTxR {
        IntEvent0ImaskDmaDoneTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn int_event0_imask_nerr(&self) -> IntEvent0ImaskNerrR {
        IntEvent0ImaskNerrR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rtout(&mut self) -> IntEvent0ImaskRtoutW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRtoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_frmerr(&mut self) -> IntEvent0ImaskFrmerrW<IntEvent0ImaskSpec> {
        IntEvent0ImaskFrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_parerr(&mut self) -> IntEvent0ImaskParerrW<IntEvent0ImaskSpec> {
        IntEvent0ImaskParerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable UART Break Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_brkerr(&mut self) -> IntEvent0ImaskBrkerrW<IntEvent0ImaskSpec> {
        IntEvent0ImaskBrkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_ovrerr(&mut self) -> IntEvent0ImaskOvrerrW<IntEvent0ImaskSpec> {
        IntEvent0ImaskOvrerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rxne(&mut self) -> IntEvent0ImaskRxneW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rxpe(&mut self) -> IntEvent0ImaskRxpeW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRxpeW::new(self, 6)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_rxint(&mut self) -> IntEvent0ImaskRxintW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRxintW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_txint(&mut self) -> IntEvent0ImaskTxintW<IntEvent0ImaskSpec> {
        IntEvent0ImaskTxintW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn int_event0_imask_eot(&mut self) -> IntEvent0ImaskEotW<IntEvent0ImaskSpec> {
        IntEvent0ImaskEotW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Address Match Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_addr_match(&mut self) -> IntEvent0ImaskAddrMatchW<IntEvent0ImaskSpec> {
        IntEvent0ImaskAddrMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn int_event0_imask_cts(&mut self) -> IntEvent0ImaskCtsW<IntEvent0ImaskSpec> {
        IntEvent0ImaskCtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx(&mut self) -> IntEvent0ImaskDmaDoneRxW<IntEvent0ImaskSpec> {
        IntEvent0ImaskDmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx(&mut self) -> IntEvent0ImaskDmaDoneTxW<IntEvent0ImaskSpec> {
        IntEvent0ImaskDmaDoneTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn int_event0_imask_nerr(&mut self) -> IntEvent0ImaskNerrW<IntEvent0ImaskSpec> {
        IntEvent0ImaskNerrW::new(self, 17)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0ImaskSpec;
impl crate::RegisterSpec for IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for IntEvent0ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
