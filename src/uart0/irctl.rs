#[doc = "Register `IRCTL` reader"]
pub type R = crate::R<IrctlSpec>;
#[doc = "Register `IRCTL` writer"]
pub type W = crate::W<IrctlSpec>;
#[doc = "IrDA encoder/decoder enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrctlIren {
    #[doc = "0: DISABLE"]
    IrctlIrenDisable = 0,
    #[doc = "1: ENABLE"]
    IrctlIrenEnable = 1,
}
impl From<IrctlIren> for bool {
    #[inline(always)]
    fn from(variant: IrctlIren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCTL_IREN` reader - IrDA encoder/decoder enable"]
pub type IrctlIrenR = crate::BitReader<IrctlIren>;
impl IrctlIrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrctlIren {
        match self.bits {
            false => IrctlIren::IrctlIrenDisable,
            true => IrctlIren::IrctlIrenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_irctl_iren_disable(&self) -> bool {
        *self == IrctlIren::IrctlIrenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_irctl_iren_enable(&self) -> bool {
        *self == IrctlIren::IrctlIrenEnable
    }
}
#[doc = "Field `IRCTL_IREN` writer - IrDA encoder/decoder enable"]
pub type IrctlIrenW<'a, REG> = crate::BitWriter<'a, REG, IrctlIren>;
impl<'a, REG> IrctlIrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn irctl_iren_disable(self) -> &'a mut crate::W<REG> {
        self.variant(IrctlIren::IrctlIrenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn irctl_iren_enable(self) -> &'a mut crate::W<REG> {
        self.variant(IrctlIren::IrctlIrenEnable)
    }
}
#[doc = "IrDA transmit pulse clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrctlIrtxclk {
    #[doc = "0: BITCLK"]
    IrctlIrtxclkBitclk = 0,
    #[doc = "1: BRCLK"]
    IrctlIrtxclkBrclk = 1,
}
impl From<IrctlIrtxclk> for bool {
    #[inline(always)]
    fn from(variant: IrctlIrtxclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCTL_IRTXCLK` reader - IrDA transmit pulse clock select"]
pub type IrctlIrtxclkR = crate::BitReader<IrctlIrtxclk>;
impl IrctlIrtxclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrctlIrtxclk {
        match self.bits {
            false => IrctlIrtxclk::IrctlIrtxclkBitclk,
            true => IrctlIrtxclk::IrctlIrtxclkBrclk,
        }
    }
    #[doc = "BITCLK"]
    #[inline(always)]
    pub fn is_irctl_irtxclk_bitclk(&self) -> bool {
        *self == IrctlIrtxclk::IrctlIrtxclkBitclk
    }
    #[doc = "BRCLK"]
    #[inline(always)]
    pub fn is_irctl_irtxclk_brclk(&self) -> bool {
        *self == IrctlIrtxclk::IrctlIrtxclkBrclk
    }
}
#[doc = "Field `IRCTL_IRTXCLK` writer - IrDA transmit pulse clock select"]
pub type IrctlIrtxclkW<'a, REG> = crate::BitWriter<'a, REG, IrctlIrtxclk>;
impl<'a, REG> IrctlIrtxclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BITCLK"]
    #[inline(always)]
    pub fn irctl_irtxclk_bitclk(self) -> &'a mut crate::W<REG> {
        self.variant(IrctlIrtxclk::IrctlIrtxclkBitclk)
    }
    #[doc = "BRCLK"]
    #[inline(always)]
    pub fn irctl_irtxclk_brclk(self) -> &'a mut crate::W<REG> {
        self.variant(IrctlIrtxclk::IrctlIrtxclkBrclk)
    }
}
#[doc = "Field `IRCTL_IRTXPL` reader - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"]
pub type IrctlIrtxplR = crate::FieldReader;
#[doc = "Field `IRCTL_IRTXPL` writer - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"]
pub type IrctlIrtxplW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "IrDA receive input UCAxRXD polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrctlIrrxpl {
    #[doc = "0: HIGHPULSE"]
    IrctlIrrxplHigh = 0,
    #[doc = "1: LOWPULSE"]
    IrctlIrrxplLow = 1,
}
impl From<IrctlIrrxpl> for bool {
    #[inline(always)]
    fn from(variant: IrctlIrrxpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCTL_IRRXPL` reader - IrDA receive input UCAxRXD polarity"]
pub type IrctlIrrxplR = crate::BitReader<IrctlIrrxpl>;
impl IrctlIrrxplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrctlIrrxpl {
        match self.bits {
            false => IrctlIrrxpl::IrctlIrrxplHigh,
            true => IrctlIrrxpl::IrctlIrrxplLow,
        }
    }
    #[doc = "HIGHPULSE"]
    #[inline(always)]
    pub fn is_irctl_irrxpl_high(&self) -> bool {
        *self == IrctlIrrxpl::IrctlIrrxplHigh
    }
    #[doc = "LOWPULSE"]
    #[inline(always)]
    pub fn is_irctl_irrxpl_low(&self) -> bool {
        *self == IrctlIrrxpl::IrctlIrrxplLow
    }
}
#[doc = "Field `IRCTL_IRRXPL` writer - IrDA receive input UCAxRXD polarity"]
pub type IrctlIrrxplW<'a, REG> = crate::BitWriter<'a, REG, IrctlIrrxpl>;
impl<'a, REG> IrctlIrrxplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIGHPULSE"]
    #[inline(always)]
    pub fn irctl_irrxpl_high(self) -> &'a mut crate::W<REG> {
        self.variant(IrctlIrrxpl::IrctlIrrxplHigh)
    }
    #[doc = "LOWPULSE"]
    #[inline(always)]
    pub fn irctl_irrxpl_low(self) -> &'a mut crate::W<REG> {
        self.variant(IrctlIrrxpl::IrctlIrrxplLow)
    }
}
impl R {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn irctl_iren(&self) -> IrctlIrenR {
        IrctlIrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn irctl_irtxclk(&self) -> IrctlIrtxclkR {
        IrctlIrtxclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"]
    #[inline(always)]
    pub fn irctl_irtxpl(&self) -> IrctlIrtxplR {
        IrctlIrtxplR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn irctl_irrxpl(&self) -> IrctlIrrxplR {
        IrctlIrrxplR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn irctl_iren(&mut self) -> IrctlIrenW<IrctlSpec> {
        IrctlIrenW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn irctl_irtxclk(&mut self) -> IrctlIrtxclkW<IrctlSpec> {
        IrctlIrtxclkW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"]
    #[inline(always)]
    pub fn irctl_irtxpl(&mut self) -> IrctlIrtxplW<IrctlSpec> {
        IrctlIrtxplW::new(self, 2)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn irctl_irrxpl(&mut self) -> IrctlIrrxplW<IrctlSpec> {
        IrctlIrrxplW::new(self, 9)
    }
}
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrctlSpec;
impl crate::RegisterSpec for IrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irctl::R`](R) reader structure"]
impl crate::Readable for IrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`irctl::W`](W) writer structure"]
impl crate::Writable for IrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRCTL to value 0"]
impl crate::Resettable for IrctlSpec {
    const RESET_VALUE: u32 = 0;
}
