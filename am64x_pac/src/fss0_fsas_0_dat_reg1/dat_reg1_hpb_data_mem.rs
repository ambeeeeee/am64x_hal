#[doc = "Register `DAT_REG1_hpb_data_mem` reader"]
pub type R = crate::R<DatReg1HpbDataMemSpec>;
#[doc = "Register `DAT_REG1_hpb_data_mem` writer"]
pub type W = crate::W<DatReg1HpbDataMemSpec>;
#[doc = "Field `HPB_DATA` reader - 31:0\\]
FSAS data region1"]
pub type HpbDataR = crate::FieldReader<u32>;
#[doc = "Field `HPB_DATA` writer - 31:0\\]
FSAS data region1"]
pub type HpbDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
FSAS data region1"]
    #[inline(always)]
    pub fn hpb_data(&self) -> HpbDataR {
        HpbDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
FSAS data region1"]
    #[inline(always)]
    #[must_use]
    pub fn hpb_data(&mut self) -> HpbDataW<DatReg1HpbDataMemSpec> {
        HpbDataW::new(self, 0)
    }
}
#[doc = "FSAS boot data region1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_reg1_hpb_data_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat_reg1_hpb_data_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatReg1HpbDataMemSpec;
impl crate::RegisterSpec for DatReg1HpbDataMemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat_reg1_hpb_data_mem::R`](R) reader structure"]
impl crate::Readable for DatReg1HpbDataMemSpec {}
#[doc = "`write(|w| ..)` method takes [`dat_reg1_hpb_data_mem::W`](W) writer structure"]
impl crate::Writable for DatReg1HpbDataMemSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAT_REG1_hpb_data_mem to value 0"]
impl crate::Resettable for DatReg1HpbDataMemSpec {
    const RESET_VALUE: u32 = 0;
}
