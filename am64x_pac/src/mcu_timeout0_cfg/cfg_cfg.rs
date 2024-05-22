#[doc = "Register `CFG_CFG` reader"]
pub type R = crate::R<CfgCfgSpec>;
#[doc = "Register `CFG_CFG` writer"]
pub type W = crate::W<CfgCfgSpec>;
#[doc = "Field `NUM_READS` reader - 7:0\\]
Total Number of slots in the read scoreboard"]
pub type NumReadsR = crate::FieldReader;
#[doc = "Field `NUM_READS` writer - 7:0\\]
Total Number of slots in the read scoreboard"]
pub type NumReadsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUM_WRITES` reader - 23:16\\]
Total Number of slots in the write scoreboard"]
pub type NumWritesR = crate::FieldReader;
#[doc = "Field `NUM_WRITES` writer - 23:16\\]
Total Number of slots in the write scoreboard"]
pub type NumWritesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Total Number of slots in the read scoreboard"]
    #[inline(always)]
    pub fn num_reads(&self) -> NumReadsR {
        NumReadsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Total Number of slots in the write scoreboard"]
    #[inline(always)]
    pub fn num_writes(&self) -> NumWritesR {
        NumWritesR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Total Number of slots in the read scoreboard"]
    #[inline(always)]
    #[must_use]
    pub fn num_reads(&mut self) -> NumReadsW<CfgCfgSpec> {
        NumReadsW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Total Number of slots in the write scoreboard"]
    #[inline(always)]
    #[must_use]
    pub fn num_writes(&mut self) -> NumWritesW<CfgCfgSpec> {
        NumWritesW::new(self, 16)
    }
}
#[doc = "The Configuration Register contains information about the configuration of the gasket.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCfgSpec;
impl crate::RegisterSpec for CfgCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cfg::R`](R) reader structure"]
impl crate::Readable for CfgCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cfg::W`](W) writer structure"]
impl crate::Writable for CfgCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CFG to value 0x0009_0015"]
impl crate::Resettable for CfgCfgSpec {
    const RESET_VALUE: u32 = 0x0009_0015;
}
