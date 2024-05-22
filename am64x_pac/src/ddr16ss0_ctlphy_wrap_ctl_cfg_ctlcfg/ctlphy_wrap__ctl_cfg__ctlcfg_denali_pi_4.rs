#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_4` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_4` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec>;
#[doc = "Field `PI_INIT_LVL_EN` reader - 0:0\\]
Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
pub type PiInitLvlEnR = crate::BitReader;
#[doc = "Field `PI_INIT_LVL_EN` writer - 0:0\\]
Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
pub type PiInitLvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_NOTCARE_PHYUPD` reader - 9:8\\]
Allow the PI to issue a master request to the controller if a phyupd_req from the PHY has been detected.bit\\[1\\]
represents supports in normal state;bit\\[0\\]
represents supports in initialization state. Set to 1 to issue the master request."]
pub type PiNotcarePhyupdR = crate::FieldReader;
#[doc = "Field `PI_NOTCARE_PHYUPD` writer - 9:8\\]
Allow the PI to issue a master request to the controller if a phyupd_req from the PHY has been detected.bit\\[1\\]
represents supports in normal state;bit\\[0\\]
represents supports in initialization state. Set to 1 to issue the master request."]
pub type PiNotcarePhyupdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TCMD_GAP` reader - 31:16\\]
Specifies the minimum gap in DFI clocks between two commands. Used to guard the timing from the last command of MC and the first command of PI when MC hand over the control of DFI to PI."]
pub type PiTcmdGapR = crate::FieldReader<u16>;
#[doc = "Field `PI_TCMD_GAP` writer - 31:16\\]
Specifies the minimum gap in DFI clocks between two commands. Used to guard the timing from the last command of MC and the first command of PI when MC hand over the control of DFI to PI."]
pub type PiTcmdGapW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_init_lvl_en(&self) -> PiInitLvlEnR {
        PiInitLvlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Allow the PI to issue a master request to the controller if a phyupd_req from the PHY has been detected.bit\\[1\\]
represents supports in normal state;bit\\[0\\]
represents supports in initialization state. Set to 1 to issue the master request."]
    #[inline(always)]
    pub fn pi_notcare_phyupd(&self) -> PiNotcarePhyupdR {
        PiNotcarePhyupdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Specifies the minimum gap in DFI clocks between two commands. Used to guard the timing from the last command of MC and the first command of PI when MC hand over the control of DFI to PI."]
    #[inline(always)]
    pub fn pi_tcmd_gap(&self) -> PiTcmdGapR {
        PiTcmdGapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_lvl_en(&mut self) -> PiInitLvlEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec> {
        PiInitLvlEnW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Allow the PI to issue a master request to the controller if a phyupd_req from the PHY has been detected.bit\\[1\\]
represents supports in normal state;bit\\[0\\]
represents supports in initialization state. Set to 1 to issue the master request."]
    #[inline(always)]
    #[must_use]
    pub fn pi_notcare_phyupd(&mut self) -> PiNotcarePhyupdW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec> {
        PiNotcarePhyupdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Specifies the minimum gap in DFI clocks between two commands. Used to guard the timing from the last command of MC and the first command of PI when MC hand over the control of DFI to PI."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcmd_gap(&mut self) -> PiTcmdGapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec> {
        PiTcmdGapW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_4::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_4::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_4 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi4Spec {
    const RESET_VALUE: u32 = 0;
}
