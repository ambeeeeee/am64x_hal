#[doc = "Register `CFG_ERR_TAG` reader"]
pub type R = crate::R<CfgErrTagSpec>;
#[doc = "Register `CFG_ERR_TAG` writer"]
pub type W = crate::W<CfgErrTagSpec>;
#[doc = "Field `CID` reader - 11:0\\]
Command ID Indicator"]
pub type CidR = crate::FieldReader<u16>;
#[doc = "Field `CID` writer - 11:0\\]
Command ID Indicator"]
pub type CidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TAG` reader - 27:16\\]
Command Tag Indicator"]
pub type TagR = crate::FieldReader<u16>;
#[doc = "Field `TAG` writer - 27:16\\]
Command Tag Indicator"]
pub type TagW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Command ID Indicator"]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Command Tag Indicator"]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Command ID Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn cid(&mut self) -> CidW<CfgErrTagSpec> {
        CidW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Command Tag Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<CfgErrTagSpec> {
        TagW::new(self, 16)
    }
}
#[doc = "This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrTagSpec;
impl crate::RegisterSpec for CfgErrTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_tag::R`](R) reader structure"]
impl crate::Readable for CfgErrTagSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_tag::W`](W) writer structure"]
impl crate::Writable for CfgErrTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_TAG to value 0"]
impl crate::Resettable for CfgErrTagSpec {
    const RESET_VALUE: u32 = 0;
}
