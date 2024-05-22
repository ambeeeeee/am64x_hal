#[doc = "Register `CFG_PIN_EN_CLR` reader"]
pub type R = crate::R<CfgPinEnClrSpec>;
#[doc = "Register `CFG_PIN_EN_CLR` writer"]
pub type W = crate::W<CfgPinEnClrSpec>;
#[doc = "Field `MSK` reader - 31:0\\]
This is the error pin influence enable clear for errors in Group A"]
pub type MskR = crate::FieldReader<u32>;
#[doc = "Field `MSK` writer - 31:0\\]
This is the error pin influence enable clear for errors in Group A"]
pub type MskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the error pin influence enable clear for errors in Group A"]
    #[inline(always)]
    pub fn msk(&self) -> MskR {
        MskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the error pin influence enable clear for errors in Group A"]
    #[inline(always)]
    #[must_use]
    pub fn msk(&mut self) -> MskW<CfgPinEnClrSpec> {
        MskW::new(self, 0)
    }
}
#[doc = "Level Error Interrupt Enabled Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_en_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_en_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPinEnClrSpec;
impl crate::RegisterSpec for CfgPinEnClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pin_en_clr::R`](R) reader structure"]
impl crate::Readable for CfgPinEnClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pin_en_clr::W`](W) writer structure"]
impl crate::Writable for CfgPinEnClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_PIN_EN_CLR to value 0"]
impl crate::Resettable for CfgPinEnClrSpec {
    const RESET_VALUE: u32 = 0;
}
