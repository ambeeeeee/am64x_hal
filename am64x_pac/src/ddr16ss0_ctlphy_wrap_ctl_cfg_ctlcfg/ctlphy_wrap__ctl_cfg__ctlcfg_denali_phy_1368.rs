#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1368` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1368` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec>;
#[doc = "Field `PHY_DLL_RST_EN` reader - 1:0\\]
PHY DDL reset software interface enable."]
pub type PhyDllRstEnR = crate::FieldReader;
#[doc = "Field `PHY_DLL_RST_EN` writer - 1:0\\]
PHY DDL reset software interface enable."]
pub type PhyDllRstEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_AC_INIT_COMPLETE_OBS` reader - 19:8\\]
Observation register for dfi_init_complete for adr and ac slice. Bit 0 is for dfi_init_complete for all slices. Bit\\[7:4\\]
is for adr slice, bit4 is adr_slice0..., if the adr slice number is 3, bit7 is 0. Bit8 is for ac_slice0; bit9 is for ac_slice 1,... READ-ONLY."]
pub type PhyAcInitCompleteObsR = crate::FieldReader<u16>;
#[doc = "Field `PHY_AC_INIT_COMPLETE_OBS` writer - 19:8\\]
Observation register for dfi_init_complete for adr and ac slice. Bit 0 is for dfi_init_complete for all slices. Bit\\[7:4\\]
is for adr slice, bit4 is adr_slice0..., if the adr slice number is 3, bit7 is 0. Bit8 is for ac_slice0; bit9 is for ac_slice 1,... READ-ONLY."]
pub type PhyAcInitCompleteObsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_DS_INIT_COMPLETE_OBS` reader - 25:24\\]
Observation register for dfi_init_complete for data slice. Bit0 is for data_slice0; bit1 is for data_slice 1,... READ-ONLY."]
pub type PhyDsInitCompleteObsR = crate::FieldReader;
#[doc = "Field `PHY_DS_INIT_COMPLETE_OBS` writer - 25:24\\]
Observation register for dfi_init_complete for data slice. Bit0 is for data_slice0; bit1 is for data_slice 1,... READ-ONLY."]
pub type PhyDsInitCompleteObsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
PHY DDL reset software interface enable."]
    #[inline(always)]
    pub fn phy_dll_rst_en(&self) -> PhyDllRstEnR {
        PhyDllRstEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Observation register for dfi_init_complete for adr and ac slice. Bit 0 is for dfi_init_complete for all slices. Bit\\[7:4\\]
is for adr slice, bit4 is adr_slice0..., if the adr slice number is 3, bit7 is 0. Bit8 is for ac_slice0; bit9 is for ac_slice 1,... READ-ONLY."]
    #[inline(always)]
    pub fn phy_ac_init_complete_obs(&self) -> PhyAcInitCompleteObsR {
        PhyAcInitCompleteObsR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Observation register for dfi_init_complete for data slice. Bit0 is for data_slice0; bit1 is for data_slice 1,... READ-ONLY."]
    #[inline(always)]
    pub fn phy_ds_init_complete_obs(&self) -> PhyDsInitCompleteObsR {
        PhyDsInitCompleteObsR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
PHY DDL reset software interface enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dll_rst_en(&mut self) -> PhyDllRstEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec> {
        PhyDllRstEnW::new(self, 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Observation register for dfi_init_complete for adr and ac slice. Bit 0 is for dfi_init_complete for all slices. Bit\\[7:4\\]
is for adr slice, bit4 is adr_slice0..., if the adr slice number is 3, bit7 is 0. Bit8 is for ac_slice0; bit9 is for ac_slice 1,... READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_init_complete_obs(
        &mut self,
    ) -> PhyAcInitCompleteObsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec> {
        PhyAcInitCompleteObsW::new(self, 8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Observation register for dfi_init_complete for data slice. Bit0 is for data_slice0; bit1 is for data_slice 1,... READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ds_init_complete_obs(
        &mut self,
    ) -> PhyDsInitCompleteObsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec> {
        PhyDsInitCompleteObsW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1368\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1368::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1368::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1368::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1368::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1368 to value 0x02"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1368Spec {
    const RESET_VALUE: u32 = 0x02;
}
