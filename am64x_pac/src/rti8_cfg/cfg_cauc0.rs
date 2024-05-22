#[doc = "Register `CFG_CAUC0` reader"]
pub type R = crate::R<CfgCauc0Spec>;
#[doc = "Register `CFG_CAUC0` writer"]
pub type W = crate::W<CfgCauc0Spec>;
#[doc = "Field `CAUC0` reader - 31:0\\]
This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
pub type Cauc0R = crate::FieldReader<u32>;
#[doc = "Field `CAUC0` writer - 31:0\\]
This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
pub type Cauc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
    #[inline(always)]
    pub fn cauc0(&self) -> Cauc0R {
        Cauc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
    #[inline(always)]
    #[must_use]
    pub fn cauc0(&mut self) -> Cauc0W<CfgCauc0Spec> {
        Cauc0W::new(self, 0)
    }
}
#[doc = "CFG_CAUC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cauc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cauc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCauc0Spec;
impl crate::RegisterSpec for CfgCauc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cauc0::R`](R) reader structure"]
impl crate::Readable for CfgCauc0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cauc0::W`](W) writer structure"]
impl crate::Writable for CfgCauc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CAUC0 to value 0"]
impl crate::Resettable for CfgCauc0Spec {
    const RESET_VALUE: u32 = 0;
}
