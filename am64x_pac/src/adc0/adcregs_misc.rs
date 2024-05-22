#[doc = "Register `ADCREGS_MISC` reader"]
pub type R = crate::R<AdcregsMiscSpec>;
#[doc = "Register `ADCREGS_MISC` writer"]
pub type W = crate::W<AdcregsMiscSpec>;
#[doc = "Field `AFE_SPARE_IN` reader - 3:0\\]
Spare inputs to AFE"]
pub type AfeSpareInR = crate::FieldReader;
#[doc = "Field `AFE_SPARE_IN` writer - 3:0\\]
Spare inputs to AFE"]
pub type AfeSpareInW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFE_SPARE_OUT` reader - 11:8\\]
Spare outputs from AFE"]
pub type AfeSpareOutR = crate::FieldReader;
#[doc = "Field `AFE_SPARE_OUT` writer - 11:8\\]
Spare outputs from AFE"]
pub type AfeSpareOutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Spare inputs to AFE"]
    #[inline(always)]
    pub fn afe_spare_in(&self) -> AfeSpareInR {
        AfeSpareInR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Spare outputs from AFE"]
    #[inline(always)]
    pub fn afe_spare_out(&self) -> AfeSpareOutR {
        AfeSpareOutR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Spare inputs to AFE"]
    #[inline(always)]
    #[must_use]
    pub fn afe_spare_in(&mut self) -> AfeSpareInW<AdcregsMiscSpec> {
        AfeSpareInW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Spare outputs from AFE"]
    #[inline(always)]
    #[must_use]
    pub fn afe_spare_out(&mut self) -> AfeSpareOutW<AdcregsMiscSpec> {
        AfeSpareOutW::new(self, 8)
    }
}
#[doc = "Spare inputs of the AFE are driven by this register, spare outputs from the AFE are read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsMiscSpec;
impl crate::RegisterSpec for AdcregsMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_misc::R`](R) reader structure"]
impl crate::Readable for AdcregsMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_misc::W`](W) writer structure"]
impl crate::Writable for AdcregsMiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_MISC to value 0"]
impl crate::Resettable for AdcregsMiscSpec {
    const RESET_VALUE: u32 = 0;
}
