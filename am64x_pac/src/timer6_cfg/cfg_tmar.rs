#[doc = "Register `CFG_TMAR` reader"]
pub type R = crate::R<CfgTmarSpec>;
#[doc = "Register `CFG_TMAR` writer"]
pub type W = crate::W<CfgTmarSpec>;
#[doc = "Field `COMPARE_VLAUE` reader - 31:0\\]
The value of the match register"]
pub type CompareVlaueR = crate::FieldReader<u32>;
#[doc = "Field `COMPARE_VLAUE` writer - 31:0\\]
The value of the match register"]
pub type CompareVlaueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the match register"]
    #[inline(always)]
    pub fn compare_vlaue(&self) -> CompareVlaueR {
        CompareVlaueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the match register"]
    #[inline(always)]
    #[must_use]
    pub fn compare_vlaue(&mut self) -> CompareVlaueW<CfgTmarSpec> {
        CompareVlaueW::new(self, 0)
    }
}
#[doc = "This register holds the match value to be compared with the counter's value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTmarSpec;
impl crate::RegisterSpec for CfgTmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tmar::R`](R) reader structure"]
impl crate::Readable for CfgTmarSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tmar::W`](W) writer structure"]
impl crate::Writable for CfgTmarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TMAR to value 0"]
impl crate::Resettable for CfgTmarSpec {
    const RESET_VALUE: u32 = 0;
}
