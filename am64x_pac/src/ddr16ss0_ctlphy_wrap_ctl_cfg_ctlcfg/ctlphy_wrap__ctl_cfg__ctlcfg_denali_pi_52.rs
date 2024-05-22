#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_52` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_52` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec>;
#[doc = "Field `PI_RDLVL_STROBE_NUM` reader - 4:0\\]
Defines the number of back to back MPC command in one read process in read eye training."]
pub type PiRdlvlStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_STROBE_NUM` writer - 4:0\\]
Defines the number of back to back MPC command in one read process in read eye training."]
pub type PiRdlvlStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_RDLVL_GATE_STROBE_NUM` reader - 12:8\\]
Defines the number of back to back MPC command in one read process in read gate training."]
pub type PiRdlvlGateStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_STROBE_NUM` writer - 12:8\\]
Defines the number of back to back MPC command in one read process in read gate training."]
pub type PiRdlvlGateStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_RD_PREAMBLE_TRAINING_EN` reader - 16:16\\]
Enable read preamble training during gate training. Set to 1 to enable."]
pub type PiRdPreambleTrainingEnR = crate::BitReader;
#[doc = "Field `PI_RD_PREAMBLE_TRAINING_EN` writer - 16:16\\]
Enable read preamble training during gate training. Set to 1 to enable."]
pub type PiRdPreambleTrainingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_REG_DIMM_ENABLE` reader - 24:24\\]
Enable registered DIMM operation. Set to 1 to enable."]
pub type PiRegDimmEnableR = crate::BitReader;
#[doc = "Field `PI_REG_DIMM_ENABLE` writer - 24:24\\]
Enable registered DIMM operation. Set to 1 to enable."]
pub type PiRegDimmEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the number of back to back MPC command in one read process in read eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_strobe_num(&self) -> PiRdlvlStrobeNumR {
        PiRdlvlStrobeNumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Defines the number of back to back MPC command in one read process in read gate training."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_strobe_num(&self) -> PiRdlvlGateStrobeNumR {
        PiRdlvlGateStrobeNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable read preamble training during gate training. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rd_preamble_training_en(&self) -> PiRdPreambleTrainingEnR {
        PiRdPreambleTrainingEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable registered DIMM operation. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_reg_dimm_enable(&self) -> PiRegDimmEnableR {
        PiRegDimmEnableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the number of back to back MPC command in one read process in read eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_strobe_num(
        &mut self,
    ) -> PiRdlvlStrobeNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec> {
        PiRdlvlStrobeNumW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Defines the number of back to back MPC command in one read process in read gate training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_strobe_num(
        &mut self,
    ) -> PiRdlvlGateStrobeNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec> {
        PiRdlvlGateStrobeNumW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable read preamble training during gate training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_preamble_training_en(
        &mut self,
    ) -> PiRdPreambleTrainingEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec> {
        PiRdPreambleTrainingEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable registered DIMM operation. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_reg_dimm_enable(
        &mut self,
    ) -> PiRegDimmEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec> {
        PiRegDimmEnableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_52::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_52::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_52 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi52Spec {
    const RESET_VALUE: u32 = 0;
}
