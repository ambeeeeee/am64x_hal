#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_782` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_782` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_1` reader - 23:0\\]
CA training RD DQ bit swizzle map 1 for address slice 1."]
pub type PhyAdrCalvlSwizzle1_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_1` writer - 23:0\\]
CA training RD DQ bit swizzle map 1 for address slice 1."]
pub type PhyAdrCalvlSwizzle1_1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_1` reader - 25:24\\]
CA training rank aggregation control bits for address slice 1."]
pub type PhyAdrCalvlRankCtrl1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_1` writer - 25:24\\]
CA training rank aggregation control bits for address slice 1."]
pub type PhyAdrCalvlRankCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
CA training RD DQ bit swizzle map 1 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle1_1(&self) -> PhyAdrCalvlSwizzle1_1R {
        PhyAdrCalvlSwizzle1_1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - 25:24\\]
CA training rank aggregation control bits for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_rank_ctrl_1(&self) -> PhyAdrCalvlRankCtrl1R {
        PhyAdrCalvlRankCtrl1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
CA training RD DQ bit swizzle map 1 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle1_1(
        &mut self,
    ) -> PhyAdrCalvlSwizzle1_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec> {
        PhyAdrCalvlSwizzle1_1W::new(self, 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
CA training rank aggregation control bits for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_rank_ctrl_1(
        &mut self,
    ) -> PhyAdrCalvlRankCtrl1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec> {
        PhyAdrCalvlRankCtrl1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_782\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_782::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_782::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_782::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_782::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_782 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy782Spec {
    const RESET_VALUE: u32 = 0;
}
