#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_346` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_346` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec>;
#[doc = "Field `PHY_VREF_SETTING_TIME_1` reader - 15:0\\]
Number of cycles for vref settle after setting is changed for slice 1."]
pub type PhyVrefSettingTime1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_VREF_SETTING_TIME_1` writer - 15:0\\]
Number of cycles for vref settle after setting is changed for slice 1."]
pub type PhyVrefSettingTime1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_1` reader - 27:16\\]
Pad VREF control settings for DQ slice 1."]
pub type PhyPadVrefCtrlDq1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_1` writer - 27:16\\]
Pad VREF control settings for DQ slice 1."]
pub type PhyPadVrefCtrlDq1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of cycles for vref settle after setting is changed for slice 1."]
    #[inline(always)]
    pub fn phy_vref_setting_time_1(&self) -> PhyVrefSettingTime1R {
        PhyVrefSettingTime1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Pad VREF control settings for DQ slice 1."]
    #[inline(always)]
    pub fn phy_pad_vref_ctrl_dq_1(&self) -> PhyPadVrefCtrlDq1R {
        PhyPadVrefCtrlDq1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Number of cycles for vref settle after setting is changed for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_setting_time_1(
        &mut self,
    ) -> PhyVrefSettingTime1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec> {
        PhyVrefSettingTime1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Pad VREF control settings for DQ slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_vref_ctrl_dq_1(
        &mut self,
    ) -> PhyPadVrefCtrlDq1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec> {
        PhyPadVrefCtrlDq1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_346\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_346::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_346::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_346::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_346::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_346 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy346Spec {
    const RESET_VALUE: u32 = 0;
}
