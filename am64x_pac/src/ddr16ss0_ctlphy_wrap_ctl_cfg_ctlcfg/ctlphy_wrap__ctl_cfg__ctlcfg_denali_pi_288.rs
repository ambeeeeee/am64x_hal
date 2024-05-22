#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_288` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_288` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec>;
#[doc = "Field `PI_MR32_DATA_0` reader - 16:0\\]
Data to program into memory mode register 32 for chip select 0."]
pub type PiMr32Data0R = crate::FieldReader<u32>;
#[doc = "Field `PI_MR32_DATA_0` writer - 16:0\\]
Data to program into memory mode register 32 for chip select 0."]
pub type PiMr32Data0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PI_MR40_DATA_0` reader - 31:24\\]
Data to program into memory mode register 40 for chip select 0."]
pub type PiMr40Data0R = crate::FieldReader;
#[doc = "Field `PI_MR40_DATA_0` writer - 31:24\\]
Data to program into memory mode register 40 for chip select 0."]
pub type PiMr40Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 32 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr32_data_0(&self) -> PiMr32Data0R {
        PiMr32Data0R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 40 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr40_data_0(&self) -> PiMr40Data0R {
        PiMr40Data0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 32 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr32_data_0(&mut self) -> PiMr32Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec> {
        PiMr32Data0W::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 40 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr40_data_0(&mut self) -> PiMr40Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec> {
        PiMr40Data0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_288\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_288::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_288::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_288::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_288::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_288 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi288Spec {
    const RESET_VALUE: u32 = 0;
}
