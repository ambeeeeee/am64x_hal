#[doc = "Register `CFG_TTGR` reader"]
pub type R = crate::R<CfgTtgrSpec>;
#[doc = "Register `CFG_TTGR` writer"]
pub type W = crate::W<CfgTtgrSpec>;
#[doc = "Field `TTRG_VLAUE` reader - 31:0\\]
The value of the trigger register. During reads, it always return 0xFFFF_FFFF"]
pub type TtrgVlaueR = crate::FieldReader<u32>;
#[doc = "Field `TTRG_VLAUE` writer - 31:0\\]
The value of the trigger register. During reads, it always return 0xFFFF_FFFF"]
pub type TtrgVlaueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the trigger register. During reads, it always return 0xFFFF_FFFF"]
    #[inline(always)]
    pub fn ttrg_vlaue(&self) -> TtrgVlaueR {
        TtrgVlaueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the trigger register. During reads, it always return 0xFFFF_FFFF"]
    #[inline(always)]
    #[must_use]
    pub fn ttrg_vlaue(&mut self) -> TtrgVlaueW<CfgTtgrSpec> {
        TtrgVlaueW::new(self, 0)
    }
}
#[doc = "This register triggers a counter reload of timer by writing any value in it\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ttgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ttgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTtgrSpec;
impl crate::RegisterSpec for CfgTtgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_ttgr::R`](R) reader structure"]
impl crate::Readable for CfgTtgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_ttgr::W`](W) writer structure"]
impl crate::Writable for CfgTtgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TTGR to value 0"]
impl crate::Resettable for CfgTtgrSpec {
    const RESET_VALUE: u32 = 0;
}
