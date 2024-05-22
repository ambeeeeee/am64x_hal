#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_289` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_289` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec>;
#[doc = "Field `PI_MR13_DATA_1` reader - 7:0\\]
Data to program into memory mode register 13 for chip select 1."]
pub type PiMr13Data1R = crate::FieldReader;
#[doc = "Field `PI_MR13_DATA_1` writer - 7:0\\]
Data to program into memory mode register 13 for chip select 1."]
pub type PiMr13Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR15_DATA_1` reader - 15:8\\]
Data to program into memory mode register 15 for chip select 1."]
pub type PiMr15Data1R = crate::FieldReader;
#[doc = "Field `PI_MR15_DATA_1` writer - 15:8\\]
Data to program into memory mode register 15 for chip select 1."]
pub type PiMr15Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR16_DATA_1` reader - 23:16\\]
Data to program into memory mode register 16 for chip select 1."]
pub type PiMr16Data1R = crate::FieldReader;
#[doc = "Field `PI_MR16_DATA_1` writer - 23:16\\]
Data to program into memory mode register 16 for chip select 1."]
pub type PiMr16Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR17_DATA_1` reader - 31:24\\]
Data to program into memory mode register 17 for chip select 1."]
pub type PiMr17Data1R = crate::FieldReader;
#[doc = "Field `PI_MR17_DATA_1` writer - 31:24\\]
Data to program into memory mode register 17 for chip select 1."]
pub type PiMr17Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 13 for chip select 1."]
    #[inline(always)]
    pub fn pi_mr13_data_1(&self) -> PiMr13Data1R {
        PiMr13Data1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 15 for chip select 1."]
    #[inline(always)]
    pub fn pi_mr15_data_1(&self) -> PiMr15Data1R {
        PiMr15Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 16 for chip select 1."]
    #[inline(always)]
    pub fn pi_mr16_data_1(&self) -> PiMr16Data1R {
        PiMr16Data1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 17 for chip select 1."]
    #[inline(always)]
    pub fn pi_mr17_data_1(&self) -> PiMr17Data1R {
        PiMr17Data1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 13 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr13_data_1(&mut self) -> PiMr13Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec> {
        PiMr13Data1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 15 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr15_data_1(&mut self) -> PiMr15Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec> {
        PiMr15Data1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 16 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr16_data_1(&mut self) -> PiMr16Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec> {
        PiMr16Data1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 17 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr17_data_1(&mut self) -> PiMr17Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec> {
        PiMr17Data1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_289\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_289::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_289::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_289::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_289::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_289 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi289Spec {
    const RESET_VALUE: u32 = 0;
}
