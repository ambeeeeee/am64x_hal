#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_416` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_416` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec>;
#[doc = "Field `PRE_4TICK_COUNT` reader - 2:0\\]
NEED TO FiLL IN ."]
pub type Pre4tickCountR = crate::FieldReader;
#[doc = "Field `PRE_4TICK_COUNT` writer - 2:0\\]
NEED TO FiLL IN ."]
pub type Pre4tickCountW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMP_2X4_TICK_PLUS_ADJ` reader - 11:8\\]
NEED TO FiLL IN ."]
pub type Tmp2x4TickPlusAdjR = crate::FieldReader;
#[doc = "Field `TMP_2X4_TICK_PLUS_ADJ` writer - 11:8\\]
NEED TO FiLL IN ."]
pub type Tmp2x4TickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMP_2X4_TICK_MINUS_ADJ` reader - 19:16\\]
NEED TO FiLL IN ."]
pub type Tmp2x4TickMinusAdjR = crate::FieldReader;
#[doc = "Field `TMP_2X4_TICK_MINUS_ADJ` writer - 19:16\\]
NEED TO FiLL IN ."]
pub type Tmp2x4TickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMP_NXN_TICK_PLUS_ADJ` reader - 27:24\\]
NEED TO FiLL IN ."]
pub type TmpNxnTickPlusAdjR = crate::FieldReader;
#[doc = "Field `TMP_NXN_TICK_PLUS_ADJ` writer - 27:24\\]
NEED TO FiLL IN ."]
pub type TmpNxnTickPlusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn pre_4tick_count(&self) -> Pre4tickCountR {
        Pre4tickCountR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tmp_2x4_tick_plus_adj(&self) -> Tmp2x4TickPlusAdjR {
        Tmp2x4TickPlusAdjR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tmp_2x4_tick_minus_adj(&self) -> Tmp2x4TickMinusAdjR {
        Tmp2x4TickMinusAdjR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn tmp_nxn_tick_plus_adj(&self) -> TmpNxnTickPlusAdjR {
        TmpNxnTickPlusAdjR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn pre_4tick_count(&mut self) -> Pre4tickCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec> {
        Pre4tickCountW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tmp_2x4_tick_plus_adj(
        &mut self,
    ) -> Tmp2x4TickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec> {
        Tmp2x4TickPlusAdjW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tmp_2x4_tick_minus_adj(
        &mut self,
    ) -> Tmp2x4TickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec> {
        Tmp2x4TickMinusAdjW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn tmp_nxn_tick_plus_adj(
        &mut self,
    ) -> TmpNxnTickPlusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec> {
        TmpNxnTickPlusAdjW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_416\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_416::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_416::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_416::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_416::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_416 to value 0x0001_0001"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl416Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
