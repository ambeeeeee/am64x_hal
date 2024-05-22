#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1369` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1369` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec>;
#[doc = "Field `PHY_UPDATE_MASK` reader - 0:0\\]
Control to disable the generation of dfi_phyupd_req and use of dfi_ctrlupd_req. If this is 0 the PHY is normal mode; if this is 1, the PHY will not respond to dfi_ctrlupd_req or not to send dfi_phyupd_req"]
pub type PhyUpdateMaskR = crate::BitReader;
#[doc = "Field `PHY_UPDATE_MASK` writer - 0:0\\]
Control to disable the generation of dfi_phyupd_req and use of dfi_ctrlupd_req. If this is 0 the PHY is normal mode; if this is 1, the PHY will not respond to dfi_ctrlupd_req or not to send dfi_phyupd_req"]
pub type PhyUpdateMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ERR_IE` reader - 8:8\\]
Control the IE signal of IO error pad."]
pub type PhyErrIeR = crate::BitReader;
#[doc = "Field `PHY_ERR_IE` writer - 8:8\\]
Control the IE signal of IO error pad."]
pub type PhyErrIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_GRP_SLV_DLY_ENC_OBS_SELECT` reader - 20:16\\]
Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
pub type PhyGrpSlvDlyEncObsSelectR = crate::FieldReader;
#[doc = "Field `PHY_GRP_SLV_DLY_ENC_OBS_SELECT` writer - 20:16\\]
Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
pub type PhyGrpSlvDlyEncObsSelectW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_GRP_SHIFT_OBS_SELECT` reader - 27:24\\]
Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
pub type PhyGrpShiftObsSelectR = crate::FieldReader;
#[doc = "Field `PHY_GRP_SHIFT_OBS_SELECT` writer - 27:24\\]
Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
pub type PhyGrpShiftObsSelectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Control to disable the generation of dfi_phyupd_req and use of dfi_ctrlupd_req. If this is 0 the PHY is normal mode; if this is 1, the PHY will not respond to dfi_ctrlupd_req or not to send dfi_phyupd_req"]
    #[inline(always)]
    pub fn phy_update_mask(&self) -> PhyUpdateMaskR {
        PhyUpdateMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control the IE signal of IO error pad."]
    #[inline(always)]
    pub fn phy_err_ie(&self) -> PhyErrIeR {
        PhyErrIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
    #[inline(always)]
    pub fn phy_grp_slv_dly_enc_obs_select(&self) -> PhyGrpSlvDlyEncObsSelectR {
        PhyGrpSlvDlyEncObsSelectR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
    #[inline(always)]
    pub fn phy_grp_shift_obs_select(&self) -> PhyGrpShiftObsSelectR {
        PhyGrpShiftObsSelectR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Control to disable the generation of dfi_phyupd_req and use of dfi_ctrlupd_req. If this is 0 the PHY is normal mode; if this is 1, the PHY will not respond to dfi_ctrlupd_req or not to send dfi_phyupd_req"]
    #[inline(always)]
    #[must_use]
    pub fn phy_update_mask(&mut self) -> PhyUpdateMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec> {
        PhyUpdateMaskW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control the IE signal of IO error pad."]
    #[inline(always)]
    #[must_use]
    pub fn phy_err_ie(&mut self) -> PhyErrIeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec> {
        PhyErrIeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_slv_dly_enc_obs_select(
        &mut self,
    ) -> PhyGrpSlvDlyEncObsSelectW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec> {
        PhyGrpSlvDlyEncObsSelectW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_shift_obs_select(
        &mut self,
    ) -> PhyGrpShiftObsSelectW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec> {
        PhyGrpShiftObsSelectW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1369\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1369::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1369::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1369::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1369::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1369 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1369Spec {
    const RESET_VALUE: u32 = 0;
}
