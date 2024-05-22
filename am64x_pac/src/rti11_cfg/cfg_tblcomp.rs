#[doc = "Register `CFG_TBLCOMP` reader"]
pub type R = crate::R<CfgTblcompSpec>;
#[doc = "Register `CFG_TBLCOMP` writer"]
pub type W = crate::W<CfgTblcompSpec>;
#[doc = "Field `TBLCOMP` reader - 31:0\\]
This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
pub type TblcompR = crate::FieldReader<u32>;
#[doc = "Field `TBLCOMP` writer - 31:0\\]
This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
pub type TblcompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
    #[inline(always)]
    pub fn tblcomp(&self) -> TblcompR {
        TblcompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
    #[inline(always)]
    #[must_use]
    pub fn tblcomp(&mut self) -> TblcompW<CfgTblcompSpec> {
        TblcompW::new(self, 0)
    }
}
#[doc = "CFG_TBLCOMP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tblcomp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tblcomp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTblcompSpec;
impl crate::RegisterSpec for CfgTblcompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tblcomp::R`](R) reader structure"]
impl crate::Readable for CfgTblcompSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tblcomp::W`](W) writer structure"]
impl crate::Writable for CfgTblcompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TBLCOMP to value 0"]
impl crate::Resettable for CfgTblcompSpec {
    const RESET_VALUE: u32 = 0;
}
