#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_373` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_373` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec>;
#[doc = "Field `ODT_EN_F0` reader - 0:0\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=0"]
pub type OdtEnF0R = crate::BitReader;
#[doc = "Field `ODT_EN_F0` writer - 0:0\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=0"]
pub type OdtEnF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT_EN_F1` reader - 8:8\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=1"]
pub type OdtEnF1R = crate::BitReader;
#[doc = "Field `ODT_EN_F1` writer - 8:8\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=1"]
pub type OdtEnF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT_EN_F2` reader - 16:16\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=2"]
pub type OdtEnF2R = crate::BitReader;
#[doc = "Field `ODT_EN_F2` writer - 16:16\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=2"]
pub type OdtEnF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_ODT_ASSERT_EXCEPT_RD` reader - 24:24\\]
Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
pub type EnOdtAssertExceptRdR = crate::BitReader;
#[doc = "Field `EN_ODT_ASSERT_EXCEPT_RD` writer - 24:24\\]
Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
pub type EnOdtAssertExceptRdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=0"]
    #[inline(always)]
    pub fn odt_en_f0(&self) -> OdtEnF0R {
        OdtEnF0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=1"]
    #[inline(always)]
    pub fn odt_en_f1(&self) -> OdtEnF1R {
        OdtEnF1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=2"]
    #[inline(always)]
    pub fn odt_en_f2(&self) -> OdtEnF2R {
        OdtEnF2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
    #[inline(always)]
    pub fn en_odt_assert_except_rd(&self) -> EnOdtAssertExceptRdR {
        EnOdtAssertExceptRdR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn odt_en_f0(&mut self) -> OdtEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec> {
        OdtEnF0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn odt_en_f1(&mut self) -> OdtEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec> {
        OdtEnF1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn odt_en_f2(&mut self) -> OdtEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec> {
        OdtEnF2W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn en_odt_assert_except_rd(
        &mut self,
    ) -> EnOdtAssertExceptRdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec> {
        EnOdtAssertExceptRdW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_373\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_373::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_373::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_373::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_373::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_373 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl373Spec {
    const RESET_VALUE: u32 = 0;
}
