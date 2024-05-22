#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_286` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_286` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec>;
#[doc = "Field `PI_MR6_VREF_1_0` reader - 5:0\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref1_0R = crate::FieldReader;
#[doc = "Field `PI_MR6_VREF_1_0` writer - 5:0\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref1_0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_MR6_VREF_1_1` reader - 13:8\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref1_1R = crate::FieldReader;
#[doc = "Field `PI_MR6_VREF_1_1` writer - 13:8\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref1_1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_MR13_DATA_0` reader - 23:16\\]
Data to program into memory mode register 13 for chip select 0."]
pub type PiMr13Data0R = crate::FieldReader;
#[doc = "Field `PI_MR13_DATA_0` writer - 23:16\\]
Data to program into memory mode register 13 for chip select 0."]
pub type PiMr13Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR15_DATA_0` reader - 31:24\\]
Data to program into memory mode register 15 for chip select 0."]
pub type PiMr15Data0R = crate::FieldReader;
#[doc = "Field `PI_MR15_DATA_0` writer - 31:24\\]
Data to program into memory mode register 15 for chip select 0."]
pub type PiMr15Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    pub fn pi_mr6_vref_1_0(&self) -> PiMr6Vref1_0R {
        PiMr6Vref1_0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    pub fn pi_mr6_vref_1_1(&self) -> PiMr6Vref1_1R {
        PiMr6Vref1_1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 13 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr13_data_0(&self) -> PiMr13Data0R {
        PiMr13Data0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 15 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr15_data_0(&self) -> PiMr15Data0R {
        PiMr15Data0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr6_vref_1_0(&mut self) -> PiMr6Vref1_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec> {
        PiMr6Vref1_0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr6_vref_1_1(&mut self) -> PiMr6Vref1_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec> {
        PiMr6Vref1_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 13 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr13_data_0(&mut self) -> PiMr13Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec> {
        PiMr13Data0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 15 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr15_data_0(&mut self) -> PiMr15Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec> {
        PiMr15Data0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_286\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_286::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_286::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_286::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_286::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_286 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi286Spec {
    const RESET_VALUE: u32 = 0;
}
