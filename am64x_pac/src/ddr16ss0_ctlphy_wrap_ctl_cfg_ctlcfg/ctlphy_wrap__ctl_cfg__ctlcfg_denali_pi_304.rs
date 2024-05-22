#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_304` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_304` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec>;
#[doc = "Field `PI_MR12_DATA_F0_0` reader - 7:0\\]
Data to program into memory mode register 12 for chip select 0 for frequency set 0."]
pub type PiMr12DataF0_0R = crate::FieldReader;
#[doc = "Field `PI_MR12_DATA_F0_0` writer - 7:0\\]
Data to program into memory mode register 12 for chip select 0 for frequency set 0."]
pub type PiMr12DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR14_DATA_F0_0` reader - 15:8\\]
Data to program into memory mode register 14 for chip select 0 for frequency set 0."]
pub type PiMr14DataF0_0R = crate::FieldReader;
#[doc = "Field `PI_MR14_DATA_F0_0` writer - 15:8\\]
Data to program into memory mode register 14 for chip select 0 for frequency set 0."]
pub type PiMr14DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR22_DATA_F0_0` reader - 23:16\\]
Data to program into memory mode register 22 for chip select 0 for frequency set 0."]
pub type PiMr22DataF0_0R = crate::FieldReader;
#[doc = "Field `PI_MR22_DATA_F0_0` writer - 23:16\\]
Data to program into memory mode register 22 for chip select 0 for frequency set 0."]
pub type PiMr22DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR23_DATA_F0_0` reader - 31:24\\]
Data to program into memory mode register 23 for chip select 0 for frequency set 0."]
pub type PiMr23DataF0_0R = crate::FieldReader;
#[doc = "Field `PI_MR23_DATA_F0_0` writer - 31:24\\]
Data to program into memory mode register 23 for chip select 0 for frequency set 0."]
pub type PiMr23DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 12 for chip select 0 for frequency set 0."]
    #[inline(always)]
    pub fn pi_mr12_data_f0_0(&self) -> PiMr12DataF0_0R {
        PiMr12DataF0_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 14 for chip select 0 for frequency set 0."]
    #[inline(always)]
    pub fn pi_mr14_data_f0_0(&self) -> PiMr14DataF0_0R {
        PiMr14DataF0_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 22 for chip select 0 for frequency set 0."]
    #[inline(always)]
    pub fn pi_mr22_data_f0_0(&self) -> PiMr22DataF0_0R {
        PiMr22DataF0_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 23 for chip select 0 for frequency set 0."]
    #[inline(always)]
    pub fn pi_mr23_data_f0_0(&self) -> PiMr23DataF0_0R {
        PiMr23DataF0_0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 12 for chip select 0 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr12_data_f0_0(
        &mut self,
    ) -> PiMr12DataF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec> {
        PiMr12DataF0_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 14 for chip select 0 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr14_data_f0_0(
        &mut self,
    ) -> PiMr14DataF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec> {
        PiMr14DataF0_0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 22 for chip select 0 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr22_data_f0_0(
        &mut self,
    ) -> PiMr22DataF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec> {
        PiMr22DataF0_0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 23 for chip select 0 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr23_data_f0_0(
        &mut self,
    ) -> PiMr23DataF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec> {
        PiMr23DataF0_0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_304\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_304::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_304::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_304::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_304::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_304 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi304Spec {
    const RESET_VALUE: u32 = 0;
}
