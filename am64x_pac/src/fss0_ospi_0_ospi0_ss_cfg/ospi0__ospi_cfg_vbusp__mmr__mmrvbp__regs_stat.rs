#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_STAT` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_STAT` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec>;
#[doc = "Field `MEM_INIT_DONE` reader - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
pub type MemInitDoneR = crate::BitReader;
#[doc = "Field `MEM_INIT_DONE` writer - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
pub type MemInitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
    #[inline(always)]
    pub fn mem_init_done(&self) -> MemInitDoneR {
        MemInitDoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_done(&mut self) -> MemInitDoneW<Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec> {
        MemInitDoneW::new(self, 1)
    }
}
#[doc = "The Status register provide general status bits for the ospi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec;
impl crate::RegisterSpec for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_STAT to value 0"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec {
    const RESET_VALUE: u32 = 0;
}
