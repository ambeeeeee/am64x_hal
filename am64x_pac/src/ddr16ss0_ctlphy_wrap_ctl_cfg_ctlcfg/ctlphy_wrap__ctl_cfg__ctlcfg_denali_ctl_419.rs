#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_419` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_419` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec>;
#[doc = "Field `TWR_TICK_MINUS_ADJ` reader - 3:0\\]
NEED TO FiLL IN ."]
pub type TwrTickMinusAdjR = crate::FieldReader;
#[doc = "Field `TWR_TICK_MINUS_ADJ` writer - 3:0\\]
NEED TO FiLL IN ."]
pub type TwrTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMP_4X2_TICK_PLUS_ADJ` reader - 11:8\\]
NEED TO FiLL IN ."]
pub type Tmp4x2TickPlusAdjR = crate::FieldReader;
#[doc = "Field `TMP_4X2_TICK_PLUS_ADJ` writer - 11:8\\]
NEED TO FiLL IN ."]
pub type Tmp4x2TickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMP_4X2_TICK_MINUS_ADJ` reader - 19:16\\]
NEED TO FiLL IN ."]
pub type Tmp4x2TickMinusAdjR = crate::FieldReader;
#[doc = "Field `TMP_4X2_TICK_MINUS_ADJ` writer - 19:16\\]
NEED TO FiLL IN ."]
pub type Tmp4x2TickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRFC_TICK_PLUS_ADJ` reader - 27:24\\]
NEED TO FiLL IN ."]
pub type TrfcTickPlusAdjR = crate::FieldReader;
#[doc = "Field `TRFC_TICK_PLUS_ADJ` writer - 27:24\\]
NEED TO FiLL IN ."]
pub type TrfcTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn twr_tick_minus_adj(&self) -> TwrTickMinusAdjR {
        TwrTickMinusAdjR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tmp_4x2_tick_plus_adj(&self) -> Tmp4x2TickPlusAdjR {
        Tmp4x2TickPlusAdjR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tmp_4x2_tick_minus_adj(&self) -> Tmp4x2TickMinusAdjR {
        Tmp4x2TickMinusAdjR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn trfc_tick_plus_adj(&self) -> TrfcTickPlusAdjR {
        TrfcTickPlusAdjR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn twr_tick_minus_adj(
        &mut self,
    ) -> TwrTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec> {
        TwrTickMinusAdjW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tmp_4x2_tick_plus_adj(
        &mut self,
    ) -> Tmp4x2TickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec> {
        Tmp4x2TickPlusAdjW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tmp_4x2_tick_minus_adj(
        &mut self,
    ) -> Tmp4x2TickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec> {
        Tmp4x2TickMinusAdjW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn trfc_tick_plus_adj(
        &mut self,
    ) -> TrfcTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec> {
        TrfcTickPlusAdjW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_419\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_419::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_419::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_419::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_419::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_419 to value 0x0200"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl419Spec {
    const RESET_VALUE: u32 = 0x0200;
}
