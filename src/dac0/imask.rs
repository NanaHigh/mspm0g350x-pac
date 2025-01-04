#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Masks MODRDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskModrdyifg {
    #[doc = "0: CLR"]
    ImaskModrdyifgClr = 0,
    #[doc = "1: SET"]
    ImaskModrdyifgSet = 1,
}
impl From<ImaskModrdyifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskModrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_MODRDYIFG` reader - Masks MODRDYIFG"]
pub type ImaskModrdyifgR = crate::BitReader<ImaskModrdyifg>;
impl ImaskModrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskModrdyifg {
        match self.bits {
            false => ImaskModrdyifg::ImaskModrdyifgClr,
            true => ImaskModrdyifg::ImaskModrdyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_modrdyifg_clr(&self) -> bool {
        *self == ImaskModrdyifg::ImaskModrdyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_modrdyifg_set(&self) -> bool {
        *self == ImaskModrdyifg::ImaskModrdyifgSet
    }
}
#[doc = "Field `IMASK_MODRDYIFG` writer - Masks MODRDYIFG"]
pub type ImaskModrdyifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskModrdyifg>;
impl<'a, REG> ImaskModrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_modrdyifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskModrdyifg::ImaskModrdyifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_modrdyifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskModrdyifg::ImaskModrdyifgSet)
    }
}
#[doc = "Masks FIFOFULLIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskFifofullifg {
    #[doc = "0: CLR"]
    ImaskFifofullifgClr = 0,
    #[doc = "1: SET"]
    ImaskFifofullifgSet = 1,
}
impl From<ImaskFifofullifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskFifofullifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_FIFOFULLIFG` reader - Masks FIFOFULLIFG"]
pub type ImaskFifofullifgR = crate::BitReader<ImaskFifofullifg>;
impl ImaskFifofullifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskFifofullifg {
        match self.bits {
            false => ImaskFifofullifg::ImaskFifofullifgClr,
            true => ImaskFifofullifg::ImaskFifofullifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_fifofullifg_clr(&self) -> bool {
        *self == ImaskFifofullifg::ImaskFifofullifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_fifofullifg_set(&self) -> bool {
        *self == ImaskFifofullifg::ImaskFifofullifgSet
    }
}
#[doc = "Field `IMASK_FIFOFULLIFG` writer - Masks FIFOFULLIFG"]
pub type ImaskFifofullifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskFifofullifg>;
impl<'a, REG> ImaskFifofullifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_fifofullifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifofullifg::ImaskFifofullifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_fifofullifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifofullifg::ImaskFifofullifgSet)
    }
}
#[doc = "Masks FIFO1B4IFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskFifo1b4ifg {
    #[doc = "0: CLR"]
    ImaskFifo1b4ifgClr = 0,
    #[doc = "1: SET"]
    ImaskFifo1b4ifgSet = 1,
}
impl From<ImaskFifo1b4ifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskFifo1b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_FIFO1B4IFG` reader - Masks FIFO1B4IFG"]
pub type ImaskFifo1b4ifgR = crate::BitReader<ImaskFifo1b4ifg>;
impl ImaskFifo1b4ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskFifo1b4ifg {
        match self.bits {
            false => ImaskFifo1b4ifg::ImaskFifo1b4ifgClr,
            true => ImaskFifo1b4ifg::ImaskFifo1b4ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_fifo1b4ifg_clr(&self) -> bool {
        *self == ImaskFifo1b4ifg::ImaskFifo1b4ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_fifo1b4ifg_set(&self) -> bool {
        *self == ImaskFifo1b4ifg::ImaskFifo1b4ifgSet
    }
}
#[doc = "Field `IMASK_FIFO1B4IFG` writer - Masks FIFO1B4IFG"]
pub type ImaskFifo1b4ifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskFifo1b4ifg>;
impl<'a, REG> ImaskFifo1b4ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_fifo1b4ifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifo1b4ifg::ImaskFifo1b4ifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_fifo1b4ifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifo1b4ifg::ImaskFifo1b4ifgSet)
    }
}
#[doc = "Masks FIFO1B2IFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskFifo1b2ifg {
    #[doc = "0: CLR"]
    ImaskFifo1b2ifgClr = 0,
    #[doc = "1: SET"]
    ImaskFifo1b2ifgSet = 1,
}
impl From<ImaskFifo1b2ifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskFifo1b2ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_FIFO1B2IFG` reader - Masks FIFO1B2IFG"]
pub type ImaskFifo1b2ifgR = crate::BitReader<ImaskFifo1b2ifg>;
impl ImaskFifo1b2ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskFifo1b2ifg {
        match self.bits {
            false => ImaskFifo1b2ifg::ImaskFifo1b2ifgClr,
            true => ImaskFifo1b2ifg::ImaskFifo1b2ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_fifo1b2ifg_clr(&self) -> bool {
        *self == ImaskFifo1b2ifg::ImaskFifo1b2ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_fifo1b2ifg_set(&self) -> bool {
        *self == ImaskFifo1b2ifg::ImaskFifo1b2ifgSet
    }
}
#[doc = "Field `IMASK_FIFO1B2IFG` writer - Masks FIFO1B2IFG"]
pub type ImaskFifo1b2ifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskFifo1b2ifg>;
impl<'a, REG> ImaskFifo1b2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_fifo1b2ifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifo1b2ifg::ImaskFifo1b2ifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_fifo1b2ifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifo1b2ifg::ImaskFifo1b2ifgSet)
    }
}
#[doc = "Masks FIFO3B4IFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskFifo3b4ifg {
    #[doc = "0: CLR"]
    ImaskFifo3b4ifgClr = 0,
    #[doc = "1: SET"]
    ImaskFifo3b4ifgSet = 1,
}
impl From<ImaskFifo3b4ifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskFifo3b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_FIFO3B4IFG` reader - Masks FIFO3B4IFG"]
pub type ImaskFifo3b4ifgR = crate::BitReader<ImaskFifo3b4ifg>;
impl ImaskFifo3b4ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskFifo3b4ifg {
        match self.bits {
            false => ImaskFifo3b4ifg::ImaskFifo3b4ifgClr,
            true => ImaskFifo3b4ifg::ImaskFifo3b4ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_fifo3b4ifg_clr(&self) -> bool {
        *self == ImaskFifo3b4ifg::ImaskFifo3b4ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_fifo3b4ifg_set(&self) -> bool {
        *self == ImaskFifo3b4ifg::ImaskFifo3b4ifgSet
    }
}
#[doc = "Field `IMASK_FIFO3B4IFG` writer - Masks FIFO3B4IFG"]
pub type ImaskFifo3b4ifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskFifo3b4ifg>;
impl<'a, REG> ImaskFifo3b4ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_fifo3b4ifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifo3b4ifg::ImaskFifo3b4ifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_fifo3b4ifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifo3b4ifg::ImaskFifo3b4ifgSet)
    }
}
#[doc = "Masks FIFOEMPTYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskFifoemptyifg {
    #[doc = "0: CLR"]
    ImaskFifoemptyifgClr = 0,
    #[doc = "1: SET"]
    ImaskFifoemptyifgSet = 1,
}
impl From<ImaskFifoemptyifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskFifoemptyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_FIFOEMPTYIFG` reader - Masks FIFOEMPTYIFG"]
pub type ImaskFifoemptyifgR = crate::BitReader<ImaskFifoemptyifg>;
impl ImaskFifoemptyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskFifoemptyifg {
        match self.bits {
            false => ImaskFifoemptyifg::ImaskFifoemptyifgClr,
            true => ImaskFifoemptyifg::ImaskFifoemptyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_fifoemptyifg_clr(&self) -> bool {
        *self == ImaskFifoemptyifg::ImaskFifoemptyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_fifoemptyifg_set(&self) -> bool {
        *self == ImaskFifoemptyifg::ImaskFifoemptyifgSet
    }
}
#[doc = "Field `IMASK_FIFOEMPTYIFG` writer - Masks FIFOEMPTYIFG"]
pub type ImaskFifoemptyifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskFifoemptyifg>;
impl<'a, REG> ImaskFifoemptyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_fifoemptyifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifoemptyifg::ImaskFifoemptyifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_fifoemptyifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifoemptyifg::ImaskFifoemptyifgSet)
    }
}
#[doc = "Masks FIFOURUNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskFifourunifg {
    #[doc = "0: CLR"]
    ImaskFifourunifgClr = 0,
    #[doc = "1: SET"]
    ImaskFifourunifgSet = 1,
}
impl From<ImaskFifourunifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskFifourunifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_FIFOURUNIFG` reader - Masks FIFOURUNIFG"]
pub type ImaskFifourunifgR = crate::BitReader<ImaskFifourunifg>;
impl ImaskFifourunifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskFifourunifg {
        match self.bits {
            false => ImaskFifourunifg::ImaskFifourunifgClr,
            true => ImaskFifourunifg::ImaskFifourunifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_fifourunifg_clr(&self) -> bool {
        *self == ImaskFifourunifg::ImaskFifourunifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_fifourunifg_set(&self) -> bool {
        *self == ImaskFifourunifg::ImaskFifourunifgSet
    }
}
#[doc = "Field `IMASK_FIFOURUNIFG` writer - Masks FIFOURUNIFG"]
pub type ImaskFifourunifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskFifourunifg>;
impl<'a, REG> ImaskFifourunifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_fifourunifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifourunifg::ImaskFifourunifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_fifourunifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFifourunifg::ImaskFifourunifgSet)
    }
}
#[doc = "Masks DMADONEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDmadoneifg {
    #[doc = "0: CLR"]
    ImaskDmadoneifgClr = 0,
    #[doc = "1: SET"]
    ImaskDmadoneifgSet = 1,
}
impl From<ImaskDmadoneifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskDmadoneifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DMADONEIFG` reader - Masks DMADONEIFG"]
pub type ImaskDmadoneifgR = crate::BitReader<ImaskDmadoneifg>;
impl ImaskDmadoneifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDmadoneifg {
        match self.bits {
            false => ImaskDmadoneifg::ImaskDmadoneifgClr,
            true => ImaskDmadoneifg::ImaskDmadoneifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_dmadoneifg_clr(&self) -> bool {
        *self == ImaskDmadoneifg::ImaskDmadoneifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_dmadoneifg_set(&self) -> bool {
        *self == ImaskDmadoneifg::ImaskDmadoneifgSet
    }
}
#[doc = "Field `IMASK_DMADONEIFG` writer - Masks DMADONEIFG"]
pub type ImaskDmadoneifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskDmadoneifg>;
impl<'a, REG> ImaskDmadoneifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_dmadoneifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmadoneifg::ImaskDmadoneifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_dmadoneifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDmadoneifg::ImaskDmadoneifgSet)
    }
}
impl R {
    #[doc = "Bit 1 - Masks MODRDYIFG"]
    #[inline(always)]
    pub fn imask_modrdyifg(&self) -> ImaskModrdyifgR {
        ImaskModrdyifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Masks FIFOFULLIFG"]
    #[inline(always)]
    pub fn imask_fifofullifg(&self) -> ImaskFifofullifgR {
        ImaskFifofullifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masks FIFO1B4IFG"]
    #[inline(always)]
    pub fn imask_fifo1b4ifg(&self) -> ImaskFifo1b4ifgR {
        ImaskFifo1b4ifgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Masks FIFO1B2IFG"]
    #[inline(always)]
    pub fn imask_fifo1b2ifg(&self) -> ImaskFifo1b2ifgR {
        ImaskFifo1b2ifgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Masks FIFO3B4IFG"]
    #[inline(always)]
    pub fn imask_fifo3b4ifg(&self) -> ImaskFifo3b4ifgR {
        ImaskFifo3b4ifgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Masks FIFOEMPTYIFG"]
    #[inline(always)]
    pub fn imask_fifoemptyifg(&self) -> ImaskFifoemptyifgR {
        ImaskFifoemptyifgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Masks FIFOURUNIFG"]
    #[inline(always)]
    pub fn imask_fifourunifg(&self) -> ImaskFifourunifgR {
        ImaskFifourunifgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Masks DMADONEIFG"]
    #[inline(always)]
    pub fn imask_dmadoneifg(&self) -> ImaskDmadoneifgR {
        ImaskDmadoneifgR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Masks MODRDYIFG"]
    #[inline(always)]
    pub fn imask_modrdyifg(&mut self) -> ImaskModrdyifgW<ImaskSpec> {
        ImaskModrdyifgW::new(self, 1)
    }
    #[doc = "Bit 8 - Masks FIFOFULLIFG"]
    #[inline(always)]
    pub fn imask_fifofullifg(&mut self) -> ImaskFifofullifgW<ImaskSpec> {
        ImaskFifofullifgW::new(self, 8)
    }
    #[doc = "Bit 9 - Masks FIFO1B4IFG"]
    #[inline(always)]
    pub fn imask_fifo1b4ifg(&mut self) -> ImaskFifo1b4ifgW<ImaskSpec> {
        ImaskFifo1b4ifgW::new(self, 9)
    }
    #[doc = "Bit 10 - Masks FIFO1B2IFG"]
    #[inline(always)]
    pub fn imask_fifo1b2ifg(&mut self) -> ImaskFifo1b2ifgW<ImaskSpec> {
        ImaskFifo1b2ifgW::new(self, 10)
    }
    #[doc = "Bit 11 - Masks FIFO3B4IFG"]
    #[inline(always)]
    pub fn imask_fifo3b4ifg(&mut self) -> ImaskFifo3b4ifgW<ImaskSpec> {
        ImaskFifo3b4ifgW::new(self, 11)
    }
    #[doc = "Bit 12 - Masks FIFOEMPTYIFG"]
    #[inline(always)]
    pub fn imask_fifoemptyifg(&mut self) -> ImaskFifoemptyifgW<ImaskSpec> {
        ImaskFifoemptyifgW::new(self, 12)
    }
    #[doc = "Bit 13 - Masks FIFOURUNIFG"]
    #[inline(always)]
    pub fn imask_fifourunifg(&mut self) -> ImaskFifourunifgW<ImaskSpec> {
        ImaskFifourunifgW::new(self, 13)
    }
    #[doc = "Bit 14 - Masks DMADONEIFG"]
    #[inline(always)]
    pub fn imask_dmadoneifg(&mut self) -> ImaskDmadoneifgW<ImaskSpec> {
        ImaskDmadoneifgW::new(self, 14)
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
