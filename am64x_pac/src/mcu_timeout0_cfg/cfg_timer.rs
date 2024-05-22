#[doc = "Register `CFG_TIMER` reader"]
pub type R = crate::R<CfgTimerSpec>;
#[doc = "Register `CFG_TIMER` writer"]
pub type W = crate::W<CfgTimerSpec>;
#[doc = "Field `CNTR` reader - 29:0\\]
Current value of the free-running timer"]
pub type CntrR = crate::FieldReader<u32>;
#[doc = "Field `CNTR` writer - 29:0\\]
Current value of the free-running timer"]
pub type CntrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `EON` reader - 31:30\\]
Current eon"]
pub type EonR = crate::FieldReader;
#[doc = "Field `EON` writer - 31:30\\]
Current eon"]
pub type EonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:29 - 29:0\\]
Current value of the free-running timer"]
    #[inline(always)]
    pub fn cntr(&self) -> CntrR {
        CntrR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Current eon"]
    #[inline(always)]
    pub fn eon(&self) -> EonR {
        EonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29 - 29:0\\]
Current value of the free-running timer"]
    #[inline(always)]
    #[must_use]
    pub fn cntr(&mut self) -> CntrW<CfgTimerSpec> {
        CntrW::new(self, 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Current eon"]
    #[inline(always)]
    #[must_use]
    pub fn eon(&mut self) -> EonW<CfgTimerSpec> {
        EonW::new(self, 30)
    }
}
#[doc = "The Timer Register contains the current value for free-running timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTimerSpec;
impl crate::RegisterSpec for CfgTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_timer::R`](R) reader structure"]
impl crate::Readable for CfgTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_timer::W`](W) writer structure"]
impl crate::Writable for CfgTimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TIMER to value 0"]
impl crate::Resettable for CfgTimerSpec {
    const RESET_VALUE: u32 = 0;
}
