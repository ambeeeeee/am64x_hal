#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_248` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_248` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec>;
#[doc = "Field `MR8_DATA_1` reader - 7:0\\]
Data to program into memory mode register 8 for each chip select. READ-ONLY."]
pub type Mr8Data1R = crate::FieldReader;
#[doc = "Field `MR8_DATA_1` writer - 7:0\\]
Data to program into memory mode register 8 for each chip select. READ-ONLY."]
pub type Mr8Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR10_DATA_F0_0` reader - 24:8\\]
Data to program into memory mode register 10. FC=0"]
pub type Mr10DataF0_0R = crate::FieldReader<u32>;
#[doc = "Field `MR10_DATA_F0_0` writer - 24:8\\]
Data to program into memory mode register 10. FC=0"]
pub type Mr10DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 8 for each chip select. READ-ONLY."]
    #[inline(always)]
    pub fn mr8_data_1(&self) -> Mr8Data1R {
        Mr8Data1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:24 - 24:8\\]
Data to program into memory mode register 10. FC=0"]
    #[inline(always)]
    pub fn mr10_data_f0_0(&self) -> Mr10DataF0_0R {
        Mr10DataF0_0R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 8 for each chip select. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn mr8_data_1(&mut self) -> Mr8Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec> {
        Mr8Data1W::new(self, 0)
    }
    #[doc = "Bits 8:24 - 24:8\\]
Data to program into memory mode register 10. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn mr10_data_f0_0(&mut self) -> Mr10DataF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec> {
        Mr10DataF0_0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_248\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_248::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_248::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_248::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_248::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_248 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl248Spec {
    const RESET_VALUE: u32 = 0;
}
