#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_72` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_72` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec>;
#[doc = "Field `PI_WDQLVL_DISABLE_DFS` reader - 0:0\\]
Disable automatic write DQ training on freq change. Set to 1 to disable."]
pub type PiWdqlvlDisableDfsR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_DISABLE_DFS` writer - 0:0\\]
Disable automatic write DQ training on freq change. Set to 1 to disable."]
pub type PiWdqlvlDisableDfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_ERROR_STATUS` reader - 9:8\\]
Holds the error associated with the write dq level error interrupt. Bit \\[0\\]
set indicates a PI_TDFI_WDQLVL_MAX parameter violation and bit \\[1\\]
set indicates a PI_TDFI_WDQLVL_RESP parameter violation. READ-ONLY."]
pub type PiWdqlvlErrorStatusR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_ERROR_STATUS` writer - 9:8\\]
Holds the error associated with the write dq level error interrupt. Bit \\[0\\]
set indicates a PI_TDFI_WDQLVL_MAX parameter violation and bit \\[1\\]
set indicates a PI_TDFI_WDQLVL_RESP parameter violation. READ-ONLY."]
pub type PiWdqlvlErrorStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WDQLVL_NEED_SAVE_RESTORE` reader - 17:16\\]
Enables the use of functional DRAM address space for write DQ training, 1 = enable, not for LPDDR4."]
pub type PiWdqlvlNeedSaveRestoreR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_NEED_SAVE_RESTORE` writer - 17:16\\]
Enables the use of functional DRAM address space for write DQ training, 1 = enable, not for LPDDR4."]
pub type PiWdqlvlNeedSaveRestoreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disable automatic write DQ training on freq change. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_wdqlvl_disable_dfs(&self) -> PiWdqlvlDisableDfsR {
        PiWdqlvlDisableDfsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Holds the error associated with the write dq level error interrupt. Bit \\[0\\]
set indicates a PI_TDFI_WDQLVL_MAX parameter violation and bit \\[1\\]
set indicates a PI_TDFI_WDQLVL_RESP parameter violation. READ-ONLY."]
    #[inline(always)]
    pub fn pi_wdqlvl_error_status(&self) -> PiWdqlvlErrorStatusR {
        PiWdqlvlErrorStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enables the use of functional DRAM address space for write DQ training, 1 = enable, not for LPDDR4."]
    #[inline(always)]
    pub fn pi_wdqlvl_need_save_restore(&self) -> PiWdqlvlNeedSaveRestoreR {
        PiWdqlvlNeedSaveRestoreR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disable automatic write DQ training on freq change. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_disable_dfs(
        &mut self,
    ) -> PiWdqlvlDisableDfsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec> {
        PiWdqlvlDisableDfsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Holds the error associated with the write dq level error interrupt. Bit \\[0\\]
set indicates a PI_TDFI_WDQLVL_MAX parameter violation and bit \\[1\\]
set indicates a PI_TDFI_WDQLVL_RESP parameter violation. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_error_status(
        &mut self,
    ) -> PiWdqlvlErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec> {
        PiWdqlvlErrorStatusW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enables the use of functional DRAM address space for write DQ training, 1 = enable, not for LPDDR4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_need_save_restore(
        &mut self,
    ) -> PiWdqlvlNeedSaveRestoreW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec> {
        PiWdqlvlNeedSaveRestoreW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_72::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_72::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_72::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_72::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_72 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi72Spec {
    const RESET_VALUE: u32 = 0;
}
