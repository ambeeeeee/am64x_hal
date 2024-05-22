#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_17` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_17` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec>;
#[doc = "Field `PI_DATA_RETENTION` reader - 0:0\\]
Monitors the readiness for the PHY to be put into data retention mode after pi_sref_entry req parameter has been written. 1 means ready for data retention. READ-ONLY."]
pub type PiDataRetentionR = crate::BitReader;
#[doc = "Field `PI_DATA_RETENTION` writer - 0:0\\]
Monitors the readiness for the PHY to be put into data retention mode after pi_sref_entry req parameter has been written. 1 means ready for data retention. READ-ONLY."]
pub type PiDataRetentionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_LOAD` reader - 8:8\\]
User request to load delays and execute software leveling. Set to 1 to trigger. WRITE-ONLY"]
pub type PiSwlvlLoadR = crate::BitReader;
#[doc = "Field `PI_SWLVL_LOAD` writer - 8:8\\]
User request to load delays and execute software leveling. Set to 1 to trigger. WRITE-ONLY"]
pub type PiSwlvlLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_OP_DONE` reader - 16:16\\]
Reports the status of the software leveling operation. Value of 1 indicates operation complete. READ-ONLY"]
pub type PiSwlvlOpDoneR = crate::BitReader;
#[doc = "Field `PI_SWLVL_OP_DONE` writer - 16:16\\]
Reports the status of the software leveling operation. Value of 1 indicates operation complete. READ-ONLY"]
pub type PiSwlvlOpDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SW_WRLVL_RESP_0` reader - 24:24\\]
Write leveling response for data slice 0. READ-ONLY"]
pub type PiSwWrlvlResp0R = crate::BitReader;
#[doc = "Field `PI_SW_WRLVL_RESP_0` writer - 24:24\\]
Write leveling response for data slice 0. READ-ONLY"]
pub type PiSwWrlvlResp0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Monitors the readiness for the PHY to be put into data retention mode after pi_sref_entry req parameter has been written. 1 means ready for data retention. READ-ONLY."]
    #[inline(always)]
    pub fn pi_data_retention(&self) -> PiDataRetentionR {
        PiDataRetentionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
User request to load delays and execute software leveling. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_load(&self) -> PiSwlvlLoadR {
        PiSwlvlLoadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reports the status of the software leveling operation. Value of 1 indicates operation complete. READ-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_op_done(&self) -> PiSwlvlOpDoneR {
        PiSwlvlOpDoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Write leveling response for data slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn pi_sw_wrlvl_resp_0(&self) -> PiSwWrlvlResp0R {
        PiSwWrlvlResp0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Monitors the readiness for the PHY to be put into data retention mode after pi_sref_entry req parameter has been written. 1 means ready for data retention. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_data_retention(
        &mut self,
    ) -> PiDataRetentionW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec> {
        PiDataRetentionW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
User request to load delays and execute software leveling. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_load(&mut self) -> PiSwlvlLoadW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec> {
        PiSwlvlLoadW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Reports the status of the software leveling operation. Value of 1 indicates operation complete. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_op_done(&mut self) -> PiSwlvlOpDoneW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec> {
        PiSwlvlOpDoneW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Write leveling response for data slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_wrlvl_resp_0(
        &mut self,
    ) -> PiSwWrlvlResp0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec> {
        PiSwWrlvlResp0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_17::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_17::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_17 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi17Spec {
    const RESET_VALUE: u32 = 0;
}
