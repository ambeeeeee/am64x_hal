#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_16` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_16` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec>;
#[doc = "Field `PI_TREF_INTERVAL` reader - 19:0\\]
Defines the cycles between refreshes to different chip selects."]
pub type PiTrefIntervalR = crate::FieldReader<u32>;
#[doc = "Field `PI_TREF_INTERVAL` writer - 19:0\\]
Defines the cycles between refreshes to different chip selects."]
pub type PiTrefIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `PI_ON_DFIBUS` reader - 24:24\\]
Monitors the state of the PI controlling the DFI bus. 1 means PI is in control. READ-ONLY."]
pub type PiOnDfibusR = crate::BitReader;
#[doc = "Field `PI_ON_DFIBUS` writer - 24:24\\]
Monitors the state of the PI controlling the DFI bus. 1 means PI is in control. READ-ONLY."]
pub type PiOnDfibusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Defines the cycles between refreshes to different chip selects."]
    #[inline(always)]
    pub fn pi_tref_interval(&self) -> PiTrefIntervalR {
        PiTrefIntervalR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Monitors the state of the PI controlling the DFI bus. 1 means PI is in control. READ-ONLY."]
    #[inline(always)]
    pub fn pi_on_dfibus(&self) -> PiOnDfibusR {
        PiOnDfibusR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Defines the cycles between refreshes to different chip selects."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tref_interval(&mut self) -> PiTrefIntervalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec> {
        PiTrefIntervalW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Monitors the state of the PI controlling the DFI bus. 1 means PI is in control. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_on_dfibus(&mut self) -> PiOnDfibusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec> {
        PiOnDfibusW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_16::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_16::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_16 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi16Spec {
    const RESET_VALUE: u32 = 0;
}
