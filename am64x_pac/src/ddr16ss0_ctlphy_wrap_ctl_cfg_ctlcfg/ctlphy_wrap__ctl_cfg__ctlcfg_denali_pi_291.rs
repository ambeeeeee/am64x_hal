#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_291` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_291` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec>;
#[doc = "Field `PI_MR40_DATA_1` reader - 7:0\\]
Data to program into memory mode register 40 for chip select 1."]
pub type PiMr40Data1R = crate::FieldReader;
#[doc = "Field `PI_MR40_DATA_1` writer - 7:0\\]
Data to program into memory mode register 40 for chip select 1."]
pub type PiMr40Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_CKE_MUX_0` reader - 12:8\\]
Command pin CKE_0 mux selector"]
pub type PiCkeMux0R = crate::FieldReader;
#[doc = "Field `PI_CKE_MUX_0` writer - 12:8\\]
Command pin CKE_0 mux selector"]
pub type PiCkeMux0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_CKE_MUX_1` reader - 20:16\\]
Command pin CKE_1 mux selector"]
pub type PiCkeMux1R = crate::FieldReader;
#[doc = "Field `PI_CKE_MUX_1` writer - 20:16\\]
Command pin CKE_1 mux selector"]
pub type PiCkeMux1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_CS_MUX_0` reader - 28:24\\]
Command pin CS_0 mux selector"]
pub type PiCsMux0R = crate::FieldReader;
#[doc = "Field `PI_CS_MUX_0` writer - 28:24\\]
Command pin CS_0 mux selector"]
pub type PiCsMux0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 40 for chip select 1."]
    #[inline(always)]
    pub fn pi_mr40_data_1(&self) -> PiMr40Data1R {
        PiMr40Data1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin CKE_0 mux selector"]
    #[inline(always)]
    pub fn pi_cke_mux_0(&self) -> PiCkeMux0R {
        PiCkeMux0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin CKE_1 mux selector"]
    #[inline(always)]
    pub fn pi_cke_mux_1(&self) -> PiCkeMux1R {
        PiCkeMux1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin CS_0 mux selector"]
    #[inline(always)]
    pub fn pi_cs_mux_0(&self) -> PiCsMux0R {
        PiCsMux0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 40 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr40_data_1(&mut self) -> PiMr40Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec> {
        PiMr40Data1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin CKE_0 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_cke_mux_0(&mut self) -> PiCkeMux0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec> {
        PiCkeMux0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin CKE_1 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_cke_mux_1(&mut self) -> PiCkeMux1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec> {
        PiCkeMux1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin CS_0 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_cs_mux_0(&mut self) -> PiCsMux0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec> {
        PiCsMux0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_291\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_291::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_291::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_291::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_291::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_291 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi291Spec {
    const RESET_VALUE: u32 = 0;
}
