#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_STAT` reader"]
pub type R = crate::R<Mmr_Mmrvbp_McanssRegsMcanssStatSpec>;
#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_STAT` writer"]
pub type W = crate::W<Mmr_Mmrvbp_McanssRegsMcanssStatSpec>;
#[doc = "Field `MEM_INIT_DONE` reader - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
pub type MemInitDoneR = crate::BitReader;
#[doc = "Field `MEM_INIT_DONE` writer - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
pub type MemInitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_FDOE` reader - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
pub type EnableFdoeR = crate::BitReader;
#[doc = "Field `ENABLE_FDOE` writer - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
pub type EnableFdoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
    #[inline(always)]
    pub fn mem_init_done(&self) -> MemInitDoneR {
        MemInitDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
    #[inline(always)]
    pub fn enable_fdoe(&self) -> EnableFdoeR {
        EnableFdoeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_done(&mut self) -> MemInitDoneW<Mmr_Mmrvbp_McanssRegsMcanssStatSpec> {
        MemInitDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
    #[inline(always)]
    #[must_use]
    pub fn enable_fdoe(&mut self) -> EnableFdoeW<Mmr_Mmrvbp_McanssRegsMcanssStatSpec> {
        EnableFdoeW::new(self, 2)
    }
}
#[doc = "The Status register provide general status bits for the MCANSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Mmrvbp_McanssRegsMcanssStatSpec;
impl crate::RegisterSpec for Mmr_Mmrvbp_McanssRegsMcanssStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__mmrvbp__mcanss_regs_mcanss_stat::R`](R) reader structure"]
impl crate::Readable for Mmr_Mmrvbp_McanssRegsMcanssStatSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__mmrvbp__mcanss_regs_mcanss_stat::W`](W) writer structure"]
impl crate::Writable for Mmr_Mmrvbp_McanssRegsMcanssStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__MMRVBP__MCANSS_REGS_MCANSS_STAT to value 0"]
impl crate::Resettable for Mmr_Mmrvbp_McanssRegsMcanssStatSpec {
    const RESET_VALUE: u32 = 0;
}
