#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_64` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_64` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec>;
#[doc = "Field `PI_VREF_CS` reader - 0:0\\]
Specifies the target chip select for the VREF training operation."]
pub type PiVrefCsR = crate::BitReader;
#[doc = "Field `PI_VREF_CS` writer - 0:0\\]
Specifies the target chip select for the VREF training operation."]
pub type PiVrefCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_VREF_PDA_EN` reader - 8:8\\]
Enable per-DRAM addressability during VREF training. Set to 1 to enable."]
pub type PiVrefPdaEnR = crate::BitReader;
#[doc = "Field `PI_VREF_PDA_EN` writer - 8:8\\]
Enable per-DRAM addressability during VREF training. Set to 1 to enable."]
pub type PiVrefPdaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_VREFLVL_DISABLE_DFS` reader - 16:16\\]
Disables automatic VREF training on freq change. Set to 1 to disable."]
pub type PiVreflvlDisableDfsR = crate::BitReader;
#[doc = "Field `PI_VREFLVL_DISABLE_DFS` writer - 16:16\\]
Disables automatic VREF training on freq change. Set to 1 to disable."]
pub type PiVreflvlDisableDfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MC_DFS_PI_SET_VREF_ENABLE` reader - 24:24\\]
Enable the PI to set VREF value after DFS issued by MC. MR12 and MR14 for LPDDR4. MR6 for DDR4. 1 means disable."]
pub type PiMcDfsPiSetVrefEnableR = crate::BitReader;
#[doc = "Field `PI_MC_DFS_PI_SET_VREF_ENABLE` writer - 24:24\\]
Enable the PI to set VREF value after DFS issued by MC. MR12 and MR14 for LPDDR4. MR6 for DDR4. 1 means disable."]
pub type PiMcDfsPiSetVrefEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Specifies the target chip select for the VREF training operation."]
    #[inline(always)]
    pub fn pi_vref_cs(&self) -> PiVrefCsR {
        PiVrefCsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable per-DRAM addressability during VREF training. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_vref_pda_en(&self) -> PiVrefPdaEnR {
        PiVrefPdaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables automatic VREF training on freq change. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_vreflvl_disable_dfs(&self) -> PiVreflvlDisableDfsR {
        PiVreflvlDisableDfsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable the PI to set VREF value after DFS issued by MC. MR12 and MR14 for LPDDR4. MR6 for DDR4. 1 means disable."]
    #[inline(always)]
    pub fn pi_mc_dfs_pi_set_vref_enable(&self) -> PiMcDfsPiSetVrefEnableR {
        PiMcDfsPiSetVrefEnableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Specifies the target chip select for the VREF training operation."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_cs(&mut self) -> PiVrefCsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec> {
        PiVrefCsW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable per-DRAM addressability during VREF training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_pda_en(&mut self) -> PiVrefPdaEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec> {
        PiVrefPdaEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables automatic VREF training on freq change. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vreflvl_disable_dfs(
        &mut self,
    ) -> PiVreflvlDisableDfsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec> {
        PiVreflvlDisableDfsW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable the PI to set VREF value after DFS issued by MC. MR12 and MR14 for LPDDR4. MR6 for DDR4. 1 means disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mc_dfs_pi_set_vref_enable(
        &mut self,
    ) -> PiMcDfsPiSetVrefEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec> {
        PiMcDfsPiSetVrefEnableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_64::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_64::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_64 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi64Spec {
    const RESET_VALUE: u32 = 0;
}
