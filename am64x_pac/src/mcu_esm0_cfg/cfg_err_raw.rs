#[doc = "Register `CFG_ERR_RAW` reader"]
pub type R = crate::R<CfgErrRawSpec>;
#[doc = "Register `CFG_ERR_RAW` writer"]
pub type W = crate::W<CfgErrRawSpec>;
#[doc = "Field `STS` reader - 2:0\\]
This is the raw status for config errors"]
pub type StsR = crate::FieldReader;
#[doc = "Field `STS` writer - 2:0\\]
This is the raw status for config errors"]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This is the raw status for config errors"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This is the raw status for config errors"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> StsW<CfgErrRawSpec> {
        StsW::new(self, 0)
    }
}
#[doc = "Raw Status/Set Register for Configuration Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrRawSpec;
impl crate::RegisterSpec for CfgErrRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_raw::R`](R) reader structure"]
impl crate::Readable for CfgErrRawSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_raw::W`](W) writer structure"]
impl crate::Writable for CfgErrRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_RAW to value 0"]
impl crate::Resettable for CfgErrRawSpec {
    const RESET_VALUE: u32 = 0;
}
