#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_85` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_85` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec>;
#[doc = "Field `PBR_NUMERIC_ORDER` reader - 0:0\\]
Enables the PBR to run REFpb commands in numeric bank order \\[0,1,2,3, etc.\\]
When disabled, the order may be modified if supported by the memory type. Set to 1 to enable."]
pub type PbrNumericOrderR = crate::BitReader;
#[doc = "Field `PBR_NUMERIC_ORDER` writer - 0:0\\]
Enables the PBR to run REFpb commands in numeric bank order \\[0,1,2,3, etc.\\]
When disabled, the order may be modified if supported by the memory type. Set to 1 to enable."]
pub type PbrNumericOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBR_MAX_BANK_WAIT` reader - 23:8\\]
Defines the maximum number of cycles that the PBR module will wait for Strategy to release the target bank until the PBR will assert the inhibit and close the target bank."]
pub type PbrMaxBankWaitR = crate::FieldReader<u16>;
#[doc = "Field `PBR_MAX_BANK_WAIT` writer - 23:8\\]
Defines the maximum number of cycles that the PBR module will wait for Strategy to release the target bank until the PBR will assert the inhibit and close the target bank."]
pub type PbrMaxBankWaitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PBR_BANK_SELECT_DELAY` reader - 27:24\\]
Defines the PBR bank select to command delay, the time from bank selection to when the command queue bank selection logic is guaranteed to have blocked the bank."]
pub type PbrBankSelectDelayR = crate::FieldReader;
#[doc = "Field `PBR_BANK_SELECT_DELAY` writer - 27:24\\]
Defines the PBR bank select to command delay, the time from bank selection to when the command queue bank selection logic is guaranteed to have blocked the bank."]
pub type PbrBankSelectDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the PBR to run REFpb commands in numeric bank order \\[0,1,2,3, etc.\\]
When disabled, the order may be modified if supported by the memory type. Set to 1 to enable."]
    #[inline(always)]
    pub fn pbr_numeric_order(&self) -> PbrNumericOrderR {
        PbrNumericOrderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Defines the maximum number of cycles that the PBR module will wait for Strategy to release the target bank until the PBR will assert the inhibit and close the target bank."]
    #[inline(always)]
    pub fn pbr_max_bank_wait(&self) -> PbrMaxBankWaitR {
        PbrMaxBankWaitR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the PBR bank select to command delay, the time from bank selection to when the command queue bank selection logic is guaranteed to have blocked the bank."]
    #[inline(always)]
    pub fn pbr_bank_select_delay(&self) -> PbrBankSelectDelayR {
        PbrBankSelectDelayR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the PBR to run REFpb commands in numeric bank order \\[0,1,2,3, etc.\\]
When disabled, the order may be modified if supported by the memory type. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pbr_numeric_order(
        &mut self,
    ) -> PbrNumericOrderW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec> {
        PbrNumericOrderW::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Defines the maximum number of cycles that the PBR module will wait for Strategy to release the target bank until the PBR will assert the inhibit and close the target bank."]
    #[inline(always)]
    #[must_use]
    pub fn pbr_max_bank_wait(
        &mut self,
    ) -> PbrMaxBankWaitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec> {
        PbrMaxBankWaitW::new(self, 8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the PBR bank select to command delay, the time from bank selection to when the command queue bank selection logic is guaranteed to have blocked the bank."]
    #[inline(always)]
    #[must_use]
    pub fn pbr_bank_select_delay(
        &mut self,
    ) -> PbrBankSelectDelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec> {
        PbrBankSelectDelayW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_85::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_85::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_85::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_85::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_85 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl85Spec {
    const RESET_VALUE: u32 = 0;
}
