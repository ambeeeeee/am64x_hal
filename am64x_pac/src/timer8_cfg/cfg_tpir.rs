#[doc = "Register `CFG_TPIR` reader"]
pub type R = crate::R<CfgTpirSpec>;
#[doc = "Register `CFG_TPIR` writer"]
pub type W = crate::W<CfgTpirSpec>;
#[doc = "Field `POSITIVE_INC_VALUE` reader - 31:0\\]
The value of the positive increment"]
pub type PositiveIncValueR = crate::FieldReader<u32>;
#[doc = "Field `POSITIVE_INC_VALUE` writer - 31:0\\]
The value of the positive increment"]
pub type PositiveIncValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the positive increment"]
    #[inline(always)]
    pub fn positive_inc_value(&self) -> PositiveIncValueR {
        PositiveIncValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the positive increment"]
    #[inline(always)]
    #[must_use]
    pub fn positive_inc_value(&mut self) -> PositiveIncValueW<CfgTpirSpec> {
        PositiveIncValueW::new(self, 0)
    }
}
#[doc = "This register is used for 1ms tick generation. The TPIR register holds the value of the positive increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tpir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tpir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTpirSpec;
impl crate::RegisterSpec for CfgTpirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tpir::R`](R) reader structure"]
impl crate::Readable for CfgTpirSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tpir::W`](W) writer structure"]
impl crate::Writable for CfgTpirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TPIR to value 0"]
impl crate::Resettable for CfgTpirSpec {
    const RESET_VALUE: u32 = 0;
}
