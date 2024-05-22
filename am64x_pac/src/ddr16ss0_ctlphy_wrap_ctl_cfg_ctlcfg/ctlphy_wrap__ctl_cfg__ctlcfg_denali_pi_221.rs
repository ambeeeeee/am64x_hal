#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_221` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_221` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec>;
#[doc = "Field `PI_VREF_EN_F2` reader - 1:0\\]
Enable VREF training during power-up initialization for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiVrefEnF2R = crate::FieldReader;
#[doc = "Field `PI_VREF_EN_F2` writer - 1:0\\]
Enable VREF training during power-up initialization for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiVrefEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TDFI_WDQLVL_WR_F0` reader - 17:8\\]
Switch time from write to read for frequency set 0."]
pub type PiTdfiWdqlvlWrF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_WR_F0` writer - 17:8\\]
Switch time from write to read for frequency set 0."]
pub type PiTdfiWdqlvlWrF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable VREF training during power-up initialization for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_vref_en_f2(&self) -> PiVrefEnF2R {
        PiVrefEnF2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Switch time from write to read for frequency set 0."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_wr_f0(&self) -> PiTdfiWdqlvlWrF0R {
        PiTdfiWdqlvlWrF0R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable VREF training during power-up initialization for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_en_f2(&mut self) -> PiVrefEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec> {
        PiVrefEnF2W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Switch time from write to read for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_wr_f0(
        &mut self,
    ) -> PiTdfiWdqlvlWrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec> {
        PiTdfiWdqlvlWrF0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_221\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_221::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_221::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_221::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_221::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_221 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi221Spec {
    const RESET_VALUE: u32 = 0;
}
