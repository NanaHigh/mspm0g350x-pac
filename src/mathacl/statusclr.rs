#[doc = "Register `STATUSCLR` writer"]
pub type W = crate::W<StatusclrSpec>;
#[doc = "Write 1 to this bit to clear STATUS.UF bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusclrClrUf {
    #[doc = "0: NO_EFFECT"]
    StatusclrClrUfNoEffect = 0,
    #[doc = "1: CLR"]
    StatusclrClrUfClr = 1,
}
impl From<StatusclrClrUf> for bool {
    #[inline(always)]
    fn from(variant: StatusclrClrUf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUSCLR_CLR_UF` writer - Write 1 to this bit to clear STATUS.UF bit"]
pub type StatusclrClrUfW<'a, REG> = crate::BitWriter<'a, REG, StatusclrClrUf>;
impl<'a, REG> StatusclrClrUfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn statusclr_clr_uf_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(StatusclrClrUf::StatusclrClrUfNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn statusclr_clr_uf_clr(self) -> &'a mut crate::W<REG> {
        self.variant(StatusclrClrUf::StatusclrClrUfClr)
    }
}
#[doc = "Write 1 to this bit to clear STATUS.OVF bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusclrClrOvf {
    #[doc = "0: NO_EFFECT"]
    StatusclrClrOvfNoEffect = 0,
    #[doc = "1: CLR"]
    StatusclrClrOvfClr = 1,
}
impl From<StatusclrClrOvf> for bool {
    #[inline(always)]
    fn from(variant: StatusclrClrOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUSCLR_CLR_OVF` writer - Write 1 to this bit to clear STATUS.OVF bit"]
pub type StatusclrClrOvfW<'a, REG> = crate::BitWriter<'a, REG, StatusclrClrOvf>;
impl<'a, REG> StatusclrClrOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn statusclr_clr_ovf_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(StatusclrClrOvf::StatusclrClrOvfNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn statusclr_clr_ovf_clr(self) -> &'a mut crate::W<REG> {
        self.variant(StatusclrClrOvf::StatusclrClrOvfClr)
    }
}
#[doc = "Write 1 to this bit to clear STATUS.ERR field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusclrClrErr {
    #[doc = "0: NO_EFFECT"]
    StatusclrClrErrNoEffect = 0,
    #[doc = "1: CLR"]
    StatusclrClrErrClr = 1,
}
impl From<StatusclrClrErr> for bool {
    #[inline(always)]
    fn from(variant: StatusclrClrErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUSCLR_CLR_ERR` writer - Write 1 to this bit to clear STATUS.ERR field"]
pub type StatusclrClrErrW<'a, REG> = crate::BitWriter<'a, REG, StatusclrClrErr>;
impl<'a, REG> StatusclrClrErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn statusclr_clr_err_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(StatusclrClrErr::StatusclrClrErrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn statusclr_clr_err_clr(self) -> &'a mut crate::W<REG> {
        self.variant(StatusclrClrErr::StatusclrClrErrClr)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this bit to clear STATUS.UF bit"]
    #[inline(always)]
    pub fn statusclr_clr_uf(&mut self) -> StatusclrClrUfW<StatusclrSpec> {
        StatusclrClrUfW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to this bit to clear STATUS.OVF bit"]
    #[inline(always)]
    pub fn statusclr_clr_ovf(&mut self) -> StatusclrClrOvfW<StatusclrSpec> {
        StatusclrClrOvfW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to this bit to clear STATUS.ERR field"]
    #[inline(always)]
    pub fn statusclr_clr_err(&mut self) -> StatusclrClrErrW<StatusclrSpec> {
        StatusclrClrErrW::new(self, 2)
    }
}
#[doc = "Status flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusclrSpec;
impl crate::RegisterSpec for StatusclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`statusclr::W`](W) writer structure"]
impl crate::Writable for StatusclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUSCLR to value 0"]
impl crate::Resettable for StatusclrSpec {
    const RESET_VALUE: u32 = 0;
}
