#[doc = "Register `CTL3` reader"]
pub type R = crate::R<Ctl3Spec>;
#[doc = "Register `CTL3` writer"]
pub type W = crate::W<Ctl3Spec>;
#[doc = "Field `CTL3_DACCODE0` reader - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Ctl3Daccode0R = crate::FieldReader;
#[doc = "Field `CTL3_DACCODE0` writer - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Ctl3Daccode0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTL3_DACCODE1` reader - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Ctl3Daccode1R = crate::FieldReader;
#[doc = "Field `CTL3_DACCODE1` writer - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Ctl3Daccode1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn ctl3_daccode0(&self) -> Ctl3Daccode0R {
        Ctl3Daccode0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn ctl3_daccode1(&self) -> Ctl3Daccode1R {
        Ctl3Daccode1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn ctl3_daccode0(&mut self) -> Ctl3Daccode0W<Ctl3Spec> {
        Ctl3Daccode0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn ctl3_daccode1(&mut self) -> Ctl3Daccode1W<Ctl3Spec> {
        Ctl3Daccode1W::new(self, 16)
    }
}
#[doc = "Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl3Spec;
impl crate::RegisterSpec for Ctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl3::R`](R) reader structure"]
impl crate::Readable for Ctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl3::W`](W) writer structure"]
impl crate::Writable for Ctl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL3 to value 0"]
impl crate::Resettable for Ctl3Spec {
    const RESET_VALUE: u32 = 0;
}
