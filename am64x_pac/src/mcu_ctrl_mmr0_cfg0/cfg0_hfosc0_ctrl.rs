#[doc = "Register `CFG0_HFOSC0_CTRL` reader"]
pub type R = crate::R<Cfg0Hfosc0CtrlSpec>;
#[doc = "Register `CFG0_HFOSC0_CTRL` writer"]
pub type W = crate::W<Cfg0Hfosc0CtrlSpec>;
#[doc = "Field `HFOSC0_CTRL_BP_C` reader - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
pub type Hfosc0CtrlBpCR = crate::BitReader;
#[doc = "Field `HFOSC0_CTRL_BP_C` writer - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
pub type Hfosc0CtrlBpCW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
    #[inline(always)]
    pub fn hfosc0_ctrl_bp_c(&self) -> Hfosc0CtrlBpCR {
        Hfosc0CtrlBpCR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_ctrl_bp_c(&mut self) -> Hfosc0CtrlBpCW<Cfg0Hfosc0CtrlSpec> {
        Hfosc0CtrlBpCW::new(self, 4)
    }
}
#[doc = "CFG0_HFOSC0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Hfosc0CtrlSpec;
impl crate::RegisterSpec for Cfg0Hfosc0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_hfosc0_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Hfosc0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_hfosc0_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Hfosc0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_HFOSC0_CTRL to value 0"]
impl crate::Resettable for Cfg0Hfosc0CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
