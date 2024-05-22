#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_100` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_100` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec>;
#[doc = "Field `PI_BIST_DATA_MASK` reader - 31:0\\]
Mask applied to data for BIST error checking. Bit \\[0\\]
controls memory data path bit \\[0\\], bit \\[1\\]
controls memory data path bit \\[1\\], etc. The mask range is the data transfer size in each memory clock cycle \\[The data on a rising edge and a failing edge\\]. Set each bit to 1 to mask."]
pub type PiBistDataMaskR = crate::FieldReader<u32>;
#[doc = "Field `PI_BIST_DATA_MASK` writer - 31:0\\]
Mask applied to data for BIST error checking. Bit \\[0\\]
controls memory data path bit \\[0\\], bit \\[1\\]
controls memory data path bit \\[1\\], etc. The mask range is the data transfer size in each memory clock cycle \\[The data on a rising edge and a failing edge\\]. Set each bit to 1 to mask."]
pub type PiBistDataMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Mask applied to data for BIST error checking. Bit \\[0\\]
controls memory data path bit \\[0\\], bit \\[1\\]
controls memory data path bit \\[1\\], etc. The mask range is the data transfer size in each memory clock cycle \\[The data on a rising edge and a failing edge\\]. Set each bit to 1 to mask."]
    #[inline(always)]
    pub fn pi_bist_data_mask(&self) -> PiBistDataMaskR {
        PiBistDataMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Mask applied to data for BIST error checking. Bit \\[0\\]
controls memory data path bit \\[0\\], bit \\[1\\]
controls memory data path bit \\[1\\], etc. The mask range is the data transfer size in each memory clock cycle \\[The data on a rising edge and a failing edge\\]. Set each bit to 1 to mask."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_data_mask(
        &mut self,
    ) -> PiBistDataMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec> {
        PiBistDataMaskW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_100::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_100::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_100::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_100::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_100 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi100Spec {
    const RESET_VALUE: u32 = 0;
}
