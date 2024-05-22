#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEccR2EndAddrRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEccR2EndAddrRegSpec>;
#[doc = "Field `ECC_END_ADR_2` reader - 16:0\\]
End caddress\\[31:16\\]
for ECC range 2. Setting the start address greater than the end address disables the range. The range is inclusive of the start and end addresses. This field must be set and kept static before using DDR."]
pub type EccEndAdr2R = crate::FieldReader<u32>;
#[doc = "Field `ECC_END_ADR_2` writer - 16:0\\]
End caddress\\[31:16\\]
for ECC range 2. Setting the start address greater than the end address disables the range. The range is inclusive of the start and end addresses. This field must be set and kept static before using DDR."]
pub type EccEndAdr2W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
End caddress\\[31:16\\]
for ECC range 2. Setting the start address greater than the end address disables the range. The range is inclusive of the start and end addresses. This field must be set and kept static before using DDR."]
    #[inline(always)]
    pub fn ecc_end_adr_2(&self) -> EccEndAdr2R {
        EccEndAdr2R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
End caddress\\[31:16\\]
for ECC range 2. Setting the start address greater than the end address disables the range. The range is inclusive of the start and end addresses. This field must be set and kept static before using DDR."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_end_adr_2(&mut self) -> EccEndAdr2W<Regs_SsCfg_SscfgEccR2EndAddrRegSpec> {
        EccEndAdr2W::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_r2_end_addr_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_r2_end_addr_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEccR2EndAddrRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEccR2EndAddrRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_r2_end_addr_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEccR2EndAddrRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_r2_end_addr_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEccR2EndAddrRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEccR2EndAddrRegSpec {
    const RESET_VALUE: u32 = 0;
}
