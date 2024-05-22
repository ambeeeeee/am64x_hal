#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_93` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_93` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec>;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_0` reader - 4:0\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyRddataEnTselDly0R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_0` writer - 4:0\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyRddataEnTselDly0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_RDDATA_EN_OE_DLY_0` reader - 12:8\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for LP4 OE extension generation for slice 0."]
pub type PhyRddataEnOeDly0R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_OE_DLY_0` writer - 12:8\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for LP4 OE extension generation for slice 0."]
pub type PhyRddataEnOeDly0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_MASTER_MODE_0` reader - 19:16\\]
Master delay line override settings for slice 0. Bit \\[0\\]
enables software half clock mode. Bit \\[1\\]
is the software half clock mode value. Bit \\[2\\]
enables software bypass mode. Bit \\[3\\]
is the software bypass mode value."]
pub type PhySwMasterMode0R = crate::FieldReader;
#[doc = "Field `PHY_SW_MASTER_MODE_0` writer - 19:16\\]
Master delay line override settings for slice 0. Bit \\[0\\]
enables software half clock mode. Bit \\[1\\]
is the software half clock mode value. Bit \\[2\\]
enables software bypass mode. Bit \\[3\\]
is the software bypass mode value."]
pub type PhySwMasterMode0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_rddata_en_tsel_dly_0(&self) -> PhyRddataEnTselDly0R {
        PhyRddataEnTselDly0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for LP4 OE extension generation for slice 0."]
    #[inline(always)]
    pub fn phy_rddata_en_oe_dly_0(&self) -> PhyRddataEnOeDly0R {
        PhyRddataEnOeDly0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Master delay line override settings for slice 0. Bit \\[0\\]
enables software half clock mode. Bit \\[1\\]
is the software half clock mode value. Bit \\[2\\]
enables software bypass mode. Bit \\[3\\]
is the software bypass mode value."]
    #[inline(always)]
    pub fn phy_sw_master_mode_0(&self) -> PhySwMasterMode0R {
        PhySwMasterMode0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_tsel_dly_0(
        &mut self,
    ) -> PhyRddataEnTselDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec> {
        PhyRddataEnTselDly0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Number of cycles that the dfi_rddata_en signal is earlier than necessary for LP4 OE extension generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_oe_dly_0(
        &mut self,
    ) -> PhyRddataEnOeDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec> {
        PhyRddataEnOeDly0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Master delay line override settings for slice 0. Bit \\[0\\]
enables software half clock mode. Bit \\[1\\]
is the software half clock mode value. Bit \\[2\\]
enables software bypass mode. Bit \\[3\\]
is the software bypass mode value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_master_mode_0(
        &mut self,
    ) -> PhySwMasterMode0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec> {
        PhySwMasterMode0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_93::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_93::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_93::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_93::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_93 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy93Spec {
    const RESET_VALUE: u32 = 0;
}
