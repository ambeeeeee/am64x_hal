#[doc = "Register `APBADDR_ETM_CPU0_TRCRSCTLR9` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcrsctlr9Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCRSCTLR9` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcrsctlr9Spec>;
#[doc = "Field `SELECT` reader - 15:0\\]
Selects one or more resources from the group that the GROUP field selects. Each bit represents a resource from the selected group.See the GROUP field description for details."]
pub type SelectR = crate::FieldReader<u16>;
#[doc = "Field `SELECT` writer - 15:0\\]
Selects one or more resources from the group that the GROUP field selects. Each bit represents a resource from the selected group.See the GROUP field description for details."]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GROUP` reader - 19:16\\]
Selects a group of resources. Possible values are: 0000 For SELECT bits 0 to 3, selects external input selector 0 to 3; other bits are reserved. 0001 For SELECT bits 0 to 7, selects processor comparator inputs 0 to 7; other bits are reserved. 0010 For SELECT bits 0 to 3, selects counter at zero 0 to 3; for SELECT bits 4 to 7, selects sequencer states 0 to 3; other bits are reserved. 0011 For SELECT bits 0 to 7, selects single-shot comparator control 0 to 7; other bits are reserved. 0100 For SELECT bits 0 to 15, selects single address comparator 0 to 15. 0101 For SELECT bits 0 to 7, selects address range comparator 0 to 7; other bits are reserved. 0110 For SELECT bits 0 to 7, selects Context ID comparator 0 to 7; other bits are reserved. 0111 For SELECT bits 0 to 7, selects VMID comparator 0 to 7; other bits are reserved. All other values are reserved."]
pub type GroupR = crate::FieldReader;
#[doc = "Field `GROUP` writer - 19:16\\]
Selects a group of resources. Possible values are: 0000 For SELECT bits 0 to 3, selects external input selector 0 to 3; other bits are reserved. 0001 For SELECT bits 0 to 7, selects processor comparator inputs 0 to 7; other bits are reserved. 0010 For SELECT bits 0 to 3, selects counter at zero 0 to 3; for SELECT bits 4 to 7, selects sequencer states 0 to 3; other bits are reserved. 0011 For SELECT bits 0 to 7, selects single-shot comparator control 0 to 7; other bits are reserved. 0100 For SELECT bits 0 to 15, selects single address comparator 0 to 15. 0101 For SELECT bits 0 to 7, selects address range comparator 0 to 7; other bits are reserved. 0110 For SELECT bits 0 to 7, selects Context ID comparator 0 to 7; other bits are reserved. 0111 For SELECT bits 0 to 7, selects VMID comparator 0 to 7; other bits are reserved. All other values are reserved."]
pub type GroupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INV` reader - 20:20\\]
Controls whether the resource that GROUP and SELECT selects is inverted: 0 The selected resource is not inverted. 1 The selected resource is inverted."]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - 20:20\\]
Controls whether the resource that GROUP and SELECT selects is inverted: 0 The selected resource is not inverted. 1 The selected resource is inverted."]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIRINV` reader - 21:21\\]
If n is an even number, controls whether the combined result from a resource pair is inverted: 0 The combined result is not inverted. 1 The combined result is inverted. If n is an odd number, this field is RES0."]
pub type PairinvR = crate::BitReader;
#[doc = "Field `PAIRINV` writer - 21:21\\]
If n is an even number, controls whether the combined result from a resource pair is inverted: 0 The combined result is not inverted. 1 The combined result is inverted. If n is an odd number, this field is RES0."]
pub type PairinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCRSCTLR9_31_22` reader - 31:22\\]
Reserved, RES0."]
pub type Res0Trcrsctlr9_31_22R = crate::FieldReader<u16>;
#[doc = "Field `RES0_TRCRSCTLR9_31_22` writer - 31:22\\]
Reserved, RES0."]
pub type Res0Trcrsctlr9_31_22W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Selects one or more resources from the group that the GROUP field selects. Each bit represents a resource from the selected group.See the GROUP field description for details."]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selects a group of resources. Possible values are: 0000 For SELECT bits 0 to 3, selects external input selector 0 to 3; other bits are reserved. 0001 For SELECT bits 0 to 7, selects processor comparator inputs 0 to 7; other bits are reserved. 0010 For SELECT bits 0 to 3, selects counter at zero 0 to 3; for SELECT bits 4 to 7, selects sequencer states 0 to 3; other bits are reserved. 0011 For SELECT bits 0 to 7, selects single-shot comparator control 0 to 7; other bits are reserved. 0100 For SELECT bits 0 to 15, selects single address comparator 0 to 15. 0101 For SELECT bits 0 to 7, selects address range comparator 0 to 7; other bits are reserved. 0110 For SELECT bits 0 to 7, selects Context ID comparator 0 to 7; other bits are reserved. 0111 For SELECT bits 0 to 7, selects VMID comparator 0 to 7; other bits are reserved. All other values are reserved."]
    #[inline(always)]
    pub fn group(&self) -> GroupR {
        GroupR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls whether the resource that GROUP and SELECT selects is inverted: 0 The selected resource is not inverted. 1 The selected resource is inverted."]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
If n is an even number, controls whether the combined result from a resource pair is inverted: 0 The combined result is not inverted. 1 The combined result is inverted. If n is an odd number, this field is RES0."]
    #[inline(always)]
    pub fn pairinv(&self) -> PairinvR {
        PairinvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcrsctlr9_31_22(&self) -> Res0Trcrsctlr9_31_22R {
        Res0Trcrsctlr9_31_22R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Selects one or more resources from the group that the GROUP field selects. Each bit represents a resource from the selected group.See the GROUP field description for details."]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SelectW<ApbaddrEtmCpu0Trcrsctlr9Spec> {
        SelectW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selects a group of resources. Possible values are: 0000 For SELECT bits 0 to 3, selects external input selector 0 to 3; other bits are reserved. 0001 For SELECT bits 0 to 7, selects processor comparator inputs 0 to 7; other bits are reserved. 0010 For SELECT bits 0 to 3, selects counter at zero 0 to 3; for SELECT bits 4 to 7, selects sequencer states 0 to 3; other bits are reserved. 0011 For SELECT bits 0 to 7, selects single-shot comparator control 0 to 7; other bits are reserved. 0100 For SELECT bits 0 to 15, selects single address comparator 0 to 15. 0101 For SELECT bits 0 to 7, selects address range comparator 0 to 7; other bits are reserved. 0110 For SELECT bits 0 to 7, selects Context ID comparator 0 to 7; other bits are reserved. 0111 For SELECT bits 0 to 7, selects VMID comparator 0 to 7; other bits are reserved. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn group(&mut self) -> GroupW<ApbaddrEtmCpu0Trcrsctlr9Spec> {
        GroupW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls whether the resource that GROUP and SELECT selects is inverted: 0 The selected resource is not inverted. 1 The selected resource is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<ApbaddrEtmCpu0Trcrsctlr9Spec> {
        InvW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
If n is an even number, controls whether the combined result from a resource pair is inverted: 0 The combined result is not inverted. 1 The combined result is inverted. If n is an odd number, this field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn pairinv(&mut self) -> PairinvW<ApbaddrEtmCpu0Trcrsctlr9Spec> {
        PairinvW::new(self, 21)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcrsctlr9_31_22(&mut self) -> Res0Trcrsctlr9_31_22W<ApbaddrEtmCpu0Trcrsctlr9Spec> {
        Res0Trcrsctlr9_31_22W::new(self, 22)
    }
}
#[doc = "Resource Selection Control Registers 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcrsctlr9Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcrsctlr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcrsctlr9::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcrsctlr9Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcrsctlr9::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcrsctlr9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCRSCTLR9 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcrsctlr9Spec {
    const RESET_VALUE: u32 = 0;
}
