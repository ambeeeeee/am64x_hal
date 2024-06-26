#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_47` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_47` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec>;
#[doc = "Field `TRP_F0` reader - 7:0\\]
DRAM TRP value in cycles. FC=0"]
pub type TrpF0R = crate::FieldReader;
#[doc = "Field `TRP_F0` writer - 7:0\\]
DRAM TRP value in cycles. FC=0"]
pub type TrpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFAW_F0` reader - 16:8\\]
DRAM TFAW value in cycles. FC=0"]
pub type TfawF0R = crate::FieldReader<u16>;
#[doc = "Field `TFAW_F0` writer - 16:8\\]
DRAM TFAW value in cycles. FC=0"]
pub type TfawF0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TCCD_L_F1` reader - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=1"]
pub type TccdLF1R = crate::FieldReader;
#[doc = "Field `TCCD_L_F1` writer - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=1"]
pub type TccdLF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP value in cycles. FC=0"]
    #[inline(always)]
    pub fn trp_f0(&self) -> TrpF0R {
        TrpF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TFAW value in cycles. FC=0"]
    #[inline(always)]
    pub fn tfaw_f0(&self) -> TfawF0R {
        TfawF0R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=1"]
    #[inline(always)]
    pub fn tccd_l_f1(&self) -> TccdLF1R {
        TccdLF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trp_f0(&mut self) -> TrpF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec> {
        TrpF0W::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TFAW value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tfaw_f0(&mut self) -> TfawF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec> {
        TfawF0W::new(self, 8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM CAS-to-CAS value within the same bank group in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tccd_l_f1(&mut self) -> TccdLF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec> {
        TccdLF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_47::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_47::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_47 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl47Spec {
    const RESET_VALUE: u32 = 0;
}
