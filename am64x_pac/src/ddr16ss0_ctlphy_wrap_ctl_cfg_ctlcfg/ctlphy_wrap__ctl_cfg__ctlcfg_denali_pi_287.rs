#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_287` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_287` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec>;
#[doc = "Field `PI_MR16_DATA_0` reader - 7:0\\]
Data to program into memory mode register 16 for chip select 0."]
pub type PiMr16Data0R = crate::FieldReader;
#[doc = "Field `PI_MR16_DATA_0` writer - 7:0\\]
Data to program into memory mode register 16 for chip select 0."]
pub type PiMr16Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR17_DATA_0` reader - 15:8\\]
Data to program into memory mode register 17 for chip select 0."]
pub type PiMr17Data0R = crate::FieldReader;
#[doc = "Field `PI_MR17_DATA_0` writer - 15:8\\]
Data to program into memory mode register 17 for chip select 0."]
pub type PiMr17Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR20_DATA_0` reader - 23:16\\]
Data to program into memory mode register 20 for chip select 0."]
pub type PiMr20Data0R = crate::FieldReader;
#[doc = "Field `PI_MR20_DATA_0` writer - 23:16\\]
Data to program into memory mode register 20 for chip select 0."]
pub type PiMr20Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 16 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr16_data_0(&self) -> PiMr16Data0R {
        PiMr16Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 17 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr17_data_0(&self) -> PiMr17Data0R {
        PiMr17Data0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 20 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr20_data_0(&self) -> PiMr20Data0R {
        PiMr20Data0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 16 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr16_data_0(&mut self) -> PiMr16Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec> {
        PiMr16Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 17 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr17_data_0(&mut self) -> PiMr17Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec> {
        PiMr17Data0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 20 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr20_data_0(&mut self) -> PiMr20Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec> {
        PiMr20Data0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_287\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_287::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_287::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_287::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_287::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_287 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi287Spec {
    const RESET_VALUE: u32 = 0;
}
