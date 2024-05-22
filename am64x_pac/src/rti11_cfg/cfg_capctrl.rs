#[doc = "Register `CFG_CAPCTRL` reader"]
pub type R = crate::R<CfgCapctrlSpec>;
#[doc = "Register `CFG_CAPCTRL` writer"]
pub type W = crate::W<CfgCapctrlSpec>;
#[doc = "Field `CAPCNTR0` reader - 0:0\\]
This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
pub type Capcntr0R = crate::BitReader;
#[doc = "Field `CAPCNTR0` writer - 0:0\\]
This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
pub type Capcntr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCNTR1` reader - 1:1\\]
This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
pub type Capcntr1R = crate::BitReader;
#[doc = "Field `CAPCNTR1` writer - 1:1\\]
This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
pub type Capcntr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
    #[inline(always)]
    pub fn capcntr0(&self) -> Capcntr0R {
        Capcntr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
    #[inline(always)]
    pub fn capcntr1(&self) -> Capcntr1R {
        Capcntr1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
    #[inline(always)]
    #[must_use]
    pub fn capcntr0(&mut self) -> Capcntr0W<CfgCapctrlSpec> {
        Capcntr0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
    #[inline(always)]
    #[must_use]
    pub fn capcntr1(&mut self) -> Capcntr1W<CfgCapctrlSpec> {
        Capcntr1W::new(self, 1)
    }
}
#[doc = "CFG_CAPCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_capctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_capctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCapctrlSpec;
impl crate::RegisterSpec for CfgCapctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_capctrl::R`](R) reader structure"]
impl crate::Readable for CfgCapctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_capctrl::W`](W) writer structure"]
impl crate::Writable for CfgCapctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CAPCTRL to value 0"]
impl crate::Resettable for CfgCapctrlSpec {
    const RESET_VALUE: u32 = 0;
}
