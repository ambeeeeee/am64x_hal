#[doc = "Register `CFG_LOW` reader"]
pub type R = crate::R<CfgLowSpec>;
#[doc = "Register `CFG_LOW` writer"]
pub type W = crate::W<CfgLowSpec>;
#[doc = "Field `STS` reader - 31:0\\]
This is the raw status for config errors"]
pub type StsR = crate::FieldReader<u32>;
#[doc = "Field `STS` writer - 31:0\\]
This is the raw status for config errors"]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the raw status for config errors"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the raw status for config errors"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> StsW<CfgLowSpec> {
        StsW::new(self, 0)
    }
}
#[doc = "Shows which groups have oustanding low priority interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLowSpec;
impl crate::RegisterSpec for CfgLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_low::R`](R) reader structure"]
impl crate::Readable for CfgLowSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_low::W`](W) writer structure"]
impl crate::Writable for CfgLowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LOW to value 0"]
impl crate::Resettable for CfgLowSpec {
    const RESET_VALUE: u32 = 0;
}
