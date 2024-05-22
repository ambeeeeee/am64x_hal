#[doc = "Register `CFG_UC0` reader"]
pub type R = crate::R<CfgUc0Spec>;
#[doc = "Register `CFG_UC0` writer"]
pub type W = crate::W<CfgUc0Spec>;
#[doc = "Field `UC0` reader - 31:0\\]
This registers holds the current value of the Up Counter 0 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 0. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 0 and Free Running Counter 0. User and privilege mode (read): value of the counter when the Free Running Counter 0 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC0 then it can take a long time until a compare matches, since RTIUC0 has to count up until it overflows."]
pub type Uc0R = crate::FieldReader<u32>;
#[doc = "Field `UC0` writer - 31:0\\]
This registers holds the current value of the Up Counter 0 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 0. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 0 and Free Running Counter 0. User and privilege mode (read): value of the counter when the Free Running Counter 0 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC0 then it can take a long time until a compare matches, since RTIUC0 has to count up until it overflows."]
pub type Uc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the current value of the Up Counter 0 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 0. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 0 and Free Running Counter 0. User and privilege mode (read): value of the counter when the Free Running Counter 0 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC0 then it can take a long time until a compare matches, since RTIUC0 has to count up until it overflows."]
    #[inline(always)]
    pub fn uc0(&self) -> Uc0R {
        Uc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the current value of the Up Counter 0 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 0. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 0 and Free Running Counter 0. User and privilege mode (read): value of the counter when the Free Running Counter 0 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC0 and RTIFRC0. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC0 then it can take a long time until a compare matches, since RTIUC0 has to count up until it overflows."]
    #[inline(always)]
    #[must_use]
    pub fn uc0(&mut self) -> Uc0W<CfgUc0Spec> {
        Uc0W::new(self, 0)
    }
}
#[doc = "CFG_UC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_uc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_uc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgUc0Spec;
impl crate::RegisterSpec for CfgUc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_uc0::R`](R) reader structure"]
impl crate::Readable for CfgUc0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_uc0::W`](W) writer structure"]
impl crate::Writable for CfgUc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_UC0 to value 0"]
impl crate::Resettable for CfgUc0Spec {
    const RESET_VALUE: u32 = 0;
}
