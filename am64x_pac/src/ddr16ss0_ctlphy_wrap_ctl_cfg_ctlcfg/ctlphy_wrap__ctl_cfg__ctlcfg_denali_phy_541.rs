#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_541` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_541` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_0` reader - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
pub type PhyAdrLp4BootSlvDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_0` writer - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
pub type PhyAdrLp4BootSlvDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_BIT_MASK_0` reader - 21:16\\]
Mask bit for address slice 0. Set to 1 to indicate that the bit is used."]
pub type PhyAdrBitMask0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_BIT_MASK_0` writer - 21:16\\]
Mask bit for address slice 0. Set to 1 to indicate that the bit is used."]
pub type PhyAdrBitMask0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_SEG_MASK_0` reader - 29:24\\]
Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SEG_MASK_0` writer - 29:24\\]
Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_lp4_boot_slv_delay_0(&self) -> PhyAdrLp4BootSlvDelay0R {
        PhyAdrLp4BootSlvDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Mask bit for address slice 0. Set to 1 to indicate that the bit is used."]
    #[inline(always)]
    pub fn phy_adr_bit_mask_0(&self) -> PhyAdrBitMask0R {
        PhyAdrBitMask0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    pub fn phy_adr_seg_mask_0(&self) -> PhyAdrSegMask0R {
        PhyAdrSegMask0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lp4_boot_slv_delay_0(
        &mut self,
    ) -> PhyAdrLp4BootSlvDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec> {
        PhyAdrLp4BootSlvDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Mask bit for address slice 0. Set to 1 to indicate that the bit is used."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_bit_mask_0(
        &mut self,
    ) -> PhyAdrBitMask0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec> {
        PhyAdrBitMask0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_seg_mask_0(
        &mut self,
    ) -> PhyAdrSegMask0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec> {
        PhyAdrSegMask0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_541\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_541::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_541::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_541::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_541::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_541 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy541Spec {
    const RESET_VALUE: u32 = 0;
}
