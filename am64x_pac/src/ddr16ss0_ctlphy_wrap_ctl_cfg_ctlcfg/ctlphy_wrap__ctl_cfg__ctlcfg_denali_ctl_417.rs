#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_417` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_417` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec>;
#[doc = "Field `TMP_NXN_TICK_MINUS_ADJ` reader - 3:0\\]
NEED TO FiLL IN ."]
pub type TmpNxnTickMinusAdjR = crate::FieldReader;
#[doc = "Field `TMP_NXN_TICK_MINUS_ADJ` writer - 3:0\\]
NEED TO FiLL IN ."]
pub type TmpNxnTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODT_TICK_PLUS_ADJ` reader - 11:8\\]
NEED TO FiLL IN ."]
pub type OdtTickPlusAdjR = crate::FieldReader;
#[doc = "Field `ODT_TICK_PLUS_ADJ` writer - 11:8\\]
NEED TO FiLL IN ."]
pub type OdtTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODT_TICK_MINUS_ADJ` reader - 19:16\\]
NEED TO FiLL IN ."]
pub type OdtTickMinusAdjR = crate::FieldReader;
#[doc = "Field `ODT_TICK_MINUS_ADJ` writer - 19:16\\]
NEED TO FiLL IN ."]
pub type OdtTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRAS_TICK_PLUS_ADJ` reader - 27:24\\]
NEED TO FiLL IN ."]
pub type TrasTickPlusAdjR = crate::FieldReader;
#[doc = "Field `TRAS_TICK_PLUS_ADJ` writer - 27:24\\]
NEED TO FiLL IN ."]
pub type TrasTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tmp_nxn_tick_minus_adj(&self) -> TmpNxnTickMinusAdjR {
        TmpNxnTickMinusAdjR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn odt_tick_plus_adj(&self) -> OdtTickPlusAdjR {
        OdtTickPlusAdjR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn odt_tick_minus_adj(&self) -> OdtTickMinusAdjR {
        OdtTickMinusAdjR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tras_tick_plus_adj(&self) -> TrasTickPlusAdjR {
        TrasTickPlusAdjR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tmp_nxn_tick_minus_adj(
        &mut self,
    ) -> TmpNxnTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec> {
        TmpNxnTickMinusAdjW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn odt_tick_plus_adj(
        &mut self,
    ) -> OdtTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec> {
        OdtTickPlusAdjW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn odt_tick_minus_adj(
        &mut self,
    ) -> OdtTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec> {
        OdtTickMinusAdjW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tras_tick_plus_adj(
        &mut self,
    ) -> TrasTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec> {
        TrasTickPlusAdjW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_417\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_417::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_417::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_417::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_417::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_417 to value 0x0101_0001"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl417Spec {
    const RESET_VALUE: u32 = 0x0101_0001;
}
