#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_62` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_62` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec>;
#[doc = "Field `PI_TCKCKEH` reader - 7:0\\]
DRAM tCKELCK Clock and command valid before CKE HIGH."]
pub type PiTckckehR = crate::FieldReader;
#[doc = "Field `PI_TCKCKEH` writer - 7:0\\]
DRAM tCKELCK Clock and command valid before CKE HIGH."]
pub type PiTckckehW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_CALVL_STROBE_NUM` reader - 12:8\\]
The consecutive dfi_calvl_strobe number when updating the CA vref data."]
pub type PiCalvlStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_CALVL_STROBE_NUM` writer - 12:8\\]
The consecutive dfi_calvl_strobe number when updating the CA vref data."]
pub type PiCalvlStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_SW_CA_TRAIN_VREF` reader - 22:16\\]
The Vref value which is set for SW step by step CA training."]
pub type PiSwCaTrainVrefR = crate::FieldReader;
#[doc = "Field `PI_SW_CA_TRAIN_VREF` writer - 22:16\\]
The Vref value which is set for SW step by step CA training."]
pub type PiSwCaTrainVrefW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_REFRESH_BETWEEN_SEGMENT_DISABLE` reader - 24:24\\]
Disable the refresh between CA first and second segment training. Set to 1 to disable."]
pub type PiRefreshBetweenSegmentDisableR = crate::BitReader;
#[doc = "Field `PI_REFRESH_BETWEEN_SEGMENT_DISABLE` writer - 24:24\\]
Disable the refresh between CA first and second segment training. Set to 1 to disable."]
pub type PiRefreshBetweenSegmentDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tCKELCK Clock and command valid before CKE HIGH."]
    #[inline(always)]
    pub fn pi_tckckeh(&self) -> PiTckckehR {
        PiTckckehR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
The consecutive dfi_calvl_strobe number when updating the CA vref data."]
    #[inline(always)]
    pub fn pi_calvl_strobe_num(&self) -> PiCalvlStrobeNumR {
        PiCalvlStrobeNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
The Vref value which is set for SW step by step CA training."]
    #[inline(always)]
    pub fn pi_sw_ca_train_vref(&self) -> PiSwCaTrainVrefR {
        PiSwCaTrainVrefR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable the refresh between CA first and second segment training. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_refresh_between_segment_disable(&self) -> PiRefreshBetweenSegmentDisableR {
        PiRefreshBetweenSegmentDisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tCKELCK Clock and command valid before CKE HIGH."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckckeh(&mut self) -> PiTckckehW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec> {
        PiTckckehW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
The consecutive dfi_calvl_strobe number when updating the CA vref data."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_strobe_num(
        &mut self,
    ) -> PiCalvlStrobeNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec> {
        PiCalvlStrobeNumW::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
The Vref value which is set for SW step by step CA training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_ca_train_vref(
        &mut self,
    ) -> PiSwCaTrainVrefW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec> {
        PiSwCaTrainVrefW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable the refresh between CA first and second segment training. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_refresh_between_segment_disable(
        &mut self,
    ) -> PiRefreshBetweenSegmentDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec> {
        PiRefreshBetweenSegmentDisableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_62::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_62::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_62 to value 0x0100_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi62Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
