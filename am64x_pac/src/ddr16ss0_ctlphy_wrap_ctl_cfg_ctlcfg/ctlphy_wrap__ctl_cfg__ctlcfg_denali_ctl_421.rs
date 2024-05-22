#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_421` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_421` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec>;
#[doc = "Field `WL_TICK_MINUS_ADJ` reader - 3:0\\]
NEED TO FiLL IN ."]
pub type WlTickMinusAdjR = crate::FieldReader;
#[doc = "Field `WL_TICK_MINUS_ADJ` writer - 3:0\\]
NEED TO FiLL IN ."]
pub type WlTickMinusAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NWR_F0` reader - 15:8\\]
DRAM NWR value in cycles. FC=0"]
pub type NwrF0R = crate::FieldReader;
#[doc = "Field `NWR_F0` writer - 15:8\\]
DRAM NWR value in cycles. FC=0"]
pub type NwrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NWR_F1` reader - 23:16\\]
DRAM NWR value in cycles. FC=1"]
pub type NwrF1R = crate::FieldReader;
#[doc = "Field `NWR_F1` writer - 23:16\\]
DRAM NWR value in cycles. FC=1"]
pub type NwrF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NWR_F2` reader - 31:24\\]
DRAM NWR value in cycles. FC=2"]
pub type NwrF2R = crate::FieldReader;
#[doc = "Field `NWR_F2` writer - 31:24\\]
DRAM NWR value in cycles. FC=2"]
pub type NwrF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn wl_tick_minus_adj(&self) -> WlTickMinusAdjR {
        WlTickMinusAdjR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM NWR value in cycles. FC=0"]
    #[inline(always)]
    pub fn nwr_f0(&self) -> NwrF0R {
        NwrF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM NWR value in cycles. FC=1"]
    #[inline(always)]
    pub fn nwr_f1(&self) -> NwrF1R {
        NwrF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM NWR value in cycles. FC=2"]
    #[inline(always)]
    pub fn nwr_f2(&self) -> NwrF2R {
        NwrF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn wl_tick_minus_adj(
        &mut self,
    ) -> WlTickMinusAdjW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec> {
        WlTickMinusAdjW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM NWR value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn nwr_f0(&mut self) -> NwrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec> {
        NwrF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM NWR value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn nwr_f1(&mut self) -> NwrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec> {
        NwrF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM NWR value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn nwr_f2(&mut self) -> NwrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec> {
        NwrF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_421\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_421::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_421::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_421::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_421::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_421 to value 0x4040_4000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl421Spec {
    const RESET_VALUE: u32 = 0x4040_4000;
}
