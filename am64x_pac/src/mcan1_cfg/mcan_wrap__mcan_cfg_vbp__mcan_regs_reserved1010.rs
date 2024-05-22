#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1010` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsReserved1010Spec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1010` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsReserved1010Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reserved field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsReserved1010Spec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsReserved1010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsReserved1010Spec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_reserved1010::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsReserved1010Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_reserved1010 to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsReserved1010Spec {
    const RESET_VALUE: u32 = 0;
}
