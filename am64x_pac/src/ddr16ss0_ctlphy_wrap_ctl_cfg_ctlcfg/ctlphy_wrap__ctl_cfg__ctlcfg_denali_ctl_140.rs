#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_140` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_140` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec>;
#[doc = "Field `PHYMSTR_ERROR_STATUS` reader - 1:0\\]
Identifies the source of any DFI PHY Master Interface errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit \\[0\\]
set indicates a TDFI_PHYMSTR_MAX or TDFI_PHYMSTR_TYPEn_MAX parmaeter violation and bit \\[1\\]
set indicates a TDFI_PHYMSTR_RESP parameter violation. READ-ONLY"]
pub type PhymstrErrorStatusR = crate::FieldReader;
#[doc = "Field `PHYMSTR_ERROR_STATUS` writer - 1:0\\]
Identifies the source of any DFI PHY Master Interface errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit \\[0\\]
set indicates a TDFI_PHYMSTR_MAX or TDFI_PHYMSTR_TYPEn_MAX parmaeter violation and bit \\[1\\]
set indicates a TDFI_PHYMSTR_RESP parameter violation. READ-ONLY"]
pub type PhymstrErrorStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHYMSTR_DFI_VERSION_4P0V1` reader - 8:8\\]
Defines the version of the DFI 4.0 specification supported. Clear to 0 for DFI 4.0 version 2 PHY Master Interface, or set to 1 for DFI 4.0 version 1 PHY Master Interface. Default is cleared to 0 for version 2."]
pub type PhymstrDfiVersion4p0v1R = crate::BitReader;
#[doc = "Field `PHYMSTR_DFI_VERSION_4P0V1` writer - 8:8\\]
Defines the version of the DFI 4.0 specification supported. Clear to 0 for DFI 4.0 version 2 PHY Master Interface, or set to 1 for DFI 4.0 version 1 PHY Master Interface. Default is cleared to 0 for version 2."]
pub type PhymstrDfiVersion4p0v1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYMSTR_TRAIN_AFTER_INIT_COMPLETE` reader - 16:16\\]
Defines how the PHY will use the PHY Master Interface for training. Clear to 0 to perform training without the PHY Master Interface, or set to 1 to use the PHY Master Interface to gain control over the DFI bus after the dfi_init_complete signal assertion for the initial training. Default is cleared to 0."]
pub type PhymstrTrainAfterInitCompleteR = crate::BitReader;
#[doc = "Field `PHYMSTR_TRAIN_AFTER_INIT_COMPLETE` writer - 16:16\\]
Defines how the PHY will use the PHY Master Interface for training. Clear to 0 to perform training without the PHY Master Interface, or set to 1 to use the PHY Master Interface to gain control over the DFI bus after the dfi_init_complete signal assertion for the initial training. Default is cleared to 0."]
pub type PhymstrTrainAfterInitCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Identifies the source of any DFI PHY Master Interface errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit \\[0\\]
set indicates a TDFI_PHYMSTR_MAX or TDFI_PHYMSTR_TYPEn_MAX parmaeter violation and bit \\[1\\]
set indicates a TDFI_PHYMSTR_RESP parameter violation. READ-ONLY"]
    #[inline(always)]
    pub fn phymstr_error_status(&self) -> PhymstrErrorStatusR {
        PhymstrErrorStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Defines the version of the DFI 4.0 specification supported. Clear to 0 for DFI 4.0 version 2 PHY Master Interface, or set to 1 for DFI 4.0 version 1 PHY Master Interface. Default is cleared to 0 for version 2."]
    #[inline(always)]
    pub fn phymstr_dfi_version_4p0v1(&self) -> PhymstrDfiVersion4p0v1R {
        PhymstrDfiVersion4p0v1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines how the PHY will use the PHY Master Interface for training. Clear to 0 to perform training without the PHY Master Interface, or set to 1 to use the PHY Master Interface to gain control over the DFI bus after the dfi_init_complete signal assertion for the initial training. Default is cleared to 0."]
    #[inline(always)]
    pub fn phymstr_train_after_init_complete(&self) -> PhymstrTrainAfterInitCompleteR {
        PhymstrTrainAfterInitCompleteR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Identifies the source of any DFI PHY Master Interface errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit \\[0\\]
set indicates a TDFI_PHYMSTR_MAX or TDFI_PHYMSTR_TYPEn_MAX parmaeter violation and bit \\[1\\]
set indicates a TDFI_PHYMSTR_RESP parameter violation. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_error_status(
        &mut self,
    ) -> PhymstrErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec> {
        PhymstrErrorStatusW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Defines the version of the DFI 4.0 specification supported. Clear to 0 for DFI 4.0 version 2 PHY Master Interface, or set to 1 for DFI 4.0 version 1 PHY Master Interface. Default is cleared to 0 for version 2."]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_dfi_version_4p0v1(
        &mut self,
    ) -> PhymstrDfiVersion4p0v1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec> {
        PhymstrDfiVersion4p0v1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines how the PHY will use the PHY Master Interface for training. Clear to 0 to perform training without the PHY Master Interface, or set to 1 to use the PHY Master Interface to gain control over the DFI bus after the dfi_init_complete signal assertion for the initial training. Default is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_train_after_init_complete(
        &mut self,
    ) -> PhymstrTrainAfterInitCompleteW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec> {
        PhymstrTrainAfterInitCompleteW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_140::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_140::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_140::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_140::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_140 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl140Spec {
    const RESET_VALUE: u32 = 0;
}
