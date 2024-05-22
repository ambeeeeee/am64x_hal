#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_92` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_92` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec>;
#[doc = "Field `PI_PARITY_IN_MUX` reader - 4:0\\]
Command pin parity mux selector"]
pub type PiParityInMuxR = crate::FieldReader;
#[doc = "Field `PI_PARITY_IN_MUX` writer - 4:0\\]
Command pin parity mux selector"]
pub type PiParityInMuxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_ACT_N_MUX` reader - 12:8\\]
Command pin ACT_N mux selector"]
pub type PiActNMuxR = crate::FieldReader;
#[doc = "Field `PI_ACT_N_MUX` writer - 12:8\\]
Command pin ACT_N mux selector"]
pub type PiActNMuxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_BG_MUX_0` reader - 20:16\\]
Command pin BG_0 mux selector"]
pub type PiBgMux0R = crate::FieldReader;
#[doc = "Field `PI_BG_MUX_0` writer - 20:16\\]
Command pin BG_0 mux selector"]
pub type PiBgMux0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_BG_MUX_1` reader - 28:24\\]
Command pin BG_1 mux selector"]
pub type PiBgMux1R = crate::FieldReader;
#[doc = "Field `PI_BG_MUX_1` writer - 28:24\\]
Command pin BG_1 mux selector"]
pub type PiBgMux1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin parity mux selector"]
    #[inline(always)]
    pub fn pi_parity_in_mux(&self) -> PiParityInMuxR {
        PiParityInMuxR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin ACT_N mux selector"]
    #[inline(always)]
    pub fn pi_act_n_mux(&self) -> PiActNMuxR {
        PiActNMuxR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin BG_0 mux selector"]
    #[inline(always)]
    pub fn pi_bg_mux_0(&self) -> PiBgMux0R {
        PiBgMux0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin BG_1 mux selector"]
    #[inline(always)]
    pub fn pi_bg_mux_1(&self) -> PiBgMux1R {
        PiBgMux1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin parity mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_parity_in_mux(&mut self) -> PiParityInMuxW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec> {
        PiParityInMuxW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin ACT_N mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_act_n_mux(&mut self) -> PiActNMuxW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec> {
        PiActNMuxW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin BG_0 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bg_mux_0(&mut self) -> PiBgMux0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec> {
        PiBgMux0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin BG_1 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bg_mux_1(&mut self) -> PiBgMux1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec> {
        PiBgMux1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_92::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_92::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_92::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_92::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_92 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi92Spec {
    const RESET_VALUE: u32 = 0;
}
