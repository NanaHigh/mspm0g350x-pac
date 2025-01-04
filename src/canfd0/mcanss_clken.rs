#[doc = "Register `MCANSS_CLKEN` reader"]
pub type R = crate::R<McanssClkenSpec>;
#[doc = "Register `MCANSS_CLKEN` writer"]
pub type W = crate::W<McanssClkenSpec>;
#[doc = "MCAN functional and MCAN/MCANSS MMR clock request enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McanssClkenClkReqen {
    #[doc = "0: CLR"]
    McanssClkenClkReqenClr = 0,
    #[doc = "1: SET"]
    McanssClkenClkReqenSet = 1,
}
impl From<McanssClkenClkReqen> for bool {
    #[inline(always)]
    fn from(variant: McanssClkenClkReqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANSS_CLKEN_CLK_REQEN` reader - MCAN functional and MCAN/MCANSS MMR clock request enable bit"]
pub type McanssClkenClkReqenR = crate::BitReader<McanssClkenClkReqen>;
impl McanssClkenClkReqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McanssClkenClkReqen {
        match self.bits {
            false => McanssClkenClkReqen::McanssClkenClkReqenClr,
            true => McanssClkenClkReqen::McanssClkenClkReqenSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mcanss_clken_clk_reqen_clr(&self) -> bool {
        *self == McanssClkenClkReqen::McanssClkenClkReqenClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mcanss_clken_clk_reqen_set(&self) -> bool {
        *self == McanssClkenClkReqen::McanssClkenClkReqenSet
    }
}
#[doc = "Field `MCANSS_CLKEN_CLK_REQEN` writer - MCAN functional and MCAN/MCANSS MMR clock request enable bit"]
pub type McanssClkenClkReqenW<'a, REG> = crate::BitWriter<'a, REG, McanssClkenClkReqen>;
impl<'a, REG> McanssClkenClkReqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn mcanss_clken_clk_reqen_clr(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkenClkReqen::McanssClkenClkReqenClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn mcanss_clken_clk_reqen_set(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkenClkReqen::McanssClkenClkReqenSet)
    }
}
impl R {
    #[doc = "Bit 0 - MCAN functional and MCAN/MCANSS MMR clock request enable bit"]
    #[inline(always)]
    pub fn mcanss_clken_clk_reqen(&self) -> McanssClkenClkReqenR {
        McanssClkenClkReqenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCAN functional and MCAN/MCANSS MMR clock request enable bit"]
    #[inline(always)]
    pub fn mcanss_clken_clk_reqen(&mut self) -> McanssClkenClkReqenW<McanssClkenSpec> {
        McanssClkenClkReqenW::new(self, 0)
    }
}
#[doc = "MCAN module clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_clken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssClkenSpec;
impl crate::RegisterSpec for McanssClkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_clken::R`](R) reader structure"]
impl crate::Readable for McanssClkenSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_clken::W`](W) writer structure"]
impl crate::Writable for McanssClkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_CLKEN to value 0"]
impl crate::Resettable for McanssClkenSpec {
    const RESET_VALUE: u32 = 0;
}
