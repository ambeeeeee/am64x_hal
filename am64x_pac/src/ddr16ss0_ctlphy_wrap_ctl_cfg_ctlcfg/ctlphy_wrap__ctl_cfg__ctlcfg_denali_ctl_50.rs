#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_50` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_50` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec>;
#[doc = "Field `TRP_F1` reader - 7:0\\]
DRAM TRP value in cycles. FC=1"]
pub type TrpF1R = crate::FieldReader;
#[doc = "Field `TRP_F1` writer - 7:0\\]
DRAM TRP value in cycles. FC=1"]
pub type TrpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFAW_F1` reader - 16:8\\]
DRAM TFAW value in cycles. FC=1"]
pub type TfawF1R = crate::FieldReader<u16>;
#[doc = "Field `TFAW_F1` writer - 16:8\\]
DRAM TFAW value in cycles. FC=1"]
pub type TfawF1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TCCD_L_F2` reader - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=2"]
pub type TccdLF2R = crate::FieldReader;
#[doc = "Field `TCCD_L_F2` writer - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=2"]
pub type TccdLF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP value in cycles. FC=1"]
    #[inline(always)]
    pub fn trp_f1(&self) -> TrpF1R {
        TrpF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TFAW value in cycles. FC=1"]
    #[inline(always)]
    pub fn tfaw_f1(&self) -> TfawF1R {
        TfawF1R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=2"]
    #[inline(always)]
    pub fn tccd_l_f2(&self) -> TccdLF2R {
        TccdLF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn trp_f1(&mut self) -> TrpF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec> {
        TrpF1W::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TFAW value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tfaw_f1(&mut self) -> TfawF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec> {
        TfawF1W::new(self, 8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tccd_l_f2(&mut self) -> TccdLF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec> {
        TccdLF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_50::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_50::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_50 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl50Spec {
    const RESET_VALUE: u32 = 0;
}
