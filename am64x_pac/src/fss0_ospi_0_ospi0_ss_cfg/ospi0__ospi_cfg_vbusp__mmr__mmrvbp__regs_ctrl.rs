#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_CTRL` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_CTRL` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec>;
#[doc = "Field `PIPELINE_MODE_FLUSH` reader - 3:3\\]
1 - Flush Cadence Flash Controller FIFO by forcin gAHB SEL low. 0 - AHB Sel to Cadence Controller is 1"]
pub type PipelineModeFlushR = crate::BitReader;
#[doc = "Field `PIPELINE_MODE_FLUSH` writer - 3:3\\]
1 - Flush Cadence Flash Controller FIFO by forcin gAHB SEL low. 0 - AHB Sel to Cadence Controller is 1"]
pub type PipelineModeFlushW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 3:3\\]
1 - Flush Cadence Flash Controller FIFO by forcin gAHB SEL low. 0 - AHB Sel to Cadence Controller is 1"]
    #[inline(always)]
    pub fn pipeline_mode_flush(&self) -> PipelineModeFlushR {
        PipelineModeFlushR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
1 - Flush Cadence Flash Controller FIFO by forcin gAHB SEL low. 0 - AHB Sel to Cadence Controller is 1"]
    #[inline(always)]
    #[must_use]
    pub fn pipeline_mode_flush(
        &mut self,
    ) -> PipelineModeFlushW<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec> {
        PipelineModeFlushW::new(self, 3)
    }
}
#[doc = "The Control Register contains general control bits for the ospi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec;
impl crate::RegisterSpec for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_CTRL to value 0"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
