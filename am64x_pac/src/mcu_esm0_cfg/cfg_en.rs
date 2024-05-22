#[doc = "Register `CFG_EN` reader"]
pub type R = crate::R<CfgEnSpec>;
#[doc = "Register `CFG_EN` writer"]
pub type W = crate::W<CfgEnSpec>;
#[doc = "Field `KEY` reader - 3:0\\]
Global Enable"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 3:0\\]
Global Enable"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Global Enable"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Global Enable"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgEnSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "The Global Enable Register has the master interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgEnSpec;
impl crate::RegisterSpec for CfgEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_en::R`](R) reader structure"]
impl crate::Readable for CfgEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_en::W`](W) writer structure"]
impl crate::Writable for CfgEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_EN to value 0"]
impl crate::Resettable for CfgEnSpec {
    const RESET_VALUE: u32 = 0;
}
