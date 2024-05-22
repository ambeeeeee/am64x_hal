#[doc = "Register `CFG0_DDR16SS_PMCTRL` reader"]
pub type R = crate::R<Cfg0Ddr16ssPmctrlSpec>;
#[doc = "Register `CFG0_DDR16SS_PMCTRL` writer"]
pub type W = crate::W<Cfg0Ddr16ssPmctrlSpec>;
#[doc = "Field `DDR16SS_PMCTRL_DATA_RETENTION` reader - 3:0\\]
DDR16SS Retention:"]
pub type Ddr16ssPmctrlDataRetentionR = crate::FieldReader;
#[doc = "Field `DDR16SS_PMCTRL_DATA_RETENTION` writer - 3:0\\]
DDR16SS Retention:"]
pub type Ddr16ssPmctrlDataRetentionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
DDR16SS Retention:"]
    #[inline(always)]
    pub fn ddr16ss_pmctrl_data_retention(&self) -> Ddr16ssPmctrlDataRetentionR {
        Ddr16ssPmctrlDataRetentionR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
DDR16SS Retention:"]
    #[inline(always)]
    #[must_use]
    pub fn ddr16ss_pmctrl_data_retention(
        &mut self,
    ) -> Ddr16ssPmctrlDataRetentionW<Cfg0Ddr16ssPmctrlSpec> {
        Ddr16ssPmctrlDataRetentionW::new(self, 0)
    }
}
#[doc = "CFG0_DDR16SS_PMCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr16ss_pmctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr16ss_pmctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Ddr16ssPmctrlSpec;
impl crate::RegisterSpec for Cfg0Ddr16ssPmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_ddr16ss_pmctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Ddr16ssPmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_ddr16ss_pmctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Ddr16ssPmctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DDR16SS_PMCTRL to value 0"]
impl crate::Resettable for Cfg0Ddr16ssPmctrlSpec {
    const RESET_VALUE: u32 = 0;
}
