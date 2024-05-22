#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_293` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_293` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec>;
#[doc = "Field `PI_RESET_N_MUX_1` reader - 4:0\\]
Command pin RESET_N_1 mux selector"]
pub type PiResetNMux1R = crate::FieldReader;
#[doc = "Field `PI_RESET_N_MUX_1` writer - 4:0\\]
Command pin RESET_N_1 mux selector"]
pub type PiResetNMux1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_MRSINGLE_DATA_0` reader - 24:8\\]
Data to program into memory mode register single write to chip select 0."]
pub type PiMrsingleData0R = crate::FieldReader<u32>;
#[doc = "Field `PI_MRSINGLE_DATA_0` writer - 24:8\\]
Data to program into memory mode register single write to chip select 0."]
pub type PiMrsingleData0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin RESET_N_1 mux selector"]
    #[inline(always)]
    pub fn pi_reset_n_mux_1(&self) -> PiResetNMux1R {
        PiResetNMux1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:24 - 24:8\\]
Data to program into memory mode register single write to chip select 0."]
    #[inline(always)]
    pub fn pi_mrsingle_data_0(&self) -> PiMrsingleData0R {
        PiMrsingleData0R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Command pin RESET_N_1 mux selector"]
    #[inline(always)]
    #[must_use]
    pub fn pi_reset_n_mux_1(&mut self) -> PiResetNMux1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec> {
        PiResetNMux1W::new(self, 0)
    }
    #[doc = "Bits 8:24 - 24:8\\]
Data to program into memory mode register single write to chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mrsingle_data_0(
        &mut self,
    ) -> PiMrsingleData0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec> {
        PiMrsingleData0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_293\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_293::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_293::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_293::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_293::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_293 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi293Spec {
    const RESET_VALUE: u32 = 0;
}
