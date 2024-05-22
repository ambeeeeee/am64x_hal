#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_75` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_75` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_WW` reader - 9:0\\]
Minimum number of DFI clocks to be inserted between write commands during the DM portion of write DQ training."]
pub type PiTdfiWdqlvlWwR = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_WW` writer - 9:0\\]
Minimum number of DFI clocks to be inserted between write commands during the DM portion of write DQ training."]
pub type PiTdfiWdqlvlWwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_SWLVL_SM2_DM_NIBBLE_START` reader - 16:16\\]
Start command for stage 2, when in the process of DM leveling or nibble mode. WRITE-ONLY"]
pub type PiSwlvlSm2DmNibbleStartR = crate::BitReader;
#[doc = "Field `PI_SWLVL_SM2_DM_NIBBLE_START` writer - 16:16\\]
Start command for stage 2, when in the process of DM leveling or nibble mode. WRITE-ONLY"]
pub type PiSwlvlSm2DmNibbleStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_NIBBLE_MODE` reader - 24:24\\]
WDQ Training Nibble mode indication. When set to 1, nibble mode is enabled, and the training timing is doubled."]
pub type PiWdqlvlNibbleModeR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_NIBBLE_MODE` writer - 24:24\\]
WDQ Training Nibble mode indication. When set to 1, nibble mode is enabled, and the training timing is doubled."]
pub type PiWdqlvlNibbleModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Minimum number of DFI clocks to be inserted between write commands during the DM portion of write DQ training."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_ww(&self) -> PiTdfiWdqlvlWwR {
        PiTdfiWdqlvlWwR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Start command for stage 2, when in the process of DM leveling or nibble mode. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_sm2_dm_nibble_start(&self) -> PiSwlvlSm2DmNibbleStartR {
        PiSwlvlSm2DmNibbleStartR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
WDQ Training Nibble mode indication. When set to 1, nibble mode is enabled, and the training timing is doubled."]
    #[inline(always)]
    pub fn pi_wdqlvl_nibble_mode(&self) -> PiWdqlvlNibbleModeR {
        PiWdqlvlNibbleModeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Minimum number of DFI clocks to be inserted between write commands during the DM portion of write DQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_ww(&mut self) -> PiTdfiWdqlvlWwW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec> {
        PiTdfiWdqlvlWwW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Start command for stage 2, when in the process of DM leveling or nibble mode. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_sm2_dm_nibble_start(
        &mut self,
    ) -> PiSwlvlSm2DmNibbleStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec> {
        PiSwlvlSm2DmNibbleStartW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
WDQ Training Nibble mode indication. When set to 1, nibble mode is enabled, and the training timing is doubled."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_nibble_mode(
        &mut self,
    ) -> PiWdqlvlNibbleModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec> {
        PiWdqlvlNibbleModeW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_75::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_75::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_75::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_75::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_75 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi75Spec {
    const RESET_VALUE: u32 = 0;
}
