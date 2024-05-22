#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_62` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_62` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec>;
#[doc = "Field `TCKESR_F2` reader - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=2"]
pub type TckesrF2R = crate::FieldReader;
#[doc = "Field `TCKESR_F2` writer - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=2"]
pub type TckesrF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCCDMW_F2` reader - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=2"]
pub type TccdmwF2R = crate::FieldReader;
#[doc = "Field `TCCDMW_F2` writer - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=2"]
pub type TccdmwF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TPPD` reader - 18:16\\]
DRAM TPPD value in cycles."]
pub type TppdR = crate::FieldReader;
#[doc = "Field `TPPD` writer - 18:16\\]
DRAM TPPD value in cycles."]
pub type TppdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=2"]
    #[inline(always)]
    pub fn tckesr_f2(&self) -> TckesrF2R {
        TckesrF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=2"]
    #[inline(always)]
    pub fn tccdmw_f2(&self) -> TccdmwF2R {
        TccdmwF2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
DRAM TPPD value in cycles."]
    #[inline(always)]
    pub fn tppd(&self) -> TppdR {
        TppdR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckesr_f2(&mut self) -> TckesrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec> {
        TckesrF2W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tccdmw_f2(&mut self) -> TccdmwF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec> {
        TccdmwF2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
DRAM TPPD value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tppd(&mut self) -> TppdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec> {
        TppdW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_62::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_62::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_62 to value 0x0004_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl62Spec {
    const RESET_VALUE: u32 = 0x0004_0000;
}
