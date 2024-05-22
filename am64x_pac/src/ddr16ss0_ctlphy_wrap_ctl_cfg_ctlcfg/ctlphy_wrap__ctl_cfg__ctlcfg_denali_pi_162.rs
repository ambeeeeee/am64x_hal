#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_162` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_162` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec>;
#[doc = "Field `PI_FREQ_SEL_FROM_REGIF` reader - 0:0\\]
In non-DFI 4.0 mode, user select the frequency copies from pi_freq_change_reg_copy."]
pub type PiFreqSelFromRegifR = crate::BitReader;
#[doc = "Field `PI_FREQ_SEL_FROM_REGIF` writer - 0:0\\]
In non-DFI 4.0 mode, user select the frequency copies from pi_freq_change_reg_copy."]
pub type PiFreqSelFromRegifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CATR` reader - 17:16\\]
It indicates LP4 DRAM CA terminition ON/OFF state. Each bit corresponds to each chip select. 1:ON 0:OFF. This parameter is active when PI_NO_CATR_READ==1. When PI_NO_CATR_READ==0, this param is inactive"]
pub type PiCatrR = crate::FieldReader;
#[doc = "Field `PI_CATR` writer - 17:16\\]
It indicates LP4 DRAM CA terminition ON/OFF state. Each bit corresponds to each chip select. 1:ON 0:OFF. This parameter is active when PI_NO_CATR_READ==1. When PI_NO_CATR_READ==0, this param is inactive"]
pub type PiCatrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_NO_CATR_READ` reader - 24:24\\]
Defines how the LPDDR4 termination status is determined. 1: PI use PI_CATR to get DRAM CA Termination status. 0: PI reads DRAM MR0.OP7 to get DRAM CA Termination status."]
pub type PiNoCatrReadR = crate::BitReader;
#[doc = "Field `PI_NO_CATR_READ` writer - 24:24\\]
Defines how the LPDDR4 termination status is determined. 1: PI use PI_CATR to get DRAM CA Termination status. 0: PI reads DRAM MR0.OP7 to get DRAM CA Termination status."]
pub type PiNoCatrReadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
In non-DFI 4.0 mode, user select the frequency copies from pi_freq_change_reg_copy."]
    #[inline(always)]
    pub fn pi_freq_sel_from_regif(&self) -> PiFreqSelFromRegifR {
        PiFreqSelFromRegifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
It indicates LP4 DRAM CA terminition ON/OFF state. Each bit corresponds to each chip select. 1:ON 0:OFF. This parameter is active when PI_NO_CATR_READ==1. When PI_NO_CATR_READ==0, this param is inactive"]
    #[inline(always)]
    pub fn pi_catr(&self) -> PiCatrR {
        PiCatrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Defines how the LPDDR4 termination status is determined. 1: PI use PI_CATR to get DRAM CA Termination status. 0: PI reads DRAM MR0.OP7 to get DRAM CA Termination status."]
    #[inline(always)]
    pub fn pi_no_catr_read(&self) -> PiNoCatrReadR {
        PiNoCatrReadR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
In non-DFI 4.0 mode, user select the frequency copies from pi_freq_change_reg_copy."]
    #[inline(always)]
    #[must_use]
    pub fn pi_freq_sel_from_regif(
        &mut self,
    ) -> PiFreqSelFromRegifW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec> {
        PiFreqSelFromRegifW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
It indicates LP4 DRAM CA terminition ON/OFF state. Each bit corresponds to each chip select. 1:ON 0:OFF. This parameter is active when PI_NO_CATR_READ==1. When PI_NO_CATR_READ==0, this param is inactive"]
    #[inline(always)]
    #[must_use]
    pub fn pi_catr(&mut self) -> PiCatrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec> {
        PiCatrW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Defines how the LPDDR4 termination status is determined. 1: PI use PI_CATR to get DRAM CA Termination status. 0: PI reads DRAM MR0.OP7 to get DRAM CA Termination status."]
    #[inline(always)]
    #[must_use]
    pub fn pi_no_catr_read(&mut self) -> PiNoCatrReadW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec> {
        PiNoCatrReadW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_162\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_162::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_162::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_162::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_162::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_162 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi162Spec {
    const RESET_VALUE: u32 = 0;
}
