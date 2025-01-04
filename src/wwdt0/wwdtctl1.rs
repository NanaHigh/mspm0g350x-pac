#[doc = "Register `WWDTCTL1` reader"]
pub type R = crate::R<Wwdtctl1Spec>;
#[doc = "Register `WWDTCTL1` writer"]
pub type W = crate::W<Wwdtctl1Spec>;
#[doc = "Close Window Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdtctl1Winsel {
    #[doc = "0: WIN0"]
    Wwdtctl1WinselWin0 = 0,
    #[doc = "1: WIN1"]
    Wwdtctl1WinselWin1 = 1,
}
impl From<Wwdtctl1Winsel> for bool {
    #[inline(always)]
    fn from(variant: Wwdtctl1Winsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTCTL1_WINSEL` reader - Close Window Select"]
pub type Wwdtctl1WinselR = crate::BitReader<Wwdtctl1Winsel>;
impl Wwdtctl1WinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtctl1Winsel {
        match self.bits {
            false => Wwdtctl1Winsel::Wwdtctl1WinselWin0,
            true => Wwdtctl1Winsel::Wwdtctl1WinselWin1,
        }
    }
    #[doc = "WIN0"]
    #[inline(always)]
    pub fn is_wwdtctl1_winsel_win0(&self) -> bool {
        *self == Wwdtctl1Winsel::Wwdtctl1WinselWin0
    }
    #[doc = "WIN1"]
    #[inline(always)]
    pub fn is_wwdtctl1_winsel_win1(&self) -> bool {
        *self == Wwdtctl1Winsel::Wwdtctl1WinselWin1
    }
}
#[doc = "Field `WWDTCTL1_WINSEL` writer - Close Window Select"]
pub type Wwdtctl1WinselW<'a, REG> = crate::BitWriter<'a, REG, Wwdtctl1Winsel>;
impl<'a, REG> Wwdtctl1WinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WIN0"]
    #[inline(always)]
    pub fn wwdtctl1_winsel_win0(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl1Winsel::Wwdtctl1WinselWin0)
    }
    #[doc = "WIN1"]
    #[inline(always)]
    pub fn wwdtctl1_winsel_win1(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl1Winsel::Wwdtctl1WinselWin1)
    }
}
#[doc = "KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wwdtctl1Key {
    #[doc = "190: _TO_UNLOCK_W_"]
    Wwdtctl1KeyUnlockW = 190,
}
impl From<Wwdtctl1Key> for u8 {
    #[inline(always)]
    fn from(variant: Wwdtctl1Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wwdtctl1Key {
    type Ux = u8;
}
impl crate::IsEnum for Wwdtctl1Key {}
#[doc = "Field `WWDTCTL1_KEY` writer - KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0."]
pub type Wwdtctl1KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Wwdtctl1Key>;
impl<'a, REG> Wwdtctl1KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_TO_UNLOCK_W_"]
    #[inline(always)]
    pub fn wwdtctl1_key_unlock_w(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl1Key::Wwdtctl1KeyUnlockW)
    }
}
impl R {
    #[doc = "Bit 0 - Close Window Select"]
    #[inline(always)]
    pub fn wwdtctl1_winsel(&self) -> Wwdtctl1WinselR {
        Wwdtctl1WinselR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Close Window Select"]
    #[inline(always)]
    pub fn wwdtctl1_winsel(&mut self) -> Wwdtctl1WinselW<Wwdtctl1Spec> {
        Wwdtctl1WinselW::new(self, 0)
    }
    #[doc = "Bits 24:31 - KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0."]
    #[inline(always)]
    pub fn wwdtctl1_key(&mut self) -> Wwdtctl1KeyW<Wwdtctl1Spec> {
        Wwdtctl1KeyW::new(self, 24)
    }
}
#[doc = "Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdtctl1Spec;
impl crate::RegisterSpec for Wwdtctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdtctl1::R`](R) reader structure"]
impl crate::Readable for Wwdtctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`wwdtctl1::W`](W) writer structure"]
impl crate::Writable for Wwdtctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDTCTL1 to value 0"]
impl crate::Resettable for Wwdtctl1Spec {
    const RESET_VALUE: u32 = 0;
}
