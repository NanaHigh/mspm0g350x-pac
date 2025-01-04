#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Field `RXDATA_DATA` reader - Received Data. When read, this field contains the data that was received by the UART."]
pub type RxdataDataR = crate::FieldReader;
#[doc = "UART Framing Error Writing to this bit has no effect. The flag is cleared by writing 1 to the FRMERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdataFrmerr {
    #[doc = "0: CLR"]
    RxdataFrmerrClr = 0,
    #[doc = "1: SET"]
    RxdataFrmerrSet = 1,
}
impl From<RxdataFrmerr> for bool {
    #[inline(always)]
    fn from(variant: RxdataFrmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDATA_FRMERR` reader - UART Framing Error Writing to this bit has no effect. The flag is cleared by writing 1 to the FRMERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO."]
pub type RxdataFrmerrR = crate::BitReader<RxdataFrmerr>;
impl RxdataFrmerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxdataFrmerr {
        match self.bits {
            false => RxdataFrmerr::RxdataFrmerrClr,
            true => RxdataFrmerr::RxdataFrmerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_rxdata_frmerr_clr(&self) -> bool {
        *self == RxdataFrmerr::RxdataFrmerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_rxdata_frmerr_set(&self) -> bool {
        *self == RxdataFrmerr::RxdataFrmerrSet
    }
}
#[doc = "UART Parity Error Writing to this bit has no effect. The flag is cleared by writing 1 to the PARERR bit in the UART EVENT ICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdataParerr {
    #[doc = "0: CLR"]
    RxdataParerrClr = 0,
    #[doc = "1: SET"]
    RxdataParerrSet = 1,
}
impl From<RxdataParerr> for bool {
    #[inline(always)]
    fn from(variant: RxdataParerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDATA_PARERR` reader - UART Parity Error Writing to this bit has no effect. The flag is cleared by writing 1 to the PARERR bit in the UART EVENT ICLR register."]
pub type RxdataParerrR = crate::BitReader<RxdataParerr>;
impl RxdataParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxdataParerr {
        match self.bits {
            false => RxdataParerr::RxdataParerrClr,
            true => RxdataParerr::RxdataParerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_rxdata_parerr_clr(&self) -> bool {
        *self == RxdataParerr::RxdataParerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_rxdata_parerr_set(&self) -> bool {
        *self == RxdataParerr::RxdataParerrSet
    }
}
#[doc = "UART Break Error Writing to this bit has no effect. The flag is cleared by writing 1 to the BRKERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdataBrkerr {
    #[doc = "0: CLR"]
    RxdataBrkerrClr = 0,
    #[doc = "1: SET"]
    RxdataBrkerrSet = 1,
}
impl From<RxdataBrkerr> for bool {
    #[inline(always)]
    fn from(variant: RxdataBrkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDATA_BRKERR` reader - UART Break Error Writing to this bit has no effect. The flag is cleared by writing 1 to the BRKERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
pub type RxdataBrkerrR = crate::BitReader<RxdataBrkerr>;
impl RxdataBrkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxdataBrkerr {
        match self.bits {
            false => RxdataBrkerr::RxdataBrkerrClr,
            true => RxdataBrkerr::RxdataBrkerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_rxdata_brkerr_clr(&self) -> bool {
        *self == RxdataBrkerr::RxdataBrkerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_rxdata_brkerr_set(&self) -> bool {
        *self == RxdataBrkerr::RxdataBrkerrSet
    }
}
#[doc = "UART Receive Overrun Error Writing to this bit has no effect. The flag is cleared by writing 1 to the OVRERR bit in the UART EVENT ICLR register. In case of a receive FIFO overflow, the FIFO contents remain valid because no further data is written when the FIFO is full. Only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdataOvrerr {
    #[doc = "0: CLR"]
    RxdataOvrerrClr = 0,
    #[doc = "1: SET"]
    RxdataOvrerrSet = 1,
}
impl From<RxdataOvrerr> for bool {
    #[inline(always)]
    fn from(variant: RxdataOvrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDATA_OVRERR` reader - UART Receive Overrun Error Writing to this bit has no effect. The flag is cleared by writing 1 to the OVRERR bit in the UART EVENT ICLR register. In case of a receive FIFO overflow, the FIFO contents remain valid because no further data is written when the FIFO is full. Only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO."]
pub type RxdataOvrerrR = crate::BitReader<RxdataOvrerr>;
impl RxdataOvrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxdataOvrerr {
        match self.bits {
            false => RxdataOvrerr::RxdataOvrerrClr,
            true => RxdataOvrerr::RxdataOvrerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_rxdata_ovrerr_clr(&self) -> bool {
        *self == RxdataOvrerr::RxdataOvrerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_rxdata_ovrerr_set(&self) -> bool {
        *self == RxdataOvrerr::RxdataOvrerrSet
    }
}
#[doc = "Noise Error. Writing to this bit has no effect. The flag is cleared by writing 1 to the NERR bit in the UART EVENT ICLR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdataNerr {
    #[doc = "0: CLR"]
    RxdataNerrClr = 0,
    #[doc = "1: SET"]
    RxdataNerrSet = 1,
}
impl From<RxdataNerr> for bool {
    #[inline(always)]
    fn from(variant: RxdataNerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDATA_NERR` reader - Noise Error. Writing to this bit has no effect. The flag is cleared by writing 1 to the NERR bit in the UART EVENT ICLR register."]
pub type RxdataNerrR = crate::BitReader<RxdataNerr>;
impl RxdataNerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxdataNerr {
        match self.bits {
            false => RxdataNerr::RxdataNerrClr,
            true => RxdataNerr::RxdataNerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_rxdata_nerr_clr(&self) -> bool {
        *self == RxdataNerr::RxdataNerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_rxdata_nerr_set(&self) -> bool {
        *self == RxdataNerr::RxdataNerrSet
    }
}
impl R {
    #[doc = "Bits 0:7 - Received Data. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn rxdata_data(&self) -> RxdataDataR {
        RxdataDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - UART Framing Error Writing to this bit has no effect. The flag is cleared by writing 1 to the FRMERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn rxdata_frmerr(&self) -> RxdataFrmerrR {
        RxdataFrmerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Parity Error Writing to this bit has no effect. The flag is cleared by writing 1 to the PARERR bit in the UART EVENT ICLR register."]
    #[inline(always)]
    pub fn rxdata_parerr(&self) -> RxdataParerrR {
        RxdataParerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Break Error Writing to this bit has no effect. The flag is cleared by writing 1 to the BRKERR bit in the UART EVENT ICLR register. This error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    pub fn rxdata_brkerr(&self) -> RxdataBrkerrR {
        RxdataBrkerrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART Receive Overrun Error Writing to this bit has no effect. The flag is cleared by writing 1 to the OVRERR bit in the UART EVENT ICLR register. In case of a receive FIFO overflow, the FIFO contents remain valid because no further data is written when the FIFO is full. Only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO."]
    #[inline(always)]
    pub fn rxdata_ovrerr(&self) -> RxdataOvrerrR {
        RxdataOvrerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Noise Error. Writing to this bit has no effect. The flag is cleared by writing 1 to the NERR bit in the UART EVENT ICLR register."]
    #[inline(always)]
    pub fn rxdata_nerr(&self) -> RxdataNerrR {
        RxdataNerrR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RxdataSpec {
    const RESET_VALUE: u32 = 0;
}
