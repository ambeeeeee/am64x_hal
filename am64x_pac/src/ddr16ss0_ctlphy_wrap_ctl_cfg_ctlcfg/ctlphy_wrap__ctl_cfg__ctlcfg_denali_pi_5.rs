#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_5` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_5` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec>;
#[doc = "Field `PI_TRAIN_ALL_FREQ_REQ` reader - 8:8\\]
Triggers training for all supported frequencies in PI_FREQ_MAP. Applies to LPDDR4 devices onlyh. Set to 1 to trigger. Only applicable after memory initialization has been completed. Can be used to train new frequencies that were not available at initialization time. WRITE-ONLY"]
pub type PiTrainAllFreqReqR = crate::BitReader;
#[doc = "Field `PI_TRAIN_ALL_FREQ_REQ` writer - 8:8\\]
Triggers training for all supported frequencies in PI_FREQ_MAP. Applies to LPDDR4 devices onlyh. Set to 1 to trigger. Only applicable after memory initialization has been completed. Can be used to train new frequencies that were not available at initialization time. WRITE-ONLY"]
pub type PiTrainAllFreqReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DFI_VERSION` reader - 16:16\\]
Define the DFI master version, set 1 for DFI4.1, set 0 for DFI4.0"]
pub type PiDfiVersionR = crate::BitReader;
#[doc = "Field `PI_DFI_VERSION` writer - 16:16\\]
Define the DFI master version, set 1 for DFI4.1, set 0 for DFI4.0"]
pub type PiDfiVersionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DFI_PHYMSTR_TYPE` reader - 25:24\\]
DFI Master Request Type used for dfi 4.1 verision: This signal indicates the required state of DRAM when PHY becomes the master. Each memory rank uses one bit. 1'b0: IDLE. The MC should close all the pages. 1'b1: IDLE or Self Refresh."]
pub type PiDfiPhymstrTypeR = crate::FieldReader;
#[doc = "Field `PI_DFI_PHYMSTR_TYPE` writer - 25:24\\]
DFI Master Request Type used for dfi 4.1 verision: This signal indicates the required state of DRAM when PHY becomes the master. Each memory rank uses one bit. 1'b0: IDLE. The MC should close all the pages. 1'b1: IDLE or Self Refresh."]
pub type PiDfiPhymstrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Triggers training for all supported frequencies in PI_FREQ_MAP. Applies to LPDDR4 devices onlyh. Set to 1 to trigger. Only applicable after memory initialization has been completed. Can be used to train new frequencies that were not available at initialization time. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_train_all_freq_req(&self) -> PiTrainAllFreqReqR {
        PiTrainAllFreqReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Define the DFI master version, set 1 for DFI4.1, set 0 for DFI4.0"]
    #[inline(always)]
    pub fn pi_dfi_version(&self) -> PiDfiVersionR {
        PiDfiVersionR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
DFI Master Request Type used for dfi 4.1 verision: This signal indicates the required state of DRAM when PHY becomes the master. Each memory rank uses one bit. 1'b0: IDLE. The MC should close all the pages. 1'b1: IDLE or Self Refresh."]
    #[inline(always)]
    pub fn pi_dfi_phymstr_type(&self) -> PiDfiPhymstrTypeR {
        PiDfiPhymstrTypeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Triggers training for all supported frequencies in PI_FREQ_MAP. Applies to LPDDR4 devices onlyh. Set to 1 to trigger. Only applicable after memory initialization has been completed. Can be used to train new frequencies that were not available at initialization time. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_train_all_freq_req(
        &mut self,
    ) -> PiTrainAllFreqReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec> {
        PiTrainAllFreqReqW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Define the DFI master version, set 1 for DFI4.1, set 0 for DFI4.0"]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfi_version(&mut self) -> PiDfiVersionW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec> {
        PiDfiVersionW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
DFI Master Request Type used for dfi 4.1 verision: This signal indicates the required state of DRAM when PHY becomes the master. Each memory rank uses one bit. 1'b0: IDLE. The MC should close all the pages. 1'b1: IDLE or Self Refresh."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfi_phymstr_type(
        &mut self,
    ) -> PiDfiPhymstrTypeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec> {
        PiDfiPhymstrTypeW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_5::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_5::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_5 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi5Spec {
    const RESET_VALUE: u32 = 0;
}
