#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_65` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_65` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec>;
#[doc = "Field `PI_INIT_COMPLETE_TO_MC_DELAY_COUNT` reader - 7:0\\]
It controls the time PI bypass CKE at the beginning of PI mask dfi_init_complete to controller."]
pub type PiInitCompleteToMcDelayCountR = crate::FieldReader;
#[doc = "Field `PI_INIT_COMPLETE_TO_MC_DELAY_COUNT` writer - 7:0\\]
It controls the time PI bypass CKE at the beginning of PI mask dfi_init_complete to controller."]
pub type PiInitCompleteToMcDelayCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_WDQLVL_VREF_EN` reader - 8:8\\]
Control for VREF training as part of non-initialization write DQ training."]
pub type PiWdqlvlVrefEnR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_VREF_EN` writer - 8:8\\]
Control for VREF training as part of non-initialization write DQ training."]
pub type PiWdqlvlVrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_BST_NUM` reader - 18:16\\]
Defines the number of write/read bursts issued at each step in write DQ training."]
pub type PiWdqlvlBstNumR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_BST_NUM` writer - 18:16\\]
Defines the number of write/read bursts issued at each step in write DQ training."]
pub type PiWdqlvlBstNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_WDQLVL_RESP_MASK` reader - 25:24\\]
Write DQ training response mask. When set to 1, the dfi_wdqlvl_en of the slice is not asserted."]
pub type PiWdqlvlRespMaskR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_RESP_MASK` writer - 25:24\\]
Write DQ training response mask. When set to 1, the dfi_wdqlvl_en of the slice is not asserted."]
pub type PiWdqlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
It controls the time PI bypass CKE at the beginning of PI mask dfi_init_complete to controller."]
    #[inline(always)]
    pub fn pi_init_complete_to_mc_delay_count(&self) -> PiInitCompleteToMcDelayCountR {
        PiInitCompleteToMcDelayCountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Control for VREF training as part of non-initialization write DQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_en(&self) -> PiWdqlvlVrefEnR {
        PiWdqlvlVrefEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the number of write/read bursts issued at each step in write DQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_bst_num(&self) -> PiWdqlvlBstNumR {
        PiWdqlvlBstNumR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Write DQ training response mask. When set to 1, the dfi_wdqlvl_en of the slice is not asserted."]
    #[inline(always)]
    pub fn pi_wdqlvl_resp_mask(&self) -> PiWdqlvlRespMaskR {
        PiWdqlvlRespMaskR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
It controls the time PI bypass CKE at the beginning of PI mask dfi_init_complete to controller."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_complete_to_mc_delay_count(
        &mut self,
    ) -> PiInitCompleteToMcDelayCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec> {
        PiInitCompleteToMcDelayCountW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control for VREF training as part of non-initialization write DQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_en(&mut self) -> PiWdqlvlVrefEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec> {
        PiWdqlvlVrefEnW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the number of write/read bursts issued at each step in write DQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_bst_num(&mut self) -> PiWdqlvlBstNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec> {
        PiWdqlvlBstNumW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Write DQ training response mask. When set to 1, the dfi_wdqlvl_en of the slice is not asserted."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_resp_mask(
        &mut self,
    ) -> PiWdqlvlRespMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec> {
        PiWdqlvlRespMaskW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_65::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_65::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_65 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi65Spec {
    const RESET_VALUE: u32 = 0;
}
