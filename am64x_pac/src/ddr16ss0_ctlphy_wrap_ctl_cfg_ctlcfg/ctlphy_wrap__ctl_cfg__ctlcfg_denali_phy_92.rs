#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_92` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_92` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec>;
#[doc = "Field `PHY_DBI_MODE_0` reader - 1:0\\]
DBI mode for slice 0. Bit \\[0\\]
enables return of DBI read data."]
pub type PhyDbiMode0R = crate::FieldReader;
#[doc = "Field `PHY_DBI_MODE_0` writer - 1:0\\]
DBI mode for slice 0. Bit \\[0\\]
enables return of DBI read data."]
pub type PhyDbiMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_WDQLVL_IE_ON_0` reader - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 0."]
pub type PhyWdqlvlIeOn0R = crate::BitReader;
#[doc = "Field `PHY_WDQLVL_IE_ON_0` writer - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 0."]
pub type PhyWdqlvlIeOn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_DLY_0` reader - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyWdqlvlRddataEnDly0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_DLY_0` writer - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyWdqlvlRddataEnDly0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_TSEL_DLY_0` reader - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyWdqlvlRddataEnTselDly0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_TSEL_DLY_0` writer - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyWdqlvlRddataEnTselDly0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
DBI mode for slice 0. Bit \\[0\\]
enables return of DBI read data."]
    #[inline(always)]
    pub fn phy_dbi_mode_0(&self) -> PhyDbiMode0R {
        PhyDbiMode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_ie_on_0(&self) -> PhyWdqlvlIeOn0R {
        PhyWdqlvlIeOn0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_rddata_en_dly_0(&self) -> PhyWdqlvlRddataEnDly0R {
        PhyWdqlvlRddataEnDly0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_rddata_en_tsel_dly_0(&self) -> PhyWdqlvlRddataEnTselDly0R {
        PhyWdqlvlRddataEnTselDly0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
DBI mode for slice 0. Bit \\[0\\]
enables return of DBI read data."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dbi_mode_0(&mut self) -> PhyDbiMode0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec> {
        PhyDbiMode0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_ie_on_0(
        &mut self,
    ) -> PhyWdqlvlIeOn0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec> {
        PhyWdqlvlIeOn0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_rddata_en_dly_0(
        &mut self,
    ) -> PhyWdqlvlRddataEnDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec> {
        PhyWdqlvlRddataEnDly0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_rddata_en_tsel_dly_0(
        &mut self,
    ) -> PhyWdqlvlRddataEnTselDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec> {
        PhyWdqlvlRddataEnTselDly0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_92::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_92::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_92::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_92::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_92 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy92Spec {
    const RESET_VALUE: u32 = 0;
}
