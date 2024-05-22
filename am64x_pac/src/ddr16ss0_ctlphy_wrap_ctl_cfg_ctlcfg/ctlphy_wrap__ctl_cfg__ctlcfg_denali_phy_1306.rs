#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1306` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1306` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec>;
#[doc = "Field `PHY_SET_DFI_INPUT_0` reader - 3:0\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput0R = crate::FieldReader;
#[doc = "Field `PHY_SET_DFI_INPUT_0` writer - 3:0\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SET_DFI_INPUT_1` reader - 11:8\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput1R = crate::FieldReader;
#[doc = "Field `PHY_SET_DFI_INPUT_1` writer - 11:8\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SET_DFI_INPUT_2` reader - 19:16\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput2R = crate::FieldReader;
#[doc = "Field `PHY_SET_DFI_INPUT_2` writer - 19:16\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SET_DFI_INPUT_3` reader - 27:24\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput3R = crate::FieldReader;
#[doc = "Field `PHY_SET_DFI_INPUT_3` writer - 27:24\\]
Used to indicate the default value of the adrctl slice bits."]
pub type PhySetDfiInput3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    pub fn phy_set_dfi_input_0(&self) -> PhySetDfiInput0R {
        PhySetDfiInput0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    pub fn phy_set_dfi_input_1(&self) -> PhySetDfiInput1R {
        PhySetDfiInput1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    pub fn phy_set_dfi_input_2(&self) -> PhySetDfiInput2R {
        PhySetDfiInput2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    pub fn phy_set_dfi_input_3(&self) -> PhySetDfiInput3R {
        PhySetDfiInput3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    #[must_use]
    pub fn phy_set_dfi_input_0(
        &mut self,
    ) -> PhySetDfiInput0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec> {
        PhySetDfiInput0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    #[must_use]
    pub fn phy_set_dfi_input_1(
        &mut self,
    ) -> PhySetDfiInput1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec> {
        PhySetDfiInput1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    #[must_use]
    pub fn phy_set_dfi_input_2(
        &mut self,
    ) -> PhySetDfiInput2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec> {
        PhySetDfiInput2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Used to indicate the default value of the adrctl slice bits."]
    #[inline(always)]
    #[must_use]
    pub fn phy_set_dfi_input_3(
        &mut self,
    ) -> PhySetDfiInput3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec> {
        PhySetDfiInput3W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1306\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1306::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1306::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1306::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1306::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1306 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1306Spec {
    const RESET_VALUE: u32 = 0;
}
