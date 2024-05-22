#[doc = "Register `CFG_TNIR` reader"]
pub type R = crate::R<CfgTnirSpec>;
#[doc = "Register `CFG_TNIR` writer"]
pub type W = crate::W<CfgTnirSpec>;
#[doc = "Field `NEGATIVE_INC_VALUE` reader - 31:0\\]
The value of the negative increment"]
pub type NegativeIncValueR = crate::FieldReader<u32>;
#[doc = "Field `NEGATIVE_INC_VALUE` writer - 31:0\\]
The value of the negative increment"]
pub type NegativeIncValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the negative increment"]
    #[inline(always)]
    pub fn negative_inc_value(&self) -> NegativeIncValueR {
        NegativeIncValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The value of the negative increment"]
    #[inline(always)]
    #[must_use]
    pub fn negative_inc_value(&mut self) -> NegativeIncValueW<CfgTnirSpec> {
        NegativeIncValueW::new(self, 0)
    }
}
#[doc = "This register is used for 1ms tick generation. The TNIR register holds the value of the negative increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tnir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tnir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTnirSpec;
impl crate::RegisterSpec for CfgTnirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tnir::R`](R) reader structure"]
impl crate::Readable for CfgTnirSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tnir::W`](W) writer structure"]
impl crate::Writable for CfgTnirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TNIR to value 0"]
impl crate::Resettable for CfgTnirSpec {
    const RESET_VALUE: u32 = 0;
}
