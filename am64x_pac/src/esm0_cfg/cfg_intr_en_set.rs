#[doc = "Register `CFG_INTR_EN_SET` reader"]
pub type R = crate::R<CfgIntrEnSetSpec>;
#[doc = "Register `CFG_INTR_EN_SET` writer"]
pub type W = crate::W<CfgIntrEnSetSpec>;
#[doc = "Field `MSK` reader - 31:0\\]
This is the mask enable set for errors in Group A"]
pub type MskR = crate::FieldReader<u32>;
#[doc = "Field `MSK` writer - 31:0\\]
This is the mask enable set for errors in Group A"]
pub type MskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the mask enable set for errors in Group A"]
    #[inline(always)]
    pub fn msk(&self) -> MskR {
        MskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the mask enable set for errors in Group A"]
    #[inline(always)]
    #[must_use]
    pub fn msk(&mut self) -> MskW<CfgIntrEnSetSpec> {
        MskW::new(self, 0)
    }
}
#[doc = "Level Error Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_intr_en_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_intr_en_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIntrEnSetSpec;
impl crate::RegisterSpec for CfgIntrEnSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_intr_en_set::R`](R) reader structure"]
impl crate::Readable for CfgIntrEnSetSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_intr_en_set::W`](W) writer structure"]
impl crate::Writable for CfgIntrEnSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_INTR_EN_SET to value 0"]
impl crate::Resettable for CfgIntrEnSetSpec {
    const RESET_VALUE: u32 = 0;
}
