#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_418` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_418` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec>;
#[doc = "Field `TRAS_TICK_MINUS_ADJ` reader - 3:0\\]
NEED TO FiLL IN ."]
pub type TrasTickMinusAdjR = crate::FieldReader;
#[doc = "Field `TRAS_TICK_MINUS_ADJ` writer - 3:0\\]
NEED TO FiLL IN ."]
pub type TrasTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRP_TICK_PLUS_ADJ` reader - 11:8\\]
NEED TO FiLL IN ."]
pub type TrpTickPlusAdjR = crate::FieldReader;
#[doc = "Field `TRP_TICK_PLUS_ADJ` writer - 11:8\\]
NEED TO FiLL IN ."]
pub type TrpTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRP_TICK_MINUS_ADJ` reader - 19:16\\]
NEED TO FiLL IN ."]
pub type TrpTickMinusAdjR = crate::FieldReader;
#[doc = "Field `TRP_TICK_MINUS_ADJ` writer - 19:16\\]
NEED TO FiLL IN ."]
pub type TrpTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TWR_TICK_PLUS_ADJ` reader - 27:24\\]
NEED TO FiLL IN ."]
pub type TwrTickPlusAdjR = crate::FieldReader;
#[doc = "Field `TWR_TICK_PLUS_ADJ` writer - 27:24\\]
NEED TO FiLL IN ."]
pub type TwrTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tras_tick_minus_adj(&self) -> TrasTickMinusAdjR {
        TrasTickMinusAdjR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn trp_tick_plus_adj(&self) -> TrpTickPlusAdjR {
        TrpTickPlusAdjR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn trp_tick_minus_adj(&self) -> TrpTickMinusAdjR {
        TrpTickMinusAdjR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn twr_tick_plus_adj(&self) -> TwrTickPlusAdjR {
        TwrTickPlusAdjR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tras_tick_minus_adj(
        &mut self,
    ) -> TrasTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec> {
        TrasTickMinusAdjW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn trp_tick_plus_adj(
        &mut self,
    ) -> TrpTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec> {
        TrpTickPlusAdjW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn trp_tick_minus_adj(
        &mut self,
    ) -> TrpTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec> {
        TrpTickMinusAdjW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn twr_tick_plus_adj(
        &mut self,
    ) -> TwrTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec> {
        TwrTickPlusAdjW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_418\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_418::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_418::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_418::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_418::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_418 to value 0x0200_0100"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl418Spec {
    const RESET_VALUE: u32 = 0x0200_0100;
}
