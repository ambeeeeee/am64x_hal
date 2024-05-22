#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_180` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_180` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec>;
#[doc = "Field `PI_TREF_F2` reader - 19:0\\]
DRAM tREF value in memory clocks for frequency set 2."]
pub type PiTrefF2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TREF_F2` writer - 19:0\\]
DRAM tREF value in memory clocks for frequency set 2."]
pub type PiTrefF2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F0` reader - 27:24\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 0, the delay between a DFI command change and a memory command."]
pub type PiTdfiCtrlDelayF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F0` writer - 27:24\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 0, the delay between a DFI command change and a memory command."]
pub type PiTdfiCtrlDelayF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM tREF value in memory clocks for frequency set 2."]
    #[inline(always)]
    pub fn pi_tref_f2(&self) -> PiTrefF2R {
        PiTrefF2R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 0, the delay between a DFI command change and a memory command."]
    #[inline(always)]
    pub fn pi_tdfi_ctrl_delay_f0(&self) -> PiTdfiCtrlDelayF0R {
        PiTdfiCtrlDelayF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM tREF value in memory clocks for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tref_f2(&mut self) -> PiTrefF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec> {
        PiTrefF2W::new(self, 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 0, the delay between a DFI command change and a memory command."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrl_delay_f0(
        &mut self,
    ) -> PiTdfiCtrlDelayF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec> {
        PiTdfiCtrlDelayF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_180\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_180::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_180::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_180::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_180::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_180 to value 0x0200_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi180Spec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
