#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_348` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_348` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec>;
#[doc = "Field `PHY_DBI_MODE_1` reader - 1:0\\]
DBI mode for slice 1. Bit \\[0\\]
enables return of DBI read data."]
pub type PhyDbiMode1R = crate::FieldReader;
#[doc = "Field `PHY_DBI_MODE_1` writer - 1:0\\]
DBI mode for slice 1. Bit \\[0\\]
enables return of DBI read data."]
pub type PhyDbiMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_WDQLVL_IE_ON_1` reader - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 1."]
pub type PhyWdqlvlIeOn1R = crate::BitReader;
#[doc = "Field `PHY_WDQLVL_IE_ON_1` writer - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 1."]
pub type PhyWdqlvlIeOn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_DLY_1` reader - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyWdqlvlRddataEnDly1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_DLY_1` writer - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyWdqlvlRddataEnDly1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_TSEL_DLY_1` reader - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyWdqlvlRddataEnTselDly1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_RDDATA_EN_TSEL_DLY_1` writer - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyWdqlvlRddataEnTselDly1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
DBI mode for slice 1. Bit \\[0\\]
enables return of DBI read data."]
    #[inline(always)]
    pub fn phy_dbi_mode_1(&self) -> PhyDbiMode1R {
        PhyDbiMode1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_ie_on_1(&self) -> PhyWdqlvlIeOn1R {
        PhyWdqlvlIeOn1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_rddata_en_dly_1(&self) -> PhyWdqlvlRddataEnDly1R {
        PhyWdqlvlRddataEnDly1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_rddata_en_tsel_dly_1(&self) -> PhyWdqlvlRddataEnTselDly1R {
        PhyWdqlvlRddataEnTselDly1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
DBI mode for slice 1. Bit \\[0\\]
enables return of DBI read data."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dbi_mode_1(&mut self) -> PhyDbiMode1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec> {
        PhyDbiMode1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
IE control, 1 meams IE is always on during WR DQ training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_ie_on_1(
        &mut self,
    ) -> PhyWdqlvlIeOn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec> {
        PhyWdqlvlIeOn1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_rddata_en_dly_1(
        &mut self,
    ) -> PhyWdqlvlRddataEnDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec> {
        PhyWdqlvlRddataEnDly1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For WR DQ training, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_rddata_en_tsel_dly_1(
        &mut self,
    ) -> PhyWdqlvlRddataEnTselDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec> {
        PhyWdqlvlRddataEnTselDly1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_348\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_348::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_348::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_348::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_348::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_348 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy348Spec {
    const RESET_VALUE: u32 = 0;
}
