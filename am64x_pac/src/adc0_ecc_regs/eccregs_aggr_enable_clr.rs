#[doc = "Register `ECCREGS_aggr_enable_clr` reader"]
pub type R = crate::R<EccregsAggrEnableClrSpec>;
#[doc = "Register `ECCREGS_aggr_enable_clr` writer"]
pub type W = crate::W<EccregsAggrEnableClrSpec>;
#[doc = "Field `PARITY` reader - 0:0\\]
interrupt enable clear for parity errors"]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - 0:0\\]
interrupt enable clear for parity errors"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - 1:1\\]
interrupt enable clear for svbus timeout errors"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 1:1\\]
interrupt enable clear for svbus timeout errors"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable clear for parity errors"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable clear for svbus timeout errors"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable clear for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<EccregsAggrEnableClrSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable clear for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<EccregsAggrEnableClrSpec> {
        TimeoutW::new(self, 1)
    }
}
#[doc = "AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_aggr_enable_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_aggr_enable_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccregsAggrEnableClrSpec;
impl crate::RegisterSpec for EccregsAggrEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccregs_aggr_enable_clr::R`](R) reader structure"]
impl crate::Readable for EccregsAggrEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`eccregs_aggr_enable_clr::W`](W) writer structure"]
impl crate::Writable for EccregsAggrEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCREGS_aggr_enable_clr to value 0"]
impl crate::Resettable for EccregsAggrEnableClrSpec {
    const RESET_VALUE: u32 = 0;
}
