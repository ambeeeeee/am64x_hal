#[doc = "Register `REGS_aggr_status_set` reader"]
pub type R = crate::R<RegsAggrStatusSetSpec>;
#[doc = "Register `REGS_aggr_status_set` writer"]
pub type W = crate::W<RegsAggrStatusSetSpec>;
#[doc = "Field `PARITY` reader - 1:0\\]
interrupt status set for parity errors"]
pub type ParityR = crate::FieldReader;
#[doc = "Field `PARITY` writer - 1:0\\]
interrupt status set for parity errors"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMEOUT` reader - 3:2\\]
interrupt status set for svbus timeout errors"]
pub type TimeoutR = crate::FieldReader;
#[doc = "Field `TIMEOUT` writer - 3:2\\]
interrupt status set for svbus timeout errors"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status set for parity errors"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status set for svbus timeout errors"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status set for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<RegsAggrStatusSetSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status set for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<RegsAggrStatusSetSpec> {
        TimeoutW::new(self, 2)
    }
}
#[doc = "AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_aggr_status_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_aggr_status_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsAggrStatusSetSpec;
impl crate::RegisterSpec for RegsAggrStatusSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_aggr_status_set::R`](R) reader structure"]
impl crate::Readable for RegsAggrStatusSetSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_aggr_status_set::W`](W) writer structure"]
impl crate::Writable for RegsAggrStatusSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_aggr_status_set to value 0"]
impl crate::Resettable for RegsAggrStatusSetSpec {
    const RESET_VALUE: u32 = 0;
}
