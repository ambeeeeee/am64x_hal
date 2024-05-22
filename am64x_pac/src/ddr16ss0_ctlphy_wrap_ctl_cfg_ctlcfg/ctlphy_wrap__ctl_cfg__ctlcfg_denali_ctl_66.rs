#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_66` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_66` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec>;
#[doc = "Field `TDAL_F0` reader - 7:0\\]
DRAM TDAL value in cycles. FC=0"]
pub type TdalF0R = crate::FieldReader;
#[doc = "Field `TDAL_F0` writer - 7:0\\]
DRAM TDAL value in cycles. FC=0"]
pub type TdalF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDAL_F1` reader - 15:8\\]
DRAM TDAL value in cycles. FC=1"]
pub type TdalF1R = crate::FieldReader;
#[doc = "Field `TDAL_F1` writer - 15:8\\]
DRAM TDAL value in cycles. FC=1"]
pub type TdalF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDAL_F2` reader - 23:16\\]
DRAM TDAL value in cycles. FC=2"]
pub type TdalF2R = crate::FieldReader;
#[doc = "Field `TDAL_F2` writer - 23:16\\]
DRAM TDAL value in cycles. FC=2"]
pub type TdalF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BSTLEN` reader - 29:24\\]
Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, program to 3 for BL8, program to 4 for BL16, or program to 5 for BL32. All other settings are reserved."]
pub type BstlenR = crate::FieldReader;
#[doc = "Field `BSTLEN` writer - 29:24\\]
Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, program to 3 for BL8, program to 4 for BL16, or program to 5 for BL32. All other settings are reserved."]
pub type BstlenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TDAL value in cycles. FC=0"]
    #[inline(always)]
    pub fn tdal_f0(&self) -> TdalF0R {
        TdalF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TDAL value in cycles. FC=1"]
    #[inline(always)]
    pub fn tdal_f1(&self) -> TdalF1R {
        TdalF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TDAL value in cycles. FC=2"]
    #[inline(always)]
    pub fn tdal_f2(&self) -> TdalF2R {
        TdalF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, program to 3 for BL8, program to 4 for BL16, or program to 5 for BL32. All other settings are reserved."]
    #[inline(always)]
    pub fn bstlen(&self) -> BstlenR {
        BstlenR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TDAL value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdal_f0(&mut self) -> TdalF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec> {
        TdalF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TDAL value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdal_f1(&mut self) -> TdalF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec> {
        TdalF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TDAL value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdal_f2(&mut self) -> TdalF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec> {
        TdalF2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, program to 3 for BL8, program to 4 for BL16, or program to 5 for BL32. All other settings are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bstlen(&mut self) -> BstlenW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec> {
        BstlenW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_66::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_66::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_66::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_66::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_66 to value 0x0200_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl66Spec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
