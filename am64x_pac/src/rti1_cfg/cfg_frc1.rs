#[doc = "Register `CFG_FRC1` reader"]
pub type R = crate::R<CfgFrc1Spec>;
#[doc = "Register `CFG_FRC1` writer"]
pub type W = crate::W<CfgFrc1Spec>;
#[doc = "Field `FRC1` reader - 31:0\\]
This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between UC1 and FRC1."]
pub type Frc1R = crate::FieldReader<u32>;
#[doc = "Field `FRC1` writer - 31:0\\]
This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between UC1 and FRC1."]
pub type Frc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between UC1 and FRC1."]
    #[inline(always)]
    pub fn frc1(&self) -> Frc1R {
        Frc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between UC1 and FRC1."]
    #[inline(always)]
    #[must_use]
    pub fn frc1(&mut self) -> Frc1W<CfgFrc1Spec> {
        Frc1W::new(self, 0)
    }
}
#[doc = "CFG_FRC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_frc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_frc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgFrc1Spec;
impl crate::RegisterSpec for CfgFrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_frc1::R`](R) reader structure"]
impl crate::Readable for CfgFrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_frc1::W`](W) writer structure"]
impl crate::Writable for CfgFrc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_FRC1 to value 0"]
impl crate::Resettable for CfgFrc1Spec {
    const RESET_VALUE: u32 = 0;
}
