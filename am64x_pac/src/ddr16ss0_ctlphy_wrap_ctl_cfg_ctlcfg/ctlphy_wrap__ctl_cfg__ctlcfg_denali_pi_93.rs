#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_93` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_93` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec>;
#[doc = "Field `PI_RAS_N_MUX` reader - 4:0\\]
Command pin RAS_N mux selector"]
pub type PiRasNMuxR = crate::FieldReader;
#[doc = "Field `PI_RAS_N_MUX` writer - 4:0\\]
Command pin RAS_N mux selector"]
pub type PiRasNMuxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_CAS_N_MUX` reader - 12:8\\]
Command pin CAS_N mux selector"]
pub type PiCasNMuxR = crate::FieldReader;
#[doc = "Field `PI_CAS_N_MUX` writer - 12:8\\]
Command pin CAS_N mux selector"]
pub type PiCasNMuxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_WE_N_MUX` reader - 20:16\\]
Command pin WE_N mux selector"]
pub type PiWeNMuxR = crate::FieldReader;
#[doc = "Field `PI_WE_N_MUX` writer - 20:16\\]
Command pin WE_N mux selector"]
pub type PiWeNMuxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_BANK_MUX_0` reader - 28:24\\]
Command pin BANK_0 mux selector"]
pub type PiBankMux0R = crate::FieldReader;
#[doc = "Field `PI_BANK_MUX_0` writer - 28:24\\]
Command pin BANK_0 mux selector"]
pub type PiBankMux0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin RAS_N mux selector"]
    #[inline(always)]
    pub fn pi_ras_n_mux(&self) -> PiRasNMuxR {
        PiRasNMuxR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin CAS_N mux selector"]
    #[inline(always)]
    pub fn pi_cas_n_mux(&self) -> PiCasNMuxR {
        PiCasNMuxR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin WE_N mux selector"]
    #[inline(always)]
    pub fn pi_we_n_mux(&self) -> PiWeNMuxR {
        PiWeNMuxR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin BANK_0 mux selector"]
    #[inline(always)]
    pub fn pi_bank_mux_0(&self) -> PiBankMux0R {
        PiBankMux0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin RAS_N mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_ras_n_mux(&mut self) -> PiRasNMuxW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec> {
        PiRasNMuxW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Command pin CAS_N mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_cas_n_mux(&mut self) -> PiCasNMuxW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec> {
        PiCasNMuxW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Command pin WE_N mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_we_n_mux(&mut self) -> PiWeNMuxW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec> {
        PiWeNMuxW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Command pin BANK_0 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bank_mux_0(&mut self) -> PiBankMux0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec> {
        PiBankMux0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_93::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_93::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_93::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_93::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_93 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi93Spec {
    const RESET_VALUE: u32 = 0;
}
