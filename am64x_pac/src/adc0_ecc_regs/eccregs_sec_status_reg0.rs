#[doc = "Register `ECCREGS_sec_status_reg0` reader"]
pub type R = crate::R<EccregsSecStatusReg0Spec>;
#[doc = "Register `ECCREGS_sec_status_reg0` writer"]
pub type W = crate::W<EccregsSecStatusReg0Spec>;
#[doc = "Field `RAM0_PEND` reader - 0:0\\]
Interrupt Pending Status for ram0_pend"]
pub type Ram0PendR = crate::BitReader;
#[doc = "Field `RAM0_PEND` writer - 0:0\\]
Interrupt Pending Status for ram0_pend"]
pub type Ram0PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1_PEND` reader - 1:1\\]
Interrupt Pending Status for ram1_pend"]
pub type Ram1PendR = crate::BitReader;
#[doc = "Field `RAM1_PEND` writer - 1:1\\]
Interrupt Pending Status for ram1_pend"]
pub type Ram1PendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for ram0_pend"]
    #[inline(always)]
    pub fn ram0_pend(&self) -> Ram0PendR {
        Ram0PendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ram1_pend"]
    #[inline(always)]
    pub fn ram1_pend(&self) -> Ram1PendR {
        Ram1PendR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for ram0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ram0_pend(&mut self) -> Ram0PendW<EccregsSecStatusReg0Spec> {
        Ram0PendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ram1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ram1_pend(&mut self) -> Ram1PendW<EccregsSecStatusReg0Spec> {
        Ram1PendW::new(self, 1)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_sec_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_sec_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccregsSecStatusReg0Spec;
impl crate::RegisterSpec for EccregsSecStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccregs_sec_status_reg0::R`](R) reader structure"]
impl crate::Readable for EccregsSecStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`eccregs_sec_status_reg0::W`](W) writer structure"]
impl crate::Writable for EccregsSecStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCREGS_sec_status_reg0 to value 0"]
impl crate::Resettable for EccregsSecStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
