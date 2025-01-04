#[doc = "Register `DMAPRIO` reader"]
pub type R = crate::R<DmaprioSpec>;
#[doc = "Register `DMAPRIO` writer"]
pub type W = crate::W<DmaprioSpec>;
#[doc = "Round robin. This bit enables the round-robin DMA channel priorities.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaprioRoundrobin {
    #[doc = "0: DISABLE"]
    DmaprioRoundrobinDisable = 0,
    #[doc = "1: ENABLE"]
    DmaprioRoundrobinEnable = 1,
}
impl From<DmaprioRoundrobin> for bool {
    #[inline(always)]
    fn from(variant: DmaprioRoundrobin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAPRIO_ROUNDROBIN` reader - Round robin. This bit enables the round-robin DMA channel priorities."]
pub type DmaprioRoundrobinR = crate::BitReader<DmaprioRoundrobin>;
impl DmaprioRoundrobinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaprioRoundrobin {
        match self.bits {
            false => DmaprioRoundrobin::DmaprioRoundrobinDisable,
            true => DmaprioRoundrobin::DmaprioRoundrobinEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmaprio_roundrobin_disable(&self) -> bool {
        *self == DmaprioRoundrobin::DmaprioRoundrobinDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmaprio_roundrobin_enable(&self) -> bool {
        *self == DmaprioRoundrobin::DmaprioRoundrobinEnable
    }
}
#[doc = "Field `DMAPRIO_ROUNDROBIN` writer - Round robin. This bit enables the round-robin DMA channel priorities."]
pub type DmaprioRoundrobinW<'a, REG> = crate::BitWriter<'a, REG, DmaprioRoundrobin>;
impl<'a, REG> DmaprioRoundrobinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmaprio_roundrobin_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaprioRoundrobin::DmaprioRoundrobinDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmaprio_roundrobin_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaprioRoundrobin::DmaprioRoundrobinEnable)
    }
}
#[doc = "Define the burst size of a block transfer, before the priority is re-evaluated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaprioBurstsz {
    #[doc = "0: INFINITI"]
    DmaprioBurstszInfiniti = 0,
    #[doc = "1: BURST_8"]
    DmaprioBurstszBurst8 = 1,
    #[doc = "2: BUSRT_16"]
    DmaprioBurstszBusrt16 = 2,
    #[doc = "3: BURST_32"]
    DmaprioBurstszBurst32 = 3,
}
impl From<DmaprioBurstsz> for u8 {
    #[inline(always)]
    fn from(variant: DmaprioBurstsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmaprioBurstsz {
    type Ux = u8;
}
impl crate::IsEnum for DmaprioBurstsz {}
#[doc = "Field `DMAPRIO_BURSTSZ` reader - Define the burst size of a block transfer, before the priority is re-evaluated"]
pub type DmaprioBurstszR = crate::FieldReader<DmaprioBurstsz>;
impl DmaprioBurstszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaprioBurstsz {
        match self.bits {
            0 => DmaprioBurstsz::DmaprioBurstszInfiniti,
            1 => DmaprioBurstsz::DmaprioBurstszBurst8,
            2 => DmaprioBurstsz::DmaprioBurstszBusrt16,
            3 => DmaprioBurstsz::DmaprioBurstszBurst32,
            _ => unreachable!(),
        }
    }
    #[doc = "INFINITI"]
    #[inline(always)]
    pub fn is_dmaprio_burstsz_infiniti(&self) -> bool {
        *self == DmaprioBurstsz::DmaprioBurstszInfiniti
    }
    #[doc = "BURST_8"]
    #[inline(always)]
    pub fn is_dmaprio_burstsz_burst_8(&self) -> bool {
        *self == DmaprioBurstsz::DmaprioBurstszBurst8
    }
    #[doc = "BUSRT_16"]
    #[inline(always)]
    pub fn is_dmaprio_burstsz_busrt_16(&self) -> bool {
        *self == DmaprioBurstsz::DmaprioBurstszBusrt16
    }
    #[doc = "BURST_32"]
    #[inline(always)]
    pub fn is_dmaprio_burstsz_burst_32(&self) -> bool {
        *self == DmaprioBurstsz::DmaprioBurstszBurst32
    }
}
#[doc = "Field `DMAPRIO_BURSTSZ` writer - Define the burst size of a block transfer, before the priority is re-evaluated"]
pub type DmaprioBurstszW<'a, REG> = crate::FieldWriter<'a, REG, 2, DmaprioBurstsz, crate::Safe>;
impl<'a, REG> DmaprioBurstszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "INFINITI"]
    #[inline(always)]
    pub fn dmaprio_burstsz_infiniti(self) -> &'a mut crate::W<REG> {
        self.variant(DmaprioBurstsz::DmaprioBurstszInfiniti)
    }
    #[doc = "BURST_8"]
    #[inline(always)]
    pub fn dmaprio_burstsz_burst_8(self) -> &'a mut crate::W<REG> {
        self.variant(DmaprioBurstsz::DmaprioBurstszBurst8)
    }
    #[doc = "BUSRT_16"]
    #[inline(always)]
    pub fn dmaprio_burstsz_busrt_16(self) -> &'a mut crate::W<REG> {
        self.variant(DmaprioBurstsz::DmaprioBurstszBusrt16)
    }
    #[doc = "BURST_32"]
    #[inline(always)]
    pub fn dmaprio_burstsz_burst_32(self) -> &'a mut crate::W<REG> {
        self.variant(DmaprioBurstsz::DmaprioBurstszBurst32)
    }
}
impl R {
    #[doc = "Bit 0 - Round robin. This bit enables the round-robin DMA channel priorities."]
    #[inline(always)]
    pub fn dmaprio_roundrobin(&self) -> DmaprioRoundrobinR {
        DmaprioRoundrobinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - Define the burst size of a block transfer, before the priority is re-evaluated"]
    #[inline(always)]
    pub fn dmaprio_burstsz(&self) -> DmaprioBurstszR {
        DmaprioBurstszR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Round robin. This bit enables the round-robin DMA channel priorities."]
    #[inline(always)]
    pub fn dmaprio_roundrobin(&mut self) -> DmaprioRoundrobinW<DmaprioSpec> {
        DmaprioRoundrobinW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Define the burst size of a block transfer, before the priority is re-evaluated"]
    #[inline(always)]
    pub fn dmaprio_burstsz(&mut self) -> DmaprioBurstszW<DmaprioSpec> {
        DmaprioBurstszW::new(self, 16)
    }
}
#[doc = "DMA Channel Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaprio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaprio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaprioSpec;
impl crate::RegisterSpec for DmaprioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaprio::R`](R) reader structure"]
impl crate::Readable for DmaprioSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaprio::W`](W) writer structure"]
impl crate::Writable for DmaprioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAPRIO to value 0"]
impl crate::Resettable for DmaprioSpec {
    const RESET_VALUE: u32 = 0;
}
