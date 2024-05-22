#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1361` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1361` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec>;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_1` reader - 1:0\\]
Select adrctl_mstr_dly_enc for the address/control slice 1 ."]
pub type PhyAdrctlMstrDlyEncSel1R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_1` writer - 1:0\\]
Select adrctl_mstr_dly_enc for the address/control slice 1 ."]
pub type PhyAdrctlMstrDlyEncSel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_2` reader - 9:8\\]
Select adrctl_mstr_dly_enc for the address/control slice 2 ."]
pub type PhyAdrctlMstrDlyEncSel2R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_2` writer - 9:8\\]
Select adrctl_mstr_dly_enc for the address/control slice 2 ."]
pub type PhyAdrctlMstrDlyEncSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_3` reader - 17:16\\]
Select adrctl_mstr_dly_enc for the address/control slice 3 ."]
pub type PhyAdrctlMstrDlyEncSel3R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_3` writer - 17:16\\]
Select adrctl_mstr_dly_enc for the address/control slice 3 ."]
pub type PhyAdrctlMstrDlyEncSel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Select adrctl_mstr_dly_enc for the address/control slice 1 ."]
    #[inline(always)]
    pub fn phy_adrctl_mstr_dly_enc_sel_1(&self) -> PhyAdrctlMstrDlyEncSel1R {
        PhyAdrctlMstrDlyEncSel1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select adrctl_mstr_dly_enc for the address/control slice 2 ."]
    #[inline(always)]
    pub fn phy_adrctl_mstr_dly_enc_sel_2(&self) -> PhyAdrctlMstrDlyEncSel2R {
        PhyAdrctlMstrDlyEncSel2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Select adrctl_mstr_dly_enc for the address/control slice 3 ."]
    #[inline(always)]
    pub fn phy_adrctl_mstr_dly_enc_sel_3(&self) -> PhyAdrctlMstrDlyEncSel3R {
        PhyAdrctlMstrDlyEncSel3R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Select adrctl_mstr_dly_enc for the address/control slice 1 ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_mstr_dly_enc_sel_1(
        &mut self,
    ) -> PhyAdrctlMstrDlyEncSel1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec> {
        PhyAdrctlMstrDlyEncSel1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select adrctl_mstr_dly_enc for the address/control slice 2 ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_mstr_dly_enc_sel_2(
        &mut self,
    ) -> PhyAdrctlMstrDlyEncSel2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec> {
        PhyAdrctlMstrDlyEncSel2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Select adrctl_mstr_dly_enc for the address/control slice 3 ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_mstr_dly_enc_sel_3(
        &mut self,
    ) -> PhyAdrctlMstrDlyEncSel3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec> {
        PhyAdrctlMstrDlyEncSel3W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1361\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1361::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1361::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1361::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1361::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1361 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1361Spec {
    const RESET_VALUE: u32 = 0;
}
