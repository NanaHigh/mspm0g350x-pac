#[doc = "Register `WRITELOCK` reader"]
pub type R = crate::R<WritelockSpec>;
#[doc = "Register `WRITELOCK` writer"]
pub type W = crate::W<WritelockSpec>;
#[doc = "ACTIVE controls whether critical SYSCTL registers are write protected or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WritelockActive {
    #[doc = "0: DISABLE"]
    WritelockActiveDisable = 0,
    #[doc = "1: ENABLE"]
    WritelockActiveEnable = 1,
}
impl From<WritelockActive> for bool {
    #[inline(always)]
    fn from(variant: WritelockActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITELOCK_ACTIVE` reader - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
pub type WritelockActiveR = crate::BitReader<WritelockActive>;
impl WritelockActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WritelockActive {
        match self.bits {
            false => WritelockActive::WritelockActiveDisable,
            true => WritelockActive::WritelockActiveEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_writelock_active_disable(&self) -> bool {
        *self == WritelockActive::WritelockActiveDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_writelock_active_enable(&self) -> bool {
        *self == WritelockActive::WritelockActiveEnable
    }
}
#[doc = "Field `WRITELOCK_ACTIVE` writer - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
pub type WritelockActiveW<'a, REG> = crate::BitWriter<'a, REG, WritelockActive>;
impl<'a, REG> WritelockActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn writelock_active_disable(self) -> &'a mut crate::W<REG> {
        self.variant(WritelockActive::WritelockActiveDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn writelock_active_enable(self) -> &'a mut crate::W<REG> {
        self.variant(WritelockActive::WritelockActiveEnable)
    }
}
impl R {
    #[doc = "Bit 0 - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
    #[inline(always)]
    pub fn writelock_active(&self) -> WritelockActiveR {
        WritelockActiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
    #[inline(always)]
    pub fn writelock_active(&mut self) -> WritelockActiveW<WritelockSpec> {
        WritelockActiveW::new(self, 0)
    }
}
#[doc = "SYSCTL register write lockout\n\nYou can [`read`](crate::Reg::read) this register and get [`writelock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writelock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritelockSpec;
impl crate::RegisterSpec for WritelockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writelock::R`](R) reader structure"]
impl crate::Readable for WritelockSpec {}
#[doc = "`write(|w| ..)` method takes [`writelock::W`](W) writer structure"]
impl crate::Writable for WritelockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITELOCK to value 0"]
impl crate::Resettable for WritelockSpec {
    const RESET_VALUE: u32 = 0;
}
