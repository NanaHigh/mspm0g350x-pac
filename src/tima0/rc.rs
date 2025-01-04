#[doc = "Register `RC` reader"]
pub type R = crate::R<RcSpec>;
#[doc = "Field `RC_RC` reader - Repeat Counter Value"]
pub type RcRcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter Value"]
    #[inline(always)]
    pub fn rc_rc(&self) -> RcRcR {
        RcRcR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Repeat counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcSpec;
impl crate::RegisterSpec for RcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc::R`](R) reader structure"]
impl crate::Readable for RcSpec {}
#[doc = "`reset()` method sets RC to value 0"]
impl crate::Resettable for RcSpec {
    const RESET_VALUE: u32 = 0;
}
