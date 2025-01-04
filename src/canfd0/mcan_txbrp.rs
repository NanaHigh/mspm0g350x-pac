#[doc = "Register `MCAN_TXBRP` reader"]
pub type R = crate::R<McanTxbrpSpec>;
#[doc = "Field `MCAN_TXBRP_TRP0` reader - Transmission Request Pending 0. Each Tx Buffer has its own Transmission Request Pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been cancelled via register TXBCR. TXBRP bits are set only for those Tx Buffers configured via TXBC. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signalled via TXBCF - after successful transmission together with the corresponding TXBTO bit - when the transmission has not yet been started at the point of cancellation - when the transmission has been aborted due to lost arbitration - when an error occurred during frame transmission In DAR mode all transmissions are automatically cancelled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions. 0 No transmission request pending 1 Transmission request pending Note: TXBRP bits which are set while a Tx scan is in progress are not considered during this particular Tx scan. In case a cancellation is requested for such a Tx Buffer, this Add Request is cancelled immediately, the corresponding TXBRP bit is reset."]
pub type McanTxbrpTrp0R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP1` reader - Transmission Request Pending 1. See description for bit 0."]
pub type McanTxbrpTrp1R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP2` reader - Transmission Request Pending 2. See description for bit 0."]
pub type McanTxbrpTrp2R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP3` reader - Transmission Request Pending 3. See description for bit 0."]
pub type McanTxbrpTrp3R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP4` reader - Transmission Request Pending 4. See description for bit 0."]
pub type McanTxbrpTrp4R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP5` reader - Transmission Request Pending 5. See description for bit 0."]
pub type McanTxbrpTrp5R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP6` reader - Transmission Request Pending 6. See description for bit 0."]
pub type McanTxbrpTrp6R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP7` reader - Transmission Request Pending 7. See description for bit 0."]
pub type McanTxbrpTrp7R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP8` reader - Transmission Request Pending 8. See description for bit 0."]
pub type McanTxbrpTrp8R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP9` reader - Transmission Request Pending 9. See description for bit 0."]
pub type McanTxbrpTrp9R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP10` reader - Transmission Request Pending 10. See description for bit 0."]
pub type McanTxbrpTrp10R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP11` reader - Transmission Request Pending 11. See description for bit 0."]
pub type McanTxbrpTrp11R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP12` reader - Transmission Request Pending 12. See description for bit 0."]
pub type McanTxbrpTrp12R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP13` reader - Transmission Request Pending 13. See description for bit 0."]
pub type McanTxbrpTrp13R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP14` reader - Transmission Request Pending 14. See description for bit 0."]
pub type McanTxbrpTrp14R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP15` reader - Transmission Request Pending 15. See description for bit 0."]
pub type McanTxbrpTrp15R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP16` reader - Transmission Request Pending 16. See description for bit 0."]
pub type McanTxbrpTrp16R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP17` reader - Transmission Request Pending 17. See description for bit 0."]
pub type McanTxbrpTrp17R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP18` reader - Transmission Request Pending 18. See description for bit 0."]
pub type McanTxbrpTrp18R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP19` reader - Transmission Request Pending 19. See description for bit 0."]
pub type McanTxbrpTrp19R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP20` reader - Transmission Request Pending 20. See description for bit 0."]
pub type McanTxbrpTrp20R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP21` reader - Transmission Request Pending 21. See description for bit 0."]
pub type McanTxbrpTrp21R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP22` reader - Transmission Request Pending 22. See description for bit 0."]
pub type McanTxbrpTrp22R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP23` reader - Transmission Request Pending 23. See description for bit 0."]
pub type McanTxbrpTrp23R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP24` reader - Transmission Request Pending 24. See description for bit 0."]
pub type McanTxbrpTrp24R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP25` reader - Transmission Request Pending 25. See description for bit 0."]
pub type McanTxbrpTrp25R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP26` reader - Transmission Request Pending 26. See description for bit 0."]
pub type McanTxbrpTrp26R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP27` reader - Transmission Request Pending 27. See description for bit 0."]
pub type McanTxbrpTrp27R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP28` reader - Transmission Request Pending 28. See description for bit 0."]
pub type McanTxbrpTrp28R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP29` reader - Transmission Request Pending 29. See description for bit 0."]
pub type McanTxbrpTrp29R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP30` reader - Transmission Request Pending 30. See description for bit 0."]
pub type McanTxbrpTrp30R = crate::BitReader;
#[doc = "Field `MCAN_TXBRP_TRP31` reader - Transmission Request Pending 31. See description for bit 0."]
pub type McanTxbrpTrp31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Request Pending 0. Each Tx Buffer has its own Transmission Request Pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been cancelled via register TXBCR. TXBRP bits are set only for those Tx Buffers configured via TXBC. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signalled via TXBCF - after successful transmission together with the corresponding TXBTO bit - when the transmission has not yet been started at the point of cancellation - when the transmission has been aborted due to lost arbitration - when an error occurred during frame transmission In DAR mode all transmissions are automatically cancelled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions. 0 No transmission request pending 1 Transmission request pending Note: TXBRP bits which are set while a Tx scan is in progress are not considered during this particular Tx scan. In case a cancellation is requested for such a Tx Buffer, this Add Request is cancelled immediately, the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn mcan_txbrp_trp0(&self) -> McanTxbrpTrp0R {
        McanTxbrpTrp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Request Pending 1. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp1(&self) -> McanTxbrpTrp1R {
        McanTxbrpTrp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Pending 2. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp2(&self) -> McanTxbrpTrp2R {
        McanTxbrpTrp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Request Pending 3. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp3(&self) -> McanTxbrpTrp3R {
        McanTxbrpTrp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Request Pending 4. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp4(&self) -> McanTxbrpTrp4R {
        McanTxbrpTrp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Request Pending 5. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp5(&self) -> McanTxbrpTrp5R {
        McanTxbrpTrp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Request Pending 6. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp6(&self) -> McanTxbrpTrp6R {
        McanTxbrpTrp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Request Pending 7. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp7(&self) -> McanTxbrpTrp7R {
        McanTxbrpTrp7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Request Pending 8. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp8(&self) -> McanTxbrpTrp8R {
        McanTxbrpTrp8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Request Pending 9. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp9(&self) -> McanTxbrpTrp9R {
        McanTxbrpTrp9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Request Pending 10. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp10(&self) -> McanTxbrpTrp10R {
        McanTxbrpTrp10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Request Pending 11. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp11(&self) -> McanTxbrpTrp11R {
        McanTxbrpTrp11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmission Request Pending 12. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp12(&self) -> McanTxbrpTrp12R {
        McanTxbrpTrp12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmission Request Pending 13. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp13(&self) -> McanTxbrpTrp13R {
        McanTxbrpTrp13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmission Request Pending 14. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp14(&self) -> McanTxbrpTrp14R {
        McanTxbrpTrp14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmission Request Pending 15. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp15(&self) -> McanTxbrpTrp15R {
        McanTxbrpTrp15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Request Pending 16. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp16(&self) -> McanTxbrpTrp16R {
        McanTxbrpTrp16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission Request Pending 17. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp17(&self) -> McanTxbrpTrp17R {
        McanTxbrpTrp17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Request Pending 18. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp18(&self) -> McanTxbrpTrp18R {
        McanTxbrpTrp18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Request Pending 19. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp19(&self) -> McanTxbrpTrp19R {
        McanTxbrpTrp19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmission Request Pending 20. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp20(&self) -> McanTxbrpTrp20R {
        McanTxbrpTrp20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmission Request Pending 21. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp21(&self) -> McanTxbrpTrp21R {
        McanTxbrpTrp21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Request Pending 22. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp22(&self) -> McanTxbrpTrp22R {
        McanTxbrpTrp22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmission Request Pending 23. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp23(&self) -> McanTxbrpTrp23R {
        McanTxbrpTrp23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Request Pending 24. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp24(&self) -> McanTxbrpTrp24R {
        McanTxbrpTrp24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission Request Pending 25. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp25(&self) -> McanTxbrpTrp25R {
        McanTxbrpTrp25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Request Pending 26. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp26(&self) -> McanTxbrpTrp26R {
        McanTxbrpTrp26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Request Pending 27. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp27(&self) -> McanTxbrpTrp27R {
        McanTxbrpTrp27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Request Pending 28. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp28(&self) -> McanTxbrpTrp28R {
        McanTxbrpTrp28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmission Request Pending 29. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp29(&self) -> McanTxbrpTrp29R {
        McanTxbrpTrp29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmission Request Pending 30. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp30(&self) -> McanTxbrpTrp30R {
        McanTxbrpTrp30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Request Pending 31. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbrp_trp31(&self) -> McanTxbrpTrp31R {
        McanTxbrpTrp31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "MCAN Tx Buffer Request Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxbrpSpec;
impl crate::RegisterSpec for McanTxbrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txbrp::R`](R) reader structure"]
impl crate::Readable for McanTxbrpSpec {}
#[doc = "`reset()` method sets MCAN_TXBRP to value 0"]
impl crate::Resettable for McanTxbrpSpec {
    const RESET_VALUE: u32 = 0;
}
