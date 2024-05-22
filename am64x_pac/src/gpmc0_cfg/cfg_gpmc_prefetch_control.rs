#[doc = "Register `CFG_GPMC_PREFETCH_CONTROL` reader"]
pub type R = crate::R<CfgGpmcPrefetchControlSpec>;
#[doc = "Register `CFG_GPMC_PREFETCH_CONTROL` writer"]
pub type W = crate::W<CfgGpmcPrefetchControlSpec>;
#[doc = "Field `STARTENGINE` reader - 0:0\\]
Resets the FIFO pointer and starts the engine"]
pub type StartengineR = crate::BitReader;
#[doc = "Field `STARTENGINE` writer - 0:0\\]
Resets the FIFO pointer and starts the engine"]
pub type StartengineW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Resets the FIFO pointer and starts the engine"]
    #[inline(always)]
    pub fn startengine(&self) -> StartengineR {
        StartengineR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Resets the FIFO pointer and starts the engine"]
    #[inline(always)]
    #[must_use]
    pub fn startengine(&mut self) -> StartengineW<CfgGpmcPrefetchControlSpec> {
        StartengineW::new(self, 0)
    }
}
#[doc = "Prefetch engine control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcPrefetchControlSpec;
impl crate::RegisterSpec for CfgGpmcPrefetchControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_prefetch_control::R`](R) reader structure"]
impl crate::Readable for CfgGpmcPrefetchControlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_prefetch_control::W`](W) writer structure"]
impl crate::Writable for CfgGpmcPrefetchControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_PREFETCH_CONTROL to value 0"]
impl crate::Resettable for CfgGpmcPrefetchControlSpec {
    const RESET_VALUE: u32 = 0;
}
