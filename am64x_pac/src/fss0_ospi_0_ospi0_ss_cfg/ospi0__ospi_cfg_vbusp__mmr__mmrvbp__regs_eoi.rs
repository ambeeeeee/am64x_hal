#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_EOI` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_EOI` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec>;
#[doc = "Field `EOI` reader - 7:0\\]
Write with bit position of targetted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
pub type EoiR = crate::FieldReader;
#[doc = "Field `EOI` writer - 7:0\\]
Write with bit position of targetted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
pub type EoiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write with bit position of targetted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
    #[inline(always)]
    pub fn eoi(&self) -> EoiR {
        EoiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write with bit position of targetted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eoi(&mut self) -> EoiW<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec> {
        EoiW::new(self, 0)
    }
}
#[doc = "End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec;
impl crate::RegisterSpec for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_EOI to value 0"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec {
    const RESET_VALUE: u32 = 0;
}
