#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_77` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_77` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec>;
#[doc = "Field `PI_DBILVL_RESP_MASK` reader - 1:0\\]
Mask for the dfi_rdlvl_resp signal during read dbi training."]
pub type PiDbilvlRespMaskR = crate::FieldReader;
#[doc = "Field `PI_DBILVL_RESP_MASK` writer - 1:0\\]
Mask for the dfi_rdlvl_resp signal during read dbi training."]
pub type PiDbilvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_BANK_DIFF` reader - 9:8\\]
Difference between number of bank pins available and number being used."]
pub type PiBankDiffR = crate::FieldReader;
#[doc = "Field `PI_BANK_DIFF` writer - 9:8\\]
Difference between number of bank pins available and number being used."]
pub type PiBankDiffW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ROW_DIFF` reader - 18:16\\]
Difference between number of address pins available and number being used."]
pub type PiRowDiffR = crate::FieldReader;
#[doc = "Field `PI_ROW_DIFF` writer - 18:16\\]
Difference between number of address pins available and number being used."]
pub type PiRowDiffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_TCCD` reader - 28:24\\]
DRAM CAS-to-CAS value in cycles."]
pub type PiTccdR = crate::FieldReader;
#[doc = "Field `PI_TCCD` writer - 28:24\\]
DRAM CAS-to-CAS value in cycles."]
pub type PiTccdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Mask for the dfi_rdlvl_resp signal during read dbi training."]
    #[inline(always)]
    pub fn pi_dbilvl_resp_mask(&self) -> PiDbilvlRespMaskR {
        PiDbilvlRespMaskR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Difference between number of bank pins available and number being used."]
    #[inline(always)]
    pub fn pi_bank_diff(&self) -> PiBankDiffR {
        PiBankDiffR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Difference between number of address pins available and number being used."]
    #[inline(always)]
    pub fn pi_row_diff(&self) -> PiRowDiffR {
        PiRowDiffR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    pub fn pi_tccd(&self) -> PiTccdR {
        PiTccdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Mask for the dfi_rdlvl_resp signal during read dbi training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dbilvl_resp_mask(
        &mut self,
    ) -> PiDbilvlRespMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec> {
        PiDbilvlRespMaskW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Difference between number of bank pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bank_diff(&mut self) -> PiBankDiffW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec> {
        PiBankDiffW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Difference between number of address pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn pi_row_diff(&mut self) -> PiRowDiffW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec> {
        PiRowDiffW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccd(&mut self) -> PiTccdW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec> {
        PiTccdW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_77::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_77::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_77::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_77::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_77 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi77Spec {
    const RESET_VALUE: u32 = 0;
}
