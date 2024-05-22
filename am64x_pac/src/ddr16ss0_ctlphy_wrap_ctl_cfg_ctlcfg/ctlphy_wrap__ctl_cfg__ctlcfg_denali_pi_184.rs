#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_184` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_184` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec>;
#[doc = "Field `PI_TODTL_2CMD_F0` reader - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 0."]
pub type PiTodtl2cmdF0R = crate::FieldReader;
#[doc = "Field `PI_TODTL_2CMD_F0` writer - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 0."]
pub type PiTodtl2cmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_ODT_EN_F0` reader - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 0."]
pub type PiOdtEnF0R = crate::BitReader;
#[doc = "Field `PI_ODT_EN_F0` writer - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 0."]
pub type PiOdtEnF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TODTL_2CMD_F1` reader - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 1."]
pub type PiTodtl2cmdF1R = crate::FieldReader;
#[doc = "Field `PI_TODTL_2CMD_F1` writer - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 1."]
pub type PiTodtl2cmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_ODT_EN_F1` reader - 24:24\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 1."]
pub type PiOdtEnF1R = crate::BitReader;
#[doc = "Field `PI_ODT_EN_F1` writer - 24:24\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 1."]
pub type PiOdtEnF1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 0."]
    #[inline(always)]
    pub fn pi_todtl_2cmd_f0(&self) -> PiTodtl2cmdF0R {
        PiTodtl2cmdF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 0."]
    #[inline(always)]
    pub fn pi_odt_en_f0(&self) -> PiOdtEnF0R {
        PiOdtEnF0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 1."]
    #[inline(always)]
    pub fn pi_todtl_2cmd_f1(&self) -> PiTodtl2cmdF1R {
        PiTodtl2cmdF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 1."]
    #[inline(always)]
    pub fn pi_odt_en_f1(&self) -> PiOdtEnF1R {
        PiOdtEnF1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todtl_2cmd_f0(&mut self) -> PiTodtl2cmdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec> {
        PiTodtl2cmdF0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_en_f0(&mut self) -> PiOdtEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec> {
        PiOdtEnF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todtl_2cmd_f1(&mut self) -> PiTodtl2cmdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec> {
        PiTodtl2cmdF1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_en_f1(&mut self) -> PiOdtEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec> {
        PiOdtEnF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_184\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_184::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_184::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_184::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_184::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_184 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi184Spec {
    const RESET_VALUE: u32 = 0;
}
