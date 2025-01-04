#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Masks TXIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskTxifg {
    #[doc = "0: CLR"]
    ImaskTxifgClr = 0,
    #[doc = "1: SET"]
    ImaskTxifgSet = 1,
}
impl From<ImaskTxifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskTxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_TXIFG` reader - Masks TXIFG in MIS register"]
pub type ImaskTxifgR = crate::BitReader<ImaskTxifg>;
impl ImaskTxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskTxifg {
        match self.bits {
            false => ImaskTxifg::ImaskTxifgClr,
            true => ImaskTxifg::ImaskTxifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_txifg_clr(&self) -> bool {
        *self == ImaskTxifg::ImaskTxifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_txifg_set(&self) -> bool {
        *self == ImaskTxifg::ImaskTxifgSet
    }
}
#[doc = "Field `IMASK_TXIFG` writer - Masks TXIFG in MIS register"]
pub type ImaskTxifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskTxifg>;
impl<'a, REG> ImaskTxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_txifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskTxifg::ImaskTxifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_txifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskTxifg::ImaskTxifgSet)
    }
}
#[doc = "Masks RXIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskRxifg {
    #[doc = "0: CLR"]
    ImaskRxifgClr = 0,
    #[doc = "1: SET"]
    ImaskRxifgSet = 1,
}
impl From<ImaskRxifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskRxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_RXIFG` reader - Masks RXIFG in MIS register"]
pub type ImaskRxifgR = crate::BitReader<ImaskRxifg>;
impl ImaskRxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskRxifg {
        match self.bits {
            false => ImaskRxifg::ImaskRxifgClr,
            true => ImaskRxifg::ImaskRxifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_rxifg_clr(&self) -> bool {
        *self == ImaskRxifg::ImaskRxifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_rxifg_set(&self) -> bool {
        *self == ImaskRxifg::ImaskRxifgSet
    }
}
#[doc = "Field `IMASK_RXIFG` writer - Masks RXIFG in MIS register"]
pub type ImaskRxifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskRxifg>;
impl<'a, REG> ImaskRxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_rxifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskRxifg::ImaskRxifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_rxifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskRxifg::ImaskRxifgSet)
    }
}
#[doc = "Masks PWRUPIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskPwrupifg {
    #[doc = "0: CLR"]
    ImaskPwrupifgClr = 0,
    #[doc = "1: SET"]
    ImaskPwrupifgSet = 1,
}
impl From<ImaskPwrupifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskPwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_PWRUPIFG` reader - Masks PWRUPIFG in MIS register"]
pub type ImaskPwrupifgR = crate::BitReader<ImaskPwrupifg>;
impl ImaskPwrupifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskPwrupifg {
        match self.bits {
            false => ImaskPwrupifg::ImaskPwrupifgClr,
            true => ImaskPwrupifg::ImaskPwrupifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_pwrupifg_clr(&self) -> bool {
        *self == ImaskPwrupifg::ImaskPwrupifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_pwrupifg_set(&self) -> bool {
        *self == ImaskPwrupifg::ImaskPwrupifgSet
    }
}
#[doc = "Field `IMASK_PWRUPIFG` writer - Masks PWRUPIFG in MIS register"]
pub type ImaskPwrupifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskPwrupifg>;
impl<'a, REG> ImaskPwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_pwrupifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPwrupifg::ImaskPwrupifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_pwrupifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPwrupifg::ImaskPwrupifgSet)
    }
}
#[doc = "Masks PWRDWNIFG in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskPwrdwnifg {
    #[doc = "0: CLR"]
    ImaskPwrdwnifgClr = 0,
    #[doc = "1: SET"]
    ImaskPwrdwnifgSet = 1,
}
impl From<ImaskPwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskPwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_PWRDWNIFG` reader - Masks PWRDWNIFG in MIS register"]
pub type ImaskPwrdwnifgR = crate::BitReader<ImaskPwrdwnifg>;
impl ImaskPwrdwnifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskPwrdwnifg {
        match self.bits {
            false => ImaskPwrdwnifg::ImaskPwrdwnifgClr,
            true => ImaskPwrdwnifg::ImaskPwrdwnifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_pwrdwnifg_clr(&self) -> bool {
        *self == ImaskPwrdwnifg::ImaskPwrdwnifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_pwrdwnifg_set(&self) -> bool {
        *self == ImaskPwrdwnifg::ImaskPwrdwnifgSet
    }
}
#[doc = "Field `IMASK_PWRDWNIFG` writer - Masks PWRDWNIFG in MIS register"]
pub type ImaskPwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskPwrdwnifg>;
impl<'a, REG> ImaskPwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_pwrdwnifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPwrdwnifg::ImaskPwrdwnifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_pwrdwnifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskPwrdwnifg::ImaskPwrdwnifgSet)
    }
}
impl R {
    #[doc = "Bit 0 - Masks TXIFG in MIS register"]
    #[inline(always)]
    pub fn imask_txifg(&self) -> ImaskTxifgR {
        ImaskTxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masks RXIFG in MIS register"]
    #[inline(always)]
    pub fn imask_rxifg(&self) -> ImaskRxifgR {
        ImaskRxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks PWRUPIFG in MIS register"]
    #[inline(always)]
    pub fn imask_pwrupifg(&self) -> ImaskPwrupifgR {
        ImaskPwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks PWRDWNIFG in MIS register"]
    #[inline(always)]
    pub fn imask_pwrdwnifg(&self) -> ImaskPwrdwnifgR {
        ImaskPwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masks TXIFG in MIS register"]
    #[inline(always)]
    pub fn imask_txifg(&mut self) -> ImaskTxifgW<ImaskSpec> {
        ImaskTxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Masks RXIFG in MIS register"]
    #[inline(always)]
    pub fn imask_rxifg(&mut self) -> ImaskRxifgW<ImaskSpec> {
        ImaskRxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Masks PWRUPIFG in MIS register"]
    #[inline(always)]
    pub fn imask_pwrupifg(&mut self) -> ImaskPwrupifgW<ImaskSpec> {
        ImaskPwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Masks PWRDWNIFG in MIS register"]
    #[inline(always)]
    pub fn imask_pwrdwnifg(&mut self) -> ImaskPwrdwnifgW<ImaskSpec> {
        ImaskPwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
