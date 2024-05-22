#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_420` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_420` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec>;
#[doc = "Field `TRFC_TICK_MINUS_ADJ` reader - 3:0\\]
NEED TO FiLL IN ."]
pub type TrfcTickMinusAdjR = crate::FieldReader;
#[doc = "Field `TRFC_TICK_MINUS_ADJ` writer - 3:0\\]
NEED TO FiLL IN ."]
pub type TrfcTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RL_TICK_PLUS_ADJ` reader - 11:8\\]
NEED TO FiLL IN ."]
pub type RlTickPlusAdjR = crate::FieldReader;
#[doc = "Field `RL_TICK_PLUS_ADJ` writer - 11:8\\]
NEED TO FiLL IN ."]
pub type RlTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RL_TICK_MINUS_ADJ` reader - 19:16\\]
NEED TO FiLL IN ."]
pub type RlTickMinusAdjR = crate::FieldReader;
#[doc = "Field `RL_TICK_MINUS_ADJ` writer - 19:16\\]
NEED TO FiLL IN ."]
pub type RlTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WL_TICK_PLUS_ADJ` reader - 27:24\\]
NEED TO FiLL IN ."]
pub type WlTickPlusAdjR = crate::FieldReader;
#[doc = "Field `WL_TICK_PLUS_ADJ` writer - 27:24\\]
NEED TO FiLL IN ."]
pub type WlTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn trfc_tick_minus_adj(&self) -> TrfcTickMinusAdjR {
        TrfcTickMinusAdjR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn rl_tick_plus_adj(&self) -> RlTickPlusAdjR {
        RlTickPlusAdjR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn rl_tick_minus_adj(&self) -> RlTickMinusAdjR {
        RlTickMinusAdjR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn wl_tick_plus_adj(&self) -> WlTickPlusAdjR {
        WlTickPlusAdjR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn trfc_tick_minus_adj(
        &mut self,
    ) -> TrfcTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec> {
        TrfcTickMinusAdjW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn rl_tick_plus_adj(&mut self) -> RlTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec> {
        RlTickPlusAdjW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn rl_tick_minus_adj(
        &mut self,
    ) -> RlTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec> {
        RlTickMinusAdjW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn wl_tick_plus_adj(&mut self) -> WlTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec> {
        WlTickPlusAdjW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_420\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_420::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_420::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_420::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_420::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_420 to value 0x0200_0201"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl420Spec {
    const RESET_VALUE: u32 = 0x0200_0201;
}
