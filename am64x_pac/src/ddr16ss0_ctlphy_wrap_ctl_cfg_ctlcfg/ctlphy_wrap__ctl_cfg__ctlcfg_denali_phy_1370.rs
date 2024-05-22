#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1370` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1370` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec>;
#[doc = "Field `PHY_GRP_SLV_DLY_ENC_OBS` reader - 10:0\\]
Observation register for all address/control group slice slave delay encoded values. READ-ONLY"]
pub type PhyGrpSlvDlyEncObsR = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_SLV_DLY_ENC_OBS` writer - 10:0\\]
Observation register for all address/control group slice slave delay encoded values. READ-ONLY"]
pub type PhyGrpSlvDlyEncObsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_GRP_SHIFT_OBS` reader - 18:16\\]
Observation register for the address/control group automatic half cycle and cycle shift values. READ-ONLY"]
pub type PhyGrpShiftObsR = crate::FieldReader;
#[doc = "Field `PHY_GRP_SHIFT_OBS` writer - 18:16\\]
Observation register for the address/control group automatic half cycle and cycle shift values. READ-ONLY"]
pub type PhyGrpShiftObsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register for all address/control group slice slave delay encoded values. READ-ONLY"]
    #[inline(always)]
    pub fn phy_grp_slv_dly_enc_obs(&self) -> PhyGrpSlvDlyEncObsR {
        PhyGrpSlvDlyEncObsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register for the address/control group automatic half cycle and cycle shift values. READ-ONLY"]
    #[inline(always)]
    pub fn phy_grp_shift_obs(&self) -> PhyGrpShiftObsR {
        PhyGrpShiftObsR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register for all address/control group slice slave delay encoded values. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_slv_dly_enc_obs(
        &mut self,
    ) -> PhyGrpSlvDlyEncObsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec> {
        PhyGrpSlvDlyEncObsW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Observation register for the address/control group automatic half cycle and cycle shift values. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_shift_obs(
        &mut self,
    ) -> PhyGrpShiftObsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec> {
        PhyGrpShiftObsW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1370\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1370::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1370::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1370::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1370::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1370 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1370Spec {
    const RESET_VALUE: u32 = 0;
}
