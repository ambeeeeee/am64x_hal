#[doc = "Register `CFG_TOCR` reader"]
pub type R = crate::R<CfgTocrSpec>;
#[doc = "Register `CFG_TOCR` writer"]
pub type W = crate::W<CfgTocrSpec>;
#[doc = "Field `OVF_COUNTER_VALUE` reader - 23:0\\]
The number of overflow events"]
pub type OvfCounterValueR = crate::FieldReader<u32>;
#[doc = "Field `OVF_COUNTER_VALUE` writer - 23:0\\]
The number of overflow events"]
pub type OvfCounterValueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
The number of overflow events"]
    #[inline(always)]
    pub fn ovf_counter_value(&self) -> OvfCounterValueR {
        OvfCounterValueR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
The number of overflow events"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_counter_value(&mut self) -> OvfCounterValueW<CfgTocrSpec> {
        OvfCounterValueW::new(self, 0)
    }
}
#[doc = "This register is used to mask the tick interrupt for a selected number of ticks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTocrSpec;
impl crate::RegisterSpec for CfgTocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tocr::R`](R) reader structure"]
impl crate::Readable for CfgTocrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tocr::W`](W) writer structure"]
impl crate::Writable for CfgTocrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TOCR to value 0"]
impl crate::Resettable for CfgTocrSpec {
    const RESET_VALUE: u32 = 0;
}
