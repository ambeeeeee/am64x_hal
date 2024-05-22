#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_aggr_enable_set` reader"]
pub type R = crate::R<BorgEccAggr_Cfg_RegsAggrEnableSetSpec>;
#[doc = "Register `BORG_ECC_AGGR__CFG__REGS_aggr_enable_set` writer"]
pub type W = crate::W<BorgEccAggr_Cfg_RegsAggrEnableSetSpec>;
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
    pub fn parity(&mut self) -> ParityW<BorgEccAggr_Cfg_RegsAggrEnableSetSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable set for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<BorgEccAggr_Cfg_RegsAggrEnableSetSpec> {
        TimeoutW::new(self, 1)
    }
}
#[doc = "AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`borg_ecc_aggr__cfg__regs_aggr_enable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`borg_ecc_aggr__cfg__regs_aggr_enable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorgEccAggr_Cfg_RegsAggrEnableSetSpec;
impl crate::RegisterSpec for BorgEccAggr_Cfg_RegsAggrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`borg_ecc_aggr__cfg__regs_aggr_enable_set::R`](R) reader structure"]
impl crate::Readable for BorgEccAggr_Cfg_RegsAggrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`borg_ecc_aggr__cfg__regs_aggr_enable_set::W`](W) writer structure"]
impl crate::Writable for BorgEccAggr_Cfg_RegsAggrEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BORG_ECC_AGGR__CFG__REGS_aggr_enable_set to value 0"]
impl crate::Resettable for BorgEccAggr_Cfg_RegsAggrEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
