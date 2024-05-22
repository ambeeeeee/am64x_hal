#[doc = "Register `CFG_FRC0` reader"]
pub type R = crate::R<CfgFrc0Spec>;
#[doc = "Register `CFG_FRC0` writer"]
pub type W = crate::W<CfgFrc0Spec>;
#[doc = "Field `FRC0` reader - 31:0\\]
This registers holds the current value of the Free Running Counter 0 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0."]
pub type Frc0R = crate::FieldReader<u32>;
#[doc = "Field `FRC0` writer - 31:0\\]
This registers holds the current value of the Free Running Counter 0 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0."]
pub type Frc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the current value of the Free Running Counter 0 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0."]
    #[inline(always)]
    pub fn frc0(&self) -> Frc0R {
        Frc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the current value of the Free Running Counter 0 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0."]
    #[inline(always)]
    #[must_use]
    pub fn frc0(&mut self) -> Frc0W<CfgFrc0Spec> {
        Frc0W::new(self, 0)
    }
}
#[doc = "CFG_FRC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_frc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_frc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgFrc0Spec;
impl crate::RegisterSpec for CfgFrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_frc0::R`](R) reader structure"]
impl crate::Readable for CfgFrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_frc0::W`](W) writer structure"]
impl crate::Writable for CfgFrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_FRC0 to value 0"]
impl crate::Resettable for CfgFrc0Spec {
    const RESET_VALUE: u32 = 0;
}
