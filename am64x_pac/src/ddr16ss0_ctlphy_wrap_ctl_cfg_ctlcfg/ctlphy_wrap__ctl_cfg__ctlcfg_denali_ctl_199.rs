#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_199` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_199` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec>;
#[doc = "Field `TFC_F2` reader - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=2"]
pub type TfcF2R = crate::FieldReader<u16>;
#[doc = "Field `TFC_F2` writer - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=2"]
pub type TfcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TCKFSPE_F2` reader - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=2"]
pub type TckfspeF2R = crate::FieldReader;
#[doc = "Field `TCKFSPE_F2` writer - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=2"]
pub type TckfspeF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKFSPX_F2` reader - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=2"]
pub type TckfspxF2R = crate::FieldReader;
#[doc = "Field `TCKFSPX_F2` writer - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=2"]
pub type TckfspxF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=2"]
    #[inline(always)]
    pub fn tfc_f2(&self) -> TfcF2R {
        TfcF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=2"]
    #[inline(always)]
    pub fn tckfspe_f2(&self) -> TckfspeF2R {
        TckfspeF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=2"]
    #[inline(always)]
    pub fn tckfspx_f2(&self) -> TckfspxF2R {
        TckfspxF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tfc_f2(&mut self) -> TfcF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec> {
        TfcF2W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckfspe_f2(&mut self) -> TckfspeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec> {
        TckfspeF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckfspx_f2(&mut self) -> TckfspxF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec> {
        TckfspxF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_199\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_199::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_199::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_199::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_199::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_199 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl199Spec {
    const RESET_VALUE: u32 = 0;
}
