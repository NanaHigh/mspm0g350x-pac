#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IflsTxiflsel {
    #[doc = "0: LVL_OFF"]
    IflsTxiflselLvlOff = 0,
    #[doc = "1: LVL_3_4"]
    IflsTxiflselLvl3_4 = 1,
    #[doc = "2: LVL_1_2"]
    IflsTxiflselLvl1_2 = 2,
    #[doc = "3: LVL_1_4"]
    IflsTxiflselLvl1_4 = 3,
    #[doc = "4: LVL_RES4"]
    IflsTxiflselLvlRes4 = 4,
    #[doc = "5: LVL_EMPTY"]
    IflsTxiflselLvlEmpty = 5,
    #[doc = "6: LVL_RES6"]
    IflsTxiflselLvlRes6 = 6,
    #[doc = "7: LEVEL_1"]
    IflsTxiflselLevel1 = 7,
}
impl From<IflsTxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: IflsTxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IflsTxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for IflsTxiflsel {}
#[doc = "Field `IFLS_TXIFLSEL` reader - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type IflsTxiflselR = crate::FieldReader<IflsTxiflsel>;
impl IflsTxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IflsTxiflsel {
        match self.bits {
            0 => IflsTxiflsel::IflsTxiflselLvlOff,
            1 => IflsTxiflsel::IflsTxiflselLvl3_4,
            2 => IflsTxiflsel::IflsTxiflselLvl1_2,
            3 => IflsTxiflsel::IflsTxiflselLvl1_4,
            4 => IflsTxiflsel::IflsTxiflselLvlRes4,
            5 => IflsTxiflsel::IflsTxiflselLvlEmpty,
            6 => IflsTxiflsel::IflsTxiflselLvlRes6,
            7 => IflsTxiflsel::IflsTxiflselLevel1,
            _ => unreachable!(),
        }
    }
    #[doc = "LVL_OFF"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_lvl_off(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLvlOff
    }
    #[doc = "LVL_3_4"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_lvl_3_4(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLvl3_4
    }
    #[doc = "LVL_1_2"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_lvl_1_2(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLvl1_2
    }
    #[doc = "LVL_1_4"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_lvl_1_4(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLvl1_4
    }
    #[doc = "LVL_RES4"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_lvl_res4(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLvlRes4
    }
    #[doc = "LVL_EMPTY"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_lvl_empty(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLvlEmpty
    }
    #[doc = "LVL_RES6"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_lvl_res6(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLvlRes6
    }
    #[doc = "LEVEL_1"]
    #[inline(always)]
    pub fn is_ifls_txiflsel_level_1(&self) -> bool {
        *self == IflsTxiflsel::IflsTxiflselLevel1
    }
}
#[doc = "Field `IFLS_TXIFLSEL` writer - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type IflsTxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, IflsTxiflsel, crate::Safe>;
impl<'a, REG> IflsTxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LVL_OFF"]
    #[inline(always)]
    pub fn ifls_txiflsel_lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLvlOff)
    }
    #[doc = "LVL_3_4"]
    #[inline(always)]
    pub fn ifls_txiflsel_lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLvl3_4)
    }
    #[doc = "LVL_1_2"]
    #[inline(always)]
    pub fn ifls_txiflsel_lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLvl1_2)
    }
    #[doc = "LVL_1_4"]
    #[inline(always)]
    pub fn ifls_txiflsel_lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLvl1_4)
    }
    #[doc = "LVL_RES4"]
    #[inline(always)]
    pub fn ifls_txiflsel_lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLvlRes4)
    }
    #[doc = "LVL_EMPTY"]
    #[inline(always)]
    pub fn ifls_txiflsel_lvl_empty(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLvlEmpty)
    }
    #[doc = "LVL_RES6"]
    #[inline(always)]
    pub fn ifls_txiflsel_lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLvlRes6)
    }
    #[doc = "LEVEL_1"]
    #[inline(always)]
    pub fn ifls_txiflsel_level_1(self) -> &'a mut crate::W<REG> {
        self.variant(IflsTxiflsel::IflsTxiflselLevel1)
    }
}
#[doc = "SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IflsRxiflsel {
    #[doc = "0: LVL_OFF"]
    IflsRxiflselLvlOff = 0,
    #[doc = "1: LVL_1_4"]
    IflsRxiflselLvl1_4 = 1,
    #[doc = "2: LVL_1_2"]
    IflsRxiflselLvl1_2 = 2,
    #[doc = "3: LVL_3_4"]
    IflsRxiflselLvl3_4 = 3,
    #[doc = "4: LVL_RES4"]
    IflsRxiflselLvlRes4 = 4,
    #[doc = "5: LVL_FULL"]
    IflsRxiflselLvlFull = 5,
    #[doc = "6: LVL_RES6"]
    IflsRxiflselLvlRes6 = 6,
    #[doc = "7: LEVEL_1"]
    IflsRxiflselLevel1 = 7,
}
impl From<IflsRxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: IflsRxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IflsRxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for IflsRxiflsel {}
#[doc = "Field `IFLS_RXIFLSEL` reader - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type IflsRxiflselR = crate::FieldReader<IflsRxiflsel>;
impl IflsRxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IflsRxiflsel {
        match self.bits {
            0 => IflsRxiflsel::IflsRxiflselLvlOff,
            1 => IflsRxiflsel::IflsRxiflselLvl1_4,
            2 => IflsRxiflsel::IflsRxiflselLvl1_2,
            3 => IflsRxiflsel::IflsRxiflselLvl3_4,
            4 => IflsRxiflsel::IflsRxiflselLvlRes4,
            5 => IflsRxiflsel::IflsRxiflselLvlFull,
            6 => IflsRxiflsel::IflsRxiflselLvlRes6,
            7 => IflsRxiflsel::IflsRxiflselLevel1,
            _ => unreachable!(),
        }
    }
    #[doc = "LVL_OFF"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_lvl_off(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLvlOff
    }
    #[doc = "LVL_1_4"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_lvl_1_4(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLvl1_4
    }
    #[doc = "LVL_1_2"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_lvl_1_2(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLvl1_2
    }
    #[doc = "LVL_3_4"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_lvl_3_4(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLvl3_4
    }
    #[doc = "LVL_RES4"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_lvl_res4(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLvlRes4
    }
    #[doc = "LVL_FULL"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_lvl_full(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLvlFull
    }
    #[doc = "LVL_RES6"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_lvl_res6(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLvlRes6
    }
    #[doc = "LEVEL_1"]
    #[inline(always)]
    pub fn is_ifls_rxiflsel_level_1(&self) -> bool {
        *self == IflsRxiflsel::IflsRxiflselLevel1
    }
}
#[doc = "Field `IFLS_RXIFLSEL` writer - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type IflsRxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, IflsRxiflsel, crate::Safe>;
impl<'a, REG> IflsRxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LVL_OFF"]
    #[inline(always)]
    pub fn ifls_rxiflsel_lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLvlOff)
    }
    #[doc = "LVL_1_4"]
    #[inline(always)]
    pub fn ifls_rxiflsel_lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLvl1_4)
    }
    #[doc = "LVL_1_2"]
    #[inline(always)]
    pub fn ifls_rxiflsel_lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLvl1_2)
    }
    #[doc = "LVL_3_4"]
    #[inline(always)]
    pub fn ifls_rxiflsel_lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLvl3_4)
    }
    #[doc = "LVL_RES4"]
    #[inline(always)]
    pub fn ifls_rxiflsel_lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLvlRes4)
    }
    #[doc = "LVL_FULL"]
    #[inline(always)]
    pub fn ifls_rxiflsel_lvl_full(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLvlFull)
    }
    #[doc = "LVL_RES6"]
    #[inline(always)]
    pub fn ifls_rxiflsel_lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLvlRes6)
    }
    #[doc = "LEVEL_1"]
    #[inline(always)]
    pub fn ifls_rxiflsel_level_1(self) -> &'a mut crate::W<REG> {
        self.variant(IflsRxiflsel::IflsRxiflselLevel1)
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    pub fn ifls_txiflsel(&self) -> IflsTxiflselR {
        IflsTxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    pub fn ifls_rxiflsel(&self) -> IflsRxiflselR {
        IflsRxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    pub fn ifls_txiflsel(&mut self) -> IflsTxiflselW<IflsSpec> {
        IflsTxiflselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    pub fn ifls_rxiflsel(&mut self) -> IflsRxiflselW<IflsSpec> {
        IflsRxiflselW::new(self, 3)
    }
}
#[doc = "UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFLS to value 0x12"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0x12;
}
