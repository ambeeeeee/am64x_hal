#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_67` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_67` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec>;
#[doc = "Field `TRP_AB_F0_0` reader - 7:0\\]
DRAM TRP all bank value in cycles. FC=0"]
pub type TrpAbF0_0R = crate::FieldReader;
#[doc = "Field `TRP_AB_F0_0` writer - 7:0\\]
DRAM TRP all bank value in cycles. FC=0"]
pub type TrpAbF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRP_AB_F1_0` reader - 15:8\\]
DRAM TRP all bank value in cycles. FC=1"]
pub type TrpAbF1_0R = crate::FieldReader;
#[doc = "Field `TRP_AB_F1_0` writer - 15:8\\]
DRAM TRP all bank value in cycles. FC=1"]
pub type TrpAbF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRP_AB_F2_0` reader - 23:16\\]
DRAM TRP all bank value in cycles. FC=2"]
pub type TrpAbF2_0R = crate::FieldReader;
#[doc = "Field `TRP_AB_F2_0` writer - 23:16\\]
DRAM TRP all bank value in cycles. FC=2"]
pub type TrpAbF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRP_AB_F0_1` reader - 31:24\\]
DRAM TRP all bank value in cycles. FC=0"]
pub type TrpAbF0_1R = crate::FieldReader;
#[doc = "Field `TRP_AB_F0_1` writer - 31:24\\]
DRAM TRP all bank value in cycles. FC=0"]
pub type TrpAbF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP all bank value in cycles. FC=0"]
    #[inline(always)]
    pub fn trp_ab_f0_0(&self) -> TrpAbF0_0R {
        TrpAbF0_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TRP all bank value in cycles. FC=1"]
    #[inline(always)]
    pub fn trp_ab_f1_0(&self) -> TrpAbF1_0R {
        TrpAbF1_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRP all bank value in cycles. FC=2"]
    #[inline(always)]
    pub fn trp_ab_f2_0(&self) -> TrpAbF2_0R {
        TrpAbF2_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRP all bank value in cycles. FC=0"]
    #[inline(always)]
    pub fn trp_ab_f0_1(&self) -> TrpAbF0_1R {
        TrpAbF0_1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP all bank value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f0_0(&mut self) -> TrpAbF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec> {
        TrpAbF0_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TRP all bank value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f1_0(&mut self) -> TrpAbF1_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec> {
        TrpAbF1_0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRP all bank value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f2_0(&mut self) -> TrpAbF2_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec> {
        TrpAbF2_0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRP all bank value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f0_1(&mut self) -> TrpAbF0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec> {
        TrpAbF0_1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_67::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_67::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_67 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl67Spec {
    const RESET_VALUE: u32 = 0;
}
