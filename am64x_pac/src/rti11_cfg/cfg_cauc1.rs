#[doc = "Register `CFG_CAUC1` reader"]
pub type R = crate::R<CfgCauc1Spec>;
#[doc = "Register `CFG_CAUC1` writer"]
pub type W = crate::W<CfgCauc1Spec>;
#[doc = "Field `CAUC1` reader - 31:0\\]
This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
pub type Cauc1R = crate::FieldReader<u32>;
#[doc = "Field `CAUC1` writer - 31:0\\]
This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
pub type Cauc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
    #[inline(always)]
    pub fn cauc1(&self) -> Cauc1R {
        Cauc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
    #[inline(always)]
    #[must_use]
    pub fn cauc1(&mut self) -> Cauc1W<CfgCauc1Spec> {
        Cauc1W::new(self, 0)
    }
}
#[doc = "CFG_CAUC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cauc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cauc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCauc1Spec;
impl crate::RegisterSpec for CfgCauc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cauc1::R`](R) reader structure"]
impl crate::Readable for CfgCauc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cauc1::W`](W) writer structure"]
impl crate::Writable for CfgCauc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CAUC1 to value 0"]
impl crate::Resettable for CfgCauc1Spec {
    const RESET_VALUE: u32 = 0;
}
