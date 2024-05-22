#[doc = "Register `CFG_RAW` reader"]
pub type R = crate::R<CfgRawSpec>;
#[doc = "Register `CFG_RAW` writer"]
pub type W = crate::W<CfgRawSpec>;
#[doc = "Field `STS` reader - 31:0\\]
This is the raw status/set for errors Group A"]
pub type StsR = crate::FieldReader<u32>;
#[doc = "Field `STS` writer - 31:0\\]
This is the raw status/set for errors Group A"]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the raw status/set for errors Group A"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the raw status/set for errors Group A"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> StsW<CfgRawSpec> {
        StsW::new(self, 0)
    }
}
#[doc = "Raw Status/Set Register for Group A Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRawSpec;
impl crate::RegisterSpec for CfgRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_raw::R`](R) reader structure"]
impl crate::Readable for CfgRawSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_raw::W`](W) writer structure"]
impl crate::Writable for CfgRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_RAW to value 0"]
impl crate::Resettable for CfgRawSpec {
    const RESET_VALUE: u32 = 0;
}
