#[doc = "Register `CFG_TIMEOUT` reader"]
pub type R = crate::R<CfgTimeoutSpec>;
#[doc = "Register `CFG_TIMEOUT` writer"]
pub type W = crate::W<CfgTimeoutSpec>;
#[doc = "Field `TO` reader - 29:0\\]
The number of cycles in each eon."]
pub type ToR = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - 29:0\\]
The number of cycles in each eon."]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 29:0\\]
The number of cycles in each eon."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 29:0\\]
The number of cycles in each eon."]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> ToW<CfgTimeoutSpec> {
        ToW::new(self, 0)
    }
}
#[doc = "The Timeout Value Register contains the timeout value for scoreboarded transactions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTimeoutSpec;
impl crate::RegisterSpec for CfgTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_timeout::R`](R) reader structure"]
impl crate::Readable for CfgTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_timeout::W`](W) writer structure"]
impl crate::Writable for CfgTimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TIMEOUT to value 0"]
impl crate::Resettable for CfgTimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
