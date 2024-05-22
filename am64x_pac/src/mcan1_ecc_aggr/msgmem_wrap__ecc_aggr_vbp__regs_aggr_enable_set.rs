#[doc = "Register `MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_set` reader"]
pub type R = crate::R<MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec>;
#[doc = "Register `MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_set` writer"]
pub type W = crate::W<MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec>;
#[doc = "Field `PARITY` reader - 0:0\\]
interrupt enable set for parity errors"]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - 0:0\\]
interrupt enable set for parity errors"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - 1:1\\]
interrupt enable set for svbus timeout errors"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 1:1\\]
interrupt enable set for svbus timeout errors"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable set for parity errors"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable set for svbus timeout errors"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable set for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable set for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec> {
        TimeoutW::new(self, 1)
    }
}
#[doc = "AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec;
impl crate::RegisterSpec for MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set::R`](R) reader structure"]
impl crate::Readable for MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set::W`](W) writer structure"]
impl crate::Writable for MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_set to value 0"]
impl crate::Resettable for MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
