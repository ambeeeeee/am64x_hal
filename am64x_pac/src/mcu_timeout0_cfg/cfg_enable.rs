#[doc = "Register `CFG_ENABLE` reader"]
pub type R = crate::R<CfgEnableSpec>;
#[doc = "Register `CFG_ENABLE` writer"]
pub type W = crate::W<CfgEnableSpec>;
#[doc = "Field `EN` reader - 3:0\\]
Enable. 0 - Disabled, All other values - Enabled."]
pub type EnR = crate::FieldReader;
#[doc = "Field `EN` writer - 3:0\\]
Enable. 0 - Disabled, All other values - Enabled."]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Enable. 0 - Disabled, All other values - Enabled."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Enable. 0 - Disabled, All other values - Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CfgEnableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "The Enable Register contains the gasket enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgEnableSpec;
impl crate::RegisterSpec for CfgEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_enable::R`](R) reader structure"]
impl crate::Readable for CfgEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_enable::W`](W) writer structure"]
impl crate::Writable for CfgEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ENABLE to value 0"]
impl crate::Resettable for CfgEnableSpec {
    const RESET_VALUE: u32 = 0;
}
