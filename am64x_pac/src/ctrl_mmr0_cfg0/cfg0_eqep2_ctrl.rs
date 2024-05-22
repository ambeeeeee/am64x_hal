#[doc = "Register `CFG0_EQEP2_CTRL` reader"]
pub type R = crate::R<Cfg0Eqep2CtrlSpec>;
#[doc = "Register `CFG0_EQEP2_CTRL` writer"]
pub type W = crate::W<Cfg0Eqep2CtrlSpec>;
#[doc = "Field `EQEP2_CTRL_SOCA_SEL` reader - 4:0\\]
Selects the source of SOCA input for EQEP2"]
pub type Eqep2CtrlSocaSelR = crate::FieldReader;
#[doc = "Field `EQEP2_CTRL_SOCA_SEL` writer - 4:0\\]
Selects the source of SOCA input for EQEP2"]
pub type Eqep2CtrlSocaSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of SOCA input for EQEP2"]
    #[inline(always)]
    pub fn eqep2_ctrl_soca_sel(&self) -> Eqep2CtrlSocaSelR {
        Eqep2CtrlSocaSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of SOCA input for EQEP2"]
    #[inline(always)]
    #[must_use]
    pub fn eqep2_ctrl_soca_sel(&mut self) -> Eqep2CtrlSocaSelW<Cfg0Eqep2CtrlSpec> {
        Eqep2CtrlSocaSelW::new(self, 0)
    }
}
#[doc = "CFG0_EQEP2_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep2_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep2_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Eqep2CtrlSpec;
impl crate::RegisterSpec for Cfg0Eqep2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_eqep2_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Eqep2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_eqep2_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Eqep2CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EQEP2_CTRL to value 0"]
impl crate::Resettable for Cfg0Eqep2CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
