#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_71` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_71` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec>;
#[doc = "Field `CA_PARITY_ERROR` reader - 0:0\\]
Contains one hot indication of registered DIMM parity errors. Value of 1 indicates an error on that DIMM. READ-ONLY"]
pub type CaParityErrorR = crate::BitReader;
#[doc = "Field `CA_PARITY_ERROR` writer - 0:0\\]
Contains one hot indication of registered DIMM parity errors. Value of 1 indicates an error on that DIMM. READ-ONLY"]
pub type CaParityErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREFRESH` reader - 8:8\\]
Initiate auto-refresh at the end of the current burst boundary. Set to 1 to trigger. WRITE-ONLY"]
pub type ArefreshR = crate::BitReader;
#[doc = "Field `AREFRESH` writer - 8:8\\]
Initiate auto-refresh at the end of the current burst boundary. Set to 1 to trigger. WRITE-ONLY"]
pub type ArefreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREF_STATUS` reader - 16:16\\]
Indicates a SR error associated with the AREF interrupt. Value of 1 indicates a violation. READ-ONLY"]
pub type ArefStatusR = crate::BitReader;
#[doc = "Field `AREF_STATUS` writer - 16:16\\]
Indicates a SR error associated with the AREF interrupt. Value of 1 indicates a violation. READ-ONLY"]
pub type ArefStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TREF_ENABLE` reader - 24:24\\]
Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
pub type TrefEnableR = crate::BitReader;
#[doc = "Field `TREF_ENABLE` writer - 24:24\\]
Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
pub type TrefEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Contains one hot indication of registered DIMM parity errors. Value of 1 indicates an error on that DIMM. READ-ONLY"]
    #[inline(always)]
    pub fn ca_parity_error(&self) -> CaParityErrorR {
        CaParityErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate auto-refresh at the end of the current burst boundary. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn arefresh(&self) -> ArefreshR {
        ArefreshR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates a SR error associated with the AREF interrupt. Value of 1 indicates a violation. READ-ONLY"]
    #[inline(always)]
    pub fn aref_status(&self) -> ArefStatusR {
        ArefStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
    #[inline(always)]
    pub fn tref_enable(&self) -> TrefEnableR {
        TrefEnableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Contains one hot indication of registered DIMM parity errors. Value of 1 indicates an error on that DIMM. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn ca_parity_error(&mut self) -> CaParityErrorW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec> {
        CaParityErrorW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate auto-refresh at the end of the current burst boundary. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn arefresh(&mut self) -> ArefreshW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec> {
        ArefreshW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates a SR error associated with the AREF interrupt. Value of 1 indicates a violation. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn aref_status(&mut self) -> ArefStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec> {
        ArefStatusW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn tref_enable(&mut self) -> TrefEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec> {
        TrefEnableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_71::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_71::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_71::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_71::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_71 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl71Spec {
    const RESET_VALUE: u32 = 0;
}
