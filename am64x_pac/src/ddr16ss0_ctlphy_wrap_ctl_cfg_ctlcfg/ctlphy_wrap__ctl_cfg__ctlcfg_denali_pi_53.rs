#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_53` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_53` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec>;
#[doc = "Field `PI_TDFI_RDDATA_EN` reader - 7:0\\]
Holds the calculated DFI tRDDATA_EN timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_en assertion. READ-ONLY"]
pub type PiTdfiRddataEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_RDDATA_EN` writer - 7:0\\]
Holds the calculated DFI tRDDATA_EN timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_en assertion. READ-ONLY"]
pub type PiTdfiRddataEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_PHY_WRLAT` reader - 15:8\\]
Holds the calculated DFI tPHY_WRLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_en assertion. READ-ONLY"]
pub type PiTdfiPhyWrlatR = crate::FieldReader;
#[doc = "Field `PI_TDFI_PHY_WRLAT` writer - 15:8\\]
Holds the calculated DFI tPHY_WRLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_en assertion. READ-ONLY"]
pub type PiTdfiPhyWrlatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_CALVL_REQ` reader - 16:16\\]
User request to initiate CA training. Set to 1 to trigger. WRITE-ONLY"]
pub type PiCalvlReqR = crate::BitReader;
#[doc = "Field `PI_CALVL_REQ` writer - 16:16\\]
User request to initiate CA training. Set to 1 to trigger. WRITE-ONLY"]
pub type PiCalvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_CS_SW` reader - 25:24\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
pub type PiCalvlCsSwR = crate::FieldReader;
#[doc = "Field `PI_CALVL_CS_SW` writer - 25:24\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
pub type PiCalvlCsSwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Holds the calculated DFI tRDDATA_EN timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_en assertion. READ-ONLY"]
    #[inline(always)]
    pub fn pi_tdfi_rddata_en(&self) -> PiTdfiRddataEnR {
        PiTdfiRddataEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Holds the calculated DFI tPHY_WRLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_en assertion. READ-ONLY"]
    #[inline(always)]
    pub fn pi_tdfi_phy_wrlat(&self) -> PiTdfiPhyWrlatR {
        PiTdfiPhyWrlatR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
User request to initiate CA training. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_calvl_req(&self) -> PiCalvlReqR {
        PiCalvlReqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
    #[inline(always)]
    pub fn pi_calvl_cs_sw(&self) -> PiCalvlCsSwR {
        PiCalvlCsSwR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Holds the calculated DFI tRDDATA_EN timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_en assertion. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rddata_en(&mut self) -> PiTdfiRddataEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec> {
        PiTdfiRddataEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Holds the calculated DFI tPHY_WRLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_en assertion. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phy_wrlat(&mut self) -> PiTdfiPhyWrlatW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec> {
        PiTdfiPhyWrlatW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
User request to initiate CA training. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_req(&mut self) -> PiCalvlReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec> {
        PiCalvlReqW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_cs_sw(&mut self) -> PiCalvlCsSwW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec> {
        PiCalvlCsSwW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_53::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_53::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_53 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi53Spec {
    const RESET_VALUE: u32 = 0;
}
