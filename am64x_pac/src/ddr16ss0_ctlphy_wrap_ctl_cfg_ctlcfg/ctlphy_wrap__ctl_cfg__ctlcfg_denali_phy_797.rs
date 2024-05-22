#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_797` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_797` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_1` reader - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
pub type PhyAdrLp4BootSlvDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_1` writer - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
pub type PhyAdrLp4BootSlvDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_BIT_MASK_1` reader - 21:16\\]
Mask bit for address slice 1. Set to 1 to indicate that the bit is used."]
pub type PhyAdrBitMask1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_BIT_MASK_1` writer - 21:16\\]
Mask bit for address slice 1. Set to 1 to indicate that the bit is used."]
pub type PhyAdrBitMask1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_SEG_MASK_1` reader - 29:24\\]
Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SEG_MASK_1` writer - 29:24\\]
Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_lp4_boot_slv_delay_1(&self) -> PhyAdrLp4BootSlvDelay1R {
        PhyAdrLp4BootSlvDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Mask bit for address slice 1. Set to 1 to indicate that the bit is used."]
    #[inline(always)]
    pub fn phy_adr_bit_mask_1(&self) -> PhyAdrBitMask1R {
        PhyAdrBitMask1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    pub fn phy_adr_seg_mask_1(&self) -> PhyAdrSegMask1R {
        PhyAdrSegMask1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lp4_boot_slv_delay_1(
        &mut self,
    ) -> PhyAdrLp4BootSlvDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec> {
        PhyAdrLp4BootSlvDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Mask bit for address slice 1. Set to 1 to indicate that the bit is used."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_bit_mask_1(
        &mut self,
    ) -> PhyAdrBitMask1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec> {
        PhyAdrBitMask1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_seg_mask_1(
        &mut self,
    ) -> PhyAdrSegMask1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec> {
        PhyAdrSegMask1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_797\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_797::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_797::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_797::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_797::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_797 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy797Spec {
    const RESET_VALUE: u32 = 0;
}
