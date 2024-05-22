#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1295` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1295` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec>;
#[doc = "Field `PHY_CSLVL_COARSE_CHK` reader - 10:0\\]
Defines the CS training coarse CA training DDL 1/16th cycle delay value."]
pub type PhyCslvlCoarseChkR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_COARSE_CHK` writer - 10:0\\]
Defines the CS training coarse CA training DDL 1/16th cycle delay value."]
pub type PhyCslvlCoarseChkW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CSLVL_COARSE_CAPTURE_CNT` reader - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training coarse CA training."]
pub type PhyCslvlCoarseCaptureCntR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_COARSE_CAPTURE_CNT` writer - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training coarse CA training."]
pub type PhyCslvlCoarseCaptureCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADRCTL_SLAVE_LOOP_CNT_UPDATE` reader - 26:24\\]
Reserved for the address/control master."]
pub type PhyAdrctlSlaveLoopCntUpdateR = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_SLAVE_LOOP_CNT_UPDATE` writer - 26:24\\]
Reserved for the address/control master."]
pub type PhyAdrctlSlaveLoopCntUpdateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the CS training coarse CA training DDL 1/16th cycle delay value."]
    #[inline(always)]
    pub fn phy_cslvl_coarse_chk(&self) -> PhyCslvlCoarseChkR {
        PhyCslvlCoarseChkR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training coarse CA training."]
    #[inline(always)]
    pub fn phy_cslvl_coarse_capture_cnt(&self) -> PhyCslvlCoarseCaptureCntR {
        PhyCslvlCoarseCaptureCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Reserved for the address/control master."]
    #[inline(always)]
    pub fn phy_adrctl_slave_loop_cnt_update(&self) -> PhyAdrctlSlaveLoopCntUpdateR {
        PhyAdrctlSlaveLoopCntUpdateR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the CS training coarse CA training DDL 1/16th cycle delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_coarse_chk(
        &mut self,
    ) -> PhyCslvlCoarseChkW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec> {
        PhyCslvlCoarseChkW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training coarse CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_coarse_capture_cnt(
        &mut self,
    ) -> PhyCslvlCoarseCaptureCntW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec> {
        PhyCslvlCoarseCaptureCntW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Reserved for the address/control master."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_slave_loop_cnt_update(
        &mut self,
    ) -> PhyAdrctlSlaveLoopCntUpdateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec> {
        PhyAdrctlSlaveLoopCntUpdateW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1295\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1295::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1295::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1295::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1295::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1295 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1295Spec {
    const RESET_VALUE: u32 = 0;
}
