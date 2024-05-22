#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_292` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_292` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec>;
#[doc = "Field `PI_CS_MUX_1` reader - 4:0\\]
Command pin CS_1 mux selector"]
pub type PiCsMux1R = crate::FieldReader;
#[doc = "Field `PI_CS_MUX_1` writer - 4:0\\]
Command pin CS_1 mux selector"]
pub type PiCsMux1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_ODT_MUX_0` reader - 12:8\\]
Command pin ODT_0 mux selector"]
pub type PiOdtMux0R = crate::FieldReader;
#[doc = "Field `PI_ODT_MUX_0` writer - 12:8\\]
Command pin ODT_0 mux selector"]
pub type PiOdtMux0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_ODT_MUX_1` reader - 20:16\\]
Command pin ODT_1 mux selector"]
pub type PiOdtMux1R = crate::FieldReader;
#[doc = "Field `PI_ODT_MUX_1` writer - 20:16\\]
Command pin ODT_1 mux selector"]
pub type PiOdtMux1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_RESET_N_MUX_0` reader - 28:24\\]
Command pin RESET_N_0 mux selector"]
pub type PiResetNMux0R = crate::FieldReader;
#[doc = "Field `PI_RESET_N_MUX_0` writer - 28:24\\]
Command pin RESET_N_0 mux selector"]
pub type PiResetNMux0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin CS_1 mux selector"]
    #[inline(always)]
    pub fn pi_cs_mux_1(&self) -> PiCsMux1R {
        PiCsMux1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin ODT_0 mux selector"]
    #[inline(always)]
    pub fn pi_odt_mux_0(&self) -> PiOdtMux0R {
        PiOdtMux0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin ODT_1 mux selector"]
    #[inline(always)]
    pub fn pi_odt_mux_1(&self) -> PiOdtMux1R {
        PiOdtMux1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin RESET_N_0 mux selector"]
    #[inline(always)]
    pub fn pi_reset_n_mux_0(&self) -> PiResetNMux0R {
        PiResetNMux0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin CS_1 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_cs_mux_1(&mut self) -> PiCsMux1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec> {
        PiCsMux1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin ODT_0 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_mux_0(&mut self) -> PiOdtMux0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec> {
        PiOdtMux0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin ODT_1 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_mux_1(&mut self) -> PiOdtMux1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec> {
        PiOdtMux1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin RESET_N_0 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_reset_n_mux_0(&mut self) -> PiResetNMux0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec> {
        PiResetNMux0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_292\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_292::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_292::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_292::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_292::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_292 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi292Spec {
    const RESET_VALUE: u32 = 0;
}
