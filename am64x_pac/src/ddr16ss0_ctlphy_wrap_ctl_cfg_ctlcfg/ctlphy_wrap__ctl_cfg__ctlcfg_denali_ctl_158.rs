#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_158` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_158` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec>;
#[doc = "Field `LOWPOWER_REFRESH_ENABLE` reader - 1:0\\]
Enable refreshes while in low power mode. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
pub type LowpowerRefreshEnableR = crate::FieldReader;
#[doc = "Field `LOWPOWER_REFRESH_ENABLE` writer - 1:0\\]
Enable refreshes while in low power mode. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
pub type LowpowerRefreshEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_CMD` reader - 14:8\\]
Low power software command request interface. Bit \\[0\\]
controls exit, bit \\[1\\]
controls entry, bits \\[4:2\\]
define the low power state, bit \\[5\\]
controls memory clock gating, bit \\[6\\]
controls controller clock gating, and bit \\[7\\]
controls lock. WRITE-ONLY"]
pub type LpCmdR = crate::FieldReader;
#[doc = "Field `LP_CMD` writer - 14:8\\]
Low power software command request interface. Bit \\[0\\]
controls exit, bit \\[1\\]
controls entry, bits \\[4:2\\]
define the low power state, bit \\[5\\]
controls memory clock gating, bit \\[6\\]
controls controller clock gating, and bit \\[7\\]
controls lock. WRITE-ONLY"]
pub type LpCmdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LPI_IDLE_WAKEUP_F0` reader - 19:16\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=0"]
pub type LpiIdleWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_IDLE_WAKEUP_F0` writer - 19:16\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=0"]
pub type LpiIdleWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_SHORT_WAKEUP_F0` reader - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=0"]
pub type LpiSrShortWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SR_SHORT_WAKEUP_F0` writer - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=0"]
pub type LpiSrShortWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable refreshes while in low power mode. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    pub fn lowpower_refresh_enable(&self) -> LowpowerRefreshEnableR {
        LowpowerRefreshEnableR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Low power software command request interface. Bit \\[0\\]
controls exit, bit \\[1\\]
controls entry, bits \\[4:2\\]
define the low power state, bit \\[5\\]
controls memory clock gating, bit \\[6\\]
controls controller clock gating, and bit \\[7\\]
controls lock. WRITE-ONLY"]
    #[inline(always)]
    pub fn lp_cmd(&self) -> LpCmdR {
        LpCmdR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=0"]
    #[inline(always)]
    pub fn lpi_idle_wakeup_f0(&self) -> LpiIdleWakeupF0R {
        LpiIdleWakeupF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=0"]
    #[inline(always)]
    pub fn lpi_sr_short_wakeup_f0(&self) -> LpiSrShortWakeupF0R {
        LpiSrShortWakeupF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable refreshes while in low power mode. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn lowpower_refresh_enable(
        &mut self,
    ) -> LowpowerRefreshEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec> {
        LowpowerRefreshEnableW::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Low power software command request interface. Bit \\[0\\]
controls exit, bit \\[1\\]
controls entry, bits \\[4:2\\]
define the low power state, bit \\[5\\]
controls memory clock gating, bit \\[6\\]
controls controller clock gating, and bit \\[7\\]
controls lock. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd(&mut self) -> LpCmdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec> {
        LpCmdW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_idle_wakeup_f0(
        &mut self,
    ) -> LpiIdleWakeupF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec> {
        LpiIdleWakeupF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_short_wakeup_f0(
        &mut self,
    ) -> LpiSrShortWakeupF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec> {
        LpiSrShortWakeupF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_158\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_158::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_158::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_158::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_158::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_158 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl158Spec {
    const RESET_VALUE: u32 = 0;
}
