#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_190` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_190` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec>;
#[doc = "Field `PI_TWR_MPR_F0` reader - 7:0\\]
Number of cycles after MPR write command and before any other command for frequency set 0."]
pub type PiTwrMprF0R = crate::FieldReader;
#[doc = "Field `PI_TWR_MPR_F0` writer - 7:0\\]
Number of cycles after MPR write command and before any other command for frequency set 0."]
pub type PiTwrMprF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TWR_MPR_F1` reader - 15:8\\]
Number of cycles after MPR write command and before any other command for frequency set 1."]
pub type PiTwrMprF1R = crate::FieldReader;
#[doc = "Field `PI_TWR_MPR_F1` writer - 15:8\\]
Number of cycles after MPR write command and before any other command for frequency set 1."]
pub type PiTwrMprF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TWR_MPR_F2` reader - 23:16\\]
Number of cycles after MPR write command and before any other command for frequency set 2."]
pub type PiTwrMprF2R = crate::FieldReader;
#[doc = "Field `PI_TWR_MPR_F2` writer - 23:16\\]
Number of cycles after MPR write command and before any other command for frequency set 2."]
pub type PiTwrMprF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_RDLVL_PAT0_EN_F0` reader - 25:24\\]
Enable PATTERN-0 for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlPat0EnF0R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_PAT0_EN_F0` writer - 25:24\\]
Enable PATTERN-0 for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlPat0EnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of cycles after MPR write command and before any other command for frequency set 0."]
    #[inline(always)]
    pub fn pi_twr_mpr_f0(&self) -> PiTwrMprF0R {
        PiTwrMprF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles after MPR write command and before any other command for frequency set 1."]
    #[inline(always)]
    pub fn pi_twr_mpr_f1(&self) -> PiTwrMprF1R {
        PiTwrMprF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Number of cycles after MPR write command and before any other command for frequency set 2."]
    #[inline(always)]
    pub fn pi_twr_mpr_f2(&self) -> PiTwrMprF2R {
        PiTwrMprF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable PATTERN-0 for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_pat0_en_f0(&self) -> PiRdlvlPat0EnF0R {
        PiRdlvlPat0EnF0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of cycles after MPR write command and before any other command for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_mpr_f0(&mut self) -> PiTwrMprF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec> {
        PiTwrMprF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles after MPR write command and before any other command for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_mpr_f1(&mut self) -> PiTwrMprF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec> {
        PiTwrMprF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Number of cycles after MPR write command and before any other command for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_mpr_f2(&mut self) -> PiTwrMprF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec> {
        PiTwrMprF2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable PATTERN-0 for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pat0_en_f0(
        &mut self,
    ) -> PiRdlvlPat0EnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec> {
        PiRdlvlPat0EnF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_190\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_190::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_190::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_190::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_190::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_190 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi190Spec {
    const RESET_VALUE: u32 = 0;
}
