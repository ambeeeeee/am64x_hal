#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_312` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_312` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec>;
#[doc = "Field `TZQCAL_F2` reader - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=2"]
pub type TzqcalF2R = crate::FieldReader<u16>;
#[doc = "Field `TZQCAL_F2` writer - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=2"]
pub type TzqcalF2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TZQLAT_F2` reader - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=2"]
pub type TzqlatF2R = crate::FieldReader;
#[doc = "Field `TZQLAT_F2` writer - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=2"]
pub type TzqlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ZQ_SW_REQ_START_LATCH_MAP` reader - 25:24\\]
Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
pub type ZqSwReqStartLatchMapR = crate::FieldReader;
#[doc = "Field `ZQ_SW_REQ_START_LATCH_MAP` writer - 25:24\\]
Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
pub type ZqSwReqStartLatchMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=2"]
    #[inline(always)]
    pub fn tzqcal_f2(&self) -> TzqcalF2R {
        TzqcalF2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=2"]
    #[inline(always)]
    pub fn tzqlat_f2(&self) -> TzqlatF2R {
        TzqlatF2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
    #[inline(always)]
    pub fn zq_sw_req_start_latch_map(&self) -> ZqSwReqStartLatchMapR {
        ZqSwReqStartLatchMapR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tzqcal_f2(&mut self) -> TzqcalF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec> {
        TzqcalF2W::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tzqlat_f2(&mut self) -> TzqlatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec> {
        TzqlatF2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
    #[inline(always)]
    #[must_use]
    pub fn zq_sw_req_start_latch_map(
        &mut self,
    ) -> ZqSwReqStartLatchMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec> {
        ZqSwReqStartLatchMapW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_312\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_312::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_312::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_312::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_312::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_312 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl312Spec {
    const RESET_VALUE: u32 = 0;
}
