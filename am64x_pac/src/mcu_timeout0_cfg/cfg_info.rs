#[doc = "Register `CFG_INFO` reader"]
pub type R = crate::R<CfgInfoSpec>;
#[doc = "Register `CFG_INFO` writer"]
pub type W = crate::W<CfgInfoSpec>;
#[doc = "Field `CUR_READS` reader - 8:0\\]
Total Number of slots in the read scoreboard"]
pub type CurReadsR = crate::FieldReader<u16>;
#[doc = "Field `CUR_READS` writer - 8:0\\]
Total Number of slots in the read scoreboard"]
pub type CurReadsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CUR_WRITES` reader - 24:16\\]
Total Number of slots in the write scoreboard"]
pub type CurWritesR = crate::FieldReader<u16>;
#[doc = "Field `CUR_WRITES` writer - 24:16\\]
Total Number of slots in the write scoreboard"]
pub type CurWritesW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Total Number of slots in the read scoreboard"]
    #[inline(always)]
    pub fn cur_reads(&self) -> CurReadsR {
        CurReadsR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Total Number of slots in the write scoreboard"]
    #[inline(always)]
    pub fn cur_writes(&self) -> CurWritesR {
        CurWritesR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Total Number of slots in the read scoreboard"]
    #[inline(always)]
    #[must_use]
    pub fn cur_reads(&mut self) -> CurReadsW<CfgInfoSpec> {
        CurReadsW::new(self, 0)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Total Number of slots in the write scoreboard"]
    #[inline(always)]
    #[must_use]
    pub fn cur_writes(&mut self) -> CurWritesW<CfgInfoSpec> {
        CurWritesW::new(self, 16)
    }
}
#[doc = "The Info Register contains information about the current state of the gasket.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgInfoSpec;
impl crate::RegisterSpec for CfgInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_info::R`](R) reader structure"]
impl crate::Readable for CfgInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_info::W`](W) writer structure"]
impl crate::Writable for CfgInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_INFO to value 0"]
impl crate::Resettable for CfgInfoSpec {
    const RESET_VALUE: u32 = 0;
}
