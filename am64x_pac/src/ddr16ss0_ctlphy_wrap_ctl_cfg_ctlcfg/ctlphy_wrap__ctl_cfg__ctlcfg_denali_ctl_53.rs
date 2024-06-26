#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_53` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_53` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec>;
#[doc = "Field `TRP_F2` reader - 7:0\\]
DRAM TRP value in cycles. FC=2"]
pub type TrpF2R = crate::FieldReader;
#[doc = "Field `TRP_F2` writer - 7:0\\]
DRAM TRP value in cycles. FC=2"]
pub type TrpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFAW_F2` reader - 16:8\\]
DRAM TFAW value in cycles. FC=2"]
pub type TfawF2R = crate::FieldReader<u16>;
#[doc = "Field `TFAW_F2` writer - 16:8\\]
DRAM TFAW value in cycles. FC=2"]
pub type TfawF2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TRTP_F0` reader - 31:24\\]
DRAM TRTP value in cycles. FC=0"]
pub type TrtpF0R = crate::FieldReader;
#[doc = "Field `TRTP_F0` writer - 31:24\\]
DRAM TRTP value in cycles. FC=0"]
pub type TrtpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP value in cycles. FC=2"]
    #[inline(always)]
    pub fn trp_f2(&self) -> TrpF2R {
        TrpF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TFAW value in cycles. FC=2"]
    #[inline(always)]
    pub fn tfaw_f2(&self) -> TfawF2R {
        TfawF2R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRTP value in cycles. FC=0"]
    #[inline(always)]
    pub fn trtp_f0(&self) -> TrtpF0R {
        TrtpF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trp_f2(&mut self) -> TrpF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec> {
        TrpF2W::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TFAW value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tfaw_f2(&mut self) -> TfawF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec> {
        TfawF2W::new(self, 8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRTP value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trtp_f0(&mut self) -> TrtpF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec> {
        TrtpF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_53::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_53::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_53 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl53Spec {
    const RESET_VALUE: u32 = 0;
}
