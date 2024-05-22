#[doc = "Register `CFG_TCVR` reader"]
pub type R = crate::R<CfgTcvrSpec>;
#[doc = "Register `CFG_TCVR` writer"]
pub type W = crate::W<CfgTcvrSpec>;
#[doc = "Field `COUNTER_VALUE` reader - 31:0\\]
The value of CVR counter"]
pub type CounterValueR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_VALUE` writer - 31:0\\]
The value of CVR counter"]
pub type CounterValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The value of CVR counter"]
    #[inline(always)]
    pub fn counter_value(&self) -> CounterValueR {
        CounterValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The value of CVR counter"]
    #[inline(always)]
    #[must_use]
    pub fn counter_value(&mut self) -> CounterValueW<CfgTcvrSpec> {
        CounterValueW::new(self, 0)
    }
}
#[doc = "This register is used for 1ms tick generation. The TCVR register defines whether next value loaded in TCRR will be the sub-period value or the over-period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTcvrSpec;
impl crate::RegisterSpec for CfgTcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tcvr::R`](R) reader structure"]
impl crate::Readable for CfgTcvrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tcvr::W`](W) writer structure"]
impl crate::Writable for CfgTcvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TCVR to value 0"]
impl crate::Resettable for CfgTcvrSpec {
    const RESET_VALUE: u32 = 0;
}
