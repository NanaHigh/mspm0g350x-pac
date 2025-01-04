#[doc = "Register `MTPR` reader"]
pub type R = crate::R<MtprSpec>;
#[doc = "Register `MTPR` writer"]
pub type W = crate::W<MtprSpec>;
#[doc = "Field `MTPR_TPR` reader - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
pub type MtprTprR = crate::FieldReader;
#[doc = "Field `MTPR_TPR` writer - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
pub type MtprTprW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
    #[inline(always)]
    pub fn mtpr_tpr(&self) -> MtprTprR {
        MtprTprR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
    #[inline(always)]
    pub fn mtpr_tpr(&mut self) -> MtprTprW<MtprSpec> {
        MtprTprW::new(self, 0)
    }
}
#[doc = "I2C Master Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`mtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtprSpec;
impl crate::RegisterSpec for MtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtpr::R`](R) reader structure"]
impl crate::Readable for MtprSpec {}
#[doc = "`write(|w| ..)` method takes [`mtpr::W`](W) writer structure"]
impl crate::Writable for MtprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTPR to value 0x01"]
impl crate::Resettable for MtprSpec {
    const RESET_VALUE: u32 = 0x01;
}
