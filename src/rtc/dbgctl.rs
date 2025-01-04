#[doc = "Register `DBGCTL` reader"]
pub type R = crate::R<DbgctlSpec>;
#[doc = "Register `DBGCTL` writer"]
pub type W = crate::W<DbgctlSpec>;
#[doc = "Debug Run.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgctlDbgrun {
    #[doc = "0: HALT"]
    DbgctlDbgrunHalt = 0,
    #[doc = "1: RUN"]
    DbgctlDbgrunRun = 1,
}
impl From<DbgctlDbgrun> for bool {
    #[inline(always)]
    fn from(variant: DbgctlDbgrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGCTL_DBGRUN` reader - Debug Run."]
pub type DbgctlDbgrunR = crate::BitReader<DbgctlDbgrun>;
impl DbgctlDbgrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgctlDbgrun {
        match self.bits {
            false => DbgctlDbgrun::DbgctlDbgrunHalt,
            true => DbgctlDbgrun::DbgctlDbgrunRun,
        }
    }
    #[doc = "HALT"]
    #[inline(always)]
    pub fn is_dbgctl_dbgrun_halt(&self) -> bool {
        *self == DbgctlDbgrun::DbgctlDbgrunHalt
    }
    #[doc = "RUN"]
    #[inline(always)]
    pub fn is_dbgctl_dbgrun_run(&self) -> bool {
        *self == DbgctlDbgrun::DbgctlDbgrunRun
    }
}
#[doc = "Field `DBGCTL_DBGRUN` writer - Debug Run."]
pub type DbgctlDbgrunW<'a, REG> = crate::BitWriter<'a, REG, DbgctlDbgrun>;
impl<'a, REG> DbgctlDbgrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HALT"]
    #[inline(always)]
    pub fn dbgctl_dbgrun_halt(self) -> &'a mut crate::W<REG> {
        self.variant(DbgctlDbgrun::DbgctlDbgrunHalt)
    }
    #[doc = "RUN"]
    #[inline(always)]
    pub fn dbgctl_dbgrun_run(self) -> &'a mut crate::W<REG> {
        self.variant(DbgctlDbgrun::DbgctlDbgrunRun)
    }
}
#[doc = "Debug Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgctlDbgint {
    #[doc = "0: OFF"]
    DbgctlDbgintOff = 0,
    #[doc = "1: ON"]
    DbgctlDbgintOn = 1,
}
impl From<DbgctlDbgint> for bool {
    #[inline(always)]
    fn from(variant: DbgctlDbgint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGCTL_DBGINT` reader - Debug Interrupt Enable."]
pub type DbgctlDbgintR = crate::BitReader<DbgctlDbgint>;
impl DbgctlDbgintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgctlDbgint {
        match self.bits {
            false => DbgctlDbgint::DbgctlDbgintOff,
            true => DbgctlDbgint::DbgctlDbgintOn,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_dbgctl_dbgint_off(&self) -> bool {
        *self == DbgctlDbgint::DbgctlDbgintOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_dbgctl_dbgint_on(&self) -> bool {
        *self == DbgctlDbgint::DbgctlDbgintOn
    }
}
#[doc = "Field `DBGCTL_DBGINT` writer - Debug Interrupt Enable."]
pub type DbgctlDbgintW<'a, REG> = crate::BitWriter<'a, REG, DbgctlDbgint>;
impl<'a, REG> DbgctlDbgintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn dbgctl_dbgint_off(self) -> &'a mut crate::W<REG> {
        self.variant(DbgctlDbgint::DbgctlDbgintOff)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn dbgctl_dbgint_on(self) -> &'a mut crate::W<REG> {
        self.variant(DbgctlDbgint::DbgctlDbgintOn)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Run."]
    #[inline(always)]
    pub fn dbgctl_dbgrun(&self) -> DbgctlDbgrunR {
        DbgctlDbgrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Interrupt Enable."]
    #[inline(always)]
    pub fn dbgctl_dbgint(&self) -> DbgctlDbgintR {
        DbgctlDbgintR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Run."]
    #[inline(always)]
    pub fn dbgctl_dbgrun(&mut self) -> DbgctlDbgrunW<DbgctlSpec> {
        DbgctlDbgrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Interrupt Enable."]
    #[inline(always)]
    pub fn dbgctl_dbgint(&mut self) -> DbgctlDbgintW<DbgctlSpec> {
        DbgctlDbgintW::new(self, 1)
    }
}
#[doc = "RTC Module Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctlSpec;
impl crate::RegisterSpec for DbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgctl::R`](R) reader structure"]
impl crate::Readable for DbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctl::W`](W) writer structure"]
impl crate::Writable for DbgctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGCTL to value 0"]
impl crate::Resettable for DbgctlSpec {
    const RESET_VALUE: u32 = 0;
}
