#[doc = "Register `MCAN_TXBTO` reader"]
pub type R = crate::R<McanTxbtoSpec>;
#[doc = "Field `MCAN_TXBTO_TO0` reader - Transmission Occurred 0. Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0 No transmission occurred 1 Transmission occurred"]
pub type McanTxbtoTo0R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO1` reader - Transmission Occurred 1. See description for bit 0."]
pub type McanTxbtoTo1R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO2` reader - Transmission Occurred 2. See description for bit 0."]
pub type McanTxbtoTo2R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO3` reader - Transmission Occurred 3. See description for bit 0."]
pub type McanTxbtoTo3R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO4` reader - Transmission Occurred 4. See description for bit 0."]
pub type McanTxbtoTo4R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO5` reader - Transmission Occurred 5. See description for bit 0."]
pub type McanTxbtoTo5R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO6` reader - Transmission Occurred 6. See description for bit 0."]
pub type McanTxbtoTo6R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO7` reader - Transmission Occurred 7. See description for bit 0."]
pub type McanTxbtoTo7R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO8` reader - Transmission Occurred 8. See description for bit 0."]
pub type McanTxbtoTo8R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO9` reader - Transmission Occurred 9. See description for bit 0."]
pub type McanTxbtoTo9R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO10` reader - Transmission Occurred 10. See description for bit 0."]
pub type McanTxbtoTo10R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO11` reader - Transmission Occurred 11. See description for bit 0."]
pub type McanTxbtoTo11R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO12` reader - Transmission Occurred 12. See description for bit 0."]
pub type McanTxbtoTo12R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO13` reader - Transmission Occurred 13. See description for bit 0."]
pub type McanTxbtoTo13R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO14` reader - Transmission Occurred 14. See description for bit 0."]
pub type McanTxbtoTo14R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO15` reader - Transmission Occurred 15. See description for bit 0."]
pub type McanTxbtoTo15R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO16` reader - Transmission Occurred 16. See description for bit 0."]
pub type McanTxbtoTo16R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO17` reader - Transmission Occurred 17. See description for bit 0."]
pub type McanTxbtoTo17R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO18` reader - Transmission Occurred 18. See description for bit 0."]
pub type McanTxbtoTo18R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO19` reader - Transmission Occurred 19. See description for bit 0."]
pub type McanTxbtoTo19R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO20` reader - Transmission Occurred 20. See description for bit 0."]
pub type McanTxbtoTo20R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO21` reader - Transmission Occurred 21. See description for bit 0."]
pub type McanTxbtoTo21R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO22` reader - Transmission Occurred 22. See description for bit 0."]
pub type McanTxbtoTo22R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO23` reader - Transmission Occurred 23. See description for bit 0."]
pub type McanTxbtoTo23R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO24` reader - Transmission Occurred 24. See description for bit 0."]
pub type McanTxbtoTo24R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO25` reader - Transmission Occurred 25. See description for bit 0."]
pub type McanTxbtoTo25R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO26` reader - Transmission Occurred 26. See description for bit 0."]
pub type McanTxbtoTo26R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO27` reader - Transmission Occurred 27. See description for bit 0."]
pub type McanTxbtoTo27R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO28` reader - Transmission Occurred 28. See description for bit 0."]
pub type McanTxbtoTo28R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO29` reader - Transmission Occurred 29. See description for bit 0."]
pub type McanTxbtoTo29R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO30` reader - Transmission Occurred 30. See description for bit 0."]
pub type McanTxbtoTo30R = crate::BitReader;
#[doc = "Field `MCAN_TXBTO_TO31` reader - Transmission Occurred 31. See description for bit 0."]
pub type McanTxbtoTo31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Occurred 0. Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0 No transmission occurred 1 Transmission occurred"]
    #[inline(always)]
    pub fn mcan_txbto_to0(&self) -> McanTxbtoTo0R {
        McanTxbtoTo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Occurred 1. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to1(&self) -> McanTxbtoTo1R {
        McanTxbtoTo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Occurred 2. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to2(&self) -> McanTxbtoTo2R {
        McanTxbtoTo2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Occurred 3. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to3(&self) -> McanTxbtoTo3R {
        McanTxbtoTo3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Occurred 4. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to4(&self) -> McanTxbtoTo4R {
        McanTxbtoTo4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Occurred 5. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to5(&self) -> McanTxbtoTo5R {
        McanTxbtoTo5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Occurred 6. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to6(&self) -> McanTxbtoTo6R {
        McanTxbtoTo6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Occurred 7. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to7(&self) -> McanTxbtoTo7R {
        McanTxbtoTo7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Occurred 8. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to8(&self) -> McanTxbtoTo8R {
        McanTxbtoTo8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Occurred 9. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to9(&self) -> McanTxbtoTo9R {
        McanTxbtoTo9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Occurred 10. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to10(&self) -> McanTxbtoTo10R {
        McanTxbtoTo10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Occurred 11. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to11(&self) -> McanTxbtoTo11R {
        McanTxbtoTo11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmission Occurred 12. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to12(&self) -> McanTxbtoTo12R {
        McanTxbtoTo12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmission Occurred 13. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to13(&self) -> McanTxbtoTo13R {
        McanTxbtoTo13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmission Occurred 14. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to14(&self) -> McanTxbtoTo14R {
        McanTxbtoTo14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmission Occurred 15. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to15(&self) -> McanTxbtoTo15R {
        McanTxbtoTo15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Occurred 16. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to16(&self) -> McanTxbtoTo16R {
        McanTxbtoTo16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission Occurred 17. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to17(&self) -> McanTxbtoTo17R {
        McanTxbtoTo17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Occurred 18. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to18(&self) -> McanTxbtoTo18R {
        McanTxbtoTo18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Occurred 19. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to19(&self) -> McanTxbtoTo19R {
        McanTxbtoTo19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmission Occurred 20. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to20(&self) -> McanTxbtoTo20R {
        McanTxbtoTo20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmission Occurred 21. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to21(&self) -> McanTxbtoTo21R {
        McanTxbtoTo21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Occurred 22. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to22(&self) -> McanTxbtoTo22R {
        McanTxbtoTo22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmission Occurred 23. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to23(&self) -> McanTxbtoTo23R {
        McanTxbtoTo23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Occurred 24. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to24(&self) -> McanTxbtoTo24R {
        McanTxbtoTo24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission Occurred 25. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to25(&self) -> McanTxbtoTo25R {
        McanTxbtoTo25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Occurred 26. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to26(&self) -> McanTxbtoTo26R {
        McanTxbtoTo26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Occurred 27. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to27(&self) -> McanTxbtoTo27R {
        McanTxbtoTo27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Occurred 28. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to28(&self) -> McanTxbtoTo28R {
        McanTxbtoTo28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmission Occurred 29. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to29(&self) -> McanTxbtoTo29R {
        McanTxbtoTo29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmission Occurred 30. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to30(&self) -> McanTxbtoTo30R {
        McanTxbtoTo30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Occurred 31. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbto_to31(&self) -> McanTxbtoTo31R {
        McanTxbtoTo31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "MCAN Tx Buffer Transmission Occurred\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxbtoSpec;
impl crate::RegisterSpec for McanTxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txbto::R`](R) reader structure"]
impl crate::Readable for McanTxbtoSpec {}
#[doc = "`reset()` method sets MCAN_TXBTO to value 0"]
impl crate::Resettable for McanTxbtoSpec {
    const RESET_VALUE: u32 = 0;
}
