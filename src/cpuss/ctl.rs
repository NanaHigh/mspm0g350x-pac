#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Used to enable/disable instruction prefetch to Flash.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlPrefetch {
    #[doc = "0: DISABLE"]
    CtlPrefetchDisable = 0,
    #[doc = "1: ENABLE"]
    CtlPrefetchEnable = 1,
}
impl From<CtlPrefetch> for bool {
    #[inline(always)]
    fn from(variant: CtlPrefetch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_PREFETCH` reader - Used to enable/disable instruction prefetch to Flash."]
pub type CtlPrefetchR = crate::BitReader<CtlPrefetch>;
impl CtlPrefetchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlPrefetch {
        match self.bits {
            false => CtlPrefetch::CtlPrefetchDisable,
            true => CtlPrefetch::CtlPrefetchEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl_prefetch_disable(&self) -> bool {
        *self == CtlPrefetch::CtlPrefetchDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl_prefetch_enable(&self) -> bool {
        *self == CtlPrefetch::CtlPrefetchEnable
    }
}
#[doc = "Field `CTL_PREFETCH` writer - Used to enable/disable instruction prefetch to Flash."]
pub type CtlPrefetchW<'a, REG> = crate::BitWriter<'a, REG, CtlPrefetch>;
impl<'a, REG> CtlPrefetchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl_prefetch_disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlPrefetch::CtlPrefetchDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl_prefetch_enable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlPrefetch::CtlPrefetchEnable)
    }
}
#[doc = "Used to enable/disable Instruction caching on flash access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlIcache {
    #[doc = "0: DISABLE"]
    CtlIcacheDisable = 0,
    #[doc = "1: ENABLE"]
    CtlIcacheEnable = 1,
}
impl From<CtlIcache> for bool {
    #[inline(always)]
    fn from(variant: CtlIcache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_ICACHE` reader - Used to enable/disable Instruction caching on flash access."]
pub type CtlIcacheR = crate::BitReader<CtlIcache>;
impl CtlIcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlIcache {
        match self.bits {
            false => CtlIcache::CtlIcacheDisable,
            true => CtlIcache::CtlIcacheEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl_icache_disable(&self) -> bool {
        *self == CtlIcache::CtlIcacheDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl_icache_enable(&self) -> bool {
        *self == CtlIcache::CtlIcacheEnable
    }
}
#[doc = "Field `CTL_ICACHE` writer - Used to enable/disable Instruction caching on flash access."]
pub type CtlIcacheW<'a, REG> = crate::BitWriter<'a, REG, CtlIcache>;
impl<'a, REG> CtlIcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl_icache_disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlIcache::CtlIcacheDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl_icache_enable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlIcache::CtlIcacheEnable)
    }
}
#[doc = "Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlLiten {
    #[doc = "0: DISABLE"]
    CtlLitenDisable = 0,
    #[doc = "1: ENABLE"]
    CtlLitenEnable = 1,
}
impl From<CtlLiten> for bool {
    #[inline(always)]
    fn from(variant: CtlLiten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_LITEN` reader - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
pub type CtlLitenR = crate::BitReader<CtlLiten>;
impl CtlLitenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlLiten {
        match self.bits {
            false => CtlLiten::CtlLitenDisable,
            true => CtlLiten::CtlLitenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl_liten_disable(&self) -> bool {
        *self == CtlLiten::CtlLitenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl_liten_enable(&self) -> bool {
        *self == CtlLiten::CtlLitenEnable
    }
}
#[doc = "Field `CTL_LITEN` writer - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
pub type CtlLitenW<'a, REG> = crate::BitWriter<'a, REG, CtlLiten>;
impl<'a, REG> CtlLitenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl_liten_disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlLiten::CtlLitenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl_liten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlLiten::CtlLitenEnable)
    }
}
impl R {
    #[doc = "Bit 0 - Used to enable/disable instruction prefetch to Flash."]
    #[inline(always)]
    pub fn ctl_prefetch(&self) -> CtlPrefetchR {
        CtlPrefetchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Used to enable/disable Instruction caching on flash access."]
    #[inline(always)]
    pub fn ctl_icache(&self) -> CtlIcacheR {
        CtlIcacheR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
    #[inline(always)]
    pub fn ctl_liten(&self) -> CtlLitenR {
        CtlLitenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to enable/disable instruction prefetch to Flash."]
    #[inline(always)]
    pub fn ctl_prefetch(&mut self) -> CtlPrefetchW<CtlSpec> {
        CtlPrefetchW::new(self, 0)
    }
    #[doc = "Bit 1 - Used to enable/disable Instruction caching on flash access."]
    #[inline(always)]
    pub fn ctl_icache(&mut self) -> CtlIcacheW<CtlSpec> {
        CtlIcacheW::new(self, 1)
    }
    #[doc = "Bit 2 - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
    #[inline(always)]
    pub fn ctl_liten(&mut self) -> CtlLitenW<CtlSpec> {
        CtlLitenW::new(self, 2)
    }
}
#[doc = "Prefetch/Cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x07"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x07;
}
