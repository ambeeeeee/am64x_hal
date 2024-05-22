#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_21` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_21` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec>;
#[doc = "Field `TSREF2PHYMSTR` reader - 5:0\\]
Specifies the minimum time after a self-refresh exit command on the DFI bus that the Controller will wait for the PHY to assert the dfi_phymstr_req signal, before completing other commands. Used when the low power control logic is expected to pass control to the PHY for training when exiting SREF."]
pub type Tsref2phymstrR = crate::FieldReader;
#[doc = "Field `TSREF2PHYMSTR` writer - 5:0\\]
Specifies the minimum time after a self-refresh exit command on the DFI bus that the Controller will wait for the PHY to assert the dfi_phymstr_req signal, before completing other commands. Used when the low power control logic is expected to pass control to the PHY for training when exiting SREF."]
pub type Tsref2phymstrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_INDEP_INIT_MODE` reader - 8:8\\]
Enable PHY independent initailization mode commands during initialization. Set to 1 to enable."]
pub type PhyIndepInitModeR = crate::BitReader;
#[doc = "Field `PHY_INDEP_INIT_MODE` writer - 8:8\\]
Enable PHY independent initailization mode commands during initialization. Set to 1 to enable."]
pub type PhyIndepInitModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIBUS_FREQ_F0` reader - 20:16\\]
Defines the DFI bus frequency. FC=0"]
pub type DfibusFreqF0R = crate::FieldReader;
#[doc = "Field `DFIBUS_FREQ_F0` writer - 20:16\\]
Defines the DFI bus frequency. FC=0"]
pub type DfibusFreqF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DFIBUS_FREQ_F1` reader - 28:24\\]
Defines the DFI bus frequency. FC=1"]
pub type DfibusFreqF1R = crate::FieldReader;
#[doc = "Field `DFIBUS_FREQ_F1` writer - 28:24\\]
Defines the DFI bus frequency. FC=1"]
pub type DfibusFreqF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Specifies the minimum time after a self-refresh exit command on the DFI bus that the Controller will wait for the PHY to assert the dfi_phymstr_req signal, before completing other commands. Used when the low power control logic is expected to pass control to the PHY for training when exiting SREF."]
    #[inline(always)]
    pub fn tsref2phymstr(&self) -> Tsref2phymstrR {
        Tsref2phymstrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable PHY independent initailization mode commands during initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_indep_init_mode(&self) -> PhyIndepInitModeR {
        PhyIndepInitModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the DFI bus frequency. FC=0"]
    #[inline(always)]
    pub fn dfibus_freq_f0(&self) -> DfibusFreqF0R {
        DfibusFreqF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the DFI bus frequency. FC=1"]
    #[inline(always)]
    pub fn dfibus_freq_f1(&self) -> DfibusFreqF1R {
        DfibusFreqF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Specifies the minimum time after a self-refresh exit command on the DFI bus that the Controller will wait for the PHY to assert the dfi_phymstr_req signal, before completing other commands. Used when the low power control logic is expected to pass control to the PHY for training when exiting SREF."]
    #[inline(always)]
    #[must_use]
    pub fn tsref2phymstr(&mut self) -> Tsref2phymstrW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec> {
        Tsref2phymstrW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable PHY independent initailization mode commands during initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_indep_init_mode(
        &mut self,
    ) -> PhyIndepInitModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec> {
        PhyIndepInitModeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the DFI bus frequency. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_freq_f0(&mut self) -> DfibusFreqF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec> {
        DfibusFreqF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the DFI bus frequency. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_freq_f1(&mut self) -> DfibusFreqF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec> {
        DfibusFreqF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_21::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_21::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_21 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl21Spec {
    const RESET_VALUE: u32 = 0;
}
